local useDom = require("@luneweb/dom")

local window = WindowBuilder.new({
	title = "Basic",
	dev = true,
})

-- useDom.once
useDom.connect(window, function(dom)
	local text = dom:createElement("h1")
	local audio = dom:createElement("audio")

	text:appendChild(audio)
	dom.body:appendChild(text)

	dom.body:listen("mouseenter", function()
		dom.body:setStyle("backgroundColor", "black")
		text:setStyle("color", "white")
		text:setProp("innerText", "Hey!")
	end)

	dom.body:listen("mouseleave", function()
		dom.body:setStyle("backgroundColor", "white")
		text:setStyle("color", "black")
		text:setProp("innerText", "Bye!")
	end)
end)
