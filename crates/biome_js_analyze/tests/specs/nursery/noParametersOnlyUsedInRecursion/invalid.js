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
