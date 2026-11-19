/* should generate diagnostics */
process.exit(0);
process.exit(1);
f(process.exit(1));
process.exit();
process["exit"](1);
process.exit?.(1);
process?.exit(1);
(process.exit)(1);
globalThis.process.exit(1);
function foo() {
	process.exit(1);
}
