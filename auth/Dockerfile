FROM rust:latest

WORKDIR /auth

COPY . .

EXPOSE 3000

RUN cargo clean
RUN cargo check

CMD cargo build --workspace --release && \
    ./target/release/auth-service
