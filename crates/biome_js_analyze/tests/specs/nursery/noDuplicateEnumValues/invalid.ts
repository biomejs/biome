/* should generate diagnostics */
enum Invalid1 {
	A = 0,
	B = 0,
}

enum Invalid2 {
	A = 1,
	B = 1.0,
	C = 0x1,
	D = 1e0,
}

enum Invalid3 {
	A = "A",
	B = 'A',
	C = `A`,
}
