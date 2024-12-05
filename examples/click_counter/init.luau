local lib = require("./ui_builder")

local data = {
	clicks = 0,
}

local window = web.createWindow("Basic App")
local handle = lib.builder(web.createWebView())

local webview: WebView
webview = handle
	:take()
	:withChannel("clicked", function()
		data.clicks += 1
        webview:evaluate(`el.innerHTML = 'Clicks: {data.clicks}'`)
        print(`Clicked: {data.clicks}`)
	end)
	:build(window)

handle:onLoad(function()
	webview:evaluate(`\
    const el = document.createElement('button');\
\
    el.style.width = '100%';\
    el.style.height = '5rem';\
    el.innerHTML = 'Clicks: {data.clicks}';\
    el.onclick = () => \{\
        window.luneweb.post('clicked');\
    };\
\
    document.body.appendChild(el);\
    `)
end)

-- reload page to fire window.onload
webview.url = webview.url
