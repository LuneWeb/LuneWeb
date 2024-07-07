# LuneWeb-rs

This repo contains all the lua libraries for creating windows and webviews using mlua

---

## Platform-specific dependencies

### Arch Linux / Manjaro

`sudo pacman -S gtk3`

`sudo pacman -S webkit2gtk-4.1`

### Debian / Ubuntu

`sudo apt install libgtk-3-dev`

`sudo apt install libwebkit2gtk-4.1-dev`

### Fedora

`sudo dnf install gtk3-devel webkit2gtk4.1-devel`

### Windows

WebView2 provided by Microsoft Edge Chromium is used. So LuneWeb supports Windows 7, 8, 10 and 11.

### macOS

WebKit is native on macOS so everything should be fine.
