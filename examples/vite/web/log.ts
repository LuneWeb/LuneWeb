const template = document.querySelector(".log") as Element;
const output = document.querySelector(".output") as Element;
const started = Date.now() / 1000;

export function log(message: any) {
  const clone = template.cloneNode(true) as Element;
  const now = Date.now() / 1000 - started;

  (clone.querySelector(".time") as Element).innerHTML = `${/\d\.\d/.exec(
    String(now)
  )}s`;
  (clone.querySelector(".msg") as Element).innerHTML = message;

  output.appendChild(clone);
}

template.remove();
