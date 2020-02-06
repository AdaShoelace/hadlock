#!/bin/bash

NAME=hadlock
DIR=/usr/local/bin/

function delete_old_bin () {
	if [ -f "$DIR$NAME" ]; then
		echo "deleting: $DIR$NAME"
		sudo rm $DIR$NAME
	fi
}


if [ "$1" == "debug" ]; then
	delete_old_bin
	echo "Building debug binary"
	cargo build
	sudo cp ../target/debug/hadlock $DIR
elif [ "$1" == "release" ]; then
	delete_old_bin
	echo "Building release binary"
	cargo build --release
	sudo cp ../target/release/hadlock $DIR
else
	echo "Please provide argument \"debug\" or \"release\""
	exit 1
fi

LOG=~/hadlock.log
if [ -f "$LOG" ]; then
	echo "deleting: $LOG"
	sudo rm $LOG
fi
