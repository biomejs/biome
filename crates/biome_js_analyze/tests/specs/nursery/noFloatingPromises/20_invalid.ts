/* Generic identity wrapper — return type is the type parameter itself */

function wrapper<F extends (...args: any) => any>(fn: F): F {
	return fn;
}

async function _doWork(): Promise<void> {
	await new Promise((resolve) => setTimeout(resolve, 100));
}

const doWork = wrapper(_doWork);

async function main() {
	doWork();
}
