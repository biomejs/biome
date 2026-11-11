/* should not generate diagnostics */

function map(map: Map<string, string>) {
	map.forEach((value) => console.log(value));
}

function readonlyMap(map: ReadonlyMap<string, string>) {
	map.forEach((value) => console.log(value));
}

function weakMap(map: WeakMap<object, string>) {
	map.forEach((value) => console.log(value));
}

function set(set: Set<string>) {
	set.forEach((value) => console.log(value));
}

function readonlySet(set: ReadonlySet<string>) {
	set.forEach((value) => console.log(value));
}

function weakSet(set: WeakSet<object>) {
	set.forEach((value) => console.log(value));
}

function union(collection: Map<string, string> | Set<string>) {
	collection.forEach((value) => console.log(value));
}

function headers(values: Headers) {
	values.forEach((value, key) => consume(key, value));
}

function formData(values: FormData) {
	values.forEach((value, key) => consume(key, value));
}

function searchParams(values: URLSearchParams) {
	values.forEach((value, key) => consume(key, value));
}

function wrapped(map: Map<string, string>) {
	(map as Map<string, string>).forEach((value) => console.log(value));
	map!.forEach((value) => console.log(value));
}

const annotated: Map<string, number> = create();
annotated.forEach((value) => console.log(value));

let mutable: Set<number> = new Set();
mutable.forEach((value) => console.log(value));

function outerMap(Map) {
	function inner(map: Map<string, string>) {
		map.forEach((value) => console.log(value));
	}
}

type Array<T> = Map<T, T>;
function shadowedArray(collection: Array<string>) {
	collection.forEach((value) => console.log(value));
}

type Uint8Array = Set<number>;
function shadowedTypedArray(collection: Uint8Array) {
	collection.forEach((value) => console.log(value));
}

function nullable(map: Map<string, string> | undefined) {
	map?.forEach((value) => console.log(value));
}

let cache: Set<string> | null = null;
cache?.forEach((value) => console.log(value));
