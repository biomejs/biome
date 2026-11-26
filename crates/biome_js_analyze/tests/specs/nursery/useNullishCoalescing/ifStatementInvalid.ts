/* should generate diagnostics */

declare let obj: { x: number } | null;
declare function makeObj(): { x: number };

// `!obj` on an object union: no fix
if (!obj) {
	obj = makeObj();
}

// Body without braces: no fix
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

// Calls in the subject: no fix
declare function getRec(): { a: number | null };
if (getRec().a == null) {
	getRec().a = 1;
}

if (getRec()['a'] == null) {
	getRec()['a'] = 1;
}

// `||=` body: only the `||=` is reported
declare let assign: { x: number } | null;
if (!assign) {
	assign ||= makeObj();
}

// Inner comment in the body: no fix
declare let commented: string | null;
if (commented == null) {
	// keep me
	commented = 'default';
}
