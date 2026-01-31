/**
 * E2E tests for search and navigation
 * Spec section 12.3: "Search and navigate"
 */
import { test, expect } from './fixtures';

test.describe('Search and Navigation', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('text=Chronicle');
  });

  test('shows search tab', async ({ page }) => {
    // Click search tab
    await page.getByRole('button', { name: 'Search' }).click();
    
    // Search input should be visible
    await expect(page.getByPlaceholder('Search notes...')).toBeVisible();
  });

  test('searches notes by content', async ({ page }) => {
    // Go to search tab
    await page.getByRole('button', { name: 'Search' }).click();
    
    // Type search query
    await page.getByPlaceholder('Search notes...').fill('welcome');
    await page.getByPlaceholder('Search notes...').press('Enter');
    
    // Should show search results - use first() to avoid multiple matches
    await expect(page.getByText('Welcome').first()).toBeVisible();
  });

  test('opens note from search results', async ({ page }) => {
    // Search
    await page.getByRole('button', { name: 'Search' }).click();
    await page.getByPlaceholder('Search notes...').fill('link target');
    await page.getByPlaceholder('Search notes...').press('Enter');
    
    // Click first result matching "Link Target"
    await page.getByText('Link Target').first().click();
    
    // Editor should show the note
    await expect(page.locator('.cm-editor')).toBeVisible();
  });

  test('navigates between notes', async ({ page }) => {
    // Open first note
    await page.getByRole('button', { name: 'Welcome' }).click();
    await expect(page.locator('.cm-editor')).toBeVisible();
    
    // Open second note
    await page.getByRole('button', { name: 'Link Target' }).click();
    
    // Should now show second note
    // The file tree button for Link Target should be selected (has bg-blue class)
    const linkTargetBtn = page.getByRole('button', { name: 'Link Target' }).first();
    await expect(linkTargetBtn).toHaveClass(/bg-blue/);
  });

  test('shows tags tab', async ({ page }) => {
    // Click tags tab
    await page.getByRole('button', { name: 'Tags' }).click();
    
    // Should show tag list
    await expect(page.getByText('#test')).toBeVisible();
    await expect(page.getByText('#welcome')).toBeVisible();
  });

  test('expands tag to show notes', async ({ page }) => {
    // Go to tags tab
    await page.getByRole('button', { name: 'Tags' }).click();
    
    // Click on a tag to expand (partial match for #test with count)
    await page.locator('button:has-text("#test")').first().click();
    
    // Should show expanded list with notes - these are smaller buttons under the tag
    // Check that there's a nested list visible
    await expect(page.locator('.border-l.border-neutral-700')).toBeVisible();
  });

  test('filters file tree by tag', async ({ page }) => {
    // Go to tags tab
    await page.getByRole('button', { name: 'Tags' }).click();
    
    // Click filter button for 'welcome' tag
    await page.getByRole('button', { name: 'filter' }).first().click();
    
    // Should switch to files tab with filter active
    await expect(page.getByText('Filtered:')).toBeVisible();
    
    // Should show clear button
    await expect(page.getByRole('button', { name: 'Clear' })).toBeVisible();
  });

  test('clears tag filter', async ({ page }) => {
    // Apply filter
    await page.getByRole('button', { name: 'Tags' }).click();
    await page.getByRole('button', { name: 'filter' }).first().click();
    
    // Clear it
    await page.getByRole('button', { name: 'Clear' }).click();
    
    // Filter banner should be gone
    await expect(page.getByText('Filtered:')).not.toBeVisible();
    
    // All notes should be visible again
    await expect(page.getByRole('button', { name: 'Welcome' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Untagged Note' })).toBeVisible();
  });
});
