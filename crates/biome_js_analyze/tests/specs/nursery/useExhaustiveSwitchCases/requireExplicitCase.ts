/* should generate diagnostics */

type Letter = "A" | "B" | "C";
const getLetter = (): Letter => Math.random() > 0.5 ? "A" : "B";

function trulyExhaustive(): number {
	switch (getLetter()) {
		case "A":
			return 1;
		case "B":
			return 1;
		case "C":
			return 1;
		default:
			throw new Error("bruh");
	}
}

function fakeNewsExhaustive(): number {
	switch (getLetter()) {
		case "A":
			return 1;
		case "B":
			return 1;
		default:
			throw new Error("bruh");
	}
}

trulyExhaustive();
fakeNewsExhaustive();
