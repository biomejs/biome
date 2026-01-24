// Should trigger the rule
window.location.href;
global.setTimeout(() => {}, 100);
self.postMessage('hello');

// Should not trigger the rule (local variables)
const window = {};
const global = {};
const self = {};
window.location.href;
global.setTimeout(() => {}, 100);
self.postMessage('hello');