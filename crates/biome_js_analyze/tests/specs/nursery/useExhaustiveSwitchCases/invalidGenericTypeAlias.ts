// `Id<"a" | "b">` substitutes to the literal union, so the missing `"b"` case is detectable.
type Id<T> = T;
declare const s: Id<"a" | "b">;
switch (s) {
	case "a":
		break;
}
