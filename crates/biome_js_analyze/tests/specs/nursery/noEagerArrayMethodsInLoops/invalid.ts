/* should generate diagnostics */
import type { Generator } from "./other";

for (const item of items.map(callback) as string[]) {}
for (const item of items.map(callback)!) {}
for (const item of items.filter(callback) satisfies string[]) {}

function arrayAssertion(items: unknown) {
	for (const item of (items as string[]).map((item) => item.trim())) {}
}

function arrayParameter(items: string[]) {
	for (const item of items.map(callback)) {}
}

function annotatedArrayVariable(source: Iterable<string>) {
	const list: string[] = [...source];
	for (const item of list.filter(callback)) {}
}

function arrayOrIterator(items: Iterator<string> | string[]) {
	for (const item of items.map(callback)) {}
}

function arrayReturnType() {
	function getItems(): string[] {
		return [];
	}
	for (const item of getItems().map(callback)) {}
}

function locallyDeclaredType() {
	interface Iterator<T> {
		map(callback: (item: T) => T): T[];
	}
	function inner(items: Iterator<string>) {
		for (const item of items.map(callback)) {}
	}
}

function typeParameterNamedIterator<Iterator extends string[]>(items: Iterator) {
	for (const item of items.map(callback)) {}
}

function importedType(items: Generator<string>) {
	for (const item of items.map(callback)) {}
}

type List = string[];

function arrayTypeAlias(items: List) {
	for (const item of items.filter(callback)) {}
}

type First = Second;
type Second = First;

function typeAliasesReferringToEachOther(items: First) {
	for (const item of items.map(callback)) {}
}
