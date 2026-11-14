type Letter = "a" | "b" | "c";

function nestedInGroup(letter: Letter) {
	switch (letter) {
		case "a":
			return 1;
		case "b":
		case "c":
			switch (letter) {
				case "b":
					return 2;
			}
	}
}
