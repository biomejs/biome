// should generate diagnostics
function foo() {}
async function bar() {}
function* generator() {}
export function exported() {}
export async function* exportedGenerator() {}
if (condition) { function nested() {} }
switch (value) { case 0: function inCase() {} }
function separateExport() {}
export { separateExport };

