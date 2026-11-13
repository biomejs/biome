function typeAlias() {
	{ type A = string } a()
}

function importEquals() {
	{ import A = B.C } a()
}

function declareFunction() {
	{ function f(): void } a()
}
