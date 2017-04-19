#RSTORED

TMP HELP
```
#!/usr/bin/env bash

RUST_BACKTRACE=1 cargo run -- -c `realpath ./etc/rstored.ini` <-d>

kill -9 `ps aux | grep "rstored" | grep -v color | awk '{print $2}'`
kill -SIGINT `ps aux | grep "rstored" | grep -v color | awk '{print $2}'`
kill -SIGTERM `ps aux | grep "rstored" | grep -v color | awk '{print $2}'`
kill -SIGHUP `ps aux | grep "rstored" | grep -v color | awk '{print $2}'`

```