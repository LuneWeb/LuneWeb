local datetime = require("@lune/datetime")

app:onLoad(function()
	app:shareMessage("Hello, WebView!")
	app:shareMessage({
		message = "Hello!",
		age = 18,
		time = datetime.now().unixTimestamp,
	})

	print(`WebView returned: {app:sendMessage("Channel1", "Hello!")}`)
end)

app:run()
