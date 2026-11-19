/* should generate diagnostics */
const name = "FOO";

delete process.env[name];
delete process["env"][name];
delete (process.env)[name];
delete process?.env[name];
delete process.env[`${name}_URL`];

function shadowed(process) {
	delete process.env[name];
}
