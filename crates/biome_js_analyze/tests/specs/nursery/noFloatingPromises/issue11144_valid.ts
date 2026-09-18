/* should not generate diagnostics */
let promise: Promise<void> | undefined;

const sleep = async (ms: number) => {
	await new Promise((resolve) => setTimeout(resolve, ms));
};

export const main = async () => {
	if (!promise) {
		promise = sleep(1000);
		await promise;
	} else {
		await promise;
	}
};
