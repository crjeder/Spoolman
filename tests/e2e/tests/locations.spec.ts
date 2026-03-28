import { test, expect } from '@playwright/test';
import { LocationsPage } from '../pages';

test.describe('Locations', () => {
  test('fixture data is visible on load', async ({ page }) => {
    const locations = new LocationsPage(page);
    await locations.goto();
    const count = await locations.getLocationRows();
    expect(count).toBeGreaterThanOrEqual(1);
  });

  test('add location appears in list', async ({ page }) => {
    const locations = new LocationsPage(page);
    await locations.goto();
    const before = await locations.getLocationRows();

    await locations.addLocation('E2E Location');

    const after = await locations.getLocationRows();
    expect(after).toBe(before + 1);
  });

  test('delete location (with 0 spools) removes it from list', async ({ page }) => {
    const locations = new LocationsPage(page);
    await locations.goto();

    // Create a location with no spools so it can be deleted.
    const uniqueName = `E2E Delete ${Date.now()}`;
    await locations.addLocation(uniqueName);
    const before = await locations.getLocationRows();

    await locations.deleteLocation(uniqueName);

    const after = await locations.getLocationRows();
    expect(after).toBe(before - 1);
  });
});
