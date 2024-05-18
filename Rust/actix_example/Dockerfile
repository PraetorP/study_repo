FROM rust:1.77.0 as builder

WORKDIR /app
RUN rustup target add x86_64-unknown-linux-musl && \
    apt update && apt install musl-tools -y

COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest 
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/actix_example actix_example
COPY .env .env
CMD [ "/app/actix_example"]