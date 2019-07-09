#!/bin/bash

cargo build

XEPHYR=$(whereis -b Xephyr | cut -f2 -d' ')
xinit ./xinitrc -- \
    "$XEPHYR" \
        :99 \
        -ac \
		-screen 1280x720 \
		-host-cursor
        #-screen 2560x1440 \
