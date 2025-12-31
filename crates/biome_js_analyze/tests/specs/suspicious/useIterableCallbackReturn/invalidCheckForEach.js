/* should generate diagnostics */
// These forEach cases should be invalid when checkForEach is true
[].forEach((a) => {
    return a.fn();
});
[].forEach(function(a) {
    return a.fn();
});
[].forEach((a) => {
    if (a) {
        return a.fn();
    }
});
[].forEach((a) => {
    if (a) {
        return;
    }
    return a.fn();
});
// Implicit return (arrow expression body without return keyword)
[].forEach(a => a.fn());
[].forEach((a) => {
    if (a) {
        throw new Error();
    }
    return a.fn();
});
