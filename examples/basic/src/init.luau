local datetime = require("@lune/datetime")
local task = require("@lune/task")

local window = WindowBuilder.new("Lune+Tauri", "http://localhost:5173")
local listen = window.message:listen()

print("Running luneweb", _VERSION)

task.spawn(listen, function(message)
	if message == "loaded" then
		window.message:send("print", "Hello, WebView!")
		window.message:send("print", {
			message = "Hello!",
			age = 18,
			time = datetime.now().unixTimestamp,
		})
	elseif typeof(message) == "table" and message.kind == "print" and message.value then
		print(message.value)
	end
end)
