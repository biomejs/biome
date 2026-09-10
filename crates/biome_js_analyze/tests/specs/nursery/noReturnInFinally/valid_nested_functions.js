/* should not generate diagnostics */

p.finally(() => {
    if (condition) {
        function nested() {
            return 1;
        }
        const expression = function () {
            return 1;
        };
        const arrow = () => {
            if (condition) return 1;
        };
        const object = {
            method() { return 1; },
            get value() { return 1; },
            set value(value) { return; },
        };
        class Nested {
            constructor() { return {}; }
            method() { return 1; }
            get value() { return 1; }
            set value(value) { return; }
            field = () => { return 1; };
        }
    }
});

p.finally(function () {
    const nested = () => {
        return 1;
    };
});

p.finally(() => {
    (function () {
        return 1;
    })();
    (() => {
        return 1;
    })();
    const a = () => "bar";
});

p.finally(() => () => {
    return 1;
});

p.finally(() => {
    if (condition) {
        cleanup();
    }
});
