/* should not generate diagnostics */

type Letter = "a" | "b" | "c";

function nestedInCase(letter: Letter) {
	switch (letter) {
		case "a":
			switch (letter) {
				case "a":
					return 1;
			}
			return 2;
		case "b":
		case "c":
			return 3;
	}
}

function nestedInDefault(letter: Letter) {
	switch (letter) {
		case "a":
			return 1;
		default:
			switch (letter) {
				case "b":
					return 2;
				case "c":
					return 3;
			}
	}
}
