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
print(typeof(webview), webview.id == webview.id)

while true do
	window.title = tostring(math.random(1, 100))
	webview:loadHtml(`<h1>{window.title}<\h1>`)
	task.wait(1)
end
