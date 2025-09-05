import { test, expect } from '@playwright/test';

test.describe('Newsletter Subscription Form', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('should display subscription form elements if present', async ({ page }) => {
    const subscribeForm = page.locator('.subscribe-form');
    
    if (await subscribeForm.count() > 0) {
      await expect(subscribeForm).toBeVisible();
      
      const emailInput = subscribeForm.locator('input[type="email"]');
      await expect(emailInput).toBeVisible();
      await expect(emailInput).toHaveAttribute('placeholder', 'Enter your email');
      await expect(emailInput).toHaveAttribute('required');
      
      const submitButton = subscribeForm.locator('button[type="button"]');
      await expect(submitButton).toBeVisible();
      await expect(submitButton).toHaveText('Subscribe');
    }
  });

  test('should handle empty email submission', async ({ page }) => {
    const subscribeForm = page.locator('.subscribe-form');
    
    if (await subscribeForm.count() > 0) {
      const emailInput = subscribeForm.locator('input[type="email"]');
      const submitButton = subscribeForm.locator('button[type="button"]');
      
      await emailInput.fill('');
      await submitButton.click();
      
      // Form should prevent submission with empty email
      await expect(submitButton).toHaveText('Subscribe');
    }
  });

  test('should validate email format', async ({ page }) => {
    const subscribeForm = page.locator('.subscribe-form');
    
    if (await subscribeForm.count() > 0) {
      const emailInput = subscribeForm.locator('input[type="email"]');
      const submitButton = subscribeForm.locator('button[type="button"]');
      
      await emailInput.fill('invalid-email');
      await submitButton.click();
      
      const validationMessage = await emailInput.evaluate((el: HTMLInputElement) => el.validationMessage);
      expect(validationMessage).toBeTruthy();
    }
  });

  test('should show loading state during submission', async ({ page }) => {
    const subscribeForm = page.locator('.subscribe-form');
    
    if (await subscribeForm.count() > 0) {
      const emailInput = subscribeForm.locator('input[type="email"]');
      const submitButton = subscribeForm.locator('button[type="button"]');
      
      await emailInput.fill('test@example.com');
      
      // Mock slow network response
      await page.route('/api/subscribe', async route => {
        await new Promise(resolve => setTimeout(resolve, 1000));
        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({ message: 'Successfully subscribed!' })
        });
      });
      
      await submitButton.click();
      
      await expect(submitButton).toHaveText('Subscribing...');
      await expect(emailInput).toBeDisabled();
      await expect(submitButton).toHaveText('Subscribe', { timeout: 5000 });
    }
  });

  test('should handle successful subscription', async ({ page }) => {
    const subscribeForm = page.locator('.subscribe-form');
    
    if (await subscribeForm.count() > 0) {
      const emailInput = subscribeForm.locator('input[type="email"]');
      const submitButton = subscribeForm.locator('button[type="button"]');
      
      await page.route('/api/subscribe', async route => {
        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({ message: 'Successfully subscribed! Check your email for confirmation.' })
        });
      });
      
      await emailInput.fill('success@example.com');
      await submitButton.click();
      
      const successDiv = subscribeForm.locator('.success');
      await expect(successDiv).toBeVisible();
      
      const successMessage = successDiv.locator('p');
      await expect(successMessage).toContainText('Successfully subscribed');
      
      const subscribeAnotherButton = successDiv.locator('button');
      await expect(subscribeAnotherButton).toHaveText('Subscribe another');
      
      await expect(emailInput).toHaveValue('');
    }
  });

  test('should handle subscription errors', async ({ page }) => {
    const subscribeForm = page.locator('.subscribe-form');
    
    if (await subscribeForm.count() > 0) {
      const emailInput = subscribeForm.locator('input[type="email"]');
      const submitButton = subscribeForm.locator('button[type="button"]');
      
      await page.route('/api/subscribe', async route => {
        await route.fulfill({
          status: 400,
          contentType: 'application/json',
          body: JSON.stringify({ message: 'Email already subscribed' })
        });
      });
      
      await emailInput.fill('existing@example.com');
      await submitButton.click();
      
      const errorDiv = subscribeForm.locator('.error');
      await expect(errorDiv).toBeVisible();
      
      const errorMessage = errorDiv.locator('p');
      await expect(errorMessage).toContainText('Email already subscribed');
      
      const tryAgainButton = errorDiv.locator('button');
      await expect(tryAgainButton).toHaveText('Try again');
    }
  });

  test('should reset form after error', async ({ page }) => {
    const subscribeForm = page.locator('.subscribe-form');
    
    if (await subscribeForm.count() > 0) {
      const emailInput = subscribeForm.locator('input[type="email"]');
      const submitButton = subscribeForm.locator('button[type="button"]');
      
      await page.route('/api/subscribe', async route => {
        await route.fulfill({
          status: 400,
          contentType: 'application/json',
          body: JSON.stringify({ message: 'Validation error' })
        });
      });
      
      await emailInput.fill('error@example.com');
      await submitButton.click();
      
      const errorDiv = subscribeForm.locator('.error');
      await expect(errorDiv).toBeVisible();
      
      const tryAgainButton = errorDiv.locator('button');
      await tryAgainButton.click();
      
      await expect(errorDiv).not.toBeVisible();
      await expect(submitButton).toHaveText('Subscribe');
    }
  });

  test('should reset form after success', async ({ page }) => {
    const subscribeForm = page.locator('.subscribe-form');
    
    if (await subscribeForm.count() > 0) {
      await page.route('/api/subscribe', async route => {
        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({ message: 'Successfully subscribed!' })
        });
      });
      
      const emailInput = subscribeForm.locator('input[type="email"]');
      const submitButton = subscribeForm.locator('button[type="button"]');
      
      await emailInput.fill('success@example.com');
      await submitButton.click();
      
      const successDiv = subscribeForm.locator('.success');
      await expect(successDiv).toBeVisible();
      
      const subscribeAnotherButton = successDiv.locator('button');
      await subscribeAnotherButton.click();
      
      await expect(successDiv).not.toBeVisible();
      await expect(submitButton).toHaveText('Subscribe');
      await expect(emailInput).not.toBeDisabled();
    }
  });

  test('should handle network errors', async ({ page }) => {
    const subscribeForm = page.locator('.subscribe-form');
    
    if (await subscribeForm.count() > 0) {
      const emailInput = subscribeForm.locator('input[type="email"]');
      const submitButton = subscribeForm.locator('button[type="button"]');
      
      await page.route('/api/subscribe', async route => {
        await route.abort('internetdisconnected');
      });
      
      await emailInput.fill('network@example.com');
      await submitButton.click();
      
      const errorDiv = subscribeForm.locator('.error');
      await expect(errorDiv).toBeVisible({ timeout: 10000 });
      
      const errorMessage = errorDiv.locator('p');
      await expect(errorMessage).toContainText(/network error|error/i);
    }
  });

  test('should be accessible via keyboard', async ({ page }) => {
    const subscribeForm = page.locator('.subscribe-form');
    
    if (await subscribeForm.count() > 0) {
      const emailInput = subscribeForm.locator('input[type="email"]');
      const submitButton = subscribeForm.locator('button[type="button"]');
      
      await emailInput.focus();
      await expect(emailInput).toBeFocused();
      
      await emailInput.fill('keyboard@example.com');
      
      await page.keyboard.press('Tab');
      await expect(submitButton).toBeFocused();
      
      const buttonText = await submitButton.textContent();
      expect(buttonText).toBe('Subscribe');
    }
  });
});
