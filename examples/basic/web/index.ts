import { message } from "luneweb-api";
import { log } from "./log";

message.listen("print", (value) => {
  log(JSON.stringify(value));

  message.postMessage({
    kind: "print",
    value: "Received message from luau",
  });
});
