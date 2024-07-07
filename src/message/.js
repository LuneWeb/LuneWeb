window._sendMessage = function sendMessage(message) {
  console.log("Received message from luau:", JSON.parse(message));
};
