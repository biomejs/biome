/* should not generate diagnostics */

// return with a value
function a() {
    return 5;
}

// return with code following the containing if
function b() {
    if (condition) {
        return;
    }
    bar();
}

// return inside for loop
function c() {
    for (let i = 0; i < 10; i++) {
        return;
    }
}

// return inside while loop
function d() {
    while (true) {
        return;
    }
}

// return inside do-while loop
function e() {
    do {
        return;
    } while (true);
}

// return inside for-in loop
function f() {
    for (const key in obj) {
        return;
    }
}

// return inside for-of loop
function g() {
    for (const x of xs) {
        return;
    }
}

// return inside switch case
function h() {
    switch (x) {
        case 1:
            return;
    }
}

// return inside finally block
function i() {
    try {
        doSomething();
    } finally {
        return;
    }
}

// return undefined (has an argument)
function j() {
    return undefined;
}

// empty function
function k() {}

// return in nested function doesn't affect outer function
function l() {
    function inner() {
        if (condition) {
            return;
        }
        doSomething();
    }
    doSomething();
}

// return with code after if/else
function m() {
    if (condition) {
        return;
    } else {
        return;
    }
    // unreachable but syntactically present
    bar();
}

// return not in last position of statement list
function n() {
    if (condition) {
        return;
        doSomething();
    }
}
