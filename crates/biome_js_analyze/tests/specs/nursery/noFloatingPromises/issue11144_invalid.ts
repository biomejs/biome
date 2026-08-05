/* should generate diagnostics */
let promise: Promise<void> | undefined;

const sleep = async (ms: number) => {
	await new Promise((resolve) => setTimeout(resolve, ms));
};

export const main = async () => {
	if (!promise) {
		promise = sleep(1000);
	}

	// The optional Promise is never handled on either path.
	promise;
};

export const assignOnly = async () => {
	// Storing the Promise does not handle it, so the reference below floats.
	promise = sleep(1000);
	promise;
};

export const callOnly = async () => {
	sleep(1000);
};
