function foo(myVar) {
    console.log(myVar);
}

function foo(_unused) {
    console.log('not using the parameter');
}

Object.fromEntries(Object.entries({a: 'A', b: 'B', c: 'C'}).map(([k, v]) => [v, k]));

new Promise((accept, _reject) => {
    window.setTimeout(accept, 1000);
});

data.filter(([_k, v]) => v > 10);
