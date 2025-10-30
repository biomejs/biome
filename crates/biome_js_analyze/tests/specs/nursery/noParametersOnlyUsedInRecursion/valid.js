// Parameter used outside recursion
function factorial(n, acc) {
    console.log(acc);
    if (n === 0) return acc;
    return factorial(n - 1, acc * n);
}

// Parameter not used at all (handled by different rule)
function foo(unused) {
    return 42;
}
