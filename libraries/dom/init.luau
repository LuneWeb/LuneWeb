local task = require("@lune/task")
local messageId = 0

export type DomElement = {
	querySelector: (DomElement, tag: string) -> DomElement?,
	dom: Dom,
	id: number,
}

export type Dom = {
	message: (channel: string, message: any) -> any,
	body: (Window) -> DomElement,
}

local function createElement(dom: Dom, id: number?): DomElement
	local element = {
		dom = dom,
		id = id,
	}

	function element.querySelector(self: DomElement, tag: string): DomElement?
		local value = self.dom.message("dom-querySelect", {
			id = self.id,
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
	--- Sends message to webview and waits for webview to return value
	function dom.message(channel: string, message: any): any
		messageId += 1

		messageThreads[messageId] = coroutine.running()
		window.message:send(channel, {
			id = messageId,
			value = message,
		})

		return coroutine.yield()
	end

	function dom.body(): DomElement
		local value = dom.message("dom-get-body", nil)

		return createElement(dom, value.elementId)
	end

	return dom
end

return getDom
