/**
 * @ignore
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

/**
 * @ignore
 * used internally to send messages from Luau to a Javascript channel
 *
 * ### Example
 * ```ts
 * window.luneweb.sendMessage("Channel1", "Hello from Luau!")
 * ```
 */
export const sendMessage = window.luneweb.sendMessage;

/**
 * crates a Channel which will receive messages from `window.luneweb.sendMessage`
 *
 * you can return values back to the sender (should be Luau if it was sent internally) aswell
 *
 * ### Example
 * ```ts
 * window.luneweb.createChannel("Channel1", message => {
 *    console.log(`Channel1 received message from Luau: ${message}`)
 *
 *    // you can return anything you want back to the sender
 *    return true
 * })
 * ```
 */
export const createChannel = window.luneweb.createChannel;
