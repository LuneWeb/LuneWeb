local scripts = require("./scripts")
local onLoadCallbacks: { () -> () } = {}
local onLoadThreads: { thread } = {}

local window = web.createWindow("GUI")
local webview = web.createWebView()
	:withInitScript(scripts.gui)
	:withInitScript("window.luneweb.post('load')")
	:withChannel("load", function(_)
		for _, thread in onLoadThreads do
			task.cancel(thread)
		end

		table.clear(onLoadThreads)

		for _, fn in onLoadCallbacks do
			table.insert(onLoadThreads, task.spawn(fn))
		end
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
	task.wait(1)
	e("h3", "LuneWeb2"):append()
end)
