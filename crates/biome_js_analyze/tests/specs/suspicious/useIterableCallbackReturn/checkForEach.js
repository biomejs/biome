// These should be invalid when checkForEach is true

// Block body with return
[].forEach((a) => {
    return a.fn();
});
[].forEach(function(a) {
    return a.fn();
});

// Concise body (expression body) - returns implicitly
[].forEach(a => a.fn());

// Conditional returns
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
[].forEach((a) => {
    if (a) {
        throw new Error();
    }
    return a.fn();
});
