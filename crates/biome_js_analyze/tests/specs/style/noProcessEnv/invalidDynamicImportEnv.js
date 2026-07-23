async function fromProcess() {
	const { env } = await import("process");
	env.NODE_ENV;
}

async function fromNodeProcess() {
	const { env } = await import("node:process");
	env.HOME;
}
