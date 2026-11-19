/* should not generate diagnostics */
function foo(process) {
	process.exit(1);
}

const process = { exit() {} };
process.exit(1);
