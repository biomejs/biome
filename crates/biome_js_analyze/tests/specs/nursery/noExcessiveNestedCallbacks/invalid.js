/* should generate diagnostics */
foo1(function () {
    foo2(function () {
        foo3(function () {
            foo4(function () {
                foo5(function () {
                    foo6(function () {
                        // Too deeply nested.
                    });
                });
            });
        });
    });
});

foo1(() => {
    foo2(() => {
        foo3(() => {
            foo4(() => {
                foo5(() => {
                    foo6(() => {
                        // Too deeply nested.
                    });
                });
            });
        });
    });
});

bar1(() => {
    bar2(() => {
        bar3(() => {
            bar4(() => {
                bar5(() => {
                    bar6(() => {
                        bar7(() => {
                            // Too deeply nested.
                            // only bar6 should be reported, not bar7.
                        });
                    });
                });
            });
        });
    });
});

baz1(function () {
    baz2(() => {
        baz3(function () {
            baz4(() => {
                baz5(function () {
                    baz6(() => {
                        // Too deeply nested with mixed callback forms.
                    });
                });
            });
        });
    });
});

qux1(() => {
    qux2(() => {
        qux3(() => {
            qux4(() => {
                qux5(() => {
                    qux6(() => {
                        // First too-deep sibling callback.
                    });

                    qux7(function () {
                        // Second too-deep sibling callback at the same level.
                    });
                });
            });
        });
    });
});

declaredWrapper1(function () {
    function declaredNestedCallbacks() {
        declaredWrapper2(() => {
            declaredWrapper3(function () {
                declaredWrapper4(() => {
                    declaredWrapper5(function () {
                        declaredWrapper6(() => {
                            // Too deeply nested inside a function declaration.
                        });
                    });
                });
            });
        });
    }

    declaredNestedCallbacks();
});

declaredSibling1(() => {
    function declaredSiblingCallbacks() {
        declaredSibling2(() => {
            declaredSibling3(() => {
                declaredSibling4(() => {
                    declaredSibling5(() => {
                        declaredSibling6(() => {
                            // First too-deep sibling inside a function declaration.
                        });

                        declaredSibling7(function () {
                            // Second too-deep sibling inside a function declaration.
                        });
                    });
                });
            });
        });
    }

    declaredSiblingCallbacks();
});
