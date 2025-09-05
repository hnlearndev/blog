import { test, expect } from '@playwright/test';

test.describe('Homepage', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('should have correct title and meta tags', async ({ page }) => {
    // Check page title
    await expect(page).toHaveTitle('Willian Nguyen - Home');
    
    // Check meta description
    const metaDescription = page.locator('meta[name="description"]');
    await expect(metaDescription).toHaveAttribute('content', "Willian's personal website");
    
    // Check viewport meta tag
    const viewport = page.locator('meta[name="viewport"]');
    await expect(viewport).toHaveAttribute('content', 'width=device-width, initial-scale=1');
  });

  test('should display main heading and introduction', async ({ page }) => {
    // Check main heading
    const mainHeading = page.locator('h1');
    await expect(mainHeading).toBeVisible();
    await expect(mainHeading).toHaveText("Hi there, I'm Willian 👋");
    
    // Check introduction paragraphs
    const intro1 = page.locator('p').first();
    await expect(intro1).toHaveText('I am a curious learner with a genuine love for new technologies.');
    
    const intro2 = page.locator('p').nth(1);
    await expect(intro2).toHaveText('Anything that I learn and find interesting, I will write about it on my blog.');
  });

  test('should display blog posts table', async ({ page }) => {
    // Check if posts table exists
    const table = page.locator('table');
    await expect(table).toBeVisible();
    
    // Check table headers
    const titleHeader = page.locator('th:has-text("Title")');
    await expect(titleHeader).toBeVisible();
    
    const dateHeader = page.locator('th:has-text("Date")');
    await expect(dateHeader).toBeVisible();
    
    // Check if there are blog post entries
    const postRows = page.locator('tbody tr');
    await expect(postRows).toHaveCount(2); // Based on the two markdown files
  });

  test('should have clickable blog post links', async ({ page }) => {
    // Get the first blog post link
    const firstPostLink = page.locator('tbody tr').first().locator('a');
    await expect(firstPostLink).toBeVisible();
    
    // Check if the link has contrast class (from your FastA component)
    await expect(firstPostLink).toHaveClass(/contrast/);
    
    // Check if clicking the link navigates to the post
    const href = await firstPostLink.getAttribute('href');
    expect(href).toMatch(/^\/posts\/\d+$/);
    
    // Click the link and verify navigation
    await firstPostLink.click();
    await expect(page).toHaveURL(href!);
  });

  test('should display post titles and dates correctly', async ({ page }) => {
    // Check first post
    const firstRow = page.locator('tbody tr').first();
    const firstPostTitle = firstRow.locator('th').first();
    const firstPostDate = firstRow.locator('th').nth(1);
    
    await expect(firstPostTitle).toContainText('Deploy my own tech blog finally');
    await expect(firstPostDate).toHaveText('15-Sep-2025'); // Based on filename format
    
    // Check second post
    const secondRow = page.locator('tbody tr').nth(1);
    const secondPostTitle = secondRow.locator('th').first();
    const secondPostDate = secondRow.locator('th').nth(1);
    
    await expect(secondPostTitle).toContainText('dummy blog');
    await expect(secondPostDate).toHaveText('25-Sep-2025');
  });

  test('should load page within acceptable time', async ({ page }) => {
    const startTime = Date.now();
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    const loadTime = Date.now() - startTime;
    
    // Page should load within 3 seconds
    expect(loadTime).toBeLessThan(3000);
  });

  test('should have proper semantic structure', async ({ page }) => {
    // Check for main landmark
    const main = page.locator('main');
    await expect(main).toBeVisible();
    
    // Check for header
    const header = page.locator('header');
    await expect(header).toBeVisible();
    
    // Check for footer
    const footer = page.locator('footer');
    await expect(footer).toBeVisible();
    
    // Check that table has proper structure
    const table = page.locator('table');
    await expect(table.locator('thead')).toBeVisible();
    await expect(table.locator('tbody')).toBeVisible();
  });

  test('should handle empty state gracefully', async ({ page }) => {
    // This test would be more relevant if you had a way to simulate no posts
    // For now, just verify the table structure exists even with posts
    const table = page.locator('table');
    await expect(table).toBeVisible();
    
    const tbody = table.locator('tbody');
    await expect(tbody).toBeVisible();
  });
});
