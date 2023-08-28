FROM rust:alpine as builder
RUN apk add --no-cache build-base

# Encourage some layer caching here rather then copying entire directory that includes docs to builder container ~CMN
WORKDIR /app/rustscan
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
RUN cargo build --release

FROM alpine:latest
RUN addgroup -S rustscan && \
    adduser -S -G rustscan rustscan && \
    ulimit -n 100000 && \
    apk add --no-cache nmap nmap-scripts wget
USER rustscan
COPY --from=builder /app/rustscan/target/release/rustscan /usr/local/bin/rustscan
ENTRYPOINT [ "/usr/local/bin/rustscan" ]
