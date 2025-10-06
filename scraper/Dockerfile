FROM rust:1.82-alpine AS builder
RUN apk add --no-cache \
    ca-certificates \
    musl-dev \
    openssl-dev \
    pkgconfig \
    libgcc \
    libstdc++ \
    # Add Perl and other build tools
    perl \
    make \
    gcc \
    # Add SSL/crypto libraries explicitly
    openssl \
    openssl-libs-static

WORKDIR /usr/src/scraper

ENV RUST_BACKTRACE=full
ENV OPENSSL_STATIC=1
ENV OPENSSL_LIB_DIR=/usr/lib
ENV OPENSSL_INCLUDE_DIR=/usr/include/openssl

COPY Cargo.toml .
COPY Cargo.lock .
# Hack to populate cache


RUN mkdir src && echo 'fn main() {println!("hallo")}' > src/main.rs && \
    cargo build --release

COPY ./src ./src

RUN touch -a -m ./src/main.rs


RUN cargo build --release 


# Start a fresh image
FROM alpine

ENV RUST_BACKTRACE=full

WORKDIR /usr/src/scraper

RUN apk add --no-cache \
    sqlite-libs \
    sqlite \
    libssl3 \
    libcrypto3 \
    busybox-suid

RUN mkdir -p /usr/src/scraper/data

COPY --from=builder /usr/src/scraper/target/release/NavarchScraper scraper
RUN chmod +x scraper

COPY ./crontab /etc/cron.d/scrape
RUN crontab /etc/cron.d/scrape


EXPOSE 8080

CMD busybox sh -c "crond -f -l 8 >> /usr/src/scraper/data/cron.log 2>&1"
