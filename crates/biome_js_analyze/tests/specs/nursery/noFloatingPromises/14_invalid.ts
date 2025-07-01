const condition = Math.random() > -1; // Always true, but dynamic to linter
condition ? Promise.reject("ternary bypass") : null;
