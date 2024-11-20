declare class WindowId

end

declare class Window
    id: WindowId
    title: string
    visible: boolean
end

declare web: {
    createWindow: (title: string?) -> Window,
}
