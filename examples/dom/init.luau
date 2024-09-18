local dom = require("@luneweb/dom")
local window = WindowBuilder.new({
	title = "Basic",
	dev = true,
})

local body = dom.body(window)

print(dom)
