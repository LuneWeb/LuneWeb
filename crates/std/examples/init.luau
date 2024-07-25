local app = require("./window")

local function loaded()
	app.window.webview:eval("console.log('Hello, World!')")
	app.window.webview:eval([[
	window.luneweb.postMessage({
		success: true
	})

	window.luneweb.listen("response", (message) => {
		if (message.success) {
			console.log("Success")
		}
	})
	]])
end

app.listen(function(message)
	if message == "loaded" then
		loaded()
	elseif typeof(message) == "table" and message.success then
		app.window.message:send("response", message)

		print("Success")
	end
end)
