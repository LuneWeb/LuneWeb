local channels = require("./channels")
local task = require("@lune/task")
local doms: { [Window]: Dom } = {}

export type DomElement = {
	querySelector: (DomElement, tag: string) -> DomElement?,
	appendChild: (DomElement, child: DomElement) -> (),
	setProp: (DomElement, k: string, v: any) -> (),
	getProp: (DomElement, k: string) -> any,
	setStyle: (DomElement, style: string, value: string) -> (),
	getStyle: (DomElement, style: string) -> string,
	listen: (DomElement, event: string, callback: () -> ()) -> (),
	_dom: Dom,
	_id: number,
}

export type Dom = {
	_callbacks: { (Dom) -> () },
	_listeners: { () -> () },
	_window: Window,
	_loaded: boolean,
	createElement: (Dom, tag: string) -> DomElement,
	body: DomElement?,
}

local function createElement(dom: Dom, id: number?): DomElement
	assert(id, "id is reqiured to create an element")

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

	function element.appendChild(self: DomElement, child: DomElement)
		task.spawn(channels.message, self._dom._window, "dom-appendChild" :: "dom-appendChild", {
			id = self._id,
			childId = child._id,
		})
	end

	function element.setProp(self: DomElement, k: string, v: any): ()
		task.spawn(channels.message, self._dom._window, "dom-setProp" :: "dom-setProp", {
			id = self._id,
			k = k,
			v = v,
		})
	end

	function element.getProp(self: DomElement, k: string): any
		return channels.message(self._dom._window, "dom-getProp", {
			id = self._id,
			k = k,
		})
	end

	function element.setStyle(self: DomElement, style: string, value: string)
		task.spawn(channels.message, self._dom._window, "dom-setStyle" :: "dom-setStyle", {
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
		table.insert(self._dom._listeners, callback)

		channels.message(self._dom._window, "dom-createListener", {
			id = self._id,
			event = event,
			listenerId = #self._dom._listeners,
		})
	end

	return element
end

local function initDom(window: Window): Dom
	local dom = {
		_window = window,
		_listeners = {},
		_callbacks = {},
		_loaded = false,
	}

	channels.register(dom._window)

	function dom.createElement(self: Dom, tag: string): DomElement
		local value = channels.message(self._window, "dom-createElement", {
			tag = tag,
		})

		return createElement(self, value.elementId)
	end

	do
		dom._window.message:listen(function(message)
			if message == "loaded" then
				table.clear(dom._listeners)

				dom.body = createElement(dom, channels.message(dom._window, "dom-getBody", nil).elementId)
				dom._loaded = true

				for _, callback in dom._callbacks do
					task.spawn(callback, dom)
				end
			elseif typeof(message) == "table" then
				if message.listenerId and dom._listeners[message.listenerId] then
					dom._listeners[message.listenerId]()
				end
			end
		end)

		doms[dom._window] = dom
	end

	return dom
end

local function useDom(window: Window, callback: (Dom) -> ()): () -> ()
	local dom = doms[window] or initDom(window)

	table.insert(dom._callbacks, callback)

	if dom._loaded then
		task.spawn(callback, dom)
	end

	return function()
		table.remove(dom._callbacks, table.find(dom._callbacks, callback))
	end
end

return {
	connect = function(window, callback)
		return useDom(window, callback)
	end,
	once = function(window, callback)
		local dc

		dc = useDom(window, function(dom)
			task.spawn(callback, dom)
			dc()
		end)
	end,
}
