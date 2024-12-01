local scripts = require("./scripts")
local onLoadCallbacks: { () -> () } = {}

local window = web.createWindow("GUI")
local webview = web.createWebView()
	:withInitScript(scripts.gui)
	:withInitScript("fetch('http://load.x', { method: 'POST' })")
	:withCustomProtocol("load", function(_, headers)
		for _, fn in onLoadCallbacks do
			task.spawn(fn)
		end

		return "success"
	end)
	:build(window)

-- scripts only run on `window.onload`
-- so we have to reload the page once
webview.url = webview.url

local function e(tag: string, innerHtml: string?)
	local self = {
		i = if innerHtml then webview:evaluate(`e("{tag}", "{innerHtml}")`) else webview:evaluate(`e("{tag}")`),
	}

	function self:append(): ()
		return webview:evaluate(`attach({self.i})`)
	end

	return self
end

table.insert(onLoadCallbacks, function()
	e("h1", "Hello, World!"):append()
end)

while true do
	-- keep webview from being gc'd
	task.wait()
end
