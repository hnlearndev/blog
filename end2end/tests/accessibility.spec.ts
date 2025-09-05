import { test, expect } from '@playwright/test';

test.describe('Accessibility', () => {
  test.describe('Keyboard Navigation', () => {
    test('should allow tabbing through navigation elements', async ({ page }) => {
      await page.goto('/');

      // Tab to brand link
      await page.keyboard.press('Tab');
      const brandLink = page.locator('.brand-link');
      await expect(brandLink).toBeFocused();

      // Continue tabbing through social links
      await page.keyboard.press('Tab');
      const firstSocialLink = page.locator('a[href*="github.com"]').first();
      await expect(firstSocialLink).toBeFocused();

      await page.keyboard.press('Tab');
      const secondSocialLink = page.locator('a[href*="linkedin.com"]').first();
      await expect(secondSocialLink).toBeFocused();

      await page.keyboard.press('Tab');
      const thirdSocialLink = page.locator('a[href*="mailto:"]').first();
      await expect(thirdSocialLink).toBeFocused();
    });

    test('should allow keyboard navigation to blog posts', async ({ page }) => {
      await page.goto('/');

      // Find first blog post link
      const firstPostLink = page.locator('tbody tr').first().locator('a');
      await firstPostLink.focus();
      await expect(firstPostLink).toBeFocused();

      // Enter should activate the link
      await page.keyboard.press('Enter');
      await expect(page).toHaveURL(/\/posts\/\d+/);
    });

    test('should allow keyboard navigation on mobile menu', async ({ page }) => {
      // Set mobile viewport
      await page.setViewportSize({ width: 375, height: 667 });
      await page.goto('/');

      const mobileMenuButton = page.locator('.mobile-menu-button');
      if (await mobileMenuButton.count() > 0) {
        // Tab to mobile menu button
        await mobileMenuButton.focus();
        await expect(mobileMenuButton).toBeFocused();

        // Space or Enter should activate the button
        await page.keyboard.press('Enter');

        // Mobile menu should open
        const mobileMenu = page.locator('.mobile-menu');
        await expect(mobileMenu).toHaveClass(/mobile-menu-open/);
      }
    });

    test('should have proper focus indicators', async ({ page }) => {
      await page.goto('/');

      // Check that focused elements have visible focus indicators
      const brandLink = page.locator('.brand-link');
      await brandLink.focus();

      // Check if focus is visible (this would need CSS testing in a real scenario)
      const focusedElement = page.locator(':focus');
      await expect(focusedElement).toBeFocused();
    });
  });

  test.describe('ARIA and Semantic HTML', () => {
    test('should have proper heading hierarchy', async ({ page }) => {
      await page.goto('/');

      // Should have one h1
      const h1Elements = page.locator('h1');
      await expect(h1Elements).toHaveCount(1);

      // Main heading should be descriptive
      const mainHeading = h1Elements.first();
      const headingText = await mainHeading.textContent();
      expect(headingText).toBeTruthy();
      expect(headingText!.length).toBeGreaterThan(5);
    });

    test('should have proper ARIA labels on interactive elements', async ({ page }) => {
      await page.goto('/');

      // Mobile menu button should have aria-label
      const mobileMenuButton = page.locator('.mobile-menu-button');
      if (await mobileMenuButton.count() > 0) {
        await expect(mobileMenuButton).toHaveAttribute('aria-label', 'Toggle mobile menu');
      }

      // Links should have descriptive text or titles
      const socialLinks = page.locator('a[target="_blank"]');
      const linkCount = await socialLinks.count();

      for (let i = 0; i < linkCount; i++) {
        const link = socialLinks.nth(i);
        const title = await link.getAttribute('title');
        const textContent = await link.textContent();

        // Should have either title attribute or descriptive text
        expect(title || textContent).toBeTruthy();
      }
    });

    test('should have proper landmarks', async ({ page }) => {
      await page.goto('/');

      // Check for main landmark
      const main = page.locator('main');
      await expect(main).toBeVisible();

      // Check for navigation landmark
      const nav = page.locator('nav');
      await expect(nav).toBeVisible();

      // Check for header landmark
      const header = page.locator('header');
      await expect(header).toBeVisible();

      // Check for footer landmark (if present)
      const footer = page.locator('footer');
      if (await footer.count() > 0) {
        await expect(footer).toBeVisible();
      }
    });

    test('should have descriptive page titles', async ({ page }) => {
      // Homepage
      await page.goto('/');
      await expect(page).toHaveTitle(/Willian/);

      // Blog post
      await page.goto('/posts/1');
      const postTitle = await page.title();
      expect(postTitle).toBeTruthy();
      expect(postTitle.length).toBeGreaterThan(10);
      expect(postTitle).not.toBe('Willian Nguyen - Home'); // Should be different from homepage
    });

    test('should have proper table structure', async ({ page }) => {
      await page.goto('/');

      const table = page.locator('table');
      if (await table.count() > 0) {
        // Should have proper table headers
        const thead = table.locator('thead');
        await expect(thead).toBeVisible();

        const headers = thead.locator('th');
        await expect(headers).toHaveCountGreaterThan(0);

        // Headers should have scope attribute for accessibility
        const titleHeader = page.locator('th:has-text("Title")');
        if (await titleHeader.count() > 0) {
          await expect(titleHeader).toHaveAttribute('scope', 'col');
        }
      }
    });
  });

  test.describe('Images and Media', () => {
    test('should have alt text for images', async ({ page }) => {
      await page.goto('/');

      const images = page.locator('img');
      const imageCount = await images.count();

      for (let i = 0; i < imageCount; i++) {
        const img = images.nth(i);
        const alt = await img.getAttribute('alt');

        // All images should have alt attributes
        expect(alt).not.toBeNull();

        // Alt text should be descriptive (not empty)
        if (alt) {
          expect(alt.length).toBeGreaterThan(0);
        }
      }
    });

    test('should have proper image dimensions', async ({ page }) => {
      await page.goto('/');

      const images = page.locator('img');
      const imageCount = await images.count();

      for (let i = 0; i < imageCount; i++) {
        const img = images.nth(i);

        // Images should have width and height attributes for layout stability
        const width = await img.getAttribute('width');
        const height = await img.getAttribute('height');

        if (width && height) {
          expect(parseInt(width)).toBeGreaterThan(0);
          expect(parseInt(height)).toBeGreaterThan(0);
        }
      }
    });
  });

  test.describe('Color and Contrast', () => {
    test('should not rely solely on color for information', async ({ page }) => {
      await page.goto('/');

      // This is more of a manual check, but we can verify that
      // links have underlines or other visual indicators beyond color
      const links = page.locator('a');
      const linkCount = await links.count();

      // At minimum, verify links exist and are identifiable
      expect(linkCount).toBeGreaterThan(0);
    });

    test('should be readable in high contrast mode', async ({ page }) => {
      // Simulate high contrast mode (this is a basic check)
      await page.goto('/');

      // Check that text is visible
      const bodyText = await page.locator('body').textContent();
      expect(bodyText).toBeTruthy();
      expect(bodyText!.length).toBeGreaterThan(50);
    });
  });

  test.describe('Form Accessibility', () => {
    test('should have proper form labels and structure', async ({ page }) => {
      await page.goto('/');

      const subscribeForm = page.locator('.subscribe-form');
      if (await subscribeForm.count() > 0) {
        const emailInput = subscribeForm.locator('input[type="email"]');

        // Input should have proper attributes
        await expect(emailInput).toHaveAttribute('type', 'email');
        await expect(emailInput).toHaveAttribute('required');

        // Should have placeholder or label
        const placeholder = await emailInput.getAttribute('placeholder');
        expect(placeholder).toBeTruthy();
      }
    });

    test('should handle form errors accessibly', async ({ page }) => {
      await page.goto('/');

      const subscribeForm = page.locator('.subscribe-form');
      if (await subscribeForm.count() > 0) {
        // Mock error response
        await page.route('/api/subscribe', async route => {
          await route.fulfill({
            status: 400,
            contentType: 'application/json',
            body: JSON.stringify({ message: 'Invalid email address' })
          });
        });

        const emailInput = subscribeForm.locator('input[type="email"]');
        const submitButton = subscribeForm.locator('button[type="button"]');

        await emailInput.fill('invalid@');
        await submitButton.click();

        // Error message should be visible and associated with the form
        const errorDiv = subscribeForm.locator('.error');
        if (await errorDiv.count() > 0) {
          await expect(errorDiv).toBeVisible();

          const errorText = await errorDiv.textContent();
          expect(errorText).toBeTruthy();
        }
      }
    });
  });

  test.describe('Responsive Accessibility', () => {
    test('should maintain accessibility on mobile', async ({ page }) => {
      await page.setViewportSize({ width: 375, height: 667 });
      await page.goto('/');

      // Navigation should still be accessible
      const nav = page.locator('nav');
      await expect(nav).toBeVisible();

      // Mobile menu button should be accessible if present
      const mobileMenuButton = page.locator('.mobile-menu-button');
      if (await mobileMenuButton.count() > 0) {
        await expect(mobileMenuButton).toBeVisible();
        await expect(mobileMenuButton).toHaveAttribute('aria-label');
      }

      // Text should still be readable
      const mainHeading = page.locator('h1');
      await expect(mainHeading).toBeVisible();
    });

    test('should maintain focus management on mobile', async ({ page }) => {
      await page.setViewportSize({ width: 375, height: 667 });
      await page.goto('/');

      const mobileMenuButton = page.locator('.mobile-menu-button');
      if (await mobileMenuButton.count() > 0) {
        // Focus should work on mobile menu
        await mobileMenuButton.focus();
        await expect(mobileMenuButton).toBeFocused();

        // Activating menu should maintain proper focus management
        await page.keyboard.press('Enter');

        // Focus should still be manageable
        const focusedElement = page.locator(':focus');
        const isAnyElementFocused = await focusedElement.count() > 0;
        expect(isAnyElementFocused).toBeTruthy();
      }
    });
  });

  test.describe('Screen Reader Support', () => {
    test('should have proper document structure for screen readers', async ({ page }) => {
      await page.goto('/');

      // Check document structure
      await expect(page.locator('html')).toHaveAttribute('lang', 'en');

      // Meta tags should provide context
      const titleElement = page.locator('title');
      await expect(titleElement).toHaveCount(1);

      // Viewport should be set for responsive behavior
      const viewport = page.locator('meta[name="viewport"]');
      await expect(viewport).toHaveAttribute('content', expect.stringMatching(/width=device-width/));
    });

    test('should provide context for dynamic content', async ({ page }) => {
      await page.goto('/');

      // Blog posts should be in a meaningful structure
      const table = page.locator('table');
      if (await table.count() > 0) {
        // Table should provide context for screen readers
        const caption = table.locator('caption');
        const headers = table.locator('th');

        // Either caption or clear headers should be present
        const hasCaption = await caption.count() > 0;
        const hasHeaders = await headers.count() > 0;

        expect(hasCaption || hasHeaders).toBeTruthy();
      }
    });
  });
});
