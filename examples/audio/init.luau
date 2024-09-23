assert(AudioBuilder, "This machine doesn't have an audio device")

local audio = AudioBuilder.fromBuffer(buffer.create(0))

print(audio.duration)

audio:play()
