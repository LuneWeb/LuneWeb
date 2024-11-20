--!nocheck
--!nolint UnknownGlobal

print("Hello, World!")

task.spawn(function()
	print("thread 2")

	task.wait(1)

	print("1 second passed in thread 2")
end)

task.wait(1)

print("1 Second has passed!")

local window = web.createWindow()
print(typeof(window), window.id == window.id)

while true do
	window.title = tostring(math.random(1, 100))
	task.wait(1)
end
