/* should generate diagnostics */

// return at end of function body
function a() {
    return;
}

// return after statements at end
function b() {
    doSomething();
    return;
}

// return at end of if-block (no code after if)
function c() {
    if (condition) {
        bar();
        return;
    }
}

// return at end of else-block (no code after if/else)
function d() {
    if (condition) {
        foo();
    } else {
        bar();
        return;
    }
}

// both branches with return, tail-positioned
function e() {
    if (condition) {
        return;
    } else {
        return;
    }
}

// return in nested if chains, all tail-positioned
function f() {
    if (a) {
        if (b) {
            return;
        }
    }
}

// return at end of try block (tail-positioned, no finally)
function g() {
    try {
        doSomething();
        return;
    } catch (e) {
        handleError();
    }
}

// return at end of catch block (tail-positioned, no finally)
function h() {
    try {
        doSomething();
    } catch (e) {
        handleError();
        return;
    }
}

// arrow function
const i = () => {
    return;
}

// return in labeled statement (tail-positioned)
function j() {
    label: {
        return;
    }
}

// return in try body of try-finally (tail-positioned)
function k() {
    try {
        return;
    } finally {
        cleanup();
    }
}

// single-statement if with return (tail-positioned)
function l() {
    if (foo) { return; }
}
