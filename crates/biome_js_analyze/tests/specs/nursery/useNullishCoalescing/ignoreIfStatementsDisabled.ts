// Tests for ignoreIfStatements: false (default detection on).
// An if that only assigns to a nullish variable reports; the ??= fix is offered
// only when the test catches exactly null/undefined.

declare let obj: { x: number } | null;
declare function makeObj(): { x: number };

// `!obj` on an object union: truthiness is equivalent to a null check, fix is safe.
if (!obj) {
	obj = makeObj();
}

// single-statement body without braces, also safe.
if (!obj) obj = makeObj();

// loose null check: always safe to fix.
declare let loose: string | null;
if (loose == null) {
	loose = 'default';
}

// compound strict check: always safe to fix.
declare let compound: string | null;
if (compound === null || compound === undefined) {
	compound = 'default';
}

// `!str` on a string union: '' is falsy but not nullish, so report WITHOUT a fix.
declare let str: string | null;
if (!str) {
	str = 'default';
}

// single strict check that misses undefined: report WITHOUT a fix.
declare let maybe: string | null | undefined;
if (maybe === null) {
	maybe = 'default';
}

// single strict `=== null` on a union without undefined: safe to fix.
declare let onlyNull: string | null;
if (onlyNull === null) {
	onlyNull = 'default';
}

// `=== undefined` on a union without null: safe to fix.
declare let onlyUndef: string | undefined;
if (onlyUndef === undefined) {
	onlyUndef = 'default';
}

// `=== undefined` on a union that also has null: report WITHOUT a fix.
declare let undefCheck: string | null | undefined;
if (undefCheck === undefined) {
	undefCheck = 'default';
}

// reversed operand order: safe to fix.
declare let reversed: string | null;
if (null == reversed) {
	reversed = 'default';
}

// vacuous compound (same literal twice) still misses undefined: report WITHOUT a fix.
declare let vacuous: string | null | undefined;
if (vacuous === null || vacuous === null) {
	vacuous = 'default';
}

// static member subject: safe to fix.
declare let rec: { a: number | null };
if (rec.a == null) {
	rec.a = 1;
}

// computed member subject: safe to fix.
declare let rec2: { a: number | null };
if (rec2['a'] == null) {
	rec2['a'] = 1;
}

// member subject with a side-effecting object: report WITHOUT a fix, since the
// original evaluates `getRec()` twice but `??=` would evaluate it once.
declare function getRec(): { a: number | null };
if (getRec().a == null) {
	getRec().a = 1;
}

// computed member on a side-effecting object: report WITHOUT a fix.
if (getRec()['a'] == null) {
	getRec()['a'] = 1;
}

// inner comment would be dropped by the rewrite: report WITHOUT a fix.
declare let commented: string | null;
if (commented == null) {
	// keep me
	commented = 'default';
}

// Negative: else branch present.
declare let withElse: { x: number } | null;
if (!withElse) {
	withElse = makeObj();
} else {
	withElse = makeObj();
}

// Negative: subject is not nullish.
declare let notNullish: { x: number };
if (!notNullish) {
	notNullish = makeObj();
}

// Negative: more than one statement in the body.
declare let multi: { x: number } | null;
if (!multi) {
	multi = makeObj();
	sideEffect();
}

// Negative: a ||= body is left to the operator logic, so the if is not reported twice.
declare let assign: { x: number } | null;
if (!assign) {
	assign ||= makeObj();
}

// Negative: the test checks a different reference than the body assigns.
declare let mismatchA: { x: number } | null;
declare let mismatchB: { x: number } | null;
if (!mismatchA) {
	mismatchB = makeObj();
}

declare function sideEffect(): void;
