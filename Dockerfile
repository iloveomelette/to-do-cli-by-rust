# syntax=docker/dockerfile:1

#
# Build Stage
#
FROM rust:1.76.0-bookworm as builder

WORKDIR /usr/src/todo
COPY . .
RUN cargo install --path .

#
# Production Stage
#
FROM debian:bookworm-slim

COPY --from=builder /usr/local/cargo/bin/todo /usr/local/bin/todo

CMD ["todo"]
