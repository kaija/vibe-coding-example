# Build stage
FROM rust:1.87 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml ./

# Copy source code
COPY src ./src
COPY static ./static

ENV DATABASE_URL=postgres://todouser:todopass@localhost:5432/todoapp

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/todo-service /app/todo-service

# Copy static files and migrations
COPY --from=builder /app/static /app/static

# Expose port
EXPOSE 3000

# Run the binary
CMD ["./todo-service"]
