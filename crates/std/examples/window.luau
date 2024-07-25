local window = WindowBuilder.new("Window1")
local listen = window.message:listen()

return {
	window = window,
	listen = function(callback)
		task.spawn(listen, callback)
	end,
}
