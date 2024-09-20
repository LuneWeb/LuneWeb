local useDom = require("@luneweb/dom")

local window = WindowBuilder.new({
	title = "Basic",
	dev = true,
})

-- useDom.connect
useDom.once(window, function(dom)
	local div = dom.body:querySelector("div")

	div:listen("mouseenter", function()
		div:setStyle("backgroundColor", "black")
	end)

	div:listen("mouseleave", function()
		div:setStyle("backgroundColor", "white")
	end)
end)
