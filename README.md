# What is contained in this repository?

This is an utility to patch Freedom Planet 2's internal resolution from 640x360 to a multiple of it, considerably lowering Moiré Pattern Interference and other assorted pixel crawl due pixel art resizing to arbitrary, non-integer scaling factors during gameplay.

![](demo.gif)

# Usage:

- [Download the tool](https://github.com/JohnnyonFlame/fp2res/releases/latest/download/fp2res.exe) (or build with `cargo build`).
- Copy `fp2res.exe` into the `Freedom Planet 2` folder, alongside the `FP2.exe` file.
- (Optional) Backup the `sharedassets0.assets` file in the `FP2_Data` folder.
- Run `fp2res.exe` and input your desired scaling factor.
- Turn on `Filter` in the in-game settings.
- Have fun!

To uninstall, use Steam's `Verify integrity of game files...` or restore your `sharedassets0.assets` backup.

# Resolution table

| Scale | Resolution |
| ----- | ---------- |
| 1     | 640x360    |
| 2     | 1280x720   |
| 3     | 1920x1080  |
| 4     | 2560x1440  |
| 5     | 3200x1800  |
| 6     | 3840x2160  |
| 7     | 4480x2520  |
| 8     | 5120x2880  |
| 9     | 5760x3240  |
| 10    | 6400x3600  |
| 11    | 7040x3960  |
| 12    | 7680x4320  |

# License

This is free software. The source files in this repository are released under the [Modified BSD License](LICENSE.md), see the license file for more information.