Object.defineProperty(window, "luneweb", {
  value: Object.freeze({
    post: function (channel, value = null) {
      window.ipc.postMessage(
        JSON.stringify({
          channel,
          value,
        })
      );
    },
  }),
});
