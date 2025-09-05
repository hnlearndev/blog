# End-to-End Testing for Willian's Tech Blog

This directory contains comprehensive end-to-end (E2E) tests for the blog application built with Leptos and Axum. The tests are built using [Playwright](https://playwright.dev/), a modern testing framework that provides reliable cross-browser testing.

## 🧪 Test Structure

The test suite is organized into several focused test files:

### Core Functionality Tests
- **`example.spec.ts`** - Basic smoke test to verify the blog is accessible and functional
- **`homepage.spec.ts`** - Comprehensive homepage testing including layout, navigation, and blog post listings
- **`blog-posts.spec.ts`** - Individual blog post page testing, content rendering, and markdown features
- **`navigation.spec.ts`** - Desktop and mobile navigation, social links, and menu interactions

### Advanced Feature Tests
- **`subscription.spec.ts`** - Newsletter subscription form functionality, validation, and error handling
- **`accessibility.spec.ts`** - Keyboard navigation, ARIA labels, and accessibility compliance
- **`performance.spec.ts`** - Page load times, SEO metadata, and Core Web Vitals
- **`responsive.spec.ts`** - Mobile and desktop layouts, responsive behavior across devices

## 🚀 Getting Started

### Prerequisites
- Node.js (version 16 or higher)
- Your blog application running on `localhost:3000`

### Installation
```bash
cd end2end
npm install
npm run install  # Installs Playwright browsers
```

### Running Tests

#### Basic Test Execution
```bash
# Run all tests headlessly
npm test

# Run tests with browser UI visible
npm run test:headed

# Run tests in interactive UI mode
npm run test:ui

# Run specific test file
npx playwright test homepage.spec.ts

# Run tests matching a pattern
npx playwright test --grep "should display"
```

#### Development & Debugging
```bash
# Debug mode (opens browser dev tools)
npm run test:debug

# Run specific test in debug mode
npx playwright test homepage.spec.ts --debug

# Generate and view test report
npm run test:report
```

#### Browser-Specific Testing
```bash
# Run on specific browser
npx playwright test --project=chromium
npx playwright test --project=firefox
npx playwright test --project=webkit

# Run on mobile devices
npx playwright test --project="Mobile Chrome"
```

## 📋 Test Coverage

### 🏠 Homepage Tests (`homepage.spec.ts`)
- **Meta tags and SEO** - Title, description, viewport
- **Content structure** - Heading, introduction text, blog posts table
- **Navigation functionality** - Links, routing, page transitions  
- **Performance** - Load times under 3 seconds
- **Semantic HTML** - Proper landmark structure

### 📝 Blog Posts Tests (`blog-posts.spec.ts`)
- **Content rendering** - Markdown to HTML conversion
- **Syntax highlighting** - Code blocks with proper formatting
- **Navigation** - Browser back/forward, internal linking
- **Error handling** - Invalid post IDs, graceful degradation
- **Performance** - Individual post load times under 2 seconds

### 🧭 Navigation Tests (`navigation.spec.ts`)
- **Desktop navigation** - Brand logo, social links, proper layout
- **Mobile navigation** - Hamburger menu, touch interactions, responsive design
- **Cross-page consistency** - Navigation elements across all pages
- **Keyboard accessibility** - Tab navigation, focus management
- **External links** - Proper `target="_blank"` and security attributes

### 📧 Subscription Form Tests (`subscription.spec.ts`)
- **Form validation** - Email format, required fields, empty submission
- **API integration** - Success/error states, loading indicators
- **User feedback** - Clear success/error messages, form reset
- **Keyboard accessibility** - Tab navigation, form submission
- **Network error handling** - Graceful degradation for connectivity issues

### ♿ Accessibility Tests (`accessibility.spec.ts`)
- **Keyboard navigation** - Full site accessibility via keyboard
- **ARIA attributes** - Proper labels, roles, and semantic markup
- **Screen reader support** - Heading hierarchy, landmark navigation
- **Focus management** - Visible focus indicators, logical tab order
- **Image accessibility** - Alt text, proper dimensions
- **Form accessibility** - Labels, error messaging, validation feedback

### ⚡ Performance Tests (`performance.spec.ts`)
- **Core Web Vitals** - LCP (<2.5s), FCP (<1.8s), CLS (<0.1)
- **Load performance** - Page load times, resource loading
- **SEO optimization** - Meta tags, structured data, canonical URLs
- **Bundle analysis** - JavaScript/CSS size limits, compression
- **Mobile performance** - Touch interactions, viewport optimization

### 📱 Responsive Design Tests (`responsive.spec.ts`)
- **Breakpoint testing** - Desktop (1200px), tablet (768px), mobile (375px)
- **Layout adaptation** - Content reflow, navigation changes
- **Touch interactions** - Proper touch target sizes (44px minimum)
- **Orientation handling** - Portrait/landscape transitions
- **Content overflow** - Text wrapping, image scaling, table responsiveness

## ⚙️ Configuration

The tests are configured via `playwright.config.ts`:

### Key Configuration Features
- **Base URL**: `http://localhost:3000`
- **Auto server startup**: Automatically runs `cargo leptos serve --release`
- **Cross-browser testing**: Chromium, Firefox, WebKit
- **Failure artifacts**: Screenshots, videos, traces on failure
- **Timeouts**: 30s test timeout, 5s assertion timeout
- **Parallel execution**: Full parallelization for faster test runs

### Environment Setup
The test configuration automatically:
1. Starts your Leptos server before running tests
2. Waits for the server to be available (up to 2 minutes)
3. Reuses existing server in development
4. Captures failure artifacts for debugging

## 🐛 Debugging Tests

### Visual Debugging
```bash
# Run with browser visible
npm run test:headed

# Open Playwright UI for interactive testing
npm run test:ui

# Debug specific failing test
npx playwright test homepage.spec.ts --debug
```

### Failure Analysis
When tests fail, Playwright automatically captures:
- **Screenshots** - Visual state at failure
- **Videos** - Full test execution recording  
- **Traces** - Detailed timeline with network requests
- **Console logs** - Browser console output

View these artifacts:
```bash
npm run test:report
```

### Common Issues & Solutions

**Server not starting:**
```bash
# Ensure your blog builds successfully
cargo leptos build --release

# Check if port 3000 is available
lsof -i :3000
```

**Tests timing out:**
```bash
# Increase timeout in playwright.config.ts
timeout: 60 * 1000  // 60 seconds
```

**Element not found:**
```bash
# Use Playwright's debug mode to inspect selectors
npx playwright test --debug
```

## 📊 Test Reports

### HTML Report
```bash
npm run test:report
```
Generates a comprehensive HTML report with:
- Test results by browser
- Failure details with screenshots
- Performance metrics
- Test execution timeline

### CI/CD Integration
The tests are configured for CI environments:
- Reduced parallelization on CI (`workers: 1`)
- Retry failed tests (2 retries on CI)
- Forbid `test.only` in CI builds
- Optimized for GitHub Actions, GitLab CI, etc.

## 🔧 Maintenance

### Adding New Tests
1. Create new `.spec.ts` file in `tests/` directory
2. Follow naming convention: `feature-name.spec.ts`
3. Use descriptive test names and organize with `describe()` blocks
4. Include both positive and negative test cases

### Updating Selectors
When UI changes break tests:
1. Use Playwright's inspector: `npx playwright test --debug`
2. Update selectors to be more resilient (prefer `data-testid`, semantic selectors)
3. Test across all browsers after updates

### Performance Baselines
Review and update performance thresholds regularly:
- Homepage load time: <3 seconds
- Blog post load time: <2 seconds
- LCP: <2.5 seconds
- CLS: <0.1

## 🎯 Best Practices

### Test Organization
- **One concern per test** - Each test should verify a single functionality
- **Descriptive names** - Test names should clearly state what they verify
- **Setup/teardown** - Use `beforeEach` for common setup
- **Page Object Model** - Consider extracting complex selectors to page objects

### Reliability
- **Wait for elements** - Always wait for elements before interaction
- **Stable selectors** - Prefer semantic selectors over brittle CSS selectors
- **Mock external services** - Mock APIs, third-party services for consistent results
- **Independent tests** - Each test should be able to run independently

### Performance
- **Parallel execution** - Tests run in parallel by default
- **Selective testing** - Use `--grep` to run specific test suites during development
- **Browser reuse** - Playwright reuses browser contexts for efficiency

This comprehensive test suite ensures your blog application works reliably across all devices, browsers, and user scenarios while maintaining high performance and accessibility standards.
