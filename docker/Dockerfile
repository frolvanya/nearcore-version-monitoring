FROM rust:1.65.0 as build

# create a new empty shell project
RUN USER=root cargo new --bin nearcore-version-monitoring
WORKDIR /nearcore-version-monitoring

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release && rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/nearcore_new_release-*
RUN cargo build --release

# our final base
FROM rust:1.65.0

# copy the build artifact from the build stage
COPY --from=build /nearcore-version-monitoring/target/release/nearcore-new-release .

# set the startup command to run your binary
ENV TELEGRAM_BOT_API="<YOUR-TELEGRAM-BOT-API>"
ENV TELEGRAM_CHAT_ID="<YOUR-TELEGRAM-CHAT-ID>"

# installing cron
RUN apt-get update && apt-get install cron -y

# configure cron
COPY crontab.txt /opt
RUN crontab /opt/crontab.txt

# create an entrypoing
COPY ./entrypoint.sh ./entrypoint.sh
ENTRYPOINT ["bash", "entrypoint.sh"]

