/* should not generate diagnostics */

enum Direction {
	Up,
	Down,
}

function move(direction: Direction) {
	switch (direction) {
		case Direction.Up:
			break;
		case Direction["Down"]:
			break;
	}
}

enum Color {
	Red = "red",
	Green = "green",
}

function paint(color: Color) {
	switch (color) {
		case Color.Red:
		case Color.Green:
			break;
	}
}
