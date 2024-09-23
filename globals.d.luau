type Callback = (message: any) -> ()

type WindowConfig = {
  title: string?,
  url: string?,
  dev: boolean?,
}

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

declare class AudioSource
  function play(self): ()
  function pause(self): ()
  function stop(self): ()

  duration: number
  position: number
  volume: number
  speed: number
end

declare WindowBuilder: {
  new: (config: WindowConfig) -> Window,
}

declare AudioBuilder: {
  new: (content: buffer) -> AudioSource,
}
