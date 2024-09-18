local window = WindowBuilder.new({
	title = "Basic",
	dev = true,
})

local dom = require("@luneweb/dom")(window)

window.message:listen(function(msg)
	if msg == "loaded" then
		local body = dom.body()
		local div = body:querySelector("div")
	end
end)
