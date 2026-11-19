/* should not generate diagnostics */
Process.exit();
var exit = process.exit;
f(process.exit);
process.exitCode = 1;
process.exiting(1);
foo.process.exit(1);
throw new Error("Something bad happened");
