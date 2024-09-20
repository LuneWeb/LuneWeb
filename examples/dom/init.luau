local getDom = require("@luneweb/dom")

local window = WindowBuilder.new({
	title = "Basic",
	dev = true,
})

window.message:listen(function(msg)
	if msg == "loaded" then
		local dom = getDom(window)
		local div = dom.body:querySelector("div")

		div:setStyle("backgroundColor", "black")

		print(div:getStyle("backgroundColor"))
	end
end)
