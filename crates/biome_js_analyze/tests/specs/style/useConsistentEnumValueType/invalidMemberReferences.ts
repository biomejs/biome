/* should generate diagnostics */

// `C` has the number value of `A`, and `D` the string value of `B`.
enum Mixed {
	A = 1,
	B = "b",
	C = A,
	D = B,
}
