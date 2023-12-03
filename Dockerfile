# First stage: Rust builder
FROM rust:alpine as builder
RUN apk add --no-cache build-base
WORKDIR /build

COPY . .
RUN cargo update
RUN cargo build --release

##############################################
# Second stage: Construct the final image
FROM alpine:latest

RUN apk update && apk add --no-cache nmap nmap-scripts wget curl

WORKDIR /app

ARG DATABASE_URL
RUN echo "DATABASE_URL=$DATABASE_URL" > .env

COPY --from=builder /build/target/release/terra_track /app/terra_track

ENTRYPOINT ["/app/terra_track"]