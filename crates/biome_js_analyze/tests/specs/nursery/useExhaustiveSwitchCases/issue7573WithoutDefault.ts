/* should generate diagnostics */

type Letter = "A" | "B" | "C";
const getLetter = (): Letter => Math.random() > 0.5 ? "A" : "B";

function incomplete(): number {
	switch (getLetter()) {
		case "A":
			return 1;
		case "B":
			return 1;
	}
}
