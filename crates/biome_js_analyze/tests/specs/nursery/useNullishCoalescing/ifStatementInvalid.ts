/* should generate diagnostics */

declare let obj: Date | null;
declare function makeObj(): Date;

// `!obj` on a built-in object class union: safe fix
if (!obj) {
	obj = makeObj();
}

// Body without braces
if (!obj) obj = makeObj();

// Loose `== null`
declare let loose: string | null;
if (loose == null) {
	loose = 'default';
}

// Strict compound check
declare let compound: string | null;
if (compound === null || compound === undefined) {
	compound = 'default';
}

// `!str` on a string union: no fix
declare let str: string | null;
if (!str) {
	str = 'default';
}

// `=== null` on a union without `undefined`
declare let onlyNull: string | null;
if (onlyNull === null) {
	onlyNull = 'default';
}

// `=== undefined` on a union without `null`
declare let onlyUndef: string | undefined;
if (onlyUndef === undefined) {
	onlyUndef = 'default';
}

// Reversed operands
declare let reversed: string | null;
if (null == reversed) {
	reversed = 'default';
}

// Loose check on a non-nullish type
declare let definite: string;
if (definite == null) {
	definite = 'default';
}

// Static member subject
declare let rec: { a: number | null };
if (rec.a == null) {
	rec.a = 1;
}

// Computed member subject
declare let rec2: { a: number | null };
if (rec2['a'] == null) {
	rec2['a'] = 1;
}

// Computed member subject with whitespace in the test
declare let rec3: { a: number | null };
if (rec3[ 'a' ] == null) {
	rec3['a'] = 1;
}

// Optional chaining in the test
declare let maybeRec: { a: string | null } | null;
if (maybeRec?.a == null) {
	maybeRec.a = 'default';
}

// Static member test and computed member target
declare let mixedAccess: { a: number | null };
if (mixedAccess.a == null) {
	mixedAccess['a'] = 1;
}

// Parentheses around the test, the assignment, and the target
declare let wrapped: string | null;
if (((wrapped == null))) {
	wrapped = 'default';
}
if (wrapped == null) {
	((wrapped)) = 'default';
}
if (wrapped == null) {
	(wrapped = 'default');
}

// Type assertion around the subject
declare let asserted: string | null;
if ((asserted as string | null) == null) {
	asserted = 'default';
}

// Compound check mixing strict and loose comparisons
declare let mixedCompound: string | null | undefined;
if (mixedCompound === null || mixedCompound == undefined) {
	mixedCompound = 'default';
}

// `??=` body
declare let nullishAssign: Date | null;
if (nullishAssign == null) {
	nullishAssign ??= makeObj();
}

// `||=` body: reported by the `if` and by the `||=`
declare let assign: Date | null;
if (!assign) {
	assign ||= makeObj();
}

// `else if`, labeled `if`, and unbraced `if` in a loop
declare let elseIfValue: string | null;
declare let labeled: string | null;
declare let inLoop: string | null;
declare const cond: boolean;
declare const items: string[];
if (cond) {
} else if (elseIfValue == null) {
	elseIfValue = 'default';
}
label: if (labeled == null) {
	labeled = 'default';
}
for (const item of items) if (inLoop == null) inLoop = item;

// Inner comment in the body: no fix
declare let commented: string | null;
if (commented == null) {
	// keep me
	commented = 'default';
}

// A type that a `0`, `''`, or `false` value can have: no fix
declare let numberObject: Number | null;
if (!numberObject) numberObject = 1;

// An object shape: no fix, because `''` can be assigned to a shape whose members all exist on `String`
declare let named: { a: string } | null;
if (!named) named = { a: '' };

// Type assertion that removes `null` from the tested value
declare let pinned: string | null;
if ((pinned as string) == null) pinned = 'v';

// `any` and `unknown`: no fix
declare let anyBraced: any;
if (!anyBraced) {
	anyBraced = 1;
}
declare let anyUnbraced: any;
if (!anyUnbraced) anyUnbraced = 1;
declare let unknownBraced: unknown;
if (!unknownBraced) {
	unknownBraced = 1;
}
declare let unknownUnbraced: unknown;
if (!unknownUnbraced) unknownUnbraced = 1;

declare let unknownOrAssign: unknown;
if (!unknownOrAssign) {
	unknownOrAssign ||= 1;
}
declare let unknownNullishAssign: unknown;
if (!unknownNullishAssign) {
	unknownNullishAssign ??= 1;
}
