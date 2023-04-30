FROM rust:latest

WORKDIR /auth

COPY ./auth .

RUN cargo clean
RUN cargo check

CMD cargo build --workspace --release && \
    ./target/release/auth-service
