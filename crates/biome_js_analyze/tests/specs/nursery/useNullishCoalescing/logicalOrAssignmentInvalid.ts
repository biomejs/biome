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

// Object shape with `null`: no fix, because `''` can be assigned to a shape whose members all exist on `String`
declare let obj: { a: string } | null;
obj ||= { a: 'default' };

// Number union with `null`: no fix
declare let mixed: number | null;
mixed ||= 100;

// Boolean union with `null`: no fix
declare let flag: boolean | null;
flag ||= true;

// Parenthesized identifier target
declare let wrapped: Date | null;
(wrapped) ||= new Date();

// Static member target
declare const record: { name: string | null };
record.name ||= 'default';

// Computed member target
declare const names: (string | null)[];
names[0] ||= 'default';

// Parenthesized member target
declare const box: { item: Date | undefined };
(box.item) ||= new Date();

// Variable target inside a type assertion
declare let asserted: Date | null;
(asserted as Date | null) ||= new Date();

// `satisfies` keeps the declared type
declare let satisfiesTarget: Date | null;
(satisfiesTarget satisfies Date | null) ||= new Date();

// An angle bracket assertion gives the asserted type
declare let angleTarget: unknown;
(<Date | null>angleTarget) ||= new Date();

// A member inside a type assertion gives the asserted type
declare const holder: { item: unknown };
(holder.item as Date | null) ||= new Date();

// `any` and `unknown`: no fix
declare let anyAssigned: any;
anyAssigned ||= 1;

declare let unknownAssigned: unknown;
unknownAssigned ||= 1;

declare const unknownMember: { v: unknown };
unknownMember.v ||= 1;
