/* should generate diagnostics */

function array(array: Array<string>) {
	array.forEach((value) => console.log(value));
}

function strings(array: string[]) {
	array.forEach((value) => console.log(value));
}

function tuple(array: [string, string]) {
	array.forEach((value) => console.log(value));
}

type Strings = string[];
function alias(array: Strings) {
	array.forEach((value) => console.log(value));
}

type Map = string[];
function aliasNamedMap(array: Map) {
	array.forEach((value) => console.log(value));
}

function partialUnion(collection: string[] | Set<string>) {
	collection.forEach((value) => console.log(value));
}

function typedArray(array: Uint8Array) {
	array.forEach((value) => console.log(value));
}

function untyped(collection: { forEach(callback: (value: string) => void): void }) {
	collection.forEach((value) => console.log(value));
}

(map as Array<string>).forEach((value) => console.log(value));

function nullableArray(array: string[] | undefined) {
	array?.forEach((value) => console.log(value));
}

function onlyNullish(value: null | undefined) {
	value?.forEach((item) => console.log(item));
}
