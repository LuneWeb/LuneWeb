declare class WindowId

end

declare class Window
    id: WindowId
    title: string
    visible: boolean
end

declare class WebView
    id: string

    function loadHtml(self, html: string): ()
    function loadUrl(self, url: string): ()
end

declare web: {
    createWindow: (title: string?) -> Window,
    createWebView: (parent: Window) -> WebView,
}
