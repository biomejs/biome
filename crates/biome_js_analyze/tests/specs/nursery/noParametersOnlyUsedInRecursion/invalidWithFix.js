// Simple case with fix
function factorial(n, acc) {
    if (n === 0) return 1;
    return factorial(n - 1, acc);
}

// Multiple parameters with fix
function fn(a, b, c) {
    if (a === 0) return 0;
    return fn(a - 1, b, c);
}

// Class method with fix
class Counter {
    count(n, acc) {
        if (n === 0) return 0;
        return this.count(n - 1, acc);
    }
}
