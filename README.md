# Willian's Personal Blog

The blog is at [wiiliannguyen.com](https://www.williannguyen.com)

## Architecture Overview

This application follows a **full-stack Rust architecture** using:

- **Frontend**: Leptos with hydration for interactive client-side features
- **Backend**: Axum web framework with PostgreSQL database
- **Rendering**: Server-side rendering (SSR) with client-side hydration
- **Content**: Markdown-based blog posts with syntax highlighting
- **Build System**: Custom build script for static content generation

## Tech Stack

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

## Project Structure

### Frontend (Leptos)

```text
src/
├── app.rs                 # App component and routing
├── client.rs              # Client-side hydration entry
├── app/
│   ├── components.rs      # Component module definitions
│   ├── pages.rs           # Page module definitions
│   ├── helpers.rs         # UI utility functions
│   ├── components/
│   │   ├── nav.rs         # Navigation component
│   │   ├── footer.rs      # Footer component
│   │   ├── theme_toggle.rs # Theme switcher
│   │   ├── subscribe_form.rs # Newsletter form
│   │   ├── content_list.rs # Content listing
│   │   ├── fast_a.rs      # Optimized anchor component
│   │   └── icons.rs       # Icon components
│   └── pages/
│       ├── homepage.rs    # Home page
│       ├── postpage.rs    # Blog post page
│       └── poempage.rs    # Poetry page
```

### Backend Architecture

```text
├── main.rs                # Application entry point
├── server.rs              # Server orchestration & middleware stack
└── server/                # Modular backend architecture
    ├── db.rs              # Database module coordinator
    ├── db/
    │   ├── config.rs      # Database URL & connection config
    │   ├── pool.rs        # PgPool initialization & management
    │   ├── state.rs       # AppState with shared resources
    │   └── error.rs       # Database-specific error handling
    ├── middleware.rs      # Middleware module coordinator
    ├── middleware/
    │   ├── cache.rs       # HTTP caching strategies
    │   ├── governor.rs    # Rate limiting (IP-based)
    │   ├── csrf.rs        # CSRF token protection
    │   ├── throttle.rs    # Request throttling
    │   ├── global_layer.rs # Middleware layer coordinator
    │   └── global_layer/
    │       ├── cors.rs    # Cross-Origin Resource Sharing
    │       └── security_headers.rs # Security headers middleware
    ├── models.rs          # Data model coordinator
    ├── models/
    │   └── subscriber.rs  # Newsletter subscriber model
    ├── repositories.rs    # Data access coordinator
    ├── repositories/
    │   └── subscriber.rs  # Database queries & data access
    ├── services.rs        # Business logic coordinator
    ├── services/
    │   └── subscriber.rs  # Newsletter business logic
    ├── handlers.rs        # Request handler coordinator
    ├── handlers/
    │   └── subscriber.rs  # HTTP request/response handling
    ├── routes.rs          # API route coordinator
    └── routes/
        └── subscriber.rs  # Newsletter API endpoints
```

### Backend Layer Relationships

```text
Data Flow & Dependencies:

┌──────────────────────────────────┐
│          Middleware Stack        │  Global & Route-specific middleware
│  (Applied in server.rs & routes/)│
└──────────────────────────────────┘
                  │
                  ▼
┌─────────────┐     ┌─────────────┐
│  Handlers   │───▶│   Routes    │  Routes use handlers
└─────────────┘     └─────────────┘
      ▲
      │ Handlers use services
┌─────────────┐
│  Services   │
└─────────────┘
      ▲
      │ Services use repositories
┌─────────────┐
│Repositories │
└─────────────┘
    ▲       ▲
    │       │ Repositories use both models and DB
┌───────┐  ┌───────┐
│ Models│  |  DB   │
└───────┘  └───────┘

Middleware Implementation:
• Global: Applied in server.rs (compression, timeout, CORS, security headers)
• Route-specific: Applied in routes/ modules (rate limiting, CSRF, caching)
  Example: subscriber routes apply no_cache, governor, throttle, and CSRF layers

Layer Responsibilities:
• Routes: HTTP endpoints + middleware application, delegate to handlers
• Handlers: HTTP request/response processing, input validation
• Services: Business logic, orchestration, transaction management
• Repositories: Data access queries, DB operations using models
• Models: Data structures, serialization, validation rules
• DB: Connection pooling, configuration, state management

```

### Server.rs Architecture Flow

```text
server::run() execution flow:
1. Environment & logging setup
2. Database pool initialization
3. AppState creation with shared resources
4. Leptos route generation
5. Middleware stack assembly (outermost → innermost):
   ├── CompressionLayer (Brotli)
   ├── TimeoutLayer (30s)
   ├── TraceLayer (request logging)
   ├── CORS layer
   └── Security headers
6. Leptos routes integration
7. API routes merging (subscriber endpoints)
8. Server binding & startup
```

## Build System

The project uses a custom build script (`build.rs`) that:

1. **Processes Markdown Files**: Reads blog posts from `contents/posts/`
2. **Syntax Highlighting**: Applies code highlighting using Syntect
3. **Static Generation**: Converts markdown to HTML at build time
4. **Optimized Output**: Generates Rust code with static post data

## Performance Features

- **Server-Side Rendering (SSR)**: Fast initial page loads
- **Hydration**: Interactive client-side features without full SPA overhead
- **Static Content**: Build-time markdown processing reduces runtime overhead
- **Compression**: Brotli compression for smaller payload sizes
- **Connection Pooling**: Efficient database connection management
- **Request Timeout**: Prevents long-running requests from blocking resources
- **HTTP Caching Strategy**: Multi-tier caching system for optimal performance

## 🗄️ Caching

- **Static Assets:**
  - Uses `Cache-Control: public, max-age=31536000` for 1-year caching.
  - Assets are versioned for cache busting, ensuring users get updates when files change.
- **API Responses:**
  - Uses `Cache-Control: public, max-age=60` for short-term caching (1 minute).
  - Improves performance for read-only endpoints and reduces database load.
- **Sensitive/Dynamic Endpoints:**
  - Uses `Cache-Control: no-store` to prevent caching of user actions and sensitive data.

## Future Features

### Centralized Error Handling

**Status**: Planned

Will add a middleware layer to catch errors, log them, and provide consistent user-friendly responses. Implementation is postponed until the app grows in complexity.

### Modulized global layer

**Status**: Planned

This should be a good practise, however, from my point of view, refactor this out of server.rs run is more troublesome than expected.

## Reference

- [FullStack Rust with Axum from Martin Fabio](https://www.amazon.com/FullStack-Rust-Axum-Server-Rendered-High-Performance-ebook/dp/B0FM6XF8YX).
- [Leptos resources](https://leptos.dev/)
- [awesome-leptos repo](https://github.com/leptos-rs/awesome-leptos)
- The wild internet, AIs and various other sources...

## Reflection

A personal reflection on this project can be found on this blog as well. Read my [project reflection post](https://williannguyen.com/posts/4).
