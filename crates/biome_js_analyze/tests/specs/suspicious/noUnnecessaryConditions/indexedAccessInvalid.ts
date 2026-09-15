/* should generate diagnostics */

const letters = ["A", "B", "C"] as const;
type Letter = (typeof letters)[number];

function check(letter: Letter) {
	if (letter) {}
	return letter ?? "A";
}

function unreachable(letter: Letter) {
	switch (letter) {
		case "D": break;
	}
}

const values = { A: 1, B: 2, C: 3 } as const;
function checkKey(key: keyof typeof values) {
	if (key) {}
}
