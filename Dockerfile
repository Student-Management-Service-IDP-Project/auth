FROM rust:latest

WORKDIR /auth

COPY . .

# Expose server and MongoDB ports.
EXPOSE 3000
EXPOSE 27017

RUN cargo build --workspace --release
CMD ["./target/release/auth-service"]
