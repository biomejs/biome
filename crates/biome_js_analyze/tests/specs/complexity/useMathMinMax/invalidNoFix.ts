/* should generate diagnostics */

// These are cases that we can't fix automatically because we aren't confident that the operands are idempotent. For example, if the operands are function calls, we can't be sure that they will return the same value if we call them again, so we can't safely replace them with a single call to Math.min or Math.max. But, we still should flag them to let the user decide how to deal with it.

foo() > bar() ? foo() : bar();

foo.bar() > 10 ? 10 : foo.bar();

async function asyncEdge() {
    return await foo.bar() > 10 ? 10 : await foo.bar();
}

function groupedTest() {
    return (foo.bar() > 10) ? 10 : foo.bar();
}
