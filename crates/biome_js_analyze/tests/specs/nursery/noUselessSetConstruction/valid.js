/* should not generate diagnostics */
const selected = new Set([1, 2]);
const other = new Set([2, 3]);
const records = new Map([[2, "value"]]);

selected.intersection(records);
const copy = new Set(selected);

// The argument isn't a known `Set`, or the keys of a known `Map`
selected.union(new Set(condition ? other : records));
selected.union(new Set(condition ? other : unknown));
selected.union(new Set(other["keys"]()));
selected.union(new Set(other.keys));
selected.union(new Set((0, other)));
selected.union(new Set(other ?? selected));
selected.union(new Set(other.keys().values()));

// The method isn't called on a known `Set`
new Set(records).union(other);
new Set(unknown).union(other);
new Set(selected.entries()).union(other);

// The argument of the method isn't a known `Set` or `Map`, or it may change the `Set`
new Set(selected).union(condition ? other : unknown);
new Set(selected).union(selected.clear() ? other : selected);
new Set(selected).union((selected = other) ? other : selected);
new Set(selected).union(delete selected.size ? other : selected);
new Set(selected).union(counter++ ? other : selected);
new Set(selected).union(tag`${selected.clear()}` ? other : selected);
new Set(selected).union(`${selected.clear()}` ? other : selected);
new Set(selected).union(new Map());
new Set(selected).union(import("./other.js") ? other : selected);
new Set(selected).union({ [selected.clear()]: 1 } ? other : selected);
new Set(selected).union(class { static cleared = selected.clear(); } ? other : selected);
new Set(selected).union(class { static { selected.clear(); } } ? other : selected);
new Set(selected).union(class extends selected.clear() {} ? other : selected);
async function awaited() {
	new Set(selected).union((await counter) ? other : selected);
}
function* yielded() {
	new Set(selected).union((yield counter) ? other : selected);
}

// Not a plain call of the method with one argument
selected.union(extra, new Set(other));
selected.union(...new Set(other));
selected.union(new Set(other).size);
selected.union([new Set(other)]);
selected.union(condition ? new Set(other) : other);
selected.unknownMethod(new Set(other));
selected.add(new Set(other));
new Set(selected).union;
new Set(selected)?.union(other);
new Set(selected).union?.(other);
new Set(selected)["union"](other);
new new Set(selected).union(other)();
new Foo(new Set(other));
union(new Set(other));

// Not a plain `new Set()` with one argument
selected.union(new Set);
selected.union(new CustomSet(other));

// `Set` and `Map` are only recognized by their plain names
selected.union(new globalThis.Set(other));
selected.union(new window.Set(other));
const globalSet = new globalThis.Set();
selected.union(new Set(globalSet));
const globalMap = new globalThis.Map();
selected.union(new Set(globalMap.keys()));

// The method doesn't return a new `Set`, or isn't called on a known `Set`
new Set(selected.union(...other));
new Set(selected.union(other).values());
new Set(selected.union(other).union(records));

function shadowedMap(Map) {
	const records = new Map();
	selected.union(new Set(records.keys()));
}

function shadowedAfterUse() {
	selected.union(new Set(other));
	var Set;
}

function reassignableVar() {
	var third = new Set();
	third.union(new Set(other));
}

function untypedParameters(selected, other) {
	selected.union(new Set(other));
}

function defaultValue(other = new Set()) {
	selected.union(new Set(other));
}

function variablesThatHoldEachOther() {
	const first = second;
	const second = first;
	first.union(new Set(second));
}

function loopVariable(sets) {
	for (const other of sets) {
		selected.union(new Set(other));
	}
}
