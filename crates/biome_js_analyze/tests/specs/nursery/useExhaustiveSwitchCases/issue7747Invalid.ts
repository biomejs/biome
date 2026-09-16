/* should generate diagnostics */

const LetterValues = ["A", "B", "C"] as const;
type Letter = (typeof LetterValues)[number];

function test(letter: Letter): number {
	switch (letter) {
		case "A":
			return 1;
		case "B":
			return 1;
	}
}

const Bar = { "A": 1, "B": 2, "C": 3 } as const;
type Letter2 = keyof (typeof Bar);

function test2(letter: Letter2): number {
	switch (letter) {
		case "A":
			return 1;
		case "B":
			return 1;
	}
}

function numericMissing(value: [-1, 0x2][number]) {
	switch (value) {
		case 2: break;
	}
}
