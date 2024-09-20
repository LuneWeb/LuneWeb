local useDom = require("@luneweb/dom")

local window = WindowBuilder.new({
	title = "Basic",
	dev = true,
})

-- useDom.once
useDom.connect(window, function(dom)
	local div = dom.body:querySelector("div")

	div:listen("mouseenter", function()
		div:setStyle("backgroundColor", "black")
		div:setStyle("color", "white")
		div:setInner("Hey!")
	end)

	div:listen("mouseleave", function()
		div:setStyle("backgroundColor", "white")
		div:setStyle("color", "black")
		div:setInner("Bye!")
	end)
end)
