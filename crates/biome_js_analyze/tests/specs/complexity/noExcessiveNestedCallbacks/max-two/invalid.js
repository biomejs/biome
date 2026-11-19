/* should generate diagnostics */
foo1(function () {
    foo2(function () {
        foo3(function () {
            // Too deeply nested with max: 2.
        });
    });
});

foo1(() => {
    foo2(() => {
        foo3(() => {
            // Too deeply nested with max: 2.
        });
    });
});
