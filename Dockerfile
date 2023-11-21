FROM rust:alpine as builder
RUN apk add --no-cache build-base

# Encourage some layer caching here rather then copying entire directory that includes docs to builder container ~CMN
WORKDIR /app/terra-track
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
RUN cargo build --release

FROM alpine:latest
RUN addgroup -S terra-track && \
    adduser -S -G terra-track terra-track && \
    ulimit -n 100000 && \
    apk add --no-cache nmap nmap-scripts wget
USER terra-track
COPY --from=builder /app/terra-track/target/release/terra_track /usr/local/bin/terra-track
ENTRYPOINT [ "/usr/local/bin/terra-track" ]
