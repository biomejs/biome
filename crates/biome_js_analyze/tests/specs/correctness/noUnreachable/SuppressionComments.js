// biome-ignore lint/correctness/noUnreachable: this comment does nothing
function SuppressionComments1() {
	beforeReturn();
	return;
	afterReturn();
}

function SuppressionComments2() {
	beforeReturn();
	return;
	// biome-ignore lint/correctness/noUnreachable: supress warning
	afterReturn();
}
