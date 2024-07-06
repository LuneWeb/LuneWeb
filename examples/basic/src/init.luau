local task = require("@lune/task")

while true do
	print(os.clock())
	task.wait(1)
end
