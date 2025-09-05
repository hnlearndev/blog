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

# Run only the passing tests

cd end2end
npx playwright test --grep "should load within acceptable time|should achieve good"

# Run tests with UI to see what's happening

npm run test:headed

# Run a single test file that works well

npx playwright test example.spec.ts
