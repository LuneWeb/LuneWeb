local datetime = require("@lune/datetime")
local message = require("@luneweb/message")

message.onLoad(function()
	message.share("Hello, Javascript!")
	message.share({
		message = "Hello!",
		age = 18,
		time = datetime.now().unixTimestamp,
	})

	print(`Javascript returned: {message.send("Channel1", "Hello!")}`)
end)
