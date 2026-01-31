/**
 * E2E tests for graph view interaction
 * Spec section 12.3: "Graph view interaction"
 */
import { test, expect } from './fixtures';

test.describe('Graph View', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    // Wait for app to load - Chronicle header is always visible
    await page.waitForSelector('text=Chronicle');
  });

  test('navigates to graph view', async ({ page }) => {
    // Click graph link in navigation
    await page.getByRole('link', { name: 'Graph' }).click();
    
    // Should show graph view header
    await expect(page.getByRole('heading', { name: 'Knowledge Graph' })).toBeVisible();
  });

  test('displays node count', async ({ page }) => {
    await page.goto('/graph');
    
    // Should show node/link count
    await expect(page.getByText(/\d+ notes/)).toBeVisible();
    await expect(page.getByText(/\d+ links/)).toBeVisible();
  });

  test('shows tag filter dropdown', async ({ page }) => {
    await page.goto('/graph');
    
    // Should have filter dropdown
    await expect(page.getByText('Filter:')).toBeVisible();
    await expect(page.getByRole('combobox')).toBeVisible();
  });

  test('filters graph by tag', async ({ page }) => {
    await page.goto('/graph');
    
    // Select a tag filter
    await page.getByRole('combobox').selectOption('test');
    
    // Node count should change (only notes with 'test' tag)
    await expect(page.getByText('2 notes')).toBeVisible();
  });

  test('clears tag filter', async ({ page }) => {
    await page.goto('/graph');
    
    // Apply filter
    await page.getByRole('combobox').selectOption('test');
    
    // Clear filter
    await page.getByRole('combobox').selectOption('');
    
    // Should show all notes again
    await expect(page.getByText('3 notes')).toBeVisible();
  });

  test('returns to editor on Escape', async ({ page }) => {
    await page.goto('/graph');
    
    // Press Escape
    await page.keyboard.press('Escape');
    
    // Should return to editor view
    await expect(page).toHaveURL('/');
  });

  test('shows SVG graph canvas', async ({ page }) => {
    await page.goto('/graph');
    
    // D3 renders to SVG
    await expect(page.locator('svg')).toBeVisible();
  });
});
