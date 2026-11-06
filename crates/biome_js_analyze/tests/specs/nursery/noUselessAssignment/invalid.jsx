/* should generate diagnostics */
function ComponentTag() {
	let A = "unused";
	A = "used";
	return <A />;
}

function MemberTag() {
	let A = "unused";
	A = "used";
	return <A.B />;
}

function ReadInOneBranch() {
	let x = "used";
	if (cond) {
		return <A prop={x} />;
	} else {
		x = "unused";
	}
}

function ChildExpression() {
	let x = 1;
	x = 2;
	return <A>{x}</A>;
}
