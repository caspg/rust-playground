FROM rust:1.45

ARG REDIS_URL

WORKDIR /app
ADD . ./
RUN cargo build
CMD ["/app/target/debug/docker-redis"]
