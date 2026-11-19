/* should generate diagnostics */

// Union with null
declare let a: string | null;
a ||= 'default';

// Union with undefined
declare let b: number | undefined;
b ||= 0;

// Union with null and undefined
declare let c: string | null | undefined;
c ||= 'fallback';

// Nullish literal
let d: null = null;
d ||= 'value';

let e: undefined = undefined;
e ||= 'value';

// Object union with `null`: no fix
declare let obj: { a: string } | null;
obj ||= { a: 'default' };

// Number union with `null`: no fix
declare let mixed: number | null;
mixed ||= 100;

// Boolean union with `null`: no fix
declare let flag: boolean | null;
flag ||= true;
