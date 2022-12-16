## General info

This script notifies you by telegram when new version of [nearcore](https://github.com/near/nearcore) comes out 

## How to use

```bash
$ export TELEGRAM_BOT_API="<YOUR-TELEGRAM-BOT-API>"
$ export TELEGRAM_CHAT_ID="<YOUR-TELEGRAM-CHAT-ID>"

$ cargo run --release
```

This will run checking script only once. Better to use with GitHub Actions or Docker 

## Docker example

```bash
docker build -t nearcore-version-monitoring .
docker run -d -it --rm --name nvm -e "TELEGRAM_BOT_API=<YOUR-TELEGRAM-BOT-API>" -e "TELEGRAM_CHAT_ID=<YOUR-TELEGRAM-CHAT-ID>" nearcore-version-monitoring
```
