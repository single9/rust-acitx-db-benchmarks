FROM rust:1-alpine3.16 as builder
WORKDIR /usr/src/api
COPY . .
RUN apk add --update alpine-sdk
RUN cargo build --bin api --release

FROM alpine:3.16
COPY --from=builder /usr/src/api/target/release/api /usr/local/bin/api
RUN wget -O /usr/local/bin/dumb-init \
      https://github.com/Yelp/dumb-init/releases/download/v1.2.5/dumb-init_1.2.5_x86_64 \
      && chmod +x /usr/local/bin/dumb-init

ENTRYPOINT ["/usr/local/bin/dumb-init", "--"]
CMD ["/usr/local/bin/api"]
