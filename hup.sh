#!/usr/bin/env bash

kill -SIGHUP `ps aux | grep "rstored" | grep -v color | awk '{print $2}'`