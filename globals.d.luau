declare class Message
  function send(self, channel: string, message: string): ()
  function listen(self): (callback: (message: string) -> ()) -> () 
end

declare class WebView
  function eval(self, src: string): ()
end

declare class Window
  message: Message
  webview: WebView
  visible: boolean
  title: string
end

declare WindowBuilder: {
  new: (title: string) -> Window,
}

declare task: {
  cancel: (thread: thread) -> (),
  defer: <T...>(functionOrThread: thread | ((T...) -> ...any), T...) -> thread,
  delay: <T...>(
    duration: number,
    functionOrThread: thread | ((T...) -> ...any),
    T...
  ) -> thread,
  spawn: <T...>(functionOrThread: thread | ((T...) -> ...any), T...) -> thread,
  wait: (duration: number?) -> number,
}

export type EncodeDecodeFormat = "json" | "yaml" | "toml"
export type CompressDecompressFormat = "brotli" | "gzip" | "lz4" | "zlib"
export type HashAlgorithm =
  | "md5"
  | "sha1"
  | "sha224"
  | "sha256"
  | "sha384"
  | "sha512"
  | "sha3-224"
  | "sha3-256"
  | "sha3-384"
  | "sha3-512"
  | "blake3"

declare serde: {
  encode: (format: EncodeDecodeFormat, value: any, pretty: boolean?) -> string,
  decode: (format: EncodeDecodeFormat, encoded: buffer | string) -> any,
  compress: (format: CompressDecompressFormat, s: buffer | string, level: number?) -> string,
  decompress: (format: CompressDecompressFormat, s: buffer | string) -> string,
  hash: (algorithm: HashAlgorithm, message: string | buffer) -> string,
  hmac: (
	  algorithm: HashAlgorithm,
	  message: string | buffer,
	  secret: string | buffer
  ) -> string,
}
