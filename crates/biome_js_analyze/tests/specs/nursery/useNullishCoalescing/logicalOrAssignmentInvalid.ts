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

// Object union with `null`: safe fix
declare let obj: { a: string } | null;
obj ||= { a: 'default' };

// Number union with `null`: no fix
declare let mixed: number | null;
mixed ||= 100;

// Boolean union with `null`: no fix
declare let flag: boolean | null;
flag ||= true;

// Parenthesized identifier target
declare let wrapped: { a: string } | null;
(wrapped) ||= { a: 'default' };

// Static member target
declare const record: { name: string | null };
record.name ||= 'default';

// Computed member target
declare const names: (string | null)[];
names[0] ||= 'default';

// Parenthesized member target
declare const box: { item: { a: string } | undefined };
(box.item) ||= { a: 'default' };

// Variable target inside a type assertion
declare let asserted: { a: string } | null;
(asserted as { a: string } | null) ||= { a: 'default' };

// `satisfies` keeps the declared type
declare let satisfiesTarget: { a: string } | null;
(satisfiesTarget satisfies { a: string } | null) ||= { a: 'default' };

// An angle bracket assertion gives the asserted type
declare let angleTarget: unknown;
(<{ a: string } | null>angleTarget) ||= { a: 'default' };

// A member inside a type assertion gives the asserted type
declare const holder: { item: unknown };
(holder.item as { a: string } | null) ||= { a: 'default' };

// `any` and `unknown`: no fix
declare let anyAssigned: any;
anyAssigned ||= 1;

declare let unknownAssigned: unknown;
unknownAssigned ||= 1;

declare const unknownMember: { v: unknown };
unknownMember.v ||= 1;
