import { message } from "luneweb-api";
import { log } from "./log";

message.listen((message) => {
  log(`Message shared from Luau: ${JSON.stringify(message)}`);
});

message.createChannel("Channel1", (message) => {
  log(`Channel1 received message from Luau: ${JSON.stringify(message)}`);

  return true;
});
