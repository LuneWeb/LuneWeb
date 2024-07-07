export {};

interface Luneweb {
  shareMessage(message?: any): void;

  listen(callback: (message?: any) => void): void;
}

declare global {
  interface Window {
    luneweb: Luneweb;
  }
}
