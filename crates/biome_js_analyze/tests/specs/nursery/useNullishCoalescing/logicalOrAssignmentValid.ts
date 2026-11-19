/* should not generate diagnostics */

// Already using `??=`
declare let a: string | null;
a ??= 'default';

// `&&=`
declare let b: string | null;
b &&= 'value';

// Non-nullish types
declare let c: string;
c ||= 'fallback';

declare let d: number;
d ||= 0;

declare let e: boolean;
e ||= true;

// Non-nullish union types
declare let f: string | number;
f ||= 'fallback';

declare let g: { a: string };
g ||= { a: 'default' };

// Non-nullish member target
declare const record: { name: string };
record.name ||= 'default';

// A type assertion that removes `null`
declare let assertedString: string | null;
(assertedString as string) ||= 'default';

// A non-null assertion
declare let nonNullTarget: { a: number } | null;
nonNullTarget! ||= { a: 1 };
