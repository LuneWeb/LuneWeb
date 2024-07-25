export {};

interface Luneweb {
  channels: { [channel: string]: (message: any) => void };

  postMessage: (value: any) => void;
  listen: (channel: string, callback: (message: any) => void) => void;

  callChannel: (channel: string, message: any) => void;
}

declare global {
  interface Window {
    luneweb: Luneweb;
  }
}
