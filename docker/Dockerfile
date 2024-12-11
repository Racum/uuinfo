FROM rust:alpine AS builder
RUN apk add musl-dev
RUN cargo install uuinfo

FROM scratch
COPY --from=builder /usr/local/cargo/bin/uuinfo /uuinfo
ENV CLICOLOR_FORCE=true
ENTRYPOINT [ "/uuinfo" ]
