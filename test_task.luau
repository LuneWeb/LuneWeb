task.spawn(function()
	print("thread 2")

	task.wait(1)

	print("1 second passed in thread 2")
end)

task.wait(1)

print("1 Second has passed!")

return {}
