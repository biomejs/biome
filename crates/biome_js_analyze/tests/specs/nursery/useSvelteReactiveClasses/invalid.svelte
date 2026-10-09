<!-- should generate diagnostics -->
<script>
	const date = new Date(8.64e15);
	date.setMonth(3);

	const map = new Map([[1, "one"]]);
	map.clear();

	const set = new Set([1, 2]);
	function remove(value) {
		set.delete(value);
	}

	const url = new URL("https://svelte.dev/");
	url.port = 80;

	const params = new URLSearchParams("foo=1&bar=2");
	params.sort();

	const computed = new Map();
	computed["set"](1, "one");

	const computedUrl = new URL("https://svelte.dev/");
	computedUrl["hash"] = "#top";

	const compoundUrl = new URL("https://svelte.dev/");
	compoundUrl.pathname += "/docs";

	const optional = new Set();
	optional?.add(1);

	const state = $state(new Map());
	state.set(1, "one");

	const raw = $state.raw(new Set());
	raw.add(1);

	const alias = new Map();
	const other = alias;
	other.set(1, "one");

	let assigned;
	assigned = new Set();
	assigned.add(1);

	const fallback = cond ? new Map() : null;
	fallback?.set(1, "one");

	const logical = existing ?? new Set();
	logical.add(1);

	let created;
	function create() {
		created = new Map();
	}
	function update() {
		created.set(1, "one");
	}

	function local() {
		const temporary = new Set();
		globalSet = temporary;
	}
	let globalSet;
	globalSet.add(1);
</script>
