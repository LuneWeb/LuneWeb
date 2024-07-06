local task = require("@lune/task")
local module = require("module")

while true do
	print(os.clock())
	task.wait(1)
end
