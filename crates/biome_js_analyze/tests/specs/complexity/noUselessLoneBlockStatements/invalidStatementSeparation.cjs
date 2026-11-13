function sameLine() {
	{ console.log("a") } console.log("b")
}

function nextLineStartsWithParenthesis() {
	{ a() }
	(b || c).d()
}

function previousStatementWithoutSemicolon() {
	a()
	{ [b, c].forEach(d) }
}

function emptyBlockBetweenStatements() {
	a()
	{}
	[b, c].forEach(d)
}

function nestedStatement() {
	{ if (a) b() } c()
}

function trailingComment() {
	{ a() /* comment */ } b()
}

function alreadyTerminated() {
	{ a(); } b()
}

function endsWithBlock() {
	{ if (a) {} } b()
}
