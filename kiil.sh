#!/usr/bin/env bash

kill -9 `ps aux | grep "rstored" | grep -v color | awk '{print $2}'`