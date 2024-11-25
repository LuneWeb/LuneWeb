task.spawn(function()
	local ret = require("./test_task")
	print(`"./test_task" returned a {typeof(ret)}.`)
end)

task.spawn(function()
	local ret = require("./test_task")
	print(`"./test_task" returned a {typeof(ret)}.`)
end)

local window = web.createWindow()
print(typeof(window), window.id == window.id)

local webview = web.createWebView(window)
print(webview)

while true do
	window.title = tostring(math.random(1, 100))
	task.wait(1)
end
