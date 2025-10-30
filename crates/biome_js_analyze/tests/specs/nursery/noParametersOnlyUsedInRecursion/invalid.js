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
