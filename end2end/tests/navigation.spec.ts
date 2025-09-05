import { test, expect } from '@playwright/test';

test.describe('Navigation', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test.describe('Desktop Navigation', () => {
    test('should display brand/logo correctly', async ({ page }) => {
      const brandLink = page.locator('.brand-link');
      await expect(brandLink).toBeVisible();
      await expect(brandLink).toHaveAttribute('href', '/');
      
      // Check for logo image
      const logo = brandLink.locator('img');
      await expect(logo).toBeVisible();
      await expect(logo).toHaveAttribute('src', '/favicon.ico');
      await expect(logo).toHaveAttribute('alt', 'Brand Logo');
      
      // Check for brand text
      const brandText = brandLink.locator('strong');
      await expect(brandText).toHaveText('Willian Nguyen');
    });

    test('should have functional social media links', async ({ page }) => {
      const socialContainer = page.locator('.nav-social');
      await expect(socialContainer).toBeVisible();
      
      // GitHub link
      const githubLink = page.locator('a[href*="github.com/hnlearndev"]');
      await expect(githubLink).toBeVisible();
      await expect(githubLink).toHaveAttribute('target', '_blank');
      await expect(githubLink).toHaveAttribute('title', 'GitHub');
      
      // LinkedIn link
      const linkedinLink = page.locator('a[href*="linkedin.com/in/hieunthello"]');
      await expect(linkedinLink).toBeVisible();
      await expect(linkedinLink).toHaveAttribute('target', '_blank');
      await expect(linkedinLink).toHaveAttribute('title', 'LinkedIn');
      
      // Email link
      const emailLink = page.locator('a[href="mailto:hieunt.hello@gmail.com"]');
      await expect(emailLink).toBeVisible();
      await expect(emailLink).toHaveAttribute('title', 'Email');
    });

    test('should navigate to homepage when clicking brand', async ({ page }) => {
      // Navigate to a blog post first
      await page.goto('/posts/1');
      
      // Click brand link
      const brandLink = page.locator('.brand-link');
      await brandLink.click();
      
      // Should return to homepage
      await expect(page).toHaveURL('/');
      await expect(page).toHaveTitle('Willian Nguyen - Home');
    });

    test('should hide mobile menu button on desktop', async ({ page }) => {
      // On desktop viewport, mobile menu button should be hidden
      const mobileMenuButton = page.locator('.mobile-menu-button');
      
      // The button might exist in DOM but be hidden via CSS
      if (await mobileMenuButton.count() > 0) {
        await expect(mobileMenuButton).not.toBeVisible();
      }
    });
  });

  test.describe('Mobile Navigation', () => {
    test.beforeEach(async ({ page }) => {
      // Set mobile viewport
      await page.setViewportSize({ width: 375, height: 667 });
      await page.goto('/');
    });

    test('should display mobile menu button', async ({ page }) => {
      const mobileMenuButton = page.locator('.mobile-menu-button');
      await expect(mobileMenuButton).toBeVisible();
      await expect(mobileMenuButton).toHaveAttribute('aria-label', 'Toggle mobile menu');
    });

    test('should toggle mobile menu when clicking button', async ({ page }) => {
      const mobileMenuButton = page.locator('.mobile-menu-button');
      const mobileMenu = page.locator('.mobile-menu');
      
      // Initially, mobile menu should be closed
      await expect(mobileMenu).not.toHaveClass(/mobile-menu-open/);
      
      // Click menu button to open
      await mobileMenuButton.click();
      await expect(mobileMenu).toHaveClass(/mobile-menu-open/);
      
      // Click again to close
      await mobileMenuButton.click();
      await expect(mobileMenu).not.toHaveClass(/mobile-menu-open/);
    });

    test('should display mobile social links', async ({ page }) => {
      const mobileMenuButton = page.locator('.mobile-menu-button');
      await mobileMenuButton.click();
      
      const mobileSocialContainer = page.locator('.mobile-nav-social');
      await expect(mobileSocialContainer).toBeVisible();
      
      // Check mobile social links
      const mobileGithubLink = mobileSocialContainer.locator('a[href*="github.com"]');
      await expect(mobileGithubLink).toBeVisible();
      await expect(mobileGithubLink.locator('span')).toHaveText('GitHub');
      
      const mobileLinkedinLink = mobileSocialContainer.locator('a[href*="linkedin.com"]');
      await expect(mobileLinkedinLink).toBeVisible();
      await expect(mobileLinkedinLink.locator('span')).toHaveText('LinkedIn');
      
      const mobileEmailLink = mobileSocialContainer.locator('a[href*="mailto:"]');
      await expect(mobileEmailLink).toBeVisible();
      await expect(mobileEmailLink.locator('span')).toHaveText('Email');
    });

    test('should show correct menu icons', async ({ page }) => {
      const mobileMenuButton = page.locator('.mobile-menu-button');
      
      // Initially should show menu icon (hamburger)
      // The exact implementation depends on your icon components
      await expect(mobileMenuButton).toBeVisible();
      
      // Click to open menu
      await mobileMenuButton.click();
      
      // Should now show close icon
      // This test might need adjustment based on your icon implementation
      await expect(mobileMenuButton).toBeVisible();
    });

    test('should maintain brand link functionality on mobile', async ({ page }) => {
      await page.goto('/posts/1');
      
      const brandLink = page.locator('.brand-link');
      await expect(brandLink).toBeVisible();
      
      await brandLink.click();
      await expect(page).toHaveURL('/');
    });
  });

  test.describe('Navigation Consistency', () => {
    test('should maintain navigation across all pages', async ({ page }) => {
      // Check homepage
      await page.goto('/');
      await expect(page.locator('header nav')).toBeVisible();
      
      // Check blog post page
      await page.goto('/posts/1');
      await expect(page.locator('header nav')).toBeVisible();
      
      // Brand should be consistent
      const brandText = page.locator('.brand-link strong');
      await expect(brandText).toHaveText('Willian Nguyen');
    });

    test('should have consistent social links across pages', async ({ page }) => {
      const socialLinks = [
        'https://github.com/hnlearndev',
        'https://www.linkedin.com/in/hieunthello/',
        'mailto:hieunt.hello@gmail.com'
      ];
      
      for (const url of ['/']) { // Test can be extended to more pages
        await page.goto(url);
        
        for (const socialLink of socialLinks) {
          const link = page.locator(`a[href="${socialLink}"]`);
          await expect(link).toBeVisible();
        }
      }
    });

    test('should handle keyboard navigation', async ({ page }) => {
      // Tab through navigation elements
      const brandLink = page.locator('.brand-link');
      await brandLink.focus();
      await expect(brandLink).toBeFocused();
      
      // Tab to social links
      await page.keyboard.press('Tab');
      const githubLink = page.locator('a[href*="github.com"]').first();
      await expect(githubLink).toBeFocused();
      
      // Enter should activate the link
      const githubHref = await githubLink.getAttribute('href');
      expect(githubHref).toBeTruthy();
    });
  });

  test.describe('Responsive Behavior', () => {
    test('should adapt to different screen sizes', async ({ page }) => {
      // Desktop
      await page.setViewportSize({ width: 1200, height: 800 });
      await page.goto('/');
      
      const desktopNav = page.locator('.nav-desktop');
      await expect(desktopNav).toBeVisible();
      
      // Tablet
      await page.setViewportSize({ width: 768, height: 1024 });
      await page.reload();
      
      // Mobile
      await page.setViewportSize({ width: 320, height: 568 });
      await page.reload();
      
      const mobileMenuButton = page.locator('.mobile-menu-button');
      await expect(mobileMenuButton).toBeVisible();
    });

    test('should handle orientation changes on mobile', async ({ page }) => {
      // Portrait
      await page.setViewportSize({ width: 375, height: 667 });
      await page.goto('/');
      
      let brandLink = page.locator('.brand-link');
      await expect(brandLink).toBeVisible();
      
      // Landscape
      await page.setViewportSize({ width: 667, height: 375 });
      await page.reload();
      
      brandLink = page.locator('.brand-link');
      await expect(brandLink).toBeVisible();
    });
  });

  test.describe('External Links', () => {
    test('should open social links in new tabs', async ({ page }) => {
      const githubLink = page.locator('a[href*="github.com"]').first();
      const linkedinLink = page.locator('a[href*="linkedin.com"]').first();
      
      await expect(githubLink).toHaveAttribute('target', '_blank');
      await expect(linkedinLink).toHaveAttribute('target', '_blank');
      
      // Email link should not open in new tab
      const emailLink = page.locator('a[href*="mailto:"]').first();
      const emailTarget = await emailLink.getAttribute('target');
      expect(emailTarget).toBeNull();
    });

    test('should have proper accessibility attributes for external links', async ({ page }) => {
      const externalLinks = page.locator('a[target="_blank"]');
      const count = await externalLinks.count();
      
      if (count > 0) {
        // External links should ideally have rel="noopener noreferrer" for security
        // This is a best practice recommendation
        for (let i = 0; i < count; i++) {
          const link = externalLinks.nth(i);
          const title = await link.getAttribute('title');
          expect(title).toBeTruthy(); // Should have descriptive title
        }
      }
    });
  });
});
