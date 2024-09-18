local task = require("@lune/task")
local messageId = 0

export type DomElement = {
	querySelector: (DomElement, tag: string) -> DomElement?,
	_dom: Dom,
	_id: number,
}

export type Dom = {
	_message: (channel: string, message: any) -> any,
	body: (self: Dom, window: Window) -> DomElement,
}

local function createElement(dom: Dom, id: number?): DomElement
	local element = {
		_dom = dom,
		_id = id,
	}

	function element.querySelector(self: DomElement, tag: string): DomElement?
		local value = self._dom._message("dom-querySelect", {
			id = self._id,
			tag = tag,
		})

		return createElement(dom, value.elementId)
	end

	return element
end

local function getDom(window: Window): Dom
	local dom = {}
	local messageThreads = {}

	task.spawn(window.message.listen, window.message, function(message)
		if typeof(message) ~= "table" then
			return
		end

		if messageThreads[message.messageId] then
			coroutine.resume(messageThreads[message.messageId], message.value)
			messageThreads[message.messageId] = nil
		end
	end)

	--- @yields
	--- Sends message to webview and waits for webview to return
	function dom._message(channel: string, message: any): any
		messageId += 1

		messageThreads[messageId] = coroutine.running()
		window.message:send(channel, {
			id = messageId,
			value = message,
		})

		return coroutine.yield()
	end

	function dom.body(self: Dom): DomElement
		local value = self._message("dom-get-body", nil)

		return createElement(self, value.elementId)
	end

	return dom
end

return getDom
