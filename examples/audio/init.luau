local source = Audio.sourceFromBuffer(buffer.create(0))

Audio.device:appendAudioSource(source)
Audio.device:play()
