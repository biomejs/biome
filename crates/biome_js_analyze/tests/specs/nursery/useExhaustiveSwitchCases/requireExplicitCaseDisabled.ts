/* should generate diagnostics */

declare const letter: "A" | "B";
switch (letter) {
	default:
		throw new Error("Unexpected letter");
}

switch (letter) {
	case "A":
		break;
	default:
		break;
}

switch (letter) {
	case "A":
	case "B":
		break;
}

switch (letter) {
	case "A":
		break;
}
