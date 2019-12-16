#!/bin/bash

cargo build

#Xephyr :3.0 -a -ac -br +xinerama +extension RANDR -screen 640×480 -host-cursor &
#Xephyr :4.0 -a -ac -br +xinerama +extension RANDR -screen 640×480 -host-cursor &
#(sleep 2; Xdmx :5 -xinput local -display :3 -display :4 @480x0 +xinerama +extension RANDR) &
#(sleep 4; env DISPLAY=:5 RUST_LOG=hadlock ./target/debug/hadlock ~/Programming/rust/hobby/hadlock/config/hadlok.json &)

XEPHYR=$(whereis -b Xephyr | cut -f2 -d' ')
xinit ./xinitrc -- \
    "$XEPHYR" \
        :99 \
        -ac \
		-screen 1280x720 \
		+xinerama \
		-host-cursor \
        #-screen 2560x1440 \
