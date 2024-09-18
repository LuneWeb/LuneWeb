export type DomElement = {}

export type Dom = {
	body: (Window) -> DomElement,
}

local function getDom(): Dom
	local dom = {}

	function dom.body(window: Window): DomElement
		return {}
	end

	return dom
end

return getDom()
