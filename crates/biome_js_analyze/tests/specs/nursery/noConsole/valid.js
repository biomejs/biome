/* should not generate diagnostics */

var a = 1;

const console = { log: (args) => { /* do nothing */} };
console.log('hello world')
