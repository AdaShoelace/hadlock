#!/bin/bash

NAME=hadlock
DIR=/usr/local/bin/

if [ -f "$DIR$NAME" ]; then
	echo "deleting: $DIR$NAME"
	sudo rm $DIR$NAME
fi

cargo build

sudo cp ../target/debug/hadlock $DIR

LOG=~/hadlock.log


if [ -f "$LOG" ]; then
	echo "deleting: $LOG"
	sudo rm $LOG
fi
