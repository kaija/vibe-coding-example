# Multi-stage Dockerfile for Rust note service

# Build stage
FROM rust:1.82 AS builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
#RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this will be cached)
#RUN RUST_LOG=debug cargo build --release && rm -rf src

# Copy source code
COPY src ./src

# Build the application
RUN RUST_LOG=debug cargo build --release

# Runtime stage
FROM rust:1.82

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN groupadd -r appuser && useradd -r -g appuser appuser

# Create app directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/note-api /app/note-api

# Copy health check script
COPY scripts/health-check.sh /app/health-check.sh
RUN chmod +x /app/health-check.sh

# Change ownership to appuser
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 8000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD /app/health-check.sh

# Run the application
CMD ["./note-api"]
