/* should generate diagnostics */

Math.min(foo(), b) < c ? Math.min(foo(), b) : c;
Math.min(...values) < c ? Math.min(...values) : c;
Math.min([...values], b) < c ? Math.min([...values], b) : c;
Math.min(tag`value`, b) < c ? Math.min(tag`value`, b) : c;
Math["min"](a, b) < c ? Math["min"](a, b) : c;
Math?.min(a, b) < c ? Math?.min(a, b) : c;
Math.min?.(a, b) < c ? Math.min?.(a, b) : c;
Math.min(a, b) < c ? Math.min(/* keep */ a, b) : c;
Math.min(a, b) /* condition */ < c ? /* branch */ Math.min(a, b) : c;
Math.min(a, b) < c
    ? Math.min(a, b) // keep branch
    : c;
Math.min(a, b) // keep condition
    < c ? Math.min(a, b) : c;
