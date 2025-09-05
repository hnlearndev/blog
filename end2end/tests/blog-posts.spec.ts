import { test, expect } from '@playwright/test';

test.describe('Blog Post Pages', () => {
  test('should display first blog post correctly', async ({ page }) => {
    await page.goto('/posts/1');
    
    // Check page title is set correctly
    await expect(page).toHaveTitle('Deploy my own tech blog finally');
    
    // Check main heading matches the post title
    const mainHeading = page.locator('h1').first();
    await expect(mainHeading).toHaveText('Deploy my own tech blog finally');
    
    // Check that content is rendered from markdown
    const subHeading = page.locator('h2').first();
    await expect(subHeading).toHaveText('Love at first sight');
  });

  test('should display second blog post correctly', async ({ page }) => {
    await page.goto('/posts/2');
    
    // Verify the second post loads
    const mainHeading = page.locator('h1').first();
    await expect(mainHeading).toBeVisible();
    
    // Check that we're on a different post
    const currentTitle = await page.title();
    expect(currentTitle).not.toBe('Deploy my own tech blog finally');
  });

  test('should handle invalid post IDs gracefully', async ({ page }) => {
    // Test non-existent post ID
    const response = await page.goto('/posts/999');
    
    // Should either show error page or handle gracefully
    // The exact behavior depends on your error handling implementation
    if (response?.status() === 404) {
      // If you handle 404s properly
      await expect(page.locator('body')).toContainText(/not found|404/i);
    } else {
      // Or if you show some kind of error message
      const bodyText = await page.locator('body').textContent();
      expect(bodyText).toBeTruthy(); // Should still render something
    }
  });

  test('should render markdown content properly', async ({ page }) => {
    await page.goto('/posts/1');
    
    // Check that markdown headings are rendered as HTML
    const h1Elements = page.locator('h1');
    await expect(h1Elements).toHaveCount(1);
    
    const h2Elements = page.locator('h2');
    await expect(h2Elements.first()).toBeVisible();
    
    // Check that paragraphs are rendered
    const paragraphs = page.locator('p');
    await expect(paragraphs).toHaveCountGreaterThan(3);
    
    // Check for links in the content (based on your markdown)
    const links = page.locator('a[href*="amazon.com"]');
    if (await links.count() > 0) {
      await expect(links.first()).toHaveAttribute('href', expect.stringMatching(/amazon\.com/));
    }
  });

  test('should handle code blocks with syntax highlighting', async ({ page }) => {
    await page.goto('/posts/1');
    
    // Look for code elements (your build script uses Syntect for highlighting)
    const codeElements = page.locator('code');
    
    // If there are code blocks, they should be properly formatted
    if (await codeElements.count() > 0) {
      const firstCodeBlock = codeElements.first();
      await expect(firstCodeBlock).toBeVisible();
      
      // Check that the code block has some styling applied
      const hasPreParent = await firstCodeBlock.locator('..').evaluate(el => el.tagName === 'PRE');
      expect(hasPreParent).toBeTruthy();
    }
  });

  test('should maintain proper document structure', async ({ page }) => {
    await page.goto('/posts/1');
    
    // Check that the page has proper semantic structure
    const main = page.locator('main');
    await expect(main).toBeVisible();
    
    // Check for header and footer from layout
    const header = page.locator('header');
    await expect(header).toBeVisible();
    
    const footer = page.locator('footer');
    await expect(footer).toBeVisible();
    
    // The blog content should be inside main
    const contentInMain = main.locator('h1');
    await expect(contentInMain).toBeVisible();
  });

  test('should be navigable from homepage', async ({ page }) => {
    // Start from homepage
    await page.goto('/');
    
    // Click on first blog post
    const firstPostLink = page.locator('tbody tr').first().locator('a');
    await firstPostLink.click();
    
    // Should navigate to the correct post
    await expect(page).toHaveURL('/posts/1');
    
    // Should show the correct content
    const heading = page.locator('h1');
    await expect(heading).toHaveText('Deploy my own tech blog finally');
  });

  test('should allow navigation back to homepage', async ({ page }) => {
    await page.goto('/posts/1');
    
    // Click on the brand link in navigation
    const brandLink = page.locator('.brand-link');
    await expect(brandLink).toBeVisible();
    await brandLink.click();
    
    // Should navigate back to homepage
    await expect(page).toHaveURL('/');
    await expect(page).toHaveTitle('Willian Nguyen - Home');
  });

  test('should load post content within acceptable time', async ({ page }) => {
    const startTime = Date.now();
    await page.goto('/posts/1');
    await page.waitForSelector('h1');
    const loadTime = Date.now() - startTime;
    
    // Post should load within 2 seconds
    expect(loadTime).toBeLessThan(2000);
  });

  test('should have proper meta tags for SEO', async ({ page }) => {
    await page.goto('/posts/1');
    
    // Title should be set to post title
    await expect(page).toHaveTitle('Deploy my own tech blog finally');
    
    // Check for meta description (if implemented)
    const metaDescription = page.locator('meta[name="description"]');
    if (await metaDescription.count() > 0) {
      const content = await metaDescription.getAttribute('content');
      expect(content).toBeTruthy();
    }
    
    // Check for viewport meta tag
    const viewport = page.locator('meta[name="viewport"]');
    await expect(viewport).toHaveAttribute('content', 'width=device-width, initial-scale=1');
  });

  test('should handle browser navigation correctly', async ({ page }) => {
    // Start from homepage
    await page.goto('/');
    
    // Navigate to first post
    await page.locator('tbody tr').first().locator('a').click();
    await expect(page).toHaveURL('/posts/1');
    
    // Navigate to second post
    await page.goto('/posts/2');
    await expect(page).not.toHaveURL('/posts/1');
    
    // Use browser back button
    await page.goBack();
    await expect(page).toHaveURL('/posts/1');
    
    // Use browser forward button
    await page.goForward();
    await expect(page).toHaveURL('/posts/2');
  });
});
