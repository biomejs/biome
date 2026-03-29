// should generate diagnostics
[1, 2, 3].map((item) => {
    if (item % 2 === 0) {
        return;
    }
});

[1, 2, 3].filter((item) => {
    if (item > 1) {
        return true;
    }
});

[1, 2, 3].map(() => void null);
