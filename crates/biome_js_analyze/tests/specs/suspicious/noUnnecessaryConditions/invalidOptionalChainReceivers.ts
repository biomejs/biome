// should generate diagnostics

declare function assert(condition: unknown): asserts condition;
type Options = { v: string | null; w: number; f(): boolean };

function bareArgument(value: string | null) {
	assert(value?.length);
	return value?.length;
}

function truthy(o: Options | undefined) {
	if (o?.v) {
		return o?.w;
	}
}

function strict(o: Options | undefined) {
	if (o?.w === 1) {
		return o?.v;
	}
}

function call(o: Options | undefined) {
	if (o?.f()) {
		return o?.w;
	}
}

function typeOf(o: Options | undefined) {
	if (typeof o?.v === "string") {
		return o?.w;
	}
}

function earlyReturn(o: Options | undefined) {
	if (o?.v == null) return;
	return o?.w;
}
