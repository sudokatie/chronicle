/**
 * E2E tests for keyboard shortcuts
 * Spec section 12.3: "Keyboard shortcuts"
 */
import { test, expect } from './fixtures';

test.describe('Keyboard Shortcuts', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('text=Chronicle');
  });

  test('Cmd+O opens quick open', async ({ page }) => {
    // Press Cmd+O
    await page.keyboard.press('Meta+o');
    
    // Quick open modal should appear
    await expect(page.getByPlaceholder('Search notes or create new...')).toBeVisible();
  });

  test('Cmd+P opens command palette', async ({ page }) => {
    // Press Cmd+P
    await page.keyboard.press('Meta+p');
    
    // Command palette should appear
    await expect(page.getByPlaceholder('Type a command...')).toBeVisible();
  });

  test('Cmd+G toggles graph view', async ({ page }) => {
    // Press Cmd+G
    await page.keyboard.press('Meta+g');
    
    // Should navigate to graph
    await expect(page).toHaveURL('/graph');
    
    // Press again to return
    await page.keyboard.press('Meta+g');
    await expect(page).toHaveURL('/');
  });

  test('Cmd+, opens settings', async ({ page }) => {
    // Press Cmd+,
    await page.keyboard.press('Meta+,');
    
    // Should navigate to settings
    await expect(page).toHaveURL('/settings');
  });

  test('Cmd+N creates new note', async ({ page }) => {
    // Mock the prompt dialog
    page.on('dialog', async (dialog) => {
      await dialog.accept('Shortcut Note');
    });
    
    // Press Cmd+N
    await page.keyboard.press('Meta+n');
    
    // New note should appear
    await expect(page.getByRole('button', { name: 'Shortcut Note' })).toBeVisible();
  });

  test('Escape closes quick open', async ({ page }) => {
    // Open quick open
    await page.keyboard.press('Meta+o');
    await expect(page.getByPlaceholder('Search notes or create new...')).toBeVisible();
    
    // Press Escape
    await page.keyboard.press('Escape');
    
    // Modal should close
    await expect(page.getByPlaceholder('Search notes or create new...')).not.toBeVisible();
  });

  test('Escape closes command palette', async ({ page }) => {
    // Open command palette
    await page.keyboard.press('Meta+p');
    await expect(page.getByPlaceholder('Type a command...')).toBeVisible();
    
    // Focus the input and press Escape
    await page.getByPlaceholder('Type a command...').press('Escape');
    
    // Modal should close
    await expect(page.getByPlaceholder('Type a command...')).not.toBeVisible();
  });

  test('quick open searches notes', async ({ page }) => {
    // Open quick open
    await page.keyboard.press('Meta+o');
    
    // Type search query
    await page.getByPlaceholder('Search notes or create new...').fill('welcome');
    
    // Should filter to matching notes - the result appears in the list as a button
    await expect(page.getByRole('button', { name: /Welcome welcome\.md/ })).toBeVisible();
  });

  test('quick open navigates with arrow keys', async ({ page }) => {
    // Open quick open
    await page.keyboard.press('Meta+o');
    
    // Arrow down to select second item
    await page.keyboard.press('ArrowDown');
    
    // Enter to select
    await page.keyboard.press('Enter');
    
    // Should open a note (editor visible)
    await expect(page.locator('.cm-editor')).toBeVisible();
  });

  test('command palette filters commands', async ({ page }) => {
    // Open command palette
    await page.keyboard.press('Meta+p');
    
    // Type to filter
    await page.getByPlaceholder('Type a command...').fill('graph');
    
    // Should show graph command
    await expect(page.getByText('Toggle Graph View')).toBeVisible();
  });

  test('command palette runs command on Enter', async ({ page }) => {
    // Open command palette
    await page.keyboard.press('Meta+p');
    
    // Filter to settings
    await page.getByPlaceholder('Type a command...').fill('settings');
    
    // Press Enter
    await page.keyboard.press('Enter');
    
    // Should navigate to settings
    await expect(page).toHaveURL('/settings');
  });
});
