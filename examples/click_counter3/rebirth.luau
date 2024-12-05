local Dom = require("../luau_packages/dom")()

local window = web.createWindow("Click Counter 3")
local dom = Dom:new(window)

local button = dom:createElement("button")
local data = {
	clicks = 0,
	multiplier = 1,
}

local rebirth = {
	shown = false,
	el = dom:createElement("button"),
}

dom:setStyle(rebirth.el, "visibility", "hidden")

function rebirth.upgradeCost(self: typeof(rebirth), data: typeof(data)): number
	return data.multiplier * 50
end

function rebirth.show(self: typeof(rebirth), data: typeof(data)): ()
	if self.shown then
		return
	end

	dom:setProp(self.el, "innerHTML", `{data.multiplier + 1}x clicks (rebirth)`)
	dom:onEvent(self.el, "onclick", function()
		if data.clicks < self:upgradeCost(data) then
			return
		end

		data.clicks = 0
		data.multiplier += 1

		dom:setStyle(self.el, "visibility", "hidden")
		dom:setProp(button, "innerHTML", `Clicks: {data.clicks}`)

		self.shown = false
	end)

	dom:setStyle(self.el, "visibility", "visible")

	self.shown = true
end

dom:setStyle(button, "width", "100%")
dom:setStyle(button, "height", "5rem")
dom:setProp(button, "innerHTML", `Clicks: {data.clicks}`)

dom:onEvent(button, "onclick", function()
	data.clicks += 1 * data.multiplier
	dom:setProp(button, "innerHTML", `Clicks: {data.clicks}`)

	if data.clicks >= rebirth:upgradeCost(data) then
		rebirth:show(data)
	end
end)
