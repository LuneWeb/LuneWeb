/**
 * used internally to send messages from Luau to Javascript
 *
 * ### Example
 * ```ts
 * window.luneweb.shareMessage("Hello from Luau!")
 * ```
 */
export const shareMessage = window.luneweb.shareMessage;

/**
 * listens to messages shared by `window.luneweb.shareMessage`
 *
 * ### Example
 * ```ts
 * window.luneweb.listen(message => {
 *    console.log(`Message shared from Luau: ${message}`)
 * })
 * ```
 */
export const listen = window.luneweb.listen;
