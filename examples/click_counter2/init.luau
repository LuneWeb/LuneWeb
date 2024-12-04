type DataScheme = {
	clicks: number,
}

local data: DataScheme = {
	clicks = 0,
}

local function createUI(data: DataScheme, webview: WebView)
	webview:evaluate(string.format(
		[[
    const b = document.createElement('button');
    b.innerHTML = 'Clicks: %*';
    b.style.width = '100%%';
    b.style.height = '5rem';
    b.onclick = () => {
        window.luneweb.post('click');
    };
    document.body.appendChild(b);
    ]],
		data.clicks
	))
end

local function updateUI(data: DataScheme, webview: WebView)
	webview:evaluate(string.format(
		[[
    b.innerHTML = 'Clicks: %*';
    ]],
		data.clicks
	))
end

do
	local window = web.createWindow("Click Counter 2")
	local webview
	webview = web.createWebView()
		:withChannel("load", function()
			createUI(data, webview)
		end)
		:withChannel("click", function()
			data.clicks += 1
			updateUI(data, webview)
		end)
		:withInitScript("window.onload = () => { window.luneweb.post('load') }")
		:build(window)
	-- window.onload doesn't fire when webview is first created
	-- so we'll fire it by reloading the page manually
	webview.url = webview.url
end
