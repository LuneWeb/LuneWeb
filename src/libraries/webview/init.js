{
    let luneweb = {}

    luneweb.postMessage = function (channel, data = null) {
        window.ipc.postMessage(JSON.stringify({
            channel,
            data,
        }))
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
