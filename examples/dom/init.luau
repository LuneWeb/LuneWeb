local getDom = require("@luneweb/dom")

local window = WindowBuilder.new({
	title = "Basic",
	dev = true,
})

window.message:listen(function(msg)
	if msg == "loaded" then
		local dom = getDom(window)
		local div = dom.body:querySelector("div")

		div:listen("mouseenter", function()
			div:setStyle("backgroundColor", "black")
		end)

		div:listen("mouseleave", function()
			div:setStyle("backgroundColor", "white")
		end)
	end
end)
