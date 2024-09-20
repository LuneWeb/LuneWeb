const elements = [];

function domElementToId(element) {
  return elements.push(element) - 1;
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

domListen("dom-getBody", (_) => {
  const elementId = domElementToId(document.body);

  return {
    elementId,
  };
});

domListen("dom-createElement", (message) => {
  const elementId = domElementToId(document.createElement(message.tag));

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

domListen("dom-appendChild", (message) => {
  const element = elements[message.id];
  const child = elements[message.childId];
  element.appendChild(child);
});

domListen("dom-setProp", (message) => {
  const element = elements[message.id];
  element[message.k] = message.v;
});

domListen("dom-getProp", (message) => {
  const element = elements[message.id];
  return element[message.k];
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
