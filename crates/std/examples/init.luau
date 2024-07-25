local window = WindowBuilder.new("Window1")
local listener = window.message:listen()

local function loaded()
	window.webview:eval("console.log('Hello, World!')")
	window.webview:eval([[
	window.luneweb.postMessage({
		success: true
	})
	]])
end

task.spawn(listener, function(_message: string)
	local message = serde.decode("json", _message)

	if message == "loaded" then
		loaded()
	elseif typeof(message) == "table" and message.success then
		print("Success")
	end
end)
