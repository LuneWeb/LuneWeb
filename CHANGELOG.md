# Changelog

All notable changes to LuneWeb will be documented in this file.

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->

## [v0.6.0-beta4]

- `Message:listen` returns a function for disconnecting itself

- `WebView:eval` checks for syntax error in macos and linux

- `AudioBuilder?` global for creating audio instances

[docs for AudioBuilder](https://github.com/LuneWeb/docs/blob/0.6.0/src/luau/audio_builder.md), [docs for AudioSource](https://github.com/LuneWeb/docs/blob/0.6.0/src/luau/audio_source.md)

## [v0.6.0-beta3]

- added methods `setProp`, `getProp` and `appendChild` to `DomElement`

- added method `createElement` to `Dom`

- `useDom` spawns callbacks in new threads now

## [v0.6.0-beta2]

- Fixed `luneweb setup` throwing "No such file or directory" error

## [v0.6.0-beta1]

- `window.message:listen` method spawns in a new thread automatically, so it doesn't yield anymore.

- `luneweb setup` installs builtin luau libraries in your home directory

- new luau library for interacting with the webview DOM [[Example Code](https://github.com/LuneWeb/LuneWeb/blob/v0.6.0-beta1/examples/dom/init.luau)]
