Object.defineProperty(window, "luneweb", {
  value: Object.freeze({
    channels: {},

    postMessage: function (value) {
      window.ipc.postMessage(JSON.stringify(value));
    },

    listen: function (channel, callback) {
      if (typeof channel !== "string") {
        console.error("Channel name must be string");
      }

      if (typeof callback !== "function") {
        console.error("Channel callback must be function");
      }

      if (window.luneweb.channels[channel]) {
        console.error(`Channel '${channel}' already exists`);
      }

      window.luneweb.channels[channel] = callback;
    },

    callChannel: function (channel, value) {
      if (typeof channel !== "string") {
        console.error("Channel name must be string");
      }

      if (typeof value !== "string") {
        console.error("Value sent to channel must be string");
      }

      if (window.luneweb.channels[channel]) {
        window.luneweb.channels[channel](value);
      } else {
        console.error(`Calling non-existent channel '${channel}'`);
      }
    },
  }),
});

window.onload = function () {
  window.luneweb.postMessage("loaded");
};
