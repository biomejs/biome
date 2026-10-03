// should generate diagnostics

function outer(arguments) {
    return function inner() {
        return arguments.callee;
    };
}

function redeclared() {
    var arguments;
    return arguments.caller;
}
