/* should not generate diagnostics */
class A {
	constructor(@Inject(tokenSync()) x) {}
	method(@decorator(readFileSync("x")) y) {}
}
