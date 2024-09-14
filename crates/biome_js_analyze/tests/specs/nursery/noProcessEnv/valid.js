/* should not generate diagnostics */
let a = Process.env;
let a = process[env];
let a = process.nextTick;
let a = process.execArgv;

const process = {
	env() {}
};
process.env;
