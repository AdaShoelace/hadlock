# Hadlock  
![Rust](https://github.com/AdaShoelace/hadlock/workflows/Rust/badge.svg?branch=master)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://bitbucket.org/lbesson/ansi-colors)  
Hadlock is a tiling and stacking window manager for X.  
It is written completely in rust with and architecture heavily inspired by redux.  


---
### Gaps
![](./resources/gaps.jpg)
### Multimonitor
![](./resources/multimonitor.gif)
---
## Layouts  
### Floating  
---
![](./resources/hadlock-floating.jpg)
---
### Master pane layout  
---
![](./resources/master_pane.jpg)
---

## Progress

### Mode independent  
- [x] Floating mode
- [x] Tiled mode (Master pane)
- [x] Close window
- [x] Start terminal
- [x] Workspaces
- [x] Multimonitor support 
- [x] Window decorations
### Floating  
- [x] Move windows  
- [x] Move windows between workspaces  
- [x] Resize windows 
- [x] Snapping widows
- [ ] Cycle through windows
### Tiled (Master pane) 
- [x] Swap master window
- [x] Move windows between workspaces 
### Tiled (General)
- [x] Keyboard navigation

---  
### Planned features  
- Custom keybindings  
- Application rules (bind an applicaiton to a specific monitor and workspace)
- ~~Window gaps~~
- Hot reloading config
- Ability to script your own window layout  


## Installation
build with `cargo build --release` and put the binary in a directory in your `$PATH` eg `/usr/local/bin`  
Create `/usr/share/xsessions/hadlock.desktop` containing:  

```
[Desktop Entry]
Encoding=UTF-8
Name=Hadlock
Exec=hadlock ~/.config/hadlock/hadlock.json
Comment=Hadlock - a wm for x
Type=Application

```


## Configuration
The config file is written is json and should be placed in `~/.config/hadlock`  

```json
{
	"borderWidth": 1,
	"borderColor": 	{
		"Custom": "#480222"
	},
	"backgroundColor": {
		"Custom": "#939393"
	},
	"focusedBackgroundColor": "DefaultFocusedBackground",
	"outerGap": 14,
	"innerGap": 6,
	"smartGaps": true,
	"defaultLayout": "ColumnMaster",
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
				"example"
			]
		},
	]
}

```

## Keybindings  
At the moment custom key bindings is not available (it will come in the future)

(`mod` = windows key/super)

**Layout independent**  
* `mod + number` change workspace  
* `shift + mod + number` move window to workspace  
* `mod + enter` start terminal   
* `shift + mod + l` circulate layout  
 

**Floating**  
* `mod + right/left/up/down` or `mod + h/j/k/l` snap window to edge  
* `shift + mod + right/left/up/down` resize window  
* `mod + mouse1 + mousemovement` move window  
* `mod + mouse1` raise window  

**Column Master**  
* `mod + right/left/up/down` or `mod + h/j/k/l` change focus window  
* `mod + m` swap master window

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
