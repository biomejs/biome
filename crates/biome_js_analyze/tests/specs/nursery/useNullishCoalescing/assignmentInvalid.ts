// ||= with nullish types should suggest ??=

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

// Object type with null (safe fix - objects are always truthy)
declare let obj: { a: string } | null;
obj ||= { a: 'default' };

// Mixed nullish + falsy types (no safe fix)
declare let mixed: number | null;
mixed ||= 100;

// Boolean union (no safe fix - false is falsy but not nullish)
declare let flag: boolean | null;
flag ||= true;
