local channels = require("./channels")

export type DomElement = {
	querySelector: (DomElement, tag: string) -> DomElement?,
	setStyle: (DomElement, style: string, value: string) -> (),
	getStyle: (DomElement, style: string) -> string,
	_dom: Dom,
	_id: number,
}

export type Dom = {
	_window: Window,
	body: (Dom) -> DomElement,
}

local function createElement(dom: Dom, id: number?): DomElement
	local element = {
		_dom = dom,
		_id = id,
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

	return element
end

local function getDom(window: Window): Dom
	local dom = {
		_window = window,
	}

	channels.register(dom._window)

	function dom.body(self: Dom): DomElement
		local value = channels.message(self._window, "dom-get-body", nil)

		return createElement(self, value.elementId)
	end

	return dom
end

return getDom
