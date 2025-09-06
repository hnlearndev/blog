# Multi-stage build for Leptos blog application
# Stage 1: Builder
FROM rustlang/rust:nightly-bookworm as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-leptos
RUN cargo install cargo-leptos

# Set working directory
WORKDIR /app

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to compile dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN echo '[lib]\nname = "blog"\npath = "src/lib.rs"' >> Cargo.toml.temp && \
    echo 'fn main() {}' > src/lib.rs && \
    mv Cargo.toml.temp Cargo.toml || true

# Build dependencies (this layer will be cached if Cargo.toml doesn't change)
RUN cargo build --release --features ssr
RUN rm -rf src

# Copy all source code
COPY . .

# Build the application with cargo-leptos
RUN cargo leptos build --release

# Stage 2: Runtime
FROM debian:bookworm-slim as runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN groupadd -r app && useradd -r -g app app

# Set working directory
WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/target/release/blog ./blog

# Copy the site directory (contains WASM, CSS, and other assets)
COPY --from=builder /app/target/site ./site

# Copy public assets
COPY --from=builder /app/public ./public

# Change ownership to app user
RUN chown -R app:app /app

# Switch to app user
USER app

# Expose port
EXPOSE 3000

# Set environment variables (these will be overridden by fly.toml)
ENV LEPTOS_ENV=PROD
ENV LEPTOS_SITE_ROOT=/app/site
ENV LEPTOS_SITE_PKG_DIR=pkg
ENV LEPTOS_SITE_ADDR=0.0.0.0:3000

# Run the application
CMD ["./blog"]
