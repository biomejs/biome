const a = 3;
function b() {
    const a = 10;
}

const c = function () {
    const a = 10;
}

function d(a) {
    a = 10;
}
d(a);

if (true) {
    const a = 5;
}

var foo = function(foo) {};

var bar = function() { function bar() {} };
