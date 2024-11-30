local scripts = require("./scripts")

local window = web.createWindow("GUI")
local webview = web.createWebView({
	parent = window,
	scripts = { scripts.gui },
})

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

e("h1", "Hello, World!"):append()

while true do
	-- keep webview from being gc'd
	task.wait()
end
