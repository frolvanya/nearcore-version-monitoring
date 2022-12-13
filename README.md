## General info

This script notifies you by email when new version of [nearcore](https://github.com/near/nearcore) comes out 

## How to use

```bash
$ export SMTP_SERVER="smtp.google.com"
$ export EMAIL_RECIPIENT="<your-email-here>"
$ export EMAIL_HOSTNAME="<hostname-for-smtp>"
$ export EMAIL_PASSWORD="<password-for-smtp>"

$ cargo run --release
```

This will run checking script only once. Better to use with GitHub Actions or Docker 
