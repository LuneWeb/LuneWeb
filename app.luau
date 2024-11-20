--!nocheck
--!nolint UnknownGlobal

task.spawn(require, "./test_task")

local window = web.createWindow()
print(typeof(window), window.id == window.id)

while true do
	window.title = tostring(math.random(1, 100))
	task.wait(1)
end
