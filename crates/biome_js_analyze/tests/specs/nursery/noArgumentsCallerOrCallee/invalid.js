// should generate diagnostics

const callee = arguments.callee;
const caller = arguments.caller;

function factorial(n) {
    return n <= 1 ? 1 : n * arguments.callee(n - 1);
}

function getCaller() {
    return arguments.caller;
}

function outer() {
    return () => arguments.callee;
}

arguments?.callee;
arguments?.caller;
(arguments).callee;
((arguments)).caller;
arguments.callee.call(null);
arguments.caller.name;
arguments.callee = value;
arguments.caller += value;
arguments.callee++;
--arguments.caller;
({ value: arguments.callee } = source);
[arguments.caller] = source;
delete arguments.callee;
