/* should not generate diagnostics */

var a = 1;
let b = 1;
const c = 1;
console.log(a, b, c);

// being used inside JSX
let value;
function Button() {}
console.log(<Button att={value}/>);

// object assignment pattern
let d, e; 
({d, e} = {d: 1, e: 2});
