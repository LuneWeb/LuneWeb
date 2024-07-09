declare class App
  function onLoad(self, message: any): ()

  function shareMessage(self, message: any): ()
  function sendMessage(self, channel: string, message: any): any

  function exit(self, exitCode: number): ()
  function run(self): ()
end

declare app: App
