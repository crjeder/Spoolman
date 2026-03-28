import { test, expect } from '@playwright/test';

test.describe('Navigation', () => {
  test('Spools page loads', async ({ page }) => {
    await page.goto('/spools');
    await page.waitForLoadState('networkidle');
    await expect(page.locator('h1')).toContainText('Spools');
    await expect(page.locator('table.data-table')).toBeVisible();
  });

  test('Filaments page loads', async ({ page }) => {
    await page.goto('/filaments');
    await page.waitForLoadState('networkidle');
    await expect(page.locator('h1')).toContainText('Filaments');
    await expect(page.locator('table.data-table')).toBeVisible();
  });

  test('Locations page loads', async ({ page }) => {
    await page.goto('/locations');
    await page.waitForLoadState('networkidle');
    await expect(page.locator('h1')).toContainText('Locations');
    await expect(page.locator('table.data-table')).toBeVisible();
  });

  test('Sidebar nav links exist', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    const nav = page.locator('nav.sidebar ul.nav-links');
    await expect(nav.locator('a', { hasText: 'Spools' })).toBeVisible();
    await expect(nav.locator('a', { hasText: 'Filaments' })).toBeVisible();
    await expect(nav.locator('a', { hasText: 'Locations' })).toBeVisible();
  });

  test('Clicking Filaments nav link navigates to /filaments', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.click('nav.sidebar a[href="/filaments"]');
    await page.waitForURL('**/filaments');
    await expect(page.locator('h1')).toContainText('Filaments');
  });

  test('Clicking Locations nav link navigates to /locations', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.click('nav.sidebar a[href="/locations"]');
    await page.waitForURL('**/locations');
    await expect(page.locator('h1')).toContainText('Locations');
  });
});
