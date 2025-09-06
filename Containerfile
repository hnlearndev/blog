# Production Containerfile optimized for Fly.io deployment with managed PostgreSQL
# Build stage
FROM rustlang/rust:nightly-bookworm as builder

# Install system dependencies and cargo-binstall for faster tool installation
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Install cargo-leptos, sqlx-cli, compatible wasm-bindgen-cli, and WebAssembly target
RUN cargo binstall cargo-leptos sqlx-cli --no-confirm --log-level warn \
    && cargo install wasm-bindgen-cli --version 0.2.100 \
    && rustup target add wasm32-unknown-unknown

# Set working directory
WORKDIR /app

# Copy dependency files first for better Docker layer caching
COPY Cargo.toml Cargo.lock ./

# Create dummy source files to build dependencies
RUN mkdir -p src \
    && echo 'fn main() {}' > src/main.rs \
    && echo 'pub fn lib() {}' > src/lib.rs

# Build dependencies only (this layer will be cached if Cargo.toml doesn't change)
RUN cargo build --release --features ssr

# Remove dummy source files
RUN rm -rf src

# Copy source code and assets
COPY src/ ./src/
COPY public/ ./public/
COPY contents/ ./contents/
COPY migrations/ ./migrations/
COPY .sqlx/ ./.sqlx/

# Copy build script if it exists
COPY build.rs ./

# Set SQLx to offline mode and build the Leptos application
ENV SQLX_OFFLINE=true
RUN cargo leptos build --release

# Runtime stage - minimal image for production
FROM debian:bookworm-slim as runtime

# Install runtime dependencies including curl for healthchecks
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd -r app \
    && useradd -r -g app app

# Set working directory
WORKDIR /app

# Copy the compiled binary and assets from builder stage
COPY --from=builder /app/target/release/blog ./blog
COPY --from=builder /app/target/site ./site
COPY --from=builder /app/public ./public
COPY --from=builder /app/migrations ./migrations

# Copy sqlx-cli for running migrations
COPY --from=builder /usr/local/cargo/bin/sqlx ./sqlx

# Create startup script for running migrations and starting the app
RUN echo '#!/bin/bash\n\
set -e\n\
echo "Running database migrations..."\n\
./sqlx migrate run --database-url "$DATABASE_URL"\n\
echo "Starting application..."\n\
exec "$@"' > /app/start.sh \
    && chmod +x /app/start.sh

# Set ownership and switch to non-root user for security
RUN chown -R app:app /app
USER app

# Expose the port
EXPOSE 3000

# Set production environment variables
ENV LEPTOS_ENV=PROD
ENV LEPTOS_SITE_ROOT=/app/site
ENV LEPTOS_SITE_PKG_DIR=pkg
ENV LEPTOS_SITE_ADDR=0.0.0.0:3000
ENV RUST_LOG=info

# Use startup script as entrypoint to run migrations before starting the app
ENTRYPOINT ["/app/start.sh"]
CMD ["./blog"]
