--!nocheck
--!nolint UnknownGlobal

print("Hello, World!")

spawn(function()
	print("thread 2")

	wait(1)

	print("1 second passed in thread 2")
end)

wait(1)

print("1 Second has passed!")
