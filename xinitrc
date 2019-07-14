# xinitrc file for testing our window manager.
xrdb -load Xresources

#compton &
# 0. Set wallpaper using feh
feh --bg-scale ./wallpaper.jpg &

# 1. Start some programs to play with.
xterm &
xclock &
#xeyes &
#pcmanfm &
#mousepad &

# 2. Start our window manager.
exec ./target/debug/hadlock
