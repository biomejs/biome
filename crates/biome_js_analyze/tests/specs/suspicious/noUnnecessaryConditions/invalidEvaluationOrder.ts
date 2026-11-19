// should generate diagnostics
// The key and the arguments after `?.` see its object as not nullish, also
// after a non-null assertion in the chain, the defaults of destructuring
// patterns and parameters see the narrowing before them, and the right operand
// of a logical assignment sees the test of its target, as in TypeScript.

type Options = { v: string | null; w: number; f(value: unknown): void };

function receiverInArguments(o: Options | undefined) {
	o?.f(o?.w);
}

function receiverInLaterLink(o: { g(): Options } | undefined) {
	o?.g().f(o?.g);
}

function receiverAfterNonNullAssertion(o: { inner?: Options } | undefined) {
	o?.inner!.f(o?.inner);
}

function parameterDefault(o: Options | undefined, w = o && o?.w) {
	return w;
}

function destructuringDefault(o: Options | undefined, source: { a?: number }) {
	if (o) {
		const { a = o?.w } = source;
		return a;
	}
}

type Empty = { kind: ""; options: Options };
type Named = { kind: "named"; options?: Options };

function logicalAssignmentDiscriminant(s: Empty | Named) {
	s.kind ||= (s.options?.f(0), "named");
}
