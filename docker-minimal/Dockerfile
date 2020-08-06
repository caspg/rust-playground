# ------------------------------------------------------------------------------
# Build Stage
# ------------------------------------------------------------------------------
FROM rust:1.45 as cargo-builder

RUN apt-get update
# We need MUSL to run inside alpine linux
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
ADD . ./
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
# RUN cargo install --path .

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------
FROM alpine:latest
COPY --from=cargo-builder /app/target/x86_64-unknown-linux-musl/release/docker-minimal /usr/local/bin/docker-minimal
CMD [ "docker-minimal" ]
