// should generate diagnostics

// some paths return values, others fall through without any return
[1, 2, 3].map((x) => {
    if (x > 2) return x;
    // falls through - likely a bug
});

// no returns at all
[1, 2, 3].map((x) => {
    console.log(x);
});
