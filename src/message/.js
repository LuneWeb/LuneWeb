const messageListeners = [];
const channels = {};

Object.defineProperty(window, "luneweb", {
  value: Object.freeze({
    shareMessage: function (message) {
      messageListeners.forEach((listener) => {
        listener(message);
      });
    },

    sendMessage: function (channelName, message) {
      const channel = channels[channelName];
      if (!channel) return;

      return JSON.stringify(channel(message));
    },

    listen: function (callback) {
      messageListeners.push(callback);
    },

    createChannel: function (channelName, callback) {
      channels[channelName] = callback;
    },
  }),
});

window.onload = () => {
  window.ipc.postMessage();
};
