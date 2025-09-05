import { test, expect } from '@playwright/test';

// This is a basic smoke test to verify the blog is working
test('blog is accessible and functional', async ({ page }) => {
  await page.goto('/');

  // Should have the correct title
  await expect(page).toHaveTitle('Willian Nguyen - Home');

  // Should have the main heading
  await expect(page.locator('h1')).toHaveText("Hi there, I'm Willian 👋");
  
  // Should have navigation
  await expect(page.locator('nav')).toBeVisible();
  
  // Should have blog posts table
  await expect(page.locator('table')).toBeVisible();
});
