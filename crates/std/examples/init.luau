local app = require("./window")

local function loaded()
	app.window.webview:eval("console.log('Hello, World!')")
	app.window.webview:eval([[
	window.luneweb.postMessage({
		success: true
	})
	]])
end

app.listen(function(_message: string)
	local message = serde.decode("json", _message)

	if message == "loaded" then
		loaded()
	elseif typeof(message) == "table" and message.success then
		print("Success")
	end
end)
