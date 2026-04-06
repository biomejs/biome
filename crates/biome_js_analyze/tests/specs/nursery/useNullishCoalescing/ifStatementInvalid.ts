// If statements that should be converted to ??=

// Truthiness check: if (!foo) { foo = bar; }
declare let a: { x: string } | null;
declare function makeFoo(): { x: string };
if (!a) {
    a = makeFoo();
}

// Loose null check: if (foo == null) { foo = bar; }
declare let b: { x: string } | null;
if (b == null) {
    b = makeFoo();
}

// Loose undefined check: if (foo == undefined) { foo = bar; }
declare let c: { x: string } | undefined;
if (c == undefined) {
    c = makeFoo();
}

// Strict null + undefined check: if (foo === null || foo === undefined) { foo = bar; }
declare let d: { x: string } | null | undefined;
if (d === null || d === undefined) {
    d = makeFoo();
}

// Bare expression statement (no block): if (!foo) foo = bar;
declare let e: { x: string } | null;
if (!e) e = makeFoo();

// Strict null only: if (foo === null) { foo = bar; }
declare let sn: { x: string } | null;
if (sn === null) {
    sn = makeFoo();
}

// Strict undefined only: if (foo === undefined) { foo = bar; }
declare let su: { x: string } | undefined;
if (su === undefined) {
    su = makeFoo();
}

// Reversed operands: if (null == foo) { foo = bar; }
declare let rev: { x: string } | null;
if (null == rev) {
    rev = makeFoo();
}

// Reversed strict: if (undefined === foo) { foo = bar; }
declare let revs: { x: string } | undefined;
if (undefined === revs) {
    revs = makeFoo();
}

// Member access: if (foo.a == null) { foo.a = bar; }
declare let ma: { a: string | null };
declare function makeString(): string;
if (ma.a == null) {
    ma.a = makeString();
}

// Member access with negation: if (!foo.a) { foo.a = bar; }
declare let mb: { a: string | null };
if (!mb.a) {
    mb.a = makeString();
}

// ||= inside if body: should report the if-statement
declare let ora: { x: string } | null;
if (ora == null) {
    ora ||= makeFoo();
}

// ??= inside if body: should report the if-statement
declare let nca: { x: string } | null;
if (nca == null) {
    nca ??= makeFoo();
}

// Bare expression with == null
declare let bare2: { x: string } | null;
if (bare2 == null) bare2 = makeFoo();

