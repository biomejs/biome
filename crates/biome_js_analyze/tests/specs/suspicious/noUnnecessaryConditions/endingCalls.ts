// should generate diagnostics

declare function fail(message: string): never;
declare function assert(condition: unknown): asserts condition;
declare const log: { fatal(message: string): never };

function localNever(value: string | null) {
	if (value === null) fail("missing");
	return value?.length;
}

function methodNever(value: string | null) {
	if (value === null) log.fatal("missing");
	return value?.length;
}

function assertFalse(value: string | null) {
	if (value === null) assert(false);
	return value?.length;
}

function literalTrue(value: string | null) {
	if (value === null) {
		if (true) return;
	}
	return value?.length;
}
