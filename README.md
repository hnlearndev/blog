<h1 align="center">

<picture>
  <source srcset="https://raw.githubusercontent.com/hnlearndev/static/refs/heads/main/blog/banner_dark.svg" media="(prefers-color-scheme: dark)">
  <img src="https://raw.githubusercontent.com/hnlearndev/static/refs/heads/main/blog/banner_light.svg" alt="Willian Nguyen's blog">
</picture>

</h1>

![Status](https://img.shields.io/badge/dynamic/json?url=https://williannguyen.com/status-badge&label=status&query=$.message&color=$.color)

The final product is [williannguyen.com](https://williannguyen.com).

Full personal reflection on this project can be found on this [post](https://williannguyen.com/posts/4).

**Note:**

_There is a regular status service to check that the website should work. This is one of the feature can be seen in the source code. If it does not (mismatch with status), please dm me. Thank you for your helps._

## ARCHITECTURE OVERVIEW

This application follows a **full-stack Rust architecture** using:

- **Frontend**: Leptos with hydration for interactive client-side features
- **Backend**: Axum web framework
- **Rendering**: Server-side rendering (SSR) with client-side hydration
- **Content**: Markdown-based blog posts with syntax highlighting
- **Build System**: Custom build script for static content generation
- **Planned Database**: SurrealDB

## TECH STACK

### Core Framework

- **[Leptos 0.8](https://leptos.dev/)** - Reactive web framework for Rust
- **[Axum 0.8](https://docs.rs/axum/)** - Modern async web framework
- **[Tokio](https://tokio.rs/)** - Async runtime

### Database (Planned)

- **[SurrealDB](https://surrealdb.com/)** - Multi-model database (planned for newsletter subscriber feature)

### Content Processing

- **[pulldown-cmark](https://docs.rs/pulldown-cmark/)** - Markdown parser
- **[syntect](https://docs.rs/syntect/)** - Syntax highlighting for code blocks
- **Static Content Generation** - Build-time markdown processing

### Development & Deployment

- **[cargo-leptos](https://crates.io/crates/cargo-leptos)** - Leptos build tool
- **Hot Reload** - Development server with live reloading
- **WASM Optimization** - Size-optimized WebAssembly builds
- **End-to-End Testing** - Playwright integration

## PROJECT STRUCTURE

### 🌐 Frontend (Leptos)

Built with Leptos 0.8 using SSR + hydration pattern. The frontend follows a responsive component architecture with clear separation of desktop/mobile layouts.

```
src/
├── client.rs              # Client-side hydration entry point
├── lib.rs                 # Shared library code
├── app/
│   ├── mod.rs             # Root App component with header/footer
│   ├── helpers.rs         # UI utility functions
│   ├── components/        # Reusable UI components
│   │   ├── nav/           # Navigation (responsive desktop/mobile)
│   │   │   ├── mod.rs     # Nav coordinator
│   │   │   ├── desktop.rs # Desktop navigation (>768px)
│   │   │   ├── mobile.rs  # Mobile navigation (≤768px)
│   │   │   └── helpers.rs # Shared nav logic & parameterized components
│   │   ├── footer/        # Footer (responsive desktop/mobile)
│   │   │   ├── mod.rs     # Footer coordinator
│   │   │   ├── desktop.rs # Desktop footer layout
│   │   │   ├── mobile.rs  # Mobile footer layout
│   │   │   ├── helpers.rs # Copyright & NewsletterSection components
│   │   │   └── subscribe_form.rs  # AutoForm-based newsletter form
│   │   ├── post_nav/      # Post navigation (prev/next links)
│   │   │   ├── mod.rs     # PostNav coordinator
│   │   │   ├── desktop.rs # Horizontal layout
│   │   │   ├── mobile.rs  # Vertical stacked layout
│   │   │   └── helpers.rs # BackToTop & PostLink components
│   │   ├── ui/            # rust-ui components (theme toggle, etc.)
│   │   ├── fast_a.rs      # Enhanced anchor component
│   │   ├── content_list.rs   # Dynamic post/poem list rendering
│   │   ├── icons.rs       # Icon mapping utilities
│   │   └── theme_toggle.rs   # Dark/light mode switcher
│   ├── pages/             # Route-level page components
│   │   ├── mod.rs         # Page coordinator
│   │   ├── homepage.rs    # Landing page
│   │   ├── postpage.rs    # Blog post list & detail pages
│   │   └── poempage.rs    # Poetry list & detail pages
│   └── hooks/             # Custom reactive hooks
│       └── use_theme_mode.rs  # Theme state management with localStorage
├── server/                # Backend (see Backend section)
└── shared/                # Shared DTOs between frontend/backend
    └── dto.rs
```

#### Frontend Architecture Patterns

- **Responsive Module Pattern**: Each complex component (`nav/`, `footer/`, `post_nav/`) uses a directory structure with `mod.rs` (coordinator), `desktop.rs`, `mobile.rs`, and `helpers.rs` (shared logic)
- **Parameterized Components**: Shared UI elements (e.g., `SocialLinks`, `NavLinks`) accept class prefixes for layout variants without duplication
- **Theme Management**: Reactive theme state persisted to localStorage, with SSR-compatible hydration
- **Static Content**: Markdown posts/poems processed at build time, rendered as static HTML with syntax highlighting
- **rust-ui Integration**: Uses rust-ui ecosystem for form generation (`autoform`), icons (`icons` crate with Leptos feature), and UI primitives
- **CSS Strategy**: PicoCSS base + Tailwind CSS v4 + custom semantic stylesheets in `public/style/`

### ⚙️ Backend (Axum) Architecture

The backend structure seen below is over-engineered for the purpose of personal blog with only public newsletter subcriber feature.

However, the industry-graded architecture is purposely used to study fullstack technology with Rust. The architecture is learnt from the book [FullStack Rust with Axum from Martin Fabio](https://www.amazon.com/FullStack-Rust-Axum-Server-Rendered-High-Performance-ebook/dp/B0FM6XF8YX)

```
├── main.rs                 # Application entry point
├── server.rs               # Server orchestration & middleware stack
└── server/                 # Modular backend architecture
    ├── db.rs               # Database module coordinator
    ├── db/
    │   ├── config.rs       # Database URL & connection config
    │   ├── pool.rs         # PgPool initialization & management
    │   ├── state.rs        # AppState with shared resources
    │   └── error.rs        # Database-specific error handling
    ├── middleware.rs       # Middleware module coordinator
    ├── middleware/
    │   ├── cache.rs        # HTTP caching strategies
    │   ├── governor.rs     # Rate limiting (IP-based)
    │   ├── csrf.rs         # CSRF token protection
    │   ├── throttle.rs     # Request throttling
    │   ├── global_layer.rs # Middleware layer coordinator
    │   └── global_layer/
    │       ├── cors.rs                 # Cross-Origin Resource Sharing
    │       └── security_headers.rs     # Security headers middleware
    ├── models.rs           # Data model coordinator
    ├── models/
    │   ├── subscriber.rs   # Newsletter subscriber model
    │   └── status.rs       # Status badge model (for shields.io)
    ├── repositories.rs     # Data access coordinator
    ├── repositories/
    │   ├── subscriber.rs   # Database queries & data access
    │   └── status.rs       # Status badge logic (checks and aggregates status)
    ├── services.rs         # Business logic coordinator
    ├── services/
    │   ├── subscriber.rs   # Newsletter business logic
    │   └── status.rs       # Status badge update logic (periodic background updater)
    ├── handlers.rs         # Request handler coordinator
    ├── handlers/
    │   ├── subscriber.rs   # HTTP request/response handling
    │   └── status.rs       # Status badge API handler (serves cached status)
    ├── routes.rs           # API route coordinator
    └── routes/
        ├── subscriber.rs   # Newsletter API endpoints
        └── status.rs       # Status badge AP endpoint (`/status-badge` for shields.io)
```

### 🏘️ Backend Layer Relationships

```
graph TD
  Middleware["🧩 Middleware Stack<br/>(server.rs & routes/)"]
  Middleware -->|Applied globally and per route| Routes["🛣️ Routes"]
  Routes -->|Use| Handlers["👐 Handlers"]
  Handlers -->|Use| Services["🛎️ Services"]
  Services -->|Use| Repositories["📦 Repositories"]
  Repositories -->|Use| Models["📄 Models"]
  Repositories -->|Use| DB["🗄️ DB"]
```

#### Middleware & Security Features

- **Global**
  - Compression: Brotli compression for responses
  - Request Timeout: Configurable request timeouts
  - Security Headers: Comprehensive HTTP security headers
- **Route specific**
  - Rate Limiting: Per-IP request throttling using Governor
  - CORS: Cross-Origin Resource Sharing configuration
  - CSRF Protection: Token-based CSRF mitigation

#### Layer Responsibilities

- Routes: HTTP endpoints + middleware application, delegate to handlers
- Handlers: HTTP request/response processing, input validation
- Services: Business logic, orchestration, transaction management
- Repositories: Data access queries, DB operations using models
- Models: Data structures, serialization, validation rules
- DB: Connection pooling, configuration, state management

## BUILD SYSTEM

The project uses a custom build script (`build.rs`) that:

1. **Processes Markdown Files**: Reads blog posts from `contents/posts/`
2. **Syntax Highlighting**: Applies code highlighting using Syntect
3. **Static Generation**: Converts markdown to HTML at build time
4. **Optimized Output**: Generates Rust code with static post data

## PERFORMANCE FEATURES

- **Server-Side Rendering (SSR)**: Fast initial page loads
- **Hydration**: Interactive client-side features without full SPA overhead
- **Static Content**: Build-time markdown processing reduces runtime overhead
- **Compression**: Brotli compression for smaller payload sizes
- **Connection Pooling**: Efficient database connection management
- **Request Timeout**: Prevents long-running requests from blocking resources
- **HTTP Caching Strategy**: Multi-tier caching system for optimal performance
- **WASM Optimization**: Aggressive size optimization for client-side bundles

## WASM BUNDLE OPTIMIZATION

The WebAssembly build process includes several standard optimization techniques to minimize bundle size:

### Optimization Techniques

- Size-focused compilation (opt-level = 'z')
- Link-time optimization (LTO)
- Strip debug symbols (strip = true)
- Abort on panic (panic = "abort")
- Single codegen unit
- Use wee_alloc for smaller WASM allocator

### Benchmark Results

Below information is obtained from the actual implementation on the project and get benchmark to show the efficency of these technuques.

| Metric               | Before Optimization | After Optimization     | Improvement       |
| -------------------- | ------------------- | ---------------------- | ----------------- |
| **Bundle Size**      | 8.5MB               | 1.5MB                  | **82.4% smaller** |
| **Gzipped Size**     | ~2.1MB              | ~400-600KB             | ~75% smaller      |
| **Load Time Impact** | Baseline            | Significantly improved | 5.6x smaller      |

### Impact

- **Faster page loads**: 82% smaller WASM bundles load much faster
- **Reduced bandwidth**: Significant savings in data transfer
- **Better mobile experience**: Smaller bundles improve performance on slower connections
- **Production ready**: Size is now within reasonable limits for web deployment

## CACHING

Deploy standard industry practices

- **Static Assets:**
  - Uses `Cache-Control: public, max-age=31536000` for 1-year caching.
  - Assets are versioned for cache busting, ensuring users get updates when files change.
- **API Responses:**
  - Uses `Cache-Control: public, max-age=60` for short-term caching (1 minute).
  - Improves performance for read-only endpoints and reduces database load.
- **Sensitive/Dynamic Endpoints:**
  - Uses `Cache-Control: no-store` to prevent caching of user actions and sensitive data.

## Future Features

- Centralized Error Handling (**Status**: Planned)

To add a middleware layer to catch errors, log them, and provide consistent user-friendly responses. Implementation is postponed until the app grows in complexity.

- Modulized global layer (**Status**: Planned)

This was planned out at the begginning with `tower` crate 's ServiceBuilder as a global layer which is then called into server.rs. However, refactoring this seperated out from server.rs run is more troublesome than expected. Until the project expands further, it is placed directly in server.rs.

## Reference

- Book [FullStack Rust with Axum from Martin Fabio](https://www.amazon.com/FullStack-Rust-Axum-Server-Rendered-High-Performance-ebook/dp/B0FM6XF8YX)
- [Leptos resources](https://leptos.dev/)
- [awesome-leptos repo](https://github.com/leptos-rs/awesome-leptos)
- The wild internet, AIs and various other sources...
