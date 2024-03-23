FROM rust:1.77 AS build
WORKDIR /app
COPY Cargo.toml .
COPY Cargo.lock .
COPY src/ ./src/
RUN cargo build --release
CMD ["/app/target/release/arsis"]
