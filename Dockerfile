FROM frolvlad/alpine-rust as builder

RUN apk add --no-cache openssl-dev

# create a new empty shell project
WORKDIR /nearcore-new-release

# copy over manifests
COPY Cargo.toml .
COPY Cargo.lock .

RUN mkdir src
RUN touch src/main.rs
RUN echo "fn main() {}" > src/main.rs

RUN cargo test
RUN cargo build --release

COPY . .
RUN touch src/main.rs

RUN cargo test
RUN cargo build --release

RUN strip target/release/nearcore-new-release

# start building the final image
FROM alpine:3.17

RUN apk add --no-cache bash libssl3 libgcc

COPY --from=builder /nearcore-new-release/target/release/nearcore-new-release .

# configure cron
COPY crontab.txt /opt
RUN /usr/bin/crontab /opt/crontab.txt

# create an entrypoint
COPY ./entrypoint.sh /
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["bash", "/entrypoint.sh"]
