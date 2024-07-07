import { message } from "lunweb";

message.listen((message) => {
  console.log(`Message shared from Luau: ${message}`);
});

message.createChannel("Channel1", (message) => {
  console.log(`Channel1 received message from Luau: ${message}`);

  return true;
});
