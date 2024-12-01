# LuneWeb

Thread safe Lua(u) bindings for [TauriApps](https://github.com/tauri-apps)

## `v2.x.x` (Work in progress)

v2 is completely independent from [lune](https://github.com/lune-org/lune), we have our own task scheduler/library implementation, which also is not fully compatible with Lune/Roblox

the new task scheduler runs in the windowing thread which results in getting immediate results when you change a property on a window + things like resizing a window will no longer block your luau threads

### Lune std libraries

once lune switches to mlua `0.10.x` the lune std libraries will be added to the globals environment

e.g. to access the net library you'll use `lune.net`

```luau
lune.net.serve(8080, function(request)
    return {
        status = 200,
        body = "Echo:\n" .. request,
    }
end)
```
