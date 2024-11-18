--!nocheck
--!nolint UnknownGlobal

task.spawn(function()
	app.createWindow()
	print("Task1")
end)

task.spawn(function()
	app.createWindow()
	print("Task2")
end)

task.spawn(function()
	print("Task3")
end)

print("Hello, World!")
