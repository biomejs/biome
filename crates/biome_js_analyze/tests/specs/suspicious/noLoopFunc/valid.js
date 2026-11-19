/* should not generate diagnostics */
for (let i = 0; i < 10; i++) {
    handlers.push(() => i);
}

let stable = 100;
for (let i = 0; i < 10; i++) {
    handlers.push(function () {
        return stable;
    });
}

for (var i = 0; i < 10; i++) {
    (() => i)();
}

for (var i = 0; i < 10; i++) {
    const current = i;
    handlers.push(function () {
        return current;
    });
}
