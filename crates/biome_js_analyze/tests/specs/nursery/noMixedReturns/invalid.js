/* should generate diagnostics */
function foo1() { if (true) return true; else return; }
var foo2 = () => { if (true) return true; else return; }
function foo3() { if (true) return; else return false; }
f(function() { if (true) return true; else return; })
f(function() { if (true) return; else return false; })
f(a => { if (true) return; else return false; })
function foo4() { if (a) return true; }
function _foo5() { if (a) return true; }
f(function foo6() { if (a) return true; });
f(function() { if (a) return true; });
f(() => { if (a) return true; });
var obj1 = {foo() { if (a) return true; }};
class A1 {foo() { if (a) return true; }};
class A2 { CapitalizedFunction() { if (a) return true; } }
({ constructor() { if (a) return true; } });

// Every mismatched `return` is reported.
function foo7(a) {
    if (a === 1) return 1;
    if (a === 2) return;
    if (a === 3) return;
    return 4;
}

// A class constructor is only exempt from the check at the end of the body.
class A3 { constructor() { if (a) return foo; return; } }
function Foo8() { if (a) return new Foo8(); return; }

// Nested functions are checked separately.
function foo9() {
    function bar() { return 1; }
    if (a) return 1;
}

// Unreachable `return` statements are compared too.
function foo10() { return 1; return; }

async function foo11() { if (a) return 1; }
function* foo12() { if (a) return 1; yield 2; }
var obj2 = { get foo() { if (a) return 1; } };

function foo13() {
    try {
        return 1;
    } catch {
        log();
    }
}

function foo14(x) {
    switch (x) {
        case 1: return 1;
        case 2: break;
        default: return 3;
    }
}

function foo15() {
    for (const item of items) {
        if (item) return item;
    }
}

export default function () { if (a) return 1; }

// Only the name written after `function` exempts a function.
const Bar1 = function () { if (a) return 1; };
const Bar2 = () => { if (a) return 1; };
