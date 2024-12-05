--!strict
local GUI_JS_IMPL = [[
        let __ELEMENTS = [];

        function e(tag, innerHtml) {
            const el = document.createElement(tag);

            el.innerHTML = innerHtml;

            return __ELEMENTS.push(el) - 1;
        }
        
        function attach(i) {
            const el = __ELEMENTS[i];
            document.body.append(el);
        }
        ]]

local ONLOAD_JS_IMPL = "window.luneweb.post('load', 0)"

local lib = {}

export type WebViewBuilderHandle = {
	__builder: WebViewBuilder,
	__onLoadCallbacks: { () -> () },
	__onLoadThreads: { thread },

	onLoad: (self: WebViewBuilderHandle, callback: () -> ()) -> WebViewBuilderHandle,
	take: (self: WebViewBuilderHandle) -> WebViewBuilder,
}

function lib.builder(builder: WebViewBuilder): WebViewBuilderHandle
	-- only becomes true after the first page reload
	local builderHandle = {
		__builder = builder,
		__onLoadCallbacks = {} :: { () -> () },
		__onLoadThreads = {} :: { thread },
	}

	builderHandle.__builder = builderHandle.__builder
		:withInitScript(GUI_JS_IMPL)
		:withInitScript(ONLOAD_JS_IMPL)
		:withChannel("load", function(body)
			assert(typeof(body) == "number", "Channel expected body to be number")
			assert(body == 0, "Channel expected body to be 0")

			for _, thread in builderHandle.__onLoadThreads do
				task.cancel(thread)
			end

			table.clear(builderHandle.__onLoadThreads)

			for _, callback in builderHandle.__onLoadCallbacks do
				table.insert(builderHandle.__onLoadThreads, task.spawn(callback))
			end
		end)

	function builderHandle.onLoad(self: WebViewBuilderHandle, callback: () -> ())
		table.insert(self.__onLoadCallbacks, callback)

		return self
	end

	function builderHandle.take(self: WebViewBuilderHandle)
		return self.__builder
	end

	return builderHandle
end

return lib
