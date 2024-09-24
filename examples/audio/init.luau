assert(AudioBuilder, "This machine doesn't have an audio device")

local fs = require("@lune/fs") :: any
local task = require("@lune/task") :: any
local audio = AudioBuilder.fromBuffer(
	buffer.fromstring(fs.readFile("F:\\DrumKits\\! Cxdy - go to drums\\#808(s)\\bomberman 808 @thecxdy.wav"))
)

while true do
	audio:play()
	task.wait(audio.duration)
end
