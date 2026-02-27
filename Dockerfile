FROM rust:1.93-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

COPY src ./src
RUN touch src/main.rs && cargo build --release

FROM alpine:3.19

RUN apk add --no-cache ca-certificates tzdata

RUN addgroup -g 1000 urlslim && \
    adduser -u 1000 -G urlslim -s /bin/sh -D urlslim

WORKDIR /app

COPY --from=builder /app/target/release/urlslim /app/urlslim

RUN mkdir -p /app/data && chown -R urlslim:urlslim /app

USER urlslim

EXPOSE 3000

ENV RUST_LOG=info
ENV HOST=0.0.0.0
ENV PORT=3000
ENV DATABASE_URL=data/links.db

CMD ["./urlslim"]
