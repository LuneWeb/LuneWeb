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

declare class AudioDevice
  function appendAudioSource(self, source: AudioSource): ()
  function play(self): ()
  function pause(self): ()
  function clear(self): ()
end

declare class AudioSource
end

declare WindowBuilder: {
  new: (config: WindowConfig) -> Window,
}

declare Audio: {
  device: AudioDevice,
  sourceFromBuffer: (content: buffer) -> AudioSource,
}
