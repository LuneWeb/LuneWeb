local window1 = WindowBuilder.new("Window1")
local window2 = WindowBuilder.new("Window2")

while true do
	task.wait(1)
	window1.title = "Hello"
	window2.title = "World"
	task.wait(1)
	window1.title = "World"
	window2.title = "Hello"

	window1.message:eval("console.log('Loop Finished')")
end
