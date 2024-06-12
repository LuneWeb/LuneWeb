# LuneWeb-rs

This repo contains all the lua libraries for creating windows and webviews using mlua

---

## Cross-Platform

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

### (NOT TESTED) macOS

WebKit is native on macOS so everything should be fine.

### Android / IOS

Not implemented yet.

---

## Crate

Since luneweb-rs isn't on crates.io, you'll have to get it from Github

```toml
[dependencies.luneweb]
git = "https://github.com/LuneWeb/LuneWeb-rs"
tag = "v0.2.4" # Double check to see if this is the latest version or not
```

---

## Getting started

Clone our [Template](https://github.com/LuneWeb/LuneWeb-template) repo to get started with using LuneWeb

Our template repo does alot of things, like bundling assets and providing the Lua instance with an api for accessing these assets, for more basic examples, you can read the `./examples` directory.
