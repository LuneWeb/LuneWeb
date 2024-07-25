local task = require("@lune/task")

local window = WindowBuilder.new("Window1", "https://github.com/luneweb")
local listen = window.message:listen()

return {
	window = window,
	listen = function(callback)
		task.spawn(listen, function(message)
			callback(message)
		end)
	end,
}
