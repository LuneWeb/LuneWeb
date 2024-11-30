declare class WindowId

end

declare class Window
    id: WindowId
    title: string
    visible: boolean
end

declare class WebView
    id: string
    url: string

    function loadHtml(self, html: string): ()
    function getCookies(self): { string }
    function openDebugger(self): ()
end

declare web: {
    createWindow: (title: string?) -> Window,
    createWebView: (parent: Window) -> WebView,
}
