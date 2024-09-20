local channels = require("./channels")

export type DomElement = {
	querySelector: (DomElement, tag: string) -> DomElement?,
	setStyle: (DomElement, style: string, value: string) -> (),
	getStyle: (DomElement, style: string) -> string,
	listen: (DomElement, event: string, callback: () -> ()) -> (),
	_dom: Dom,
	_id: number,
	_listeners: { () -> () },
}

export type Dom = {
	_window: Window,
	body: DomElement,
}

local function createElement(dom: Dom, id: number?): DomElement
	assert(id, "id is reqiured to create an element")

	local element = {
		_dom = dom,
		_id = id,
		_listeners = {},
	}

	function element.querySelector(self: DomElement, tag: string): DomElement?
		local value = channels.message(self._dom._window, "dom-querySelect", {
			id = self._id,
			tag = tag,
		})

		return createElement(dom, value.elementId)
	end

	function element.setStyle(self: DomElement, style: string, value: string)
		channels.message(self._dom._window, "dom-setStyle", {
			id = self._id,
			style = style,
			value = value,
		})
	end

	function element.getStyle(self: DomElement, style: string): string
		return channels.message(self._dom._window, "dom-getStyle", {
			id = self._id,
			style = style,
		})
	end

	function element.listen(self: DomElement, event: string, callback: () -> ())
		table.insert(self._listeners, callback)

		channels.message(self._dom._window, "dom-createListener", {
			id = self._id,
			event = event,
			listenerId = #self._listeners,
		})
	end

	do
		element._dom._window.message:listen(function(message)
			if typeof(message) ~= "table" then
				return
			end

			if message.listenerId and element._listeners[message.listenerId] then
				element._listeners[message.listenerId]()
			end
		end)
	end

	return element
end

--[[
Must be called everytime the page is (re)loaded
]]
local function getDom(window: Window): Dom
	local dom = {
		_window = window,
	}

	do
		channels.register(dom._window)
		dom.body = createElement(dom, channels.message(dom._window, "dom-getBody", nil).elementId)
	end

	return dom
end

return getDom
