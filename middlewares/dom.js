const cache = {};
const elements = {};
let reservedId = 0;

function domElementToId(element) {
  if (cache[element]) {
    return cache[element];
  }

  cache[element] = reservedId++;
  elements[reservedId] = element;

  return domElementToId(element);
}

function domListen(channel, callback) {
  window.luneweb.listen(channel, (message) => {
    let result;

    try {
      result = callback(message.value);
    } catch (e) {
      console.warn(e);
      result = null;
    }

    window.luneweb.postMessage({
      messageId: message.id,
      value: result,
    });
  });
}

domListen("dom-get-body", (message) => {
  const elementId = domElementToId(document.body);

  return {
    elementId,
  };
});

domListen("dom-querySelect", (message) => {
  const elementId = domElementToId(
    elements[message.id].querySelector(message.tag)
  );

  return {
    elementId,
  };
});
