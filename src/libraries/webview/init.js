{
    let luneweb = {}

    // ? this is a temporary workaround for messages to not get replaced
    // TODO: ^^^ put messages into an ordered list instead of a channel
    let sentMessage = false
    const sendMessageCooldown = 36;

    luneweb.postMessage = function (channel, data = null) {
        let inner = () => {
            sentMessage = true
            window.ipc.postMessage(JSON.stringify({
                channel,
                data,
            }))

            setTimeout(() => {
                sentMessage = false
            }, sendMessageCooldown)
        }

        if (sentMessage) {
            let loop = setInterval(() => {
                if (!sentMessage) {
                    clearInterval(loop)
                    inner()
                }
            }, sendMessageCooldown)
        } else {
            inner()
        }
    }

    luneweb.postInternalMessage = function (action, data = null) {
        window.ipc.postMessage(JSON.stringify({
            action,
            data,
            __internal: true,
        }))
    }

    window.lune = luneweb
    window.luneweb = luneweb
}

console.log = (...data) => {
    let str = ""
    for (index in data) {
        if (str.length > 0) {
            str += " "
        }

        str += data[index]
    }

    window.luneweb.postInternalMessage("print", str);
}
