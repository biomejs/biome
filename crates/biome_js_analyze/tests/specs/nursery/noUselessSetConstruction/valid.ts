/* should not generate diagnostics */
function argumentTypeArguments(selected: Set<number>, other: Set<number>) {
	selected.union(new Set<number>(other));
}
function receiverTypeArguments(selected: Set<number>, other: Set<number>) {
	new Set<number>(selected).union(other);
}
function resultTypeArguments(selected: Set<number>, other: Set<number>) {
	new Set<number>(selected.union(other));
}
function optionalSet(selected: Set<number> | undefined, other: Set<number>) {
	selected.union(new Set(other));
}
function argumentSatisfies(selected: Set<number>, other: ReadonlySet<number>) {
	selected.union(new Set(other) satisfies Set<number>);
}
function receiverSatisfies(selected: ReadonlySet<number>, other: Set<number>) {
	(new Set(selected) satisfies Set<number>).union(other);
}
function resultAsIterable(selected: Set<number>, other: Set<number>) {
	new Set(selected.union(other) as Iterable<number>).has(1);
}
function setLike(selected: { union(other: unknown): number[] }, other: Set<number>) {
	return new Set(selected.union(other));
}
function destructuredArray() {
	const [other] = new Set([[2, 3]]);
	const selected = new Set([1, 2]);
	selected.union(new Set(other));
}
function setOrMap(selected: Set<number> | Map<number, string>, other: Set<number>) {
	selected.union(new Set(other));
}
function setOrMapVariable(selected: Set<number>) {
	const either: Set<number> | Map<number, string> = new Set();
	selected.union(new Set(either));
}
function setOrMapAssertions(selected: Set<number>, other: Set<number>, unknownValue: unknown) {
	selected.union(new Set(other satisfies Set<number> | Map<number, string>));
	selected.union(new Set(unknownValue as Set<number> | Map<number, string>));
}
function otherTypes(selected: WeakSet<object>, other: Array<object>, third: number[]) {
	selected.union(new Set(other));
	new Set(third).union(new Set(other));
}
function otherTypeAssertions(selected: Set<number>, other: number[]) {
	selected.union(new Set(other as Iterable<number>));
	selected.union(new Set(other as any));
	selected.union(new Set(other satisfies unknown));
}
function qualifiedAndCombinedTypes(selected: globalThis.Set<number>, other: Set<number> & { extra: true }) {
	selected.union(new Set(other));
	other.union(new Set(selected));
}
function argumentWrapped(selected: Set<number>, other: ReadonlySet<number>) {
	selected.union(new Set(other) as Set<number>);
	selected.union(new Set(other)!);
}
function receiverWrapped(selected: ReadonlySet<number>, other: Set<number>) {
	(new Set(selected) as Set<number>).union(other);
	new Set(selected)!.union(other);
}
function resultWrapped(selected: Set<number>, other: Set<number>) {
	new Set(selected.union(other)!);
}
function userDefinedType() {
	type Set<T> = { union(other: unknown): T[] };
	type ReadonlyMap<K, V> = [K, V][];
	function check(selected: Set<number>, records: ReadonlyMap<number, string>) {
		return new Set(selected.union(records));
	}
}
function typeParameter<Set extends { union(other: unknown): number[] }>(selected: Set, other: unknown) {
	return new Set(selected.union(other));
}
function reassignableWithOtherType(values: number[]) {
	let selected: Iterable<number> = new Set(values);
	return new Set(selected.union(values));
}
function parameterWithOtherType(selected: Set<number>, other: Iterable<number> = new Set()) {
	selected.union(new Set(other));
}
function destructuredParameter({ other }: { other: Set<number> }, selected: Set<number>) {
	selected.union(new Set(other));
}
function restParameter(selected: Set<number>, ...other: Set<number>[]) {
	selected.union(new Set(other));
}

// The rule can't tell that a property, an item of a collection, or the result of a call is a `Set` or a `Map`
function mapProperty(selected: Set<number>, records: { value: Map<number, string> }) {
	return selected.union(new Set(records.value.keys()));
}
function setProperty(selected: { value: Set<number> }, other: Set<number>) {
	return new Set(selected.value.union(other));
}
function setItem(source: Set<Set<number>>, selected: Set<number>) {
	const [other] = source;
	return selected.union(new Set(other));
}
function objectProperty() {
	const b = new Set();
	new Set({ value: new Set([1]) }.value).union(b);
}
function objectPropertyInVariable() {
	const b = new Set();
	const result = new Set({ value: new Set([1]) }.value).union(b);
}
function objectPropertyResult() {
	const b = new Set();
	new Set({ value: new Set([1]) }.value.union(b));
}
function objectPropertyResultInVariable() {
	const b = new Set();
	const result = new Set({ value: new Set([1]) }.value.union(b));
}
function callResult(selected: Set<number>) {
	selected.union(new Set(load()));
}
class Fields {
	items: Set<number> = new Set();
	check(selected: Set<number>) {
		return selected.union(new Set(this.items));
	}
}
declare function load(): Set<number>;
