/* should generate diagnostics */

declare const letter: "A" | "B" | "C";

switch (letter) {
	// Runtime fallback.
	default:
		throw new Error("Unexpected letter");
}

switch (letter) {
	default:
		throw new Error("Unexpected letter");
	// Handle A explicitly.
	case "A":
		break;
}

switch (letter) {
	case "A":
		break;
	// Runtime fallback between cases.
	default:
		throw new Error("Unexpected letter");
	case "B":
		break;
}

switch (letter) {
	case "A":
	case "B":
		console.log(letter);
	// A and B also execute the fallback.
	default:
		console.log("fallback");
}

declare const numeric: 0 | 1 | 2;
switch (numeric) {
	case 0:
		break;
	default:
		break;
}

declare const boolean: boolean;
switch (boolean) {
	case true:
		break;
	default:
		break;
}

declare const nullable: "A" | null | undefined;
switch (nullable) {
	case "A":
		break;
	default:
		break;
}

declare const bigint: 1n | 2n;
switch (bigint) {
	case 1n:
		break;
	default:
		break;
}

type A = "A";
type B = "B";
declare const aliased: A | B;
switch (aliased) {
	case "A":
		break;
	default:
		break;
}

declare const branded: ("A" & { brand: true }) | "B";
switch (branded) {
	case "B":
		break;
	default:
		break;
}

declare const event: { kind: "start" } | { kind: "stop" };
switch (event.kind) {
	case "start":
		break;
	default:
		break;
}

declare const dynamicCase: "A" | "B";
switch (letter) {
	case dynamicCase:
		break;
	case "C":
		break;
	default:
		break;
}

switch (letter) {
	case "A":
		break;
}
