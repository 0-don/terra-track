FROM rust:latest as build

WORKDIR /app
COPY . .
RUN cargo build --release

FROM rust:latest
COPY --from=build /app/target/release/terra-track .
COPY --from=build /app/.env .
CMD ["./terra-track"]
