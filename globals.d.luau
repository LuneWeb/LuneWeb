declare class Message
  function eval(self, src: string): ()
end

declare class Window
  message: Message
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
