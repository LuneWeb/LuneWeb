declare class Task
  cancel: (thread: thread): (),
  defer: <T...>(functionOrThread: thread | (T...) -> ...any, ...: T...): thread,
  delay: <T...>(
    duration: number,
    functionOrThread: thread | (T...) -> ...any,
    ...: T...
  ): thread,
  spawn: <T...>(functionOrThread: thread | (T...) -> ...any, ...: T...): thread,
  wait: (duration: number?): number,
end

declare class Window
  visible: boolean
  title: string
end

declare WindowBuilder: {
  new: (title: string) -> Window,
}

declare task: Task
