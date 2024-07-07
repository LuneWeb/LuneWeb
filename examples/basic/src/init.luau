local task = require("@lune/task")
local message = require("@luneweb/message")

message.send("Hello, Javascript!")

task.wait(5)

print("Hello, World! 🤖")
