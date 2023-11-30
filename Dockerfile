# Use a Rust base image
FROM rust:latest as builder

# Install CMake
RUN apt update && \
    apt install -y cmake && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the source code
COPY . .

# Build the Rust project
RUN cargo build --release

# Create a new, smaller image without the build dependencies
FROM ubuntu:23.04

WORKDIR /app

# Copy just the compiled binary from the previous stage
COPY --from=builder /app/target/release/inx-edcas-indexer /app/
COPY --from=builder /app/createTables.sql /app/

# Set the entry point
CMD ["./inx-edcas-indexer"]