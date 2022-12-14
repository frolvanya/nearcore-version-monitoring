FROM ekidd/rust-musl-builder as builder

# create a new empty shell project
WORKDIR /usr/bin/local/
RUN cargo new --bin nearcore-new-release
WORKDIR nearcore-new-release

# copy over manifests
COPY Cargo.toml .

RUN cargo test
RUN cargo build --release

COPY . .
RUN sudo touch src/main.rs

RUN cargo test
RUN cargo build --release

RUN strip target/x86_64-unknown-linux-musl/release/nearcore-new-release

# Start building the final image
FROM scratch
WORKDIR /home/nvm/
COPY --from=builder /home/rust/target/x86_64-unknown-linux-musl/release/nearcore-new-release .

# installing cron
RUN apt-get update && apt-get install cron -y

# configure cron
COPY crontab.txt /opt
RUN crontab /opt/crontab.txt

# create an entrypoint
COPY ./entrypoint.sh ./entrypoint.sh
ENTRYPOINT ["bash", "entrypoint.sh"]
