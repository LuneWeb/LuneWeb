local datetime = require("@lune/datetime")
local message = require("@luneweb/message")

message.send("Hello, Javascript!")
message.send({
	message = "Hello!",
	age = 18,
	time = datetime.now().unixTimestamp,
})

print("Hello, World! 🤖")
