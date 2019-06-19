# xinitrc file for testing our window manager.
xrdb -load Xresources
# 0. Set wallpaper using feh
feh --bg-scale ./wallpaper.jpg &

# 1. Start some programs to play with.
xclock &
xeyes &
#pcmanfm &
xterm &

# 2. Start our window manager.
exec ./target/debug/hadlock
