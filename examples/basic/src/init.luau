local datetime = require("@lune/datetime")

app:onLoad(function()
	app:shareMessage("Hello, Javascript!")
	app:shareMessage({
		message = "Hello!",
		age = 18,
		time = datetime.now().unixTimestamp,
	})

	print(`Javascript returned: {app:sendMessage("Channel1", "Hello!")}`)
end)

app:run()
