// should generate diagnostics
[1].some(false || (() => null));
[1].filter(true && (() => null));
