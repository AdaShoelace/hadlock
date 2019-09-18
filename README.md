# Hadlock  

### Work in progress
Hadlock is a (soon to be) tiling window manager for X.  
Built for learning purposes.  

---
![](hadlock-alpha.gif)
---

## Progress

### Mode independent  
- [x] Floating mode
- [ ] Tiled mode  
- [x] Close window 
- [x] Start terminal 
- [x] Workspaces 
- [ ] Multimonitor support 
- [x] Window decorations 
- [ ] Undecorated windows 
- [ ] Text in decoration  
### Floating  
- [x] Move windows (Floating)  
- [x] Move windows between workspaces(Floating)  
- [x] Resize windows (Floating) 
- [x] Snapping widows (Floating) 
### Tiled  
- [ ] Move windows (Tiled) 
- [ ] Move windows between workspaces(Tiled) 
- [ ] Resize windows (Tiled) 

## Installation
_TBD_

## Configuration
_TBD_

## Testing
In order to test/run hadlock some dependencies are needed:
```
xorg-server-xephyr
xorg-xinit
libxinerama
```

Run `./run.sh` to start a test session in a nested X server. The default `xinitrc` in this repo attempts to run `alacritty`, `compton`, and `polybar`. Install them or customize the `xinitrc` to your liking.

## Honorable mentions
During the development of Hadlock I've found alot of inspiration in other projects and gotten help and insight from people far more experienced than myself.
Therefore I'd like to give credit to these awesome projects and thank those that was kind enough to help me out.

WMs Hadlock was inspired by:
- [BerryWM](https://github.com/JLErvin/berry)
- [LeftWM](https://github.com/leftwm/leftwm)
- [Wtfw](https://github.com/Kintaro/wtftw)

Thanks:
- [lex148](https://github.com/lex148)
