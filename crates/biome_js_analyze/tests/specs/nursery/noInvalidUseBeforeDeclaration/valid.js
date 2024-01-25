
g();
function g() { f(); }
function f() {}

new C(); class C {}

export { X }; const X = 1;

let a; console.log(a);

function h() { Y; }; const Y = 0;

const aFunction = (a = '111', b = a) => {
    console.info(a,b);
}

const aFunction = (b = a) => {
    console.info(a,b);
}

const aFunction = ({a = '111', b = a}) => {
    console.info(a,b);
}

const aFunction = ({b = a}) => {
    console.info(a,b);
}
