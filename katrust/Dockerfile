


FROM ekidd/rust-musl-builder:stable as builder

RUN USER=root cargo new --bin katyusha
WORKDIR ./katyusha
ADD . ./
ENV HOST=localhost \
    PORT=8945
RUN cargo build --bin katyusha --release --target x86_64-unknown-linux-musl


FROM alpine:latest
ARG APP=/usr/src/app

ENV TZ=Etc/UTC \
    APP_USER=katyusha

RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER

RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/*

COPY --from=builder /home/rust/src/katyusha/target/x86_64-unknown-linux-musl/release/katyusha ${APP}/katyusha

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./katyusha"]
