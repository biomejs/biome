/* should generate diagnostics */
foo();
var afterExpression = 1;

let okayBeforeExpression = 1;
foo();
var afterLetAndExpression = 2;

if (condition) {
    var insideBlock = 1;
}

for (var loopValue = 0; loopValue < 1; loopValue++) {}

function invalidFunction() {
    "use strict";
    doSomething();
    var afterStatement = 1;
}

class InvalidExample {
    static {
        doSomething();
        var afterStaticStatement = 1;
    }
}

export function helper() {}
export var exportedAfterStatement = 1;
