# First stage: Rust builder
FROM rust:alpine as builder
RUN apk add --no-cache build-base
WORKDIR /build

# Copy the entire workspace
COPY . .

# Cache dependencies (for each workspace component and the main application)
RUN cargo update

# Now build the real application (build the entire workspace)
RUN cargo build --release


##############################################
# Second stage: Construct the final image
FROM alpine:latest

# User and group setup
RUN addgroup -S terra-track && adduser -S -G terra-track terra-track

# Install necessary packages
RUN apk add --no-cache nmap nmap-scripts wget curl

# Set the working directory
WORKDIR /app

# Copy the built binary from the builder stage
# Replace 'terra_track' with the correct binary name if different
COPY --from=builder /build/target/release/terra_track /app/terra_track

# Set non-root user
USER terra-track

# Set the binary as entrypoint
ENTRYPOINT ["/app/terra_track"]
