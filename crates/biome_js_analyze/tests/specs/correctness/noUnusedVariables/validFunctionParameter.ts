/* should not generate diagnostics */

function a(a: number) {}
a(1);

function b({a, b}: {a: number, b: string}) {}
b({a: 1, b: "2"});
