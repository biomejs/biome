/* should not generate diagnostics */

// void expressions should not trigger diagnostics even with checkForEach: true
// because void returns undefined, which is "no return value"
[].forEach(a => void a.fn());
[].forEach(() => void null);
