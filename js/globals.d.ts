export {};

interface Luneweb {
  shareMessage(message?: any): void;
  sendMessage(channel: string, message?: any): void;

  listen(callback: (message?: any) => void): void;
  createChannel(channel: string, callback: (message?: any) => void): void;
}

declare global {
  interface Window {
    luneweb: Luneweb;
  }
}
