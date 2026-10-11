// should generate diagnostics

function parameter(arguments) {
    arguments.callee;
    arguments?.caller;
    arguments.callee = value;
}

function local() {
    const arguments = {};
    arguments.caller;
    (arguments).callee;
    arguments.caller++;
}

function nested() {
    const arguments = {};
    return () => arguments.callee;
}

function destructured({ arguments }) {
    arguments.callee;
    ({ value: arguments.caller } = source);
}

function hoisted() {
    arguments.callee;
    var arguments = {};
}

function escapedParameter(\u0061rguments) {
    arguments.callee;
    \u{61}rguments.c\u0061ller = value;
}

function escapedLocal() {
    const arguments = {};
    \u0061rguments.c\u{61}llee;
}
