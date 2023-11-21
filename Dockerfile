# First stage: Rust builder
FROM rust:alpine as builder
RUN apk add --no-cache build-base
WORKDIR /build

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
# This dummy build step is to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -f target/release/deps/terra_track*

# Now build the real application
COPY src/ src/
RUN cargo build --release

# Second stage: Construct the final image
FROM alpine:latest


# User and group setup
RUN addgroup -S terra-track && adduser -S -G terra-track terra-track

# Install necessary packages
RUN apk add --no-cache nmap nmap-scripts wget curl

# Set the working directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /build/target/release/terra_track /app/terra_track

# Set non-root user
USER terra-track

# Set ulimit (consider handling this in your application or entrypoint script)
# Note: The effect of ulimit here is limited. Consider using Docker's --ulimit flag.
# RUN ulimit -n 100000

# Set the binary as entrypoint
ENTRYPOINT ["/app/terra_track"]
