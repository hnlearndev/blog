# Willian's Personal Blog

Visit: [williannguyen.com](https://www.williannguyen.com)

## Overview

A fast, modern blog built with full-stack Rust:

- **Frontend:** Leptos (SSR + hydration)
- **Backend:** Axum + PostgreSQL
- **Content:** Markdown posts, syntax highlighting
- **Build:** Static content generation at compile time

## Features

- Server-side rendering for speed
- WASM-optimized client bundle
- Markdown-based posts
- Syntax highlighting for code
- Multi-tier caching for assets and APIs
- Security: rate limiting, CORS, CSRF, security headers, cache
- Newsletter subscription

## Structure

```text
src/
  app.rs             # Main app & routing
  app/
    components/      # UI components (nav, footer, theme toggle, etc.)
    pages/           # Route-level pages (home, post, poem)
    helpers.rs       # Shared utilities
  server.rs          # Axum server setup
  server/            # Backend modules (db, middleware, models, routes, handlers,services, repositories,...)
  build.rs           # Static content build script
  contents/          # Markdown posts
```

## Build & Deploy

- Build with `cargo-leptos`
- Deploy to Fly.io with managed Postgres

## Reference

- Book [FullStack Rust with Axum from Martin Fabio](https://www.amazon.com/FullStack-Rust-Axum-Server-Rendered-High-Performance-ebook/dp/B0FM6XF8YX)
- [Leptos resources](https://leptos.dev/)
- [awesome-leptos repo](https://github.com/leptos-rs/awesome-leptos)
- The wild internet, AIs and various other sources...

## Reflection

A personal reflection on this project can be found [on this blog post](https://williannguyen.com/posts/4).
