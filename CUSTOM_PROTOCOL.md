## Custom protocol

This section will give you some advice on using custom protocols

- In the response message make sure to provide these two headers: Access-Control-Allow-Origin and Access-Control-Allow-Headers
```luau
return {
    body = "...",
    headers = {
        ["Access-Control-Allow-Origin"] = "*",
        ["Access-Control-Allow-Headers"] = "*",
    }
}
```

- Don't use `webview_builder:with_url` or `webview:load_url` on custom protocols, they work fine on Windows but will crash your app on Linux, this something that `wry` has to solve and is not possible to be fixed by LuneWeb
```luau
webviewBuilder:with_url("app://localhost/") -- this will crash the webview and have unexpected behavior
```

## Example

So let's say we want an html file that also uses a css file, this is how we should go about making our custom protocol
```luau
local Fs = require("@lune/fs")

local webviewBuilder = require("@luneweb/webview").new()

webviewBuilder:with_custom_protocol("app", function(req)
	if req.path == "/style.css" then
		return {
			body = Fs.readFile("style.css"),
			headers = {
				["Access-Control-Allow-Origin"] = "*",
				["Access-Control-Allow-Headers"] = "*",
			},
		}
	end

	return {
		body = "",
	}
end)

webviewBuilder:with_html(Fs.readFile("index.html"))
```

![image](https://github.com/LuneWeb/LuneWeb-rs/assets/127131961/e0087d9d-e9f4-4404-b79a-5842b0c21a0e)
