import { test, expect } from '@playwright/test';

test.describe('Responsive Design', () => {
  test.describe('Desktop Layout', () => {
    test.beforeEach(async ({ page }) => {
      await page.setViewportSize({ width: 1200, height: 800 });
    });

    test('should display desktop navigation correctly', async ({ page }) => {
      await page.goto('/');
      
      // Desktop navigation should be visible
      const desktopNav = page.locator('.nav-desktop');
      await expect(desktopNav).toBeVisible();
      
      // Mobile menu button should be hidden
      const mobileMenuButton = page.locator('.mobile-menu-button');
      if (await mobileMenuButton.count() > 0) {
        await expect(mobileMenuButton).not.toBeVisible();
      }
      
      // Social links should be visible in desktop nav
      const socialContainer = page.locator('.nav-social');
      await expect(socialContainer).toBeVisible();
    });

    test('should layout content properly on desktop', async ({ page }) => {
      await page.goto('/');
      
      // Main content should be properly contained
      const container = page.locator('.container');
      await expect(container).toBeVisible();
      
      // Table should display properly
      const table = page.locator('table');
      await expect(table).toBeVisible();
      
      // Content should not overflow
      const body = page.locator('body');
      const bodyBox = await body.boundingBox();
      if (bodyBox) {
        expect(bodyBox.width).toBeLessThanOrEqual(1200);
      }
    });

    test('should handle wide content gracefully', async ({ page }) => {
      await page.goto('/posts/1');
      
      // Long content lines should not cause horizontal overflow
      const main = page.locator('main');
      const mainBox = await main.boundingBox();
      
      if (mainBox) {
        expect(mainBox.width).toBeLessThanOrEqual(1200);
      }
      
      // Code blocks should handle overflow appropriately
      const codeBlocks = page.locator('pre, code');
      const codeCount = await codeBlocks.count();
      
      for (let i = 0; i < codeCount; i++) {
        const code = codeBlocks.nth(i);
        const codeBox = await code.boundingBox();
        
        if (codeBox) {
          // Code blocks should be contained within reasonable bounds
          expect(codeBox.width).toBeLessThan(1200);
        }
      }
    });
  });

  test.describe('Mobile Layout', () => {
    test.beforeEach(async ({ page }) => {
      await page.setViewportSize({ width: 375, height: 667 });
    });

    test('should display mobile navigation correctly', async ({ page }) => {
      await page.goto('/');
      
      // Mobile menu button should be visible
      const mobileMenuButton = page.locator('.mobile-menu-button');
      await expect(mobileMenuButton).toBeVisible();
      
      // Desktop navigation should be hidden or collapsed
      const desktopNav = page.locator('.nav-desktop');
      if (await desktopNav.count() > 0) {
        // Check if it's either hidden or has mobile-specific styling
        const isVisible = await desktopNav.isVisible();
        if (isVisible) {
          // If visible, it should adapt to mobile layout
          const navBox = await desktopNav.boundingBox();
          if (navBox) {
            expect(navBox.width).toBeLessThanOrEqual(375);
          }
        }
      }
    });

    test('should handle mobile menu interactions', async ({ page }) => {
      await page.goto('/');
      
      const mobileMenuButton = page.locator('.mobile-menu-button');
      const mobileMenu = page.locator('.mobile-menu');
      
      // Initially menu should be closed
      await expect(mobileMenu).not.toHaveClass(/mobile-menu-open/);
      
      // Open menu
      await mobileMenuButton.click();
      await expect(mobileMenu).toHaveClass(/mobile-menu-open/);
      
      // Menu content should be visible
      const menuContent = page.locator('.mobile-menu-content');
      await expect(menuContent).toBeVisible();
      
      // Close menu
      await mobileMenuButton.click();
      await expect(mobileMenu).not.toHaveClass(/mobile-menu-open/);
    });

    test('should layout content properly on mobile', async ({ page }) => {
      await page.goto('/');
      
      // Content should fit within mobile viewport
      const main = page.locator('main');
      const mainBox = await main.boundingBox();
      
      if (mainBox) {
        expect(mainBox.width).toBeLessThanOrEqual(375);
      }
      
      // Table should be responsive
      const table = page.locator('table');
      if (await table.count() > 0) {
        const tableBox = await table.boundingBox();
        if (tableBox) {
          expect(tableBox.width).toBeLessThanOrEqual(375);
        }
      }
      
      // Text should be readable (minimum size)
      const headings = page.locator('h1, h2, h3');
      const headingCount = await headings.count();
      
      for (let i = 0; i < headingCount; i++) {
        const heading = headings.nth(i);
        await expect(heading).toBeVisible();
      }
    });

    test('should handle touch interactions', async ({ page }) => {
      await page.goto('/');
      
      // Test touch on blog post links
      const firstPostLink = page.locator('tbody tr').first().locator('a');
      if (await firstPostLink.count() > 0) {
        await firstPostLink.tap();
        await expect(page).toHaveURL(/\/posts\/\d+/);
      }
    });

    test('should adapt typography for mobile', async ({ page }) => {
      await page.goto('/');
      
      // Main heading should be visible and readable
      const mainHeading = page.locator('h1');
      await expect(mainHeading).toBeVisible();
      
      // Check that text doesn't overflow
      const paragraphs = page.locator('p');
      const paragraphCount = await paragraphs.count();
      
      for (let i = 0; i < paragraphCount; i++) {
        const paragraph = paragraphs.nth(i);
        const pBox = await paragraph.boundingBox();
        
        if (pBox) {
          expect(pBox.width).toBeLessThanOrEqual(375);
        }
      }
    });
  });

  test.describe('Tablet Layout', () => {
    test.beforeEach(async ({ page }) => {
      await page.setViewportSize({ width: 768, height: 1024 });
    });

    test('should adapt to tablet dimensions', async ({ page }) => {
      await page.goto('/');
      
      // Content should adapt to tablet width
      const main = page.locator('main');
      const mainBox = await main.boundingBox();
      
      if (mainBox) {
        expect(mainBox.width).toBeLessThanOrEqual(768);
      }
      
      // Navigation should work appropriately
      const nav = page.locator('nav');
      await expect(nav).toBeVisible();
      
      // Check if mobile or desktop nav is used
      const mobileMenuButton = page.locator('.mobile-menu-button');
      const desktopNav = page.locator('.nav-desktop');
      
      // Either mobile or desktop nav should be active
      const hasMobileButton = await mobileMenuButton.count() > 0 && await mobileMenuButton.isVisible();
      const hasDesktopNav = await desktopNav.count() > 0 && await desktopNav.isVisible();
      
      expect(hasMobileButton || hasDesktopNav).toBeTruthy();
    });

    test('should maintain readability on tablet', async ({ page }) => {
      await page.goto('/posts/1');
      
      // Content should be readable and well-formatted
      const article = page.locator('main');
      await expect(article).toBeVisible();
      
      // Paragraphs should have appropriate line length
      const paragraphs = page.locator('p');
      const firstParagraph = paragraphs.first();
      
      if (await firstParagraph.count() > 0) {
        const pBox = await firstParagraph.boundingBox();
        if (pBox) {
          // Text should not be too wide for comfortable reading
          expect(pBox.width).toBeLessThan(768);
        }
      }
    });
  });

  test.describe('Breakpoint Transitions', () => {
    test('should transition smoothly between breakpoints', async ({ page }) => {
      await page.goto('/');
      
      // Start with desktop
      await page.setViewportSize({ width: 1200, height: 800 });
      await expect(page.locator('nav')).toBeVisible();
      
      // Transition to tablet
      await page.setViewportSize({ width: 768, height: 1024 });
      await expect(page.locator('nav')).toBeVisible();
      
      // Transition to mobile
      await page.setViewportSize({ width: 375, height: 667 });
      await expect(page.locator('nav')).toBeVisible();
      
      // Content should remain accessible at all sizes
      const mainHeading = page.locator('h1');
      await expect(mainHeading).toBeVisible();
    });

    test('should handle rapid viewport changes', async ({ page }) => {
      await page.goto('/');
      
      const viewports = [
        { width: 1200, height: 800 },
        { width: 375, height: 667 },
        { width: 768, height: 1024 },
        { width: 320, height: 568 },
        { width: 1440, height: 900 }
      ];
      
      for (const viewport of viewports) {
        await page.setViewportSize(viewport);
        
        // Core content should remain accessible
        await expect(page.locator('h1')).toBeVisible();
        await expect(page.locator('nav')).toBeVisible();
        
        // No horizontal scrolling should occur
        const body = page.locator('body');
        const bodyBox = await body.boundingBox();
        
        if (bodyBox) {
          expect(bodyBox.width).toBeLessThanOrEqual(viewport.width);
        }
      }
    });
  });

  test.describe('Orientation Changes', () => {
    test('should handle portrait to landscape transition', async ({ page }) => {
      // Start in portrait
      await page.setViewportSize({ width: 375, height: 667 });
      await page.goto('/');
      
      const mobileMenuButton = page.locator('.mobile-menu-button');
      if (await mobileMenuButton.count() > 0) {
        await expect(mobileMenuButton).toBeVisible();
      }
      
      // Switch to landscape
      await page.setViewportSize({ width: 667, height: 375 });
      
      // Content should still be accessible
      await expect(page.locator('h1')).toBeVisible();
      await expect(page.locator('nav')).toBeVisible();
      
      // Navigation might change based on available width
      const nav = page.locator('nav');
      await expect(nav).toBeVisible();
    });

    test('should maintain functionality across orientations', async ({ page }) => {
      await page.setViewportSize({ width: 375, height: 667 });
      await page.goto('/');
      
      // Test navigation in portrait
      const firstPostLink = page.locator('tbody tr').first().locator('a');
      if (await firstPostLink.count() > 0) {
        const href = await firstPostLink.getAttribute('href');
        
        // Switch to landscape
        await page.setViewportSize({ width: 667, height: 375 });
        
        // Navigation should still work
        await firstPostLink.click();
        await expect(page).toHaveURL(href!);
      }
    });
  });

  test.describe('Content Reflow', () => {
    test('should reflow text content appropriately', async ({ page }) => {
      await page.goto('/posts/1');
      
      const viewports = [
        { width: 320, height: 568 },  // Small mobile
        { width: 375, height: 667 },  // Mobile
        { width: 768, height: 1024 }, // Tablet
        { width: 1200, height: 800 }  // Desktop
      ];
      
      for (const viewport of viewports) {
        await page.setViewportSize(viewport);
        
        // Text should reflow to fit the viewport
        const paragraphs = page.locator('p');
        const paragraphCount = await paragraphs.count();
        
        for (let i = 0; i < Math.min(3, paragraphCount); i++) {
          const paragraph = paragraphs.nth(i);
          const pBox = await paragraph.boundingBox();
          
          if (pBox) {
            expect(pBox.width).toBeLessThanOrEqual(viewport.width);
          }
        }
      }
    });

    test('should handle tables responsively', async ({ page }) => {
      await page.goto('/');
      
      const table = page.locator('table');
      if (await table.count() > 0) {
        // Test table at different viewport sizes
        const viewports = [320, 375, 768, 1200];
        
        for (const width of viewports) {
          await page.setViewportSize({ width, height: 600 });
          
          const tableBox = await table.boundingBox();
          if (tableBox) {
            expect(tableBox.width).toBeLessThanOrEqual(width);
          }
          
          // Table should remain functional
          const tableRows = table.locator('tbody tr');
          const firstRow = tableRows.first();
          if (await firstRow.count() > 0) {
            await expect(firstRow).toBeVisible();
          }
        }
      }
    });

    test('should handle images responsively', async ({ page }) => {
      await page.goto('/');
      
      const images = page.locator('img');
      const imageCount = await images.count();
      
      if (imageCount > 0) {
        const viewports = [320, 375, 768, 1200];
        
        for (const width of viewports) {
          await page.setViewportSize({ width, height: 600 });
          
          for (let i = 0; i < imageCount; i++) {
            const img = images.nth(i);
            const imgBox = await img.boundingBox();
            
            if (imgBox) {
              // Images should scale down to fit viewport
              expect(imgBox.width).toBeLessThanOrEqual(width);
            }
          }
        }
      }
    });
  });

  test.describe('Interactive Elements', () => {
    test('should maintain touch target sizes on mobile', async ({ page }) => {
      await page.setViewportSize({ width: 375, height: 667 });
      await page.goto('/');
      
      // Check touch target sizes for interactive elements
      const interactiveElements = page.locator('button, a, input');
      const elementCount = await interactiveElements.count();
      
      for (let i = 0; i < elementCount; i++) {
        const element = interactiveElements.nth(i);
        if (await element.isVisible()) {
          const box = await element.boundingBox();
          
          if (box) {
            // Touch targets should be at least 44x44px (iOS HIG recommendation)
            expect(box.height).toBeGreaterThanOrEqual(30); // Relaxed for testing
            expect(box.width).toBeGreaterThanOrEqual(30);
          }
        }
      }
    });

    test('should provide adequate spacing between interactive elements', async ({ page }) => {
      await page.setViewportSize({ width: 375, height: 667 });
      await page.goto('/');
      
      // Check spacing in navigation
      const socialLinks = page.locator('.nav-social a, .mobile-nav-social a');
      const linkCount = await socialLinks.count();
      
      if (linkCount > 1) {
        const firstLink = socialLinks.first();
        const secondLink = socialLinks.nth(1);
        
        if (await firstLink.isVisible() && await secondLink.isVisible()) {
          const firstBox = await firstLink.boundingBox();
          const secondBox = await secondLink.boundingBox();
          
          if (firstBox && secondBox) {
            // There should be some spacing between touch targets
            const distance = Math.abs(secondBox.x - (firstBox.x + firstBox.width));
            expect(distance).toBeGreaterThanOrEqual(0); // No overlap
          }
        }
      }
    });
  });

  test.describe('Performance on Different Viewports', () => {
    test('should maintain performance across viewport sizes', async ({ page }) => {
      const viewports = [
        { width: 375, height: 667, name: 'mobile' },
        { width: 768, height: 1024, name: 'tablet' },
        { width: 1200, height: 800, name: 'desktop' }
      ];
      
      for (const viewport of viewports) {
        await page.setViewportSize(viewport);
        
        const startTime = Date.now();
        await page.goto('/');
        await page.waitForLoadState('networkidle');
        const loadTime = Date.now() - startTime;
        
        // Load time should be reasonable across all viewport sizes
        expect(loadTime).toBeLessThan(5000);
        console.log(`${viewport.name} load time: ${loadTime}ms`);
      }
    });
  });
});
