/* should not generate diagnostics */

declare const letter: "A" | "B";
switch (letter) {
	case "A":
	case "B":
		break;
	default:
		throw new Error("Unexpected letter");
}

switch (letter) {
	case "A":
	case "B":
		break;
}

switch (letter) {
	default:
		throw new Error("Unexpected letter");
	case "A":
	case "B":
		break;
}

declare const mixed: 0 | true | false | null | undefined;
switch (mixed) {
	case 0:
	case true:
	case false:
	case null:
	case undefined:
		break;
	default:
		break;
}

declare const single: "A";
switch (single) {
	case "A":
		break;
	default:
		break;
}

declare const bigint: 1n | 2n;
switch (bigint) {
	case 0x1n:
	case 2n:
		break;
	default:
		break;
}

declare const n: number;
switch (true) {
	case n > 0:
		break;
	default:
		break;
}

declare const string: string;
declare const any: any;
declare const unknown: unknown;
declare const never: never;
switch (n) { default: break; }
switch (string) { default: break; }
switch (any) { default: break; }
switch (unknown) { default: break; }
switch (never) { default: break; }
switch (unresolved) { default: break; }

switch (string) { case "A": break; }

// biome-ignore lint/nursery/useExhaustiveSwitchCases: Intentionally delegate B to the fallback.
switch (letter) {
	case "A":
		break;
	default:
		break;
}
