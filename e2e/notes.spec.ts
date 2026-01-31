/**
 * E2E tests for note creation and editing
 * Spec section 12.3: "Create and edit notes"
 */
import { test, expect } from './fixtures';

test.describe('Notes - Create and Edit', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    // Wait for vault to load
    await page.waitForSelector('text=Chronicle');
  });

  test('displays file tree with notes', async ({ page }) => {
    // Should show the mock notes in the file tree
    await expect(page.getByRole('button', { name: 'Welcome' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Link Target' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Untagged Note' })).toBeVisible();
  });

  test('opens a note when clicked', async ({ page }) => {
    // Click on a note in the file tree
    await page.getByRole('button', { name: 'Welcome' }).click();
    
    // Editor should show the note content
    await expect(page.locator('.cm-editor')).toBeVisible();
    
    // Backlinks panel heading should be visible
    await expect(page.getByRole('heading', { name: 'Backlinks' })).toBeVisible();
  });

  test('shows note metadata in panel', async ({ page }) => {
    // Open a note
    await page.getByRole('button', { name: 'Welcome' }).click();
    
    // Should show metadata - use heading for Note Info
    await expect(page.getByRole('heading', { name: 'Note Info' })).toBeVisible();
    await expect(page.getByText('Created')).toBeVisible();
    await expect(page.getByText('Modified')).toBeVisible();
    await expect(page.getByText('Word Count')).toBeVisible();
    // Tags label is in the metadata panel (use term/definition structure)
    await expect(page.locator('dt:has-text("Tags")')).toBeVisible();
  });

  test('shows tags on note', async ({ page }) => {
    // Open a note with tags
    await page.getByRole('button', { name: 'Welcome' }).click();
    
    // Should show the tags - they're in spans with 'x' buttons
    await expect(page.locator('span:has-text("test"):has(button)')).toBeVisible();
    await expect(page.locator('span:has-text("welcome"):has(button)')).toBeVisible();
  });

  test('shows backlinks for linked note', async ({ page }) => {
    // Open the link target note
    await page.getByRole('button', { name: 'Link Target' }).click();
    
    // Should show backlink from Welcome - look in the backlinks panel
    await expect(page.getByRole('heading', { name: 'Backlinks' })).toBeVisible();
    // The backlink shows as a button with the source note title
    await expect(page.locator('[class*="backlink"], button:has(.font-medium:has-text("Welcome"))')).toBeVisible();
  });

  test('creates a new note via button', async ({ page }) => {
    // Mock the prompt dialog
    page.on('dialog', async (dialog) => {
      await dialog.accept('Test Note');
    });
    
    // Click new note button
    await page.getByRole('button', { name: '+ New Note' }).click();
    
    // New note should appear in file tree
    await expect(page.getByRole('button', { name: 'Test Note' })).toBeVisible();
  });

  test('shows context menu on right-click', async ({ page }) => {
    // Right-click on a note
    await page.getByRole('button', { name: 'Welcome' }).click({ button: 'right' });
    
    // Context menu should appear
    await expect(page.getByRole('button', { name: 'Rename' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Delete' })).toBeVisible();
  });
});
