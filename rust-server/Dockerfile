# ---- Stage 1: Build Rust Binary ----
FROM rust:latest AS builder

WORKDIR /usr/src/app

# Install Node.js for any build steps that might require it
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get update && apt-get install -y nodejs git

# Set git configs for GitHub HTTPS fetch
RUN git config --global url."https://github.com/".insteadOf ssh://git@github.com/ && \
    git config --global net.git-fetch-with-cli true

# Copy full source code
COPY . .

# Build Rust binary
RUN cargo build --release --verbose && \
    echo "✅ Build complete: checking binary" && ls -la target/release


# ---- Stage 2: Runtime ----
FROM debian:bookworm-slim

# Install Node.js for Vite + React apps
RUN apt-get update && apt-get install -y curl && \
    curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs && \
    npm install -g pnpm # Optional: faster installs

# Create a non-root user
RUN useradd -m appuser

# Create working directory for generated React builds
RUN mkdir /sclera_builds && chown -R appuser:appuser /sclera_builds

# Copy Rust binary from build stage
COPY --from=builder /usr/src/app/target/release/rust-server /usr/local/bin/rust-server

# Set user for security
USER appuser

EXPOSE 3000

CMD ["rust-server"]
