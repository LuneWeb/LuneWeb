Object.defineProperty(window, "luneweb", {
  value: Object.freeze({
    post: function (channel, value) {
      window.ipc.postMessage(
        JSON.stringify({
          channel,
          value: JSON.stringify(value) | null,
        })
      );
    },
  }),
});
