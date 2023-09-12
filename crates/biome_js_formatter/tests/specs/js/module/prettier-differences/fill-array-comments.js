[
 	// Prettier prints the `-2` element on the same line as the `-3`.
	// This is the case because Prettier doesn't add virtual groups around `fill` elements, making it return `true` when it
	// encounters the first hard line break. As it happens, this line comment contains a hard line break, making
	// Prettier believe that the `-3` with this leading comment all fits on the line, which, obviously, isn't the case.
	-3,
	-2
]
