/* should generate diagnostics */

class Promise<T> {}
declare const value: Promise<void>;

async function test(): globalThis.Promise<void> {
    await value;
}
