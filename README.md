# Willian's Tech Blog

## 🏗️ Architecture Overview

This application follows a **full-stack Rust architecture** using:

- **Frontend**: Leptos with hydration for interactive client-side features
- **Backend**: Axum web framework with PostgreSQL database
- **Rendering**: Server-side rendering (SSR) with client-side hydration
- **Content**: Markdown-based blog posts with syntax highlighting
- **Build System**: Custom build script for static content generation

## 🚀 Tech Stack

### Core Framework

- **[Leptos 0.8](https://leptos.dev/)** - Reactive web framework for Rust
- **[Axum 0.8](https://docs.rs/axum/)** - Modern async web framework
- **[Tokio](https://tokio.rs/)** - Async runtime

### Database & ORM

- **[PostgreSQL](https://www.postgresql.org/)** - Primary database
- **[SQLx 0.8](https://docs.rs/sqlx/)** - Async SQL toolkit with compile-time checked queries
- **Database Migrations** - Version-controlled schema management

### Content Processing

- **[pulldown-cmark](https://docs.rs/pulldown-cmark/)** - Markdown parser
- **[syntect](https://docs.rs/syntect/)** - Syntax highlighting for code blocks
- **Static Content Generation** - Build-time markdown processing

### Middleware & Security

- **Rate Limiting** - Per-IP request throttling using Governor
- **CORS** - Cross-Origin Resource Sharing configuration
- **CSRF Protection** - Token-based CSRF mitigation
- **Security Headers** - Comprehensive HTTP security headers
- **Request Timeout** - Configurable request timeouts
- **Compression** - Brotli compression for responses

### Development & Deployment

- **[cargo-leptos](https://crates.io/crates/cargo-leptos)** - Leptos build tool
- **Hot Reload** - Development server with live reloading
- **WASM Optimization** - Size-optimized WebAssembly builds
- **End-to-End Testing** - Playwright integration

## 🗂️ Project Structure

```text
src/
├── app.rs                 # Leptos app component and routing
├── client.rs              # Client-side hydration entry point
├── main.rs                # Server entry point
├── lib.rs                 # Library root
├── server.rs              # Server configuration and setup
├── shared.rs              # Shared utilities
├── app/
│   ├── homepage.rs        # Home page component
│   ├── postpage.rs        # Blog post page component
│   ├── nav.rs             # Navigation component
│   ├── footer.rs          # Footer component
│   ├── subscribe_form.rs  # Newsletter subscription form
│   ├── helpers.rs         # UI helper functions
│   └── icons.rs           # Icon components
└── server/
    ├── db/
    │   ├── config.rs      # Database configuration
    │   ├── pool.rs        # Connection pool setup
    │   ├── state.rs       # Application state
    │   └── error.rs       # Database error handling
    ├── middleware/
    │   ├── governor.rs    # Rate limiting
    │   ├── csrf.rs        # CSRF protection
    │   ├── throttle.rs    # Request throttling
    │   └── global_layer/
    │       ├── cors.rs    # CORS configuration
    │       └── security_headers.rs  # Security headers
    ├── models/
    │   └── subscriber.rs  # Data models
    ├── handlers/
    │   └── subscriber.rs  # Request handlers
    ├── services/
    │   └── subscriber.rs  # Business logic
    ├── repositories/
    │   └── subscriber.rs  # Data access layer
    └── routes/
        └── subscriber.rs  # API routes
```

## 🔧 Build System

The project uses a custom build script (`build.rs`) that:

1. **Processes Markdown Files**: Reads blog posts from `contents/posts/`
2. **Syntax Highlighting**: Applies code highlighting using Syntect
3. **Static Generation**: Converts markdown to HTML at build time
4. **Optimized Output**: Generates Rust code with static post data

## 📊 Performance Features

- **Server-Side Rendering (SSR)**: Fast initial page loads
- **Hydration**: Interactive client-side features without full SPA overhead
- **Static Content**: Build-time markdown processing reduces runtime overhead
- **Compression**: Brotli compression for smaller payload sizes
- **Connection Pooling**: Efficient database connection management
- **Request Timeout**: Prevents long-running requests from blocking resources
- **HTTP Caching Strategy**: Multi-tier caching system for optimal performance

## 🗄️ Caching Architecture

The application implements a sophisticated HTTP caching strategy with three distinct cache policies:

### Static Assets Cache
```rust
// Long-term cache for immutable assets
public, max-age=31536000 // 1 year
```
- **Applied to**: WASM files, CSS, JavaScript, images
- **Duration**: 1 year (31,536,000 seconds)
- **Strategy**: Immutable assets with filename-based cache busting
- **Benefits**: Eliminates repeated downloads, reduces bandwidth

### API Response Cache
```rust
// Short-term cache for read-only data
public, max-age=60 // 1 minute
```
- **Applied to**: Blog post listings, static content APIs
- **Duration**: 1 minute (60 seconds)
- **Strategy**: Brief caching for data that changes infrequently
- **Benefits**: Reduces database load, faster response times

### Dynamic Content (No Cache)
```rust
// No caching for sensitive operations
no-store
```
- **Applied to**: Subscription endpoints, form submissions
- **Strategy**: Always fetch fresh data
- **Benefits**: Ensures data consistency for user actions

### Cache Implementation

**Location**: `src/server/middleware/cache.rs`

```rust
use axum::http::header::{CACHE_CONTROL, HeaderValue};
use tower_http::set_header::SetResponseHeaderLayer;

// Three distinct caching strategies
pub fn static_cache_layer() -> SetResponseHeaderLayer<HeaderValue>
pub fn api_cache_layer() -> SetResponseHeaderLayer<HeaderValue>
pub fn no_cache_layer() -> SetResponseHeaderLayer<HeaderValue>
```

**Integration**: Applied as Tower middleware layers in the Axum router

## 🚀 Deployment

### Cloud Deployment (Fly.io)

The application is configured for deployment on **Fly.io** with the following setup:

#### Configuration (`fly.toml`)
```toml
app = 'blog-wtb-9980'
primary_region = 'sin'  # Singapore region

[build]
  dockerfile = 'Containerfile'

[env]
  LEPTOS_ENV = 'PROD'
  LEPTOS_SITE_ROOT = '/app/site'
  LEPTOS_SITE_PKG_DIR = 'pkg'
  LEPTOS_SITE_ADDR = '0.0.0.0:3000'
  RUST_LOG = 'info'

[http_service]
  internal_port = 3000
  force_https = true
  auto_stop_machines = 'stop'
  auto_start_machines = true
  min_machines_running = 0

[[vm]]
  size = 'shared-cpu-1x'
```

#### Database Setup
- **Database**: Fly.io Managed PostgreSQL (`1zvn90kjx74rkpew`)
- **Region**: NRT (Tokyo) for low-latency access from Singapore
- **Connection**: Automatic via `DATABASE_URL` secret
- **Status**: Production-ready with 10GB allocated disk

#### Deployment Commands
```bash
# Deploy to production
flyctl deploy

# Check deployment status
flyctl status

# View logs
flyctl logs

# Scale resources (if needed)
flyctl scale count 2
flyctl scale vm shared-cpu-2x
```

#### Container Build
**Multi-stage Dockerfile** (`Containerfile`):
1. **Builder Stage**: Rust nightly + cargo-leptos installation
2. **Dependency Caching**: Smart layer caching for faster rebuilds
3. **Application Build**: Full release build with optimizations
4. **Runtime Stage**: Minimal Debian slim with only runtime dependencies
5. **Security**: Non-root user execution

**Build Time**: ~25-40 minutes (first build), ~5-15 minutes (subsequent builds)

#### Production Features
- **HTTPS**: Automatic SSL/TLS via Fly.io
- **Health Checks**: Automated health monitoring
- **Auto-scaling**: Automatic machine start/stop based on traffic
- **Zero-downtime**: Rolling deployments
- **Global Edge**: Fly.io's global network for low-latency access

### Local Development

#### Prerequisites
```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install nightly
rustup default nightly

# Install cargo-leptos
cargo install cargo-leptos

# Install PostgreSQL (local development)
sudo dnf install postgresql postgresql-server  # Fedora
# OR
brew install postgresql  # macOS
```

#### Setup & Run
```bash
# Clone and setup
git clone <repository>
cd blog

# Setup local database
creatuser postgres
createdb blog_dev

# Run migrations
cargo install sqlx-cli
sqlx migrate run

# Development server with hot reload
cargo leptos watch

# Production build
cargo leptos build --release

# Container build (for testing)
podman build -f Containerfile -t blog:latest .
podman run -p 3000:3000 blog:latest
```

#### Environment Configuration
- **Local**: Uses `.env` file with local PostgreSQL
- **Production**: Uses Fly.io secrets (overrides `.env`)
- **Database URL**: Automatically managed per environment

### Performance Characteristics

#### Build Performance
- **Local Development**: Incremental compilation ~10-30s
- **Container Build**: Full release build ~25-40 minutes
- **Deployment**: ~2-5 minutes after container build

#### Runtime Performance
- **Cold Start**: <500ms (Fly.io machine startup)
- **Response Time**: <100ms for cached content
- **Database**: <50ms queries (regional proximity)
- **Static Assets**: Served via CDN with 1-year cache

## 🚧 Future Features

### Centralized Error Handling

**Status**: Planned for future implementation

**Description**: A comprehensive error handling middleware layer that would:

- Catch and handle all middleware errors (timeouts, panics, etc.)
- Provide consistent error responses across the application
- Log errors appropriately with different severity levels
- Transform technical errors into user-friendly messages

**Current Challenge**: Implementing a robust `HandleErrorLayer` in Axum requires careful composition with other middleware layers. The layer needs to be correctly positioned in the middleware stack and handle the specific error types produced by each layer (e.g., `tower::timeout::error::Elapsed` for timeout errors).

**Development Assessment**:

- **Pros**: Better user experience, improved debugging, consistent error responses
- **Cons**: Complex implementation with Axum's type system, requires significant development effort for a simple blog application
- **Decision**: Postponed until the application grows in complexity and the benefits outweigh the development overhead

## 🧪 Testing

### End-to-End Testing

The project includes comprehensive E2E testing using **Playwright**:

```bash
cd end2end

# Install dependencies
npm install

# Run all tests
npx playwright test

# Run specific performance tests
npx playwright test --grep "should load within acceptable time|should achieve good"

# Run tests with UI (headed mode)
npm run test:headed

# Run a specific test file
npx playwright test example.spec.ts

# Generate test report
npx playwright show-report
```

### Test Coverage

- **Performance Testing**: Page load times, Core Web Vitals
- **Functionality Testing**: Navigation, form submissions, content rendering
- **Cross-browser Testing**: Chromium, Firefox, WebKit
- **Responsive Testing**: Multiple viewport sizes

### CI/CD Integration

Tests are designed to run in CI/CD pipelines with:
- Automated test execution on pull requests
- Performance regression detection
- Visual regression testing capabilities
