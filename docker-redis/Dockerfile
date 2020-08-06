FROM rust:1.45

WORKDIR /app
ADD . ./
RUN cargo build
CMD ["/app/target/debug/docker-redis"]
