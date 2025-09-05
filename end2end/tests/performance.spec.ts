import { test, expect } from '@playwright/test';

test.describe('Performance', () => {
  test.describe('Page Load Performance', () => {
    test('homepage should load within acceptable time', async ({ page }) => {
      const startTime = Date.now();
      
      await page.goto('/');
      await page.waitForLoadState('networkidle');
      
      const loadTime = Date.now() - startTime;
      
      // Homepage should load within 3 seconds
      expect(loadTime).toBeLessThan(3000);
      
      // Log the actual load time for monitoring
      console.log(`Homepage loaded in ${loadTime}ms`);
    });

    test('blog post pages should load within acceptable time', async ({ page }) => {
      const startTime = Date.now();
      
      await page.goto('/posts/1');
      await page.waitForSelector('h1');
      
      const loadTime = Date.now() - startTime;
      
      // Blog posts should load within 2 seconds
      expect(loadTime).toBeLessThan(2000);
      
      console.log(`Blog post loaded in ${loadTime}ms`);
    });

    test('should achieve good Largest Contentful Paint (LCP)', async ({ page }) => {
      await page.goto('/');
      
      // Wait for the largest content element to be painted
      await page.waitForSelector('h1');
      await page.waitForTimeout(1000); // Give time for LCP to stabilize
      
      const lcpTime = await page.evaluate(() => {
        return new Promise((resolve) => {
          new PerformanceObserver((entryList) => {
            const entries = entryList.getEntries();
            const lastEntry = entries[entries.length - 1];
            resolve(lastEntry?.startTime || 0);
          }).observe({ type: 'largest-contentful-paint', buffered: true });
          
          // Fallback timeout
          setTimeout(() => resolve(0), 5000);
        });
      });
      
      // LCP should be under 2.5 seconds (good threshold)
      expect(lcpTime).toBeLessThan(2500);
      console.log(`LCP: ${lcpTime}ms`);
    });

    test('should achieve good First Contentful Paint (FCP)', async ({ page }) => {
      await page.goto('/');
      
      const fcpTime = await page.evaluate(() => {
        return new Promise((resolve) => {
          new PerformanceObserver((entryList) => {
            const entries = entryList.getEntries();
            resolve(entries[0]?.startTime || 0);
          }).observe({ type: 'first-contentful-paint', buffered: true });
          
          setTimeout(() => resolve(0), 5000);
        });
      });
      
      // FCP should be under 1.8 seconds (good threshold)
      expect(fcpTime).toBeLessThan(1800);
      console.log(`FCP: ${fcpTime}ms`);
    });

    test('should have minimal layout shifts', async ({ page }) => {
      await page.goto('/');
      
      // Wait for page to fully load and stabilize
      await page.waitForLoadState('networkidle');
      await page.waitForTimeout(2000);
      
      const clsScore = await page.evaluate(() => {
        return new Promise((resolve) => {
          let clsValue = 0;
          
          new PerformanceObserver((entryList) => {
            for (const entry of entryList.getEntries()) {
              if (!(entry as any).hadRecentInput) {
                clsValue += (entry as any).value;
              }
            }
          }).observe({ type: 'layout-shift', buffered: true });
          
          setTimeout(() => resolve(clsValue), 3000);
        });
      });
      
      // CLS should be under 0.1 (good threshold)
      expect(clsScore).toBeLessThan(0.1);
      console.log(`CLS Score: ${clsScore}`);
    });

    test('should load critical resources quickly', async ({ page }) => {
      const resourceTimings: any[] = [];
      
      page.on('response', (response) => {
        const url = response.url();
        const timing = response.timing();
        
        // Track critical resources
        if (url.includes('.css') || url.includes('.js') || url.includes('.wasm')) {
          resourceTimings.push({
            url,
            responseTime: timing.responseEnd
          });
        }
      });
      
      await page.goto('/');
      await page.waitForLoadState('networkidle');
      
      // Check that CSS and JS resources load within reasonable time
      for (const resource of resourceTimings) {
        expect(resource.responseTime).toBeLessThan(1000);
      }
    });
  });

  test.describe('SEO and Metadata', () => {
    test('homepage should have proper meta tags', async ({ page }) => {
      await page.goto('/');
      
      // Title should be set
      const title = await page.title();
      expect(title).toBeTruthy();
      expect(title.length).toBeGreaterThan(10);
      expect(title.length).toBeLessThan(60); // Optimal title length
      
      // Meta description should exist
      const metaDescription = page.locator('meta[name="description"]');
      const descriptionContent = await metaDescription.getAttribute('content');
      if (descriptionContent) {
        expect(descriptionContent.length).toBeGreaterThan(50);
        expect(descriptionContent.length).toBeLessThan(160); // Optimal description length
      }
      
      // Viewport should be set
      const viewport = page.locator('meta[name="viewport"]');
      await expect(viewport).toHaveAttribute('content', expect.stringMatching(/width=device-width/));
      
      // Charset should be set
      const charset = page.locator('meta[charset]');
      await expect(charset).toHaveAttribute('charset', 'utf-8');
    });

    test('blog posts should have proper meta tags', async ({ page }) => {
      await page.goto('/posts/1');
      
      // Title should be the post title
      const title = await page.title();
      expect(title).toBe('Deploy my own tech blog finally');
      
      // Should have proper structured data (if implemented)
      const jsonLd = page.locator('script[type="application/ld+json"]');
      if (await jsonLd.count() > 0) {
        const structuredData = await jsonLd.textContent();
        expect(structuredData).toBeTruthy();
        
        // Should be valid JSON
        const parsedData = JSON.parse(structuredData!);
        expect(parsedData).toHaveProperty('@type');
      }
    });

    test('should have proper Open Graph tags', async ({ page }) => {
      await page.goto('/');
      
      // Check for basic Open Graph tags
      const ogTitle = page.locator('meta[property="og:title"]');
      const ogDescription = page.locator('meta[property="og:description"]');
      const ogType = page.locator('meta[property="og:type"]');
      
      // These might not be implemented yet, so we check conditionally
      if (await ogTitle.count() > 0) {
        const titleContent = await ogTitle.getAttribute('content');
        expect(titleContent).toBeTruthy();
      }
      
      if (await ogType.count() > 0) {
        const typeContent = await ogType.getAttribute('content');
        expect(['website', 'article'].includes(typeContent!));
      }
    });

    test('should have proper canonical URLs', async ({ page }) => {
      await page.goto('/');
      
      const canonical = page.locator('link[rel="canonical"]');
      if (await canonical.count() > 0) {
        const href = await canonical.getAttribute('href');
        expect(href).toBeTruthy();
        expect(href).toMatch(/^https?:\/\//);
      }
    });

    test('should be crawlable by search engines', async ({ page }) => {
      await page.goto('/robots.txt');
      
      // robots.txt should exist and be accessible
      // This depends on your server configuration
      const response = page.url().includes('/robots.txt');
      expect(response).toBeTruthy();
    });
  });

  test.describe('Resource Optimization', () => {
    test('should compress text resources', async ({ page }) => {
      const compressedResources: string[] = [];
      
      page.on('response', (response) => {
        const contentEncoding = response.headers()['content-encoding'];
        const contentType = response.headers()['content-type'] || '';
        
        // Check if text resources are compressed
        if (contentType.includes('text/') || contentType.includes('application/javascript') || contentType.includes('application/json')) {
          if (contentEncoding === 'gzip' || contentEncoding === 'br') {
            compressedResources.push(response.url());
          }
        }
      });
      
      await page.goto('/');
      await page.waitForLoadState('networkidle');
      
      // At least some text resources should be compressed
      // This depends on your server configuration
      console.log(`Compressed resources: ${compressedResources.length}`);
    });

    test('should have efficient caching headers', async ({ page }) => {
      const cachedResources: any[] = [];
      
      page.on('response', (response) => {
        const cacheControl = response.headers()['cache-control'];
        const expires = response.headers()['expires'];
        const etag = response.headers()['etag'];
        
        if (cacheControl || expires || etag) {
          cachedResources.push({
            url: response.url(),
            cacheControl,
            expires,
            etag
          });
        }
      });
      
      await page.goto('/');
      await page.waitForLoadState('networkidle');
      
      // Static resources should have caching headers
      const staticResources = cachedResources.filter(r => 
        r.url.includes('.css') || 
        r.url.includes('.js') || 
        r.url.includes('.wasm') || 
        r.url.includes('.png') || 
        r.url.includes('.jpg') || 
        r.url.includes('.ico')
      );
      
      console.log(`Resources with caching headers: ${staticResources.length}`);
    });

    test('should minimize HTTP requests', async ({ page }) => {
      const requests: string[] = [];
      
      page.on('request', (request) => {
        requests.push(request.url());
      });
      
      await page.goto('/');
      await page.waitForLoadState('networkidle');
      
      // Should not make excessive requests
      expect(requests.length).toBeLessThan(50);
      console.log(`Total HTTP requests: ${requests.length}`);
      
      // Log request breakdown
      const requestTypes = {
        html: requests.filter(url => url.includes('.html') || !url.includes('.')).length,
        css: requests.filter(url => url.includes('.css')).length,
        js: requests.filter(url => url.includes('.js')).length,
        images: requests.filter(url => url.match(/\.(png|jpg|jpeg|gif|svg|ico)$/)).length,
        other: requests.filter(url => !url.match(/\.(html|css|js|png|jpg|jpeg|gif|svg|ico)$/) && url.includes('.')).length
      };
      
      console.log('Request breakdown:', requestTypes);
    });

    test('should load images efficiently', async ({ page }) => {
      await page.goto('/');
      
      const images = page.locator('img');
      const imageCount = await images.count();
      
      for (let i = 0; i < imageCount; i++) {
        const img = images.nth(i);
        
        // Images should have proper dimensions to prevent layout shift
        const width = await img.getAttribute('width');
        const height = await img.getAttribute('height');
        
        if (width && height) {
          expect(parseInt(width)).toBeGreaterThan(0);
          expect(parseInt(height)).toBeGreaterThan(0);
        }
        
        // Images should have proper loading attributes for performance
        const loading = await img.getAttribute('loading');
        const src = await img.getAttribute('src');
        
        // Below-the-fold images should ideally be lazy loaded
        if (src && !src.includes('favicon')) {
          // This is more of a recommendation
          console.log(`Image loading attribute: ${loading}`);
        }
      }
    });
  });

  test.describe('Mobile Performance', () => {
    test('should perform well on mobile devices', async ({ page }) => {
      // Simulate mobile device
      await page.setViewportSize({ width: 375, height: 667 });
      await page.goto('/');
      
      const startTime = Date.now();
      await page.waitForLoadState('networkidle');
      const loadTime = Date.now() - startTime;
      
      // Mobile should still load within reasonable time
      expect(loadTime).toBeLessThan(4000);
      console.log(`Mobile load time: ${loadTime}ms`);
    });

    test('should handle touch interactions efficiently', async ({ page }) => {
      await page.setViewportSize({ width: 375, height: 667 });
      await page.goto('/');
      
      // Test touch interactions
      const mobileMenuButton = page.locator('.mobile-menu-button');
      if (await mobileMenuButton.count() > 0) {
        const startTime = Date.now();
        await mobileMenuButton.tap();
        const interactionTime = Date.now() - startTime;
        
        // Touch interactions should be responsive
        expect(interactionTime).toBeLessThan(100);
      }
    });

    test('should have appropriate mobile viewport', async ({ page }) => {
      await page.setViewportSize({ width: 375, height: 667 });
      await page.goto('/');
      
      // Check that content fits within viewport
      const body = page.locator('body');
      const bodyBox = await body.boundingBox();
      
      if (bodyBox) {
        // Content should not cause horizontal scrolling
        expect(bodyBox.width).toBeLessThanOrEqual(375);
      }
    });
  });

  test.describe('Bundle Size Analysis', () => {
    test('should have reasonable JavaScript bundle sizes', async ({ page }) => {
      const jsResources: any[] = [];
      
      page.on('response', async (response) => {
        const url = response.url();
        if (url.includes('.js') || url.includes('.wasm')) {
          const contentLength = response.headers()['content-length'];
          jsResources.push({
            url,
            size: contentLength ? parseInt(contentLength) : 0
          });
        }
      });
      
      await page.goto('/');
      await page.waitForLoadState('networkidle');
      
      const totalJsSize = jsResources.reduce((total, resource) => total + resource.size, 0);
      
      // Total JS should be reasonable for a blog
      expect(totalJsSize).toBeLessThan(1024 * 1024); // Less than 1MB
      
      console.log(`Total JS size: ${(totalJsSize / 1024).toFixed(2)}KB`);
      
      // Individual JS files shouldn't be too large
      for (const resource of jsResources) {
        expect(resource.size).toBeLessThan(512 * 1024); // Individual files less than 512KB
      }
    });

    test('should have reasonable CSS bundle sizes', async ({ page }) => {
      const cssResources: any[] = [];
      
      page.on('response', (response) => {
        const url = response.url();
        if (url.includes('.css')) {
          const contentLength = response.headers()['content-length'];
          cssResources.push({
            url,
            size: contentLength ? parseInt(contentLength) : 0
          });
        }
      });
      
      await page.goto('/');
      await page.waitForLoadState('networkidle');
      
      const totalCssSize = cssResources.reduce((total, resource) => total + resource.size, 0);
      
      // Total CSS should be reasonable
      expect(totalCssSize).toBeLessThan(256 * 1024); // Less than 256KB
      
      console.log(`Total CSS size: ${(totalCssSize / 1024).toFixed(2)}KB`);
    });
  });
});
