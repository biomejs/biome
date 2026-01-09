/* should not generate diagnostics */
enum Valid1 {
	A = 0,
	B = 1,
}

enum Valid2 {
	A = "A",
	B = 'B',
	C = `C`,
}

enum Valid3 {
	A = 0,
	B = "0",
}
