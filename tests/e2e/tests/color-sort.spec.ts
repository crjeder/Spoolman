import { test, expect } from '@playwright/test';

/** Parse "background:rgba(R,G,B,...)" or "background:rgb(R,G,B)" → {r,g,b} */
function parseSwatchRgb(style: string | null): { r: number; g: number; b: number } | null {
  if (!style) return null;
  const m = style.match(/rgba?\((\d+),\s*(\d+),\s*(\d+)/);
  if (!m) return null;
  return { r: parseInt(m[1]), g: parseInt(m[2]), b: parseInt(m[3]) };
}

/**
 * Set a color input value via JS and fire both 'input' and 'change' events
 * (needed because Playwright cannot fill <input type="color"> directly in
 * headless Chromium, and Leptos listens for 'change' to update the signal).
 */
async function setColorPickerValue(page: import('@playwright/test').Page, hex: string): Promise<void> {
  await page.evaluate((h: string) => {
    const input = document.querySelector('.color-popup input[type="color"]') as HTMLInputElement | null;
    if (!input) throw new Error('color picker input not found in DOM');
    input.value = h;
    input.dispatchEvent(new Event('input', { bubbles: true }));
    input.dispatchEvent(new Event('change', { bubbles: true }));
  }, hex);
}

/**
 * Open the color picker popup (if not already open), set the color level
 * selector, and set the color input value.  Closes the popup when done.
 */
async function activateColorSearch(
  page: import('@playwright/test').Page,
  level: 'fine' | 'medium' | 'coarse',
  hex: string,
): Promise<void> {
  // Level selector is always visible inside the color-head <th>
  await page.selectOption('.color-threshold-select', level);

  // The color input lives inside the popup that opens on .color-head-label click
  await page.click('.color-head-label');
  await page.locator('.color-popup').waitFor({ state: 'visible' });
  await setColorPickerValue(page, hex);

  // Dismiss the popup via the backdrop so it doesn't obscure the table
  await page.click('.color-backdrop');
  await page.locator('.color-popup').waitFor({ state: 'hidden' });

  // Small yield for Leptos reactivity to re-render the sorted table
  await page.waitForTimeout(300);
}

// ---------------------------------------------------------------------------

test.describe('Color delta sort', () => {
  /**
   * Fixture: default sort is registered desc.
   *   First row (registered newest): spool 2150 "Navy" rgb(0,0,128)  – NOT red.
   *   Pure-red spools (delta = 0 from #ff0000): 2008, 2024, 2048, 2077, 2081, 2097, …
   *
   * Test verifies:
   *   • Off  →  first row is navy-ish (not red)
   *   • Coarse + #ff0000  →  first row is red-ish (delta-sort kicked in)
   *   • Off again  →  first row reverts to navy-ish
   */
  test('activating color level re-sorts spools by ascending ΔE distance', async ({ page }) => {
    await page.goto('/spools');
    await page.waitForLoadState('networkidle');
    await page.locator('table.data-table').waitFor({ state: 'visible' });

    const firstSwatch = page
      .locator('table.data-table tbody tr')
      .first()
      .locator('.color-swatch')
      .first();

    // --- Default (Off): first row is spool 2150 Navy, rgb(0,0,128) ---
    const styleOff = await firstSwatch.getAttribute('style');
    const rgbOff = parseSwatchRgb(styleOff);
    expect(rgbOff, 'default first-row swatch should parse').not.toBeNull();
    // Navy is clearly not red
    expect(rgbOff!.r, 'default first row should not be red (it is Navy)').toBeLessThan(100);
    expect(rgbOff!.b, 'default first row should be blue-ish (Navy)').toBeGreaterThan(80);

    // --- Activate Coarse + pure red #ff0000 ---
    await activateColorSearch(page, 'coarse', '#ff0000');

    const styleRed = await firstSwatch.getAttribute('style');
    const rgbRed = parseSwatchRgb(styleRed);
    expect(rgbRed, 'first-row swatch after red sort should parse').not.toBeNull();
    // Pure-red spools have R=255, G=0, B=0 — R channel clearly dominates
    expect(rgbRed!.r, 'first swatch should be red-ish with red color active').toBeGreaterThan(180);
    expect(rgbRed!.b, 'first swatch B channel should be low for red sort result').toBeLessThan(80);

    // --- Restore Off: first row should revert to the original navy spool ---
    await page.selectOption('.color-threshold-select', 'off');
    await page.waitForTimeout(300);

    const styleRestored = await firstSwatch.getAttribute('style');
    const rgbRestored = parseSwatchRgb(styleRestored);
    expect(rgbRestored, 'first-row swatch after Off restore should parse').not.toBeNull();
    expect(rgbRestored!.r, 'first swatch R should be low again (navy) after Off').toBeLessThan(100);
    expect(rgbRestored!.b, 'first swatch should be blue-ish again after Off').toBeGreaterThan(80);
  });

  /**
   * Multi-color spool ranks by its CLOSEST color, not its first color.
   *
   * Fixture spool 2090 "Indigo Blue Silk":
   *   colors[0] = indigo  rgb(75,0,130)   — CIEDE2000 to #0000ff >> 35 (excluded by Coarse)
   *   colors[1] = blue    rgb(0,0,255)    — CIEDE2000 to #0000ff = 0
   *
   * With registered-desc sort, spool 2090 is at position 144 (page 6) — NOT on page 1.
   *
   * With Coarse + #0000ff:
   *   • The filter uses .any(), so spool 2090 passes (its blue second color is within threshold).
   *   • The sort uses min_delta, so spool 2090 gets rank 0 (delta = 0 from blue second color).
   *   • Therefore spool 2090 MUST appear on page 1.
   *
   * If only the first color were used for the sort key, spool 2090's key would be
   * delta(indigo, #0000ff) >> 0, pushing it far down the list.
   */
  test('multi-color spool ranks by its closest color, not its first color', async ({ page }) => {
    await page.goto('/spools');
    await page.waitForLoadState('networkidle');
    await page.locator('table.data-table').waitFor({ state: 'visible' });

    // Confirm spool 2090 is NOT on page 1 with default (registered-desc) sort.
    const getVisibleIds = async () =>
      page
        .locator('table.data-table tbody tr')
        .evaluateAll((trs: HTMLTableRowElement[]) =>
          trs.map(tr => tr.querySelector('td a')?.textContent?.trim() ?? ''),
        );

    const idsOff = await getVisibleIds();
    expect(idsOff, 'spool 2090 should not be on page 1 with registered-desc sort').not.toContain('2090');

    // Activate Coarse + pure blue
    await activateColorSearch(page, 'coarse', '#0000ff');

    // Spool 2090's second color is pure blue (delta = 0).
    // min_delta for 2090 = 0  →  it must be among the first rows on page 1.
    const idsBlue = await getVisibleIds();
    expect(
      idsBlue,
      'spool 2090 (Indigo Blue Silk) should appear on page 1 when sorted by blue delta — ' +
        'its second color (pure blue) gives it min_delta = 0',
    ).toContain('2090');
  });
});
