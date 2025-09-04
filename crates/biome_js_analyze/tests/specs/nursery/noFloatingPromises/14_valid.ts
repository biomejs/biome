/* should not generate diagnostics */

const condition = 0; // Always falsy.
condition ? Promise.reject("ternary bypass") : null;
