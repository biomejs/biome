/* should not generate diagnostics */
if (a !== b) {}
if (!(a === b)) {}
if (!(a !== b)) {}
if (a === !b) {}
if ((!a) === b) {}
if (!a == b) {}
if (!a != b) {}
if (!!a === b) {}
const isOk = a === b;
const isNotOk = a !== b;
