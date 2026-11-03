/* should not generate diagnostics */
const a = 1 | 1;
const b = 0 | 1;
const c = 1.4 | +0;
const d = 1.4 | -0;
const e = 1.4 | (.5 - 0.5);
const f = 1.4 & 0xFFFFFFFF;
const g = 1.4 & 0;
const h = ~3.9;
const i = 1.1 >> 1;
const j = 0 << 1;
const k = 1.1 >>> 0;
const l = 1.4 | "0";
let m = 0;
m |= 1;
m &= 0;
m >>>= 0;
const n = Math.trunc(1.4);

parseInt(value, 10);
parseInt(String(value));
parseInt(String(value), 16);
Number.parseInt(String(value), 16);
parseInt(String(value), "10");
parseInt?.(String(value), 10);
parseInt(String(...value), 10);
Number.parseInt(String(...value), 10);
parseInt(String(value), ...[10]);
Number.parseInt(String(value), ...[10]);
parseInt(String(value, extra), 10);
parseInt(String(), 10);
parseInt(new String(value), 10);
parseInt(String?.(value), 10);
parseInt(String(value), 10, extra);
Number.parseInt(String(value), 10, extra);
Number.parseInt?.(String(value), 10);
Number?.parseInt(String(value), 10);
Number["parseInt"](String(value), 10);
Math.parseInt(String(value), 10);

function shadowedParseInt(parseInt) {
	parseInt(String(value), 10);
}
function shadowedNumber(Number) {
	Number.parseInt(String(value), 10);
}
function shadowedString(String) {
	parseInt(String(value), 10);
	Number.parseInt(String(value), 10);
}

// `Math.trunc()` throws a `TypeError` for a BigInt, while `~~` and `parseInt` accept one.
~~10n;
~~-10n;
~~BigInt(value);
parseInt(String(10n), 10);
Number.parseInt(String(BigInt(value)), 10);
const big = 10n;
~~big;
const alsoBig = big;
parseInt(String(alsoBig), 10);
function bigParameter(count = 1n) {
	return ~~count;
}

// A local `Math` would replace the built-in object in the fix.
function compute(x, Math) {
	return x | 0;
}
{
	const Math = { trunc: () => 0 };
	~~value;
	parseInt(String(value), 10);
}
