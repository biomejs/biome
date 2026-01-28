/* should not generate diagnostics */

globalThis;

globalThis.foo;

globalThis[foo];

const {foo} = globalThis;

globalThis.navigator;

globalThis.location;

window.innerWidth;
window.innerHeight;

self.postMessage('Hello');

globalThis.addEventListener('click', () => {});
globalThis.addEventListener('resize', () => {});
globalThis.addEventListener('load', () => {});
globalThis.addEventListener('unload', () => {});
