/* should not generate diagnostics */

// Valid: ignored hook in conditional
function Component1({ a }) {
    if (a == 1) {
        useIgnoredHook();
    }
}

// Valid: ignored hook in nested function
function Component2() {
    function helper() {
        useIgnoredHook();
    }
}

// Valid: ignored hook after early return
function Component3({ a }) {
    if (a != 1) {
        return;
    }
    useIgnoredHook();
}

// Valid: ignored hook at module level
useIgnoredHook();

// Valid: multiple ignored hooks
function Component5() {
    if (condition) {
        useIgnored1();
        useIgnored2();
    }
}

// Valid: ignored hook in nested arrow function
function Component6() {
    const handler = () => {
        useIgnoredHook();
    };
}

// Valid: ignored hook called from non-component
function notAComponent() {
    useIgnoredHook();
}
