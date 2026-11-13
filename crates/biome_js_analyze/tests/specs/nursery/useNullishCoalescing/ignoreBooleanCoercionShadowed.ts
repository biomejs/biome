// Tests for ignoreBooleanCoercion: true
// `Boolean` is declared in this module, so it is not the global built-in and
// every case below SHOULD generate a diagnostic.

function Boolean(value: unknown): boolean {
	return !!value;
}

declare const a: string | null;
declare const b: string;

const r1 = Boolean(a || 'default');

const r2 = Boolean(a || b || 'default');

const r3 = Boolean((a || 'default'));

declare let assigned: string | null;
const r4 = Boolean(assigned ||= 'default');
