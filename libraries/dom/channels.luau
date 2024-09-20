export type Channel =
	"dom-getBody"
	| "dom-querySelect"
	| "dom-createElement"
	| "dom-setProp"
	| "dom-getProp"
	| "dom-setStyle"
	| "dom-getStyle"
	| "dom-createListener"

local messageId = 0
local messageThreads = {}
local api = {}

--[[
Will start listening to messages returned by webview
]]
function api.register(window: Window)
	window.message:listen(function(message)
		if typeof(message) ~= "table" then
			return
		end

		if message.messageId and messageThreads[message.messageId] then
			coroutine.resume(messageThreads[message.messageId], message.value)
			messageThreads[message.messageId] = nil
		end
	end)
end

--[[
Sends message to webview and waits for webview to return

@yields
]]
function api.message(window: Window, channel: Channel, message: any): any
	messageId += 1
	messageThreads[messageId] = coroutine.running()

	window.message:send(channel, {
		id = messageId,
		value = message,
	})

	return coroutine.yield()
end

return api
