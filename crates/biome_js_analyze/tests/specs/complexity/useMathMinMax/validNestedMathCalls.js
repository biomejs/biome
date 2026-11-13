/* should not generate diagnostics */

function shadowed(Math, a, b, c) {
    return Math.min(a, b) < c ? Math.min(a, b) : c;
}

Math.min(a++, b) < c ? Math.min(a++, b) : c;
Math.max((a = 1), b) > c ? Math.max((a = 1), b) : c;
Math.min(delete object.key, b) < c ? Math.min(delete object.key, b) : c;
