local device = Audio.newAudioDevice()
local source = Audio.newAudioSource(buffer.create(0))

device:appendAudioSource(source)
device:play()
