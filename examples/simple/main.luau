--!nocheck
local eventLoop = require("@luneweb/event_loop")
local window = require("@luneweb/window").new()

local webviewBuilder = require("@luneweb/webview").new()
webviewBuilder:with_url("https://github.com/luneweb")
webviewBuilder:build(window)

eventLoop.subscribe(window.id, function(event)
	if event == "CloseRequested" then
		window:close()
	end
end)
