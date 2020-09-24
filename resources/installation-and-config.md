## Installation

### Arch
Thanks to [orhun](https://github.com/orhun) Hadlock can now be installed via `aur` using for example `yay -S hadlock`
### Build from scratch
build with `cargo build --release` and put the binary in a directory in your `$PATH` eg `/usr/local/bin`  
Create `/usr/share/xsessions/hadlock.desktop` containing:  

```
[Desktop Entry]
Encoding=UTF-8
Name=Hadlock
Exec=hadlock ~/.config/hadlock/hadlock.ron
Comment=Hadlock - a wm for x
Type=Application

```


## Configuration
The config file is written is ron (Rust object notation) and should be placed in `~/.config/hadlock/hadlock.ron`  
This config is just an example until a proper wiki has been created.  

```ron
(
    modKey: "Super",
    borderWidth: 2,
    borderColor: Custom ("#6aac7e"),
    backgroundColor: Custom ("#99978b"),
    focusedBackgroundColor: DefaultFocusedBackground,
    outerGap: 14,
    innerGap: 6,
    smartGaps: true,
    defaultLayout: ColumnMaster,
    workspaces: {
        1: "1",
        2: "2",
        3: "3",
        4: "4",
        5: "5",
        6: "6",
        7: "7",
        8: "8",
        9: "9"
    },
    terminal: "alacritty",
    keyBindings: [
        (
            modKey: Some("Shift"),
            key: Letter("q"),
            effect: Kill
        ),
        (
            modKey: Some("Shift"),
            key: Letter("r"),
            effect: Reorder
        ),
        (
            modKey: Some("Shift"),
            key: Letter("c"),
            effect: Center
        ),
        (
            key: Letter("Return"),
            effect: OpenTerm
        ),
        (
            modKey: Some("Shift"),
            key: Letter("Right"),
            effect: Resize(10, Horizontal)
        ),
        (
            modKey: Some("Shift"),
            key: Letter("Left"),
            effect: Resize(-10, Horizontal)
        ),
        (
            modKey: Some("Shift"),
            key: Letter("Down"),
            effect: Resize(10, Vertical)
        ),
        (
            modKey: Some("Shift"),
            key: Letter("Up"),
            effect: Resize(-10, Vertical)
        ),
        (
            modKey: Some("Shift"),
            key: Letter("f"),
            effect: ToggleMonocle
        ),
        (
            key: Letter("f"),
            effect: ToggleMaximize
        ),
        (
            modKey: Some("Shift"),
            key: Letter("e"),
            effect: Exit
        ),
        (
            modKey: Some("Shift"),
            key: Letter("l"),
            effect: CirculateLayout
        ),
        (
            key: Letter("Right"),
            effect: ShiftWindow(East)
        ),
        (
            key: Letter("Left"),
            effect: ShiftWindow(West)
        ),
        (
            key: Letter("Up"),
            effect: ShiftWindow(North)
        ),
        (
            key: Letter("Down"),
            effect: ShiftWindow(South)
        ),
        (
            modKey: Some("Shift"),
            key: Letter("m"),
            effect: SwapMaster
        ),
        (
            key: Number,
            effect: ChangeCurrentWorkspace
        ),
        (
            key: Number,
            modKey: Some("Shift"),
            effect: MoveToWorkspace
        ),
        (
            key: Letter("d"),
            effect: Custom ((
                    execTime: Now,
                    program: "dmenu_recency",
                    args: []
            ))
        ),
        (
            key: Letter("Control"),
            effect: Custom((
                    execTime: Now,
                    program: "pactl",
                    args: [
                    "set-sink-volume",
                    "2",
                    "+5%"
                    ]
            ))
        ),
        (
            key: Letter("Control"),
            modKey: Some("Shift"),
            effect: Custom((
                    execTime: Now,
                    program: "pactl",
                    args: [
                    "set-sink-volume",
                    "2",
                    "-5%"
                    ]
            ))
        )

        ],
        commands: [
            (
                execTime: Pre,
                program: "feh",
                args: [
                "--bg-scale",
                "~/Pictures/triangles.jpg"
                ]
            ),
            (
                execTime: Post,
                program: "polybar",
                args: [
				"-q",
                "--config=~/.config/polybar/polyconf",
                "DisplayPort-1"
                ]
            ),
        ]
        )

```

## Keybindings  
Fully customizable  
See example config
