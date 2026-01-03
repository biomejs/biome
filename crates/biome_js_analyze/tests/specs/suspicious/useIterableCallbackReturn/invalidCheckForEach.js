// These should trigger errors when checkForEach: true

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
[].forEach((a) => {
    if (a) {
        throw new Error();
    }
    return a.fn();
});
