/* should not generate diagnostics */
let foo;
const bar = "null";
const nil = undefined;
if (foo === null) {}
if (null === foo) {}
if (foo !== null) {}
if (null !== foo) {}
if ((null) === foo) {}
if (foo == undefined) {}
Object.create(null);
Object.create(null, { foo: { value: 1 } });
Object.create((null));
(Object.create)(null);
Object.create();
Object.create(bar);
Object.create("null");
useRef(null);
React.useRef(null);
parentNode.insertBefore(newNode, null);
parentNode?.insertBefore(newNode, null);
document.body.insertBefore(newNode, null);
foo = Object.create(null);
