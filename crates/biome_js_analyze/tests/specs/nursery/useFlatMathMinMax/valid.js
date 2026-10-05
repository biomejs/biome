/* should not generate diagnostics */

Math.max(a, b, c);
Math.min(a, b, c);
Math.max(Math.min(a, b), c);
Math.min(Math.max(a, b), c);
Math.max(-Math.max(a, b), c);
Math.max(foo(Math.max(a, b)), c);
max(max(a, b), c);
Math.max.apply(Math, [Math.max(a, b), c]);
Math["max"](Math["max"](a, b), c);
Math.max?.(Math.max(a, b), c);
Math?.max(Math.max(a, b), c);
Math.max(Math.max?.(a, b), c);
Math.max(Math?.max(a, b), c);
globalThis.Math.max(Math.max(a, b), c);
Number.max(Number.max(a, b), c);
Math.hypot(Math.hypot(a, b), c);

function withShadowedMath(Math) {
    return Math.max(Math.max(a, b), c);
}
