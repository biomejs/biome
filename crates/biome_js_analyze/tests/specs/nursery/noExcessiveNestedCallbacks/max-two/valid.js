/* should not generate diagnostics */
foo1(function () {
    foo2(function () {});
});

foo1(() => {
    foo2(() => {});
});

foo1(handleFoo1);

function handleFoo1() {
    foo2(handleFoo2);
}

function handleFoo2() {
    foo3();
}
