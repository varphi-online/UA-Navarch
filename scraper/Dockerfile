FROM rust:1.82-alpine AS builder
RUN apk add ca-certificates musl-dev openssl-dev pkgconfig libgcc gcompat libstdc++
WORKDIR /usr/src/scraper

ENV RUST_BACKTRACE=full
COPY Cargo.toml .
COPY Cargo.lock .
# Hack to populate cache
RUN mkdir src && echo 'fn main() {}' > src/main.rs && \
    cargo build --release

COPY . .

RUN cargo build --release 

# Start a fresh image
FROM alpine

ENV RUST_BACKTRACE=full

WORKDIR /usr/src/scraper

RUN apk add --no-cache sqlite-libs sqlite

RUN mkdir -p /usr/src/scraper/data

COPY --from=builder /usr/src/scraper/target/release/NavarchScraper scraper
RUN chmod +x scraper

VOLUME ["/usr/src/scraper/data"]

EXPOSE 8080

CMD ["./scraper", "-l", "K"]
