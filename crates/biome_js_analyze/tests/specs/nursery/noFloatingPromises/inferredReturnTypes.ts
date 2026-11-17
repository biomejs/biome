// should generate diagnostics

declare const flag: boolean;

function maybeTask() {
	if (flag) return Promise.resolve(1);
}

// A path reaches the end of `maybeTask`, so the call can give `undefined`.
maybeTask();
