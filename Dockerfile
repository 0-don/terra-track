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

# Install necessary packages
RUN apk update && apk add --no-cache nmap nmap-scripts wget curl

# Set the working directory
WORKDIR /app

ARG DATABASE_URL
RUN echo "DATABASE_URL=$DATABASE_URL" > .env


# nmap --script-updatedb


# Copy the built binary from the builder stage
# Replace 'terra_track' with the correct binary name if different
COPY --from=builder /build/target/release/terra_track /app/terra_track

# Set the binary as entrypoint
ENTRYPOINT ["/app/terra_track"]
