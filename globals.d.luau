declare class WindowId

end

declare class Window
    id: WindowId
    title: string
    visible: boolean
end

declare class WebViewBuilder
    function withInitScript(self, script: string): WebViewBuilder
    function withChannel(self, name: string, callback: (value: unknown) -> ()): WebViewBuilder
    function build(self, parent: Window): WebView
end

declare class WebView
    id: string
    url: string

    function loadHtml(self, html: string): ()
    function getCookies(self): { string }
    function openDebugger(self): ()
    function evaluate(self, script: string): unknown
end

declare web: {
    createWindow: (title: string?) -> Window,
    createWebView: () -> WebViewBuilder,
}

declare task: {
    wait: (secs: number?) -> (),
    spawn: <T...>(f: (T...) -> (), T...) -> (),
    defer: <T...>(f: (T...) -> (), T...) -> (),
    cancel: (thread: thread) -> (),
}
