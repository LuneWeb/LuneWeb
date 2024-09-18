local window = WindowBuilder.new({
	title = "Basic",
	dev = true,
})

print(_VERSION)

window.message:listen(function(x)
	print("listener1:", x)
end)

window.message:listen(function(x)
	print("listener2:", x)
end)

window.webview:eval([[
window.luneweb.postMessage(1)
window.luneweb.postMessage(2)
window.luneweb.postMessage(3)
window.luneweb.postMessage(4)
]])
