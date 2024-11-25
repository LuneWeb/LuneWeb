declare class WindowId

end

declare class Window
    id: WindowId
    title: string
    visible: boolean
end

declare class WebView
    
end

declare web: {
    createWindow: (title: string?) -> Window,
    createWebView: (parent: Window) -> WebView,
}
