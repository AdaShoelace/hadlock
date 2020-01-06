# Hadlock  

### Work in progress
Hadlock is a (soon to be) tiling window manager for X.  
The architecture behind hadlock is inspired by redux but is not as strict as can be.  

---
![](hadlock-multi-monitor.gif)
---

## Progress

### Mode independent  
- [x] Floating mode
- [ ] Tiled mode
- [x] Close window
- [x] Start terminal
- [x] Workspaces
- [x] Multimonitor support 
- [x] Window decorations
- [ ] Decorated windows
- [ ] Text in decoration  
### Floating  
- [x] Move windows  
- [x] Move windows between workspaces  
- [x] Resize windows 
- [x] Snapping widows
- [ ] Cycle through windows
### Tiled  
- [ ] Move windows 
- [ ] Move windows between workspaces 
- [ ] Resize windows 

## Installation
Create `/usr/share/xsessions/hadlock.desktop` containing:  
```
[Desktop Entry]
Encoding=UTF-8
Name=Hadlock
Exec=hadlock 
Comment=Hadlock - a wm for x
Type=Application

```


## Configuration
The config file is written is json and should be placed in `~/.config/hadlock`  
```
{
	"decorationHeight": 20,
	"borderWidth": 2,
	"innerBorderWidth": 0,
	"borderColor": 	{
		"Custom": 9437222
	},
	"backgroundColor": "DefaultBackground",
	"focusedBackgroundColor": "DefaultFocusedBackground",
	"workspaces": {
		"1": "1",
		"2": "2",
		"3": "3",
		"4": "4",
		"5": "5",
		"6": "6",
		"7": "7",
		"8": "8",
		"9": "9"
	},
	"terminal" : "alacritty",
	"commands": [
		{
			"execTime": "Pre",
			"program": "feh",
			"args": [
				"--bg-scale",
				"~/Pictures/triangles.jpg"
			]
		},
		{
			"execTime": "Post",
			"program": "polybar",
			"args": [
				"--config=./polyconf",
				"--log=ERROR",
				"example"
			]
		}
	]
}

```  
At the moment decorations is not available and custom color codes is written in dec, this will change!  

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
