Object.defineProperty(window, "luneweb", {
  value: Object.freeze({
    shareMessage: function (message) {
      console.log("Received message from luau:", message);
    },
  }),
});
