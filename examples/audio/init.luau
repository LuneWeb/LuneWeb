assert(AudioBuilder, "This machine doesn't have an audio device")

local fs = require("@lune/fs")
local task = require("@lune/task")
local audio = AudioBuilder.fromBuffer(buffer.fromstring(fs.readFile("examples/audio/808.wav")))

print(`playing 808\
- duration: {audio.duration}\
- speed: {audio.speed}\
- volume: {audio.volume}`)

while true do
	audio:play()
	task.wait(audio.duration)
end
