/* should not generate diagnostics */

declare function hasKind<K extends string>(value: unknown, kind: K): value is { kind: K };

function constrainedTypeArgument(shape: { kind: "a"; x: 1 } | { kind: "b"; y: 2 } | null) {
	if (!hasKind(shape, "a")) {
		if (shape) {
		}
	}
}

const isStr = (value: unknown) => typeof value === "string";
declare function onlyA(value: string): value is "a";
declare function onlyA(value: string | null): value is string;

function overloadAfterUndecomposedGuard(value: string | null) {
	if (isStr(value) && !onlyA(value)) {
		if (value) {
		}
	}
}
