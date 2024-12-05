local Dom = require("./dom")()

local window = web.createWindow("Click Counter 3")
local dom = Dom:new(window)

local button = dom:createElement("button")
local data = {
	clicks = 0,
}

dom:setStyle(button, "width", "100%")
dom:setStyle(button, "height", "5rem")
dom:setProp(button, "innerHTML", `Clicks: {data.clicks}`)

dom:onEvent(button, "onclick", function()
	data.clicks += 1
	dom:setProp(button, "innerHTML", `Clicks: {data.clicks}`)
end)
