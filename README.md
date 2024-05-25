# LuneWeb-rs

This repo contains all the lua libraries for creating windows and webviews using mlua

---

## Example usage

```rs
// We're using a forked version of lune from 'https://github.com/LuneWeb/lune' which allows us to use this struct called 'GlobalsContextBuilder' to customize the globals
use lune_std::context::GlobalsContextBuilder;
use mlua_luau_scheduler::Scheduler;

let lua = Rc::new(mlua::Lua::new());
let builder = GlobalsContextBuilder::default();

// Make our Lua struct ready to be used by luneweb and lune libraries
luneweb::lua::patch_lua(&lua);

// Inject luneweb libraries
luneweb::lua::inject_globals(&builder)?;

// Inject lune libraries
lune_std::inject_globals(&lua, builder)?;

let sched = Scheduler::new(lua);
let path = PathBuf::from("src/init.luau"); // path to our luau code
let chunk = fs::read_to_string(&path)?;

let main = lua.load(chunk).set_name(path.to_string_lossy().to_string());
sched.push_thread_back(main, ())?;
sched.run().await;
```

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

## Getting started

Clone our [Template](https://github.com/LuneWeb/LuneWeb-template) repo to get started with using LuneWeb
