local window1 = WindowBuilder.new("Window1")
local window2 = WindowBuilder.new("Window2")

print(window1.visible, window2.visible)

window2.visible = false

print(window1.visible, window2.visible)

print(window1.title)

window1.title = "Hello, World!"

print(window1.title)
