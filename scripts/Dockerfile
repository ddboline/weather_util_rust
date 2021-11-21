FROM rust:alpine3.14 as buildstage

RUN apk add musl-dev

RUN mkdir -p /weather_util_rust && \
    cd /weather_util_rust && \
    cargo install weather_util_rust --root=/weather_util_rust

FROM alpine:3.14

COPY --from=buildstage /weather_util_rust/bin/* /usr/bin/

STOPSIGNAL SIGTERM

CMD ["weather-util-rust"]
