/* should not generate diagnostics */

type Dup =
	| { kind: "o\x6e"; flag: number }
	| { kind: "on"; flag: 1 };

// Both variants' discriminants unescape to "on", so `flag` stays
// `number | 1` and the inner condition is not always truthy.
function escapedDiscriminant(x: Dup) {
	if (x.kind === "on") {
		if (x.flag) {
			x;
		}
	}
}
