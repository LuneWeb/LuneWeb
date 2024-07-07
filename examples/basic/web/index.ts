import { message } from "lunweb";

message.listen((message) => {
  console.log(`Message shared from Luau: ${message}`);
});
