/**
 * Opens a channel and creates a listener for it
 *
 * ### Example
 * ```ts
 * listen("ChannelName", (message) => {
 *    console.log(`Message shared from Luau: ${message}`)
 * })
 * ```
 */
export const listen = window.luneweb.listen;

/**
 * Sends a message to the Rust backend
 *
 * ### Note
 *
 * Provided value will be stringified into JSON before getting sent to the backend
 *
 * ### Example
 * ```ts
 * postMessage({ a: true, b: false, c: 1000 })
 * ```
 */
export const postMessage = window.luneweb.postMessage;
