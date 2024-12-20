FROM rust:alpine AS builder
RUN apk add --no-cache libc-dev cargo-edit
# Build just the dependencies with version 0.0.0 so they're cached
WORKDIR /app
COPY Cargo.toml Cargo.lock /app
RUN mkdir -p src && echo 'fn main() {}' > /app/src/main.rs
RUN cargo fetch
RUN cargo build --release --locked
# Set the version
COPY . /app
ARG VERSION=0.0.0
RUN cargo set-version $VERSION
# Build the release binary
RUN cargo build --release

# Build final image with minimal dependencies
FROM alpine:latest
COPY --from=builder /app/target/release/tremolo /bin/tremolo
WORKDIR /
ENTRYPOINT ["tremolo"]
