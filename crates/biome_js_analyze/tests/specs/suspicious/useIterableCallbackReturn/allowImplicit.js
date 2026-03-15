// Valid: empty return is accepted with allowImplicit
[1, 2, 3].map((x) => {
    if (x > 2) return x;
    return;
});

// Valid: all paths return values
[1, 2, 3].map((x) => {
    if (x > 2) return x;
    return 0;
});

// Valid: implicit return (arrow expression)
[1, 2, 3].map((x) => x * 2);

// Invalid: some paths return values, others fall through without any return
[1, 2, 3].map((x) => {
    if (x > 2) return x;
    // falls through - likely a bug
});

// Invalid: no returns at all
[1, 2, 3].map((x) => {
    console.log(x);
});

// Valid: only empty returns (all paths covered)
[1, 2, 3].forEach((x) => {
    if (x > 2) return;
    console.log(x);
});
