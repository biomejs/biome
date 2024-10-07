function foo(myVar) {
    console.log('foo');
}

const data = [[1, 1], [2, 4], [3, 9], [4, 16], [5, 25]];
data.filter(([k, v]) => v > 10);

[{ a: 1, b: 2, c: 3 }].map(({a, b, c}) => a + c);

new Promise((accept, reject) => {
    window.setTimeout(accept, 1000);
});

// parameter a is not used
{(function (a) { })}
{(function ({a}) { })}
{(function ([a]) { })}
(function (a, b) {
    console.log(b);
})

// parameter b is not used
(function (a, b) {
    console.log(a);
})

function withSpread({ a: { x }, ...rest }) {
    return rest;
}

function withArraySpread([a, ...rest]) {
    return rest;
}
