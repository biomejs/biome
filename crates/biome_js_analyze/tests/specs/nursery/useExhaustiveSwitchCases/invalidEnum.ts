/* should generate diagnostics */

// Biome types a member of an enum by its value, so it names a missing member
// by its value, as `2` for `Direction.Left` and `"green"` for `Color.Green`.
enum Direction {
	Up,
	Down,
	Left,
}

function move(direction: Direction) {
	switch (direction) {
		case Direction.Up:
			break;
		case Direction.Down:
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
			break;
	}
}
