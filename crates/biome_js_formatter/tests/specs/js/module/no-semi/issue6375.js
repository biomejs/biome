const Transport = {}
const newRequest = {}

/**
 * SAFETY: monkey patching and getting around the provided type definitions.
 */
;(Transport.prototype as any).request = newRequest
