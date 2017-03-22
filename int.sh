#!/usr/bin/env bash

kill -SIGINT `ps aux | grep "rstored" | grep -v color | awk '{print $2}'`