local task = require("@lune/task")

local window = WindowBuilder.new("Basic", "about:blank")

task.spawn(window.message.listen, window.message, function(x)
	print("listener1:", x)
end)
task.spawn(window.message.listen, window.message, function(x)
	print("listener2:", x)
end)

window.webview:eval([[
window.luneweb.postMessage(1)
window.luneweb.postMessage(2)
window.luneweb.postMessage(3)
window.luneweb.postMessage(4)
]])
