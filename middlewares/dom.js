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

domListen("dom-getBody", (message) => {
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

domListen("dom-setInner", (message) => {
  const element = elements[message.id];
  element.innerHTML = message.html;
});

domListen("dom-setStyle", (message) => {
  const element = elements[message.id];
  element.style[message.style] = message.value;
});

domListen("dom-getStyle", (message) => {
  const element = elements[message.id];
  return element.style[message.style];
});

domListen("dom-createListener", (message) => {
  const element = elements[message.id];
  const listenerId = message.listenerId;

  element.addEventListener(message.event, () => {
    window.luneweb.postMessage({
      listenerId,
    });
  });
});
