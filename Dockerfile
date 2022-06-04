FROM rust:1.57.0-alpine

RUN apk add --no-cache musl-dev
WORKDIR /usr/src/app
COPY . ./
RUN cargo build --release
CMD ["./target/release/comments"]