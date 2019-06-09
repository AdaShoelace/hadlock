# xinitrc file for testing our window manager.

# 1. Start some programs to play with.
xclock &
xeyes &
sleep 2 && xterm &

# 2. Start our window manager.
exec ./target/debug/hadlock
