# Use Rust 1.90.0 which supports edition2024
FROM rust:1.90.0-bookworm AS builder

# Install system dependencies for building
RUN apt-get update && apt-get install -y \
  pkg-config \
  libsqlite3-dev \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy all source code including SQLx query data
COPY . .

# Add WASM target for frontend
RUN rustup target add wasm32-unknown-unknown

# Install dioxus-cli
RUN cargo install dioxus-cli

# Build using docker profile (sets SQLX_OFFLINE=true)
RUN cargo build --profile docker --workspace

# Build the frontend separately with dioxus-cli
RUN dx build --release --package frontend

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
  ca-certificates \
  sqlite3 \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Create non-root user
RUN useradd -m -u 1000 appuser

# Copy the built backend binary
COPY --from=builder /app/target/release/aazan ./

# Copy the frontend static files
COPY --from=builder /app/target/dx/frontend/release/web/public ./dist

# Copy database migrations
COPY --from=builder /app/backend/migrations ./migrations
# Create data directory for database
RUN mkdir -p /app/data
# Set runtime DATABASE_URL
ENV DATABASE_URL="sqlite:/app/data/aazan.db"

# Set ownership
RUN chown -R appuser:appuser /app
USER appuser

# Expose port for Fly.io
EXPOSE 8080

# Set port environment variable
ENV PORT=8080

# Run the application
CMD ["./aazan"]
