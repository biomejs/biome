// Tests for ignoreIfStatements: true.
// if statements assigning to a nullish variable are ignored, but || and ||= still report.

declare let a: string | null;

// ignored
if (!a) {
	a = 'default';
}

// ignored
if (a == null) {
	a = 'fallback';
}

// || still reported
declare const s: string | null;
const r1 = s || 'x';

// ||= still reported
declare let b: string | null;
b ||= 'y';
