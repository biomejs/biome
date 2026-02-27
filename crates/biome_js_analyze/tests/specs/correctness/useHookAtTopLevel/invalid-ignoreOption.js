/* should generate diagnostics */

// Valid: ignored hook after early return
function Component3({ a }) {
    if (a != 1) {
        return;
    }
    useIgnoredHook();
}

// Invalid: non-ignored hook in conditional (should still fail)
function Component4({ a }) {
    if (a == 1) {
        useRealHook();
    }
}

// Invalid: non-ignored hook called from non-component (should fail)
function notAComponent2() {
    useRealHook();
}
