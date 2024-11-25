# Use the official Rust image as a base
FROM rust:latest AS builder

# Create and set the working directory
WORKDIR /usr/src/myapp

# Copy the Cargo.toml and Cargo.lock files to build dependencies first
COPY Cargo.toml Cargo.lock ./
 
# Now copy the entire project into the container
COPY . .

# Build the actual project in release mode
RUN cargo build --release

# Start a new stage to keep the image small
FROM debian:bullseye-slim

# Install required dependencies for running the Actix app
RUN apt-get update && apt-get install -y \
    libc6 \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled Actix binary from the builder stage
COPY --from=builder /usr/src/myapp/target/release/user_service /usr/local/bin/myapp

# Set the entrypoint to the binary
ENTRYPOINT ["/usr/local/bin/myapp"]

# Expose port 9090 for the Actix server
EXPOSE 9090
