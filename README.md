#RSTORED



##huptest
``` kill -SIGHUP `ps aux | grep "rstored" | grep -v color | awk '{print $2}'` ```