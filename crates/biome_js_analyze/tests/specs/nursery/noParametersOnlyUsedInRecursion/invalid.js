// Simple case: parameter only used in recursion
function factorial(n, acc) {
    if (n === 0) return 1;
    return factorial(n - 1, acc);
}

// Multiple parameters only in recursion
function fn(a, b, c) {
    if (a === 0) return 0;
    return fn(a - 1, b, c);
}

// Arrow function with recursion
const countdown = (n, acc) => {
    if (n === 0) return 0;
    return countdown(n - 1, acc);
};

// Method in class
class Counter {
    count(n, acc) {
        if (n === 0) return 0;
        return this.count(n - 1, acc);
    }
}

// Parameter with arithmetic in recursion
function countdown(n, step) {
    if (n === 0) return 0;
    return countdown(n - step, step);
}

// Multiple operations
function compute(a, b) {
    if (a === 0) return 0;
    return compute(a - 1, b * 2 + 1);
}

// Unary operations
function negate(n, flag) {
    if (n === 0) return 0;
    return negate(n - 1, !flag);
}

// Object method
const obj = {
    count(n, step) {
        if (n === 0) return 0;
        return this.count(n - step, step);
    }
};

// Assignment expression with recursive arrow function
foo = (n, acc) => {
    if (n === 0) return 0;
    return foo(n - 1, acc);
};

// Separate declaration and assignment with arrow function
let bar;
bar = (x, unused) => {
    if (x === 0) return 0;
    return bar(x - 1, unused);
};

// Logical AND operator
function fnAnd(n, acc) {
    if (n === 0) return 0;
    return fnAnd(n - 1, acc && true);
}

// Logical OR operator
function fnOr(n, acc) {
    if (n === 0) return 0;
    return fnOr(n - 1, acc || 0);
}

// Nullish coalescing operator
function fnNullish(n, acc) {
    if (n === 0) return 0;
    return fnNullish(n - 1, acc ?? 0);
}

// Nested logical operators
function fnNested(n, acc) {
    if (n === 0) return 0;
    return fnNested(n - 1, (acc || 0) && true);
}

// Conditional expression with parameter in consequent
function fnCondConsequent(n, acc) {
    if (n === 0) return 0;
    return fnCondConsequent(n - 1, n > 5 ? acc : 0);
}

// Conditional expression with parameter in alternate
function fnCondAlternate(n, acc) {
    if (n === 0) return 0;
    return fnCondAlternate(n - 1, n > 5 ? 0 : acc);
}

// Conditional expression with parameter in test
function fnCondTest(n, flag) {
    if (n === 0) return 0;
    return fnCondTest(n - 1, flag ? true : false);
}

// Nested conditional expressions
function fnCondNested(n, acc) {
    if (n === 0) return 0;
    return fnCondNested(n - 1, n > 5 ? (n > 10 ? acc : 0) : 0);
}

// Optional chaining in class method
class CounterOptional {
    count(n, acc) {
        if (n === 0) return 0;
        return this?.count(n - 1, acc);
    }
}

// Optional chaining in object method
const objOptional = {
    count(n, step) {
        if (n === 0) return 0;
        return this?.count(n - step, step);
    }
};

// Computed member with string literal
class CounterComputed {
    count(n, acc) {
        if (n === 0) return 0;
        return this["count"](n - 1, acc);
    }
}

// Optional chaining with computed member
class CounterOptionalComputed {
    count(n, acc) {
        if (n === 0) return 0;
        return this?.["count"](n - 1, acc);
    }
}
