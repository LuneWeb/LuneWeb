local DOM_SCRIPT_NAME = debug.info(1, "s")

type DomMetatable = {
	__index: {
		createElement: (self: Dom, tag: string) -> number,

		setProp: (self: Dom, elementId: number, k: string, v: string) -> (),
		setStyle: (self: Dom, elementId: number, k: string, v: string) -> (),

		onEvent: (self: Dom, elementId: number, event: string, callback: () -> ()) -> (),
	},
}

type DomBuilder = DomMetatable & {
	new: (self: DomBuilder, window: Window) -> Dom,
}

type EventData = {
	elementId: number,
	event: string,
}

type ElementData = {
	tag: string,
	props: { [string]: string },
	styles: { [string]: string },
	events: { [string]: { () -> () } },
}

type DomData = {
	webview: WebView,
	__data: {
		elements: { ElementData },
	},
}

export type Dom = typeof(setmetatable({} :: DomData, {} :: DomMetatable))

local Dom = {
	__index = {},
}

function Dom.new(self: DomBuilder, window: Window): Dom
	local dom: Dom

	dom = setmetatable({
		webview = web.createWebView()
			:withInitScript([[
			const ELEMENTS = [];
			]])
			:withInitScript("window.luneweb.post('load')")
			:withChannel("load", function()
				for _, elementData in dom.__data.elements do
					local id = dom:createElement(elementData.tag)

					for k, v in elementData.props do
						dom:setProp(id, k, v)
					end

					for k, v in elementData.styles do
						dom:setStyle(id, k, v)
					end

					for k, v in elementData.events do
						for _, callback in v do
							dom:onEvent(id, k, callback)
						end
					end
				end
			end)
			:withChannel("event", function(data0)
				-- dictionary type validation is a bit yucky in luau 😬
				assert(typeof(data0) == "table")
				local data1 = data0 :: { elementId: unknown, event: unknown }
				assert(typeof(data1.elementId) == "number")
				assert(typeof(data1.event) == "string")
				local data = data0 :: EventData
				local element = dom.__data.elements[data.elementId]

				for _, callback in element.events[data.event] do
					task.spawn(callback)
				end
			end)
			:build(window),
		__data = {
			elements = {},
		},
	}, self)

	dom.webview.url = dom.webview.url

	return dom
end

function Dom.__index.createElement(self: Dom, tag: string): number
	local id = self.webview:evaluate(string.format(
		[[{
		const EL = document.createElement('%s');
		document.body.appendChild(EL);
    	ELEMENTS.push(EL)
		}]],
		tag
	))

	assert(typeof(id) == "number", "expected createElement to receive element's id")

	if debug.info(2, "s") ~= DOM_SCRIPT_NAME then
		table.insert(self.__data.elements, {
			tag = tag,
			props = {},
			styles = {},
			events = {},
		})

		assert(id == #self.__data.elements, "expected element id to be the same on both luau and webview")
	end

	return id
end

function Dom.__index.setProp(self: Dom, id: number, k: string, v: string): ()
	self.webview:evaluate(string.format(
		[[
		ELEMENTS[%x]['%s'] = '%s';
		]],
		id - 1,
		k,
		v
	))

	local data = self.__data.elements[id]
	data.props[k] = v
end

function Dom.__index.setStyle(self: Dom, id: number, k: string, v: string): ()
	self.webview:evaluate(string.format(
		[[
		ELEMENTS[%x].style['%s'] = '%s';
		]],
		id - 1,
		k,
		v
	))

	local data = self.__data.elements[id]
	data.styles[k] = v
end

function Dom.__index.onEvent(self: Dom, id: number, event: string, callback: () -> ()): ()
	self.webview:evaluate(string.format(
		[[{
		const ID = %x;
		const EVENT = '%s';
		ELEMENTS[ID][EVENT] = function() {
			window.luneweb.post('event', { elementId: ID + 1, event: EVENT });
		};
		}]],
		id - 1,
		event
	))

	if debug.info(2, "s") ~= DOM_SCRIPT_NAME then
		local elementData = self.__data.elements[id]

		if elementData.events[event] == nil then
			elementData.events[event] = {}
		end

		table.insert(elementData.events[event], callback)
	end
end

-- for return type validation
return function(): DomBuilder
	return Dom
end
