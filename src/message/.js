const messageListeners = [];

Object.defineProperty(window, "luneweb", {
  value: Object.freeze({
    shareMessage: function (message) {
      messageListeners.forEach((listener) => {
        listener(message);
      });
    },

    listen: function (callback) {
      messageListeners.push(callback);
    },
  }),
});
