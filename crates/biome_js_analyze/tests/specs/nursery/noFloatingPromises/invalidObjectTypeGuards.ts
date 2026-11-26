// should generate diagnostics
// A type-guard function or an assertion function whose target is an object
// type narrows the tested value, as in TypeScript, so a value narrowed to a
// promise and not handled is reported.

interface Job {
	run(): void;
}

declare function isPromise(input: unknown): input is Promise<void>;
declare function isJob(input: unknown): input is Job;
declare function assertPromise(input: unknown): asserts input is Promise<void>;

function trueBranch(value: unknown) {
	if (isPromise(value)) {
		value;
	}
}

function falseBranch(value: Promise<void> | Job) {
	if (!isJob(value)) {
		value;
	}
}

function assertion(value: unknown) {
	assertPromise(value);
	value;
}
