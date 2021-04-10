


# TODOs:
# pull a rust image
# create katyusha group and user with password
# move the whole project into the katyusha home directory
# change the owner of the folder containing the source code
# run the bot in development mode using "cargo run" command


FROM ekidd/rust-musl-builder:stable as builder

RUN USER=root cargo new --bin katyusha
WORKDIR ./core
# COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --bin katyusha --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/katyusha*
RUN cargo build --bin katyusha --release


FROM alpine:latest

ARG APP=/usr/src/app

EXPOSE 8587

ENV TZ=Etc/UTC \
    APP_USER=appuser

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
