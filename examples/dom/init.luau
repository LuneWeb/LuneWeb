local getDom = require("@luneweb/dom")

local window = WindowBuilder.new({
	title = "Basic",
	dev = true,
})

local dom = getDom(window)

window.message:listen(function(msg)
	if msg == "loaded" then
		local body = dom:body()
		local div = body:querySelector("div")

		div:setStyle("backgroundColor", "black")

		print(div:getStyle("backgroundColor"))
	end
end)
