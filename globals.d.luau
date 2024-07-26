type Callback = (message: any) -> ()

declare class Message
  function send(self, channel: string, message: any): ()
  function listen(self, callback: Callback): () 
end

declare class WebView
  function eval(self, src: string): ()
end

declare class Window
  message: Message
  webview: WebView
  visible: boolean
  title: string
  url: string
end

declare WindowBuilder: {
  new: (title: string, url: string) -> Window,
}
