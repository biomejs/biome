/* should generate diagnostics */
function argumentReadonlySet(selected: Set<number>, other: ReadonlySet<number>) {
	return selected.union(new Set(other));
}
function argumentReadonlyMap(selected: Set<number>, records: ReadonlyMap<number, string>) {
	return selected.union(new Set(records.keys()));
}
function argumentMap(selected: Set<number>, records: Map<number, string>) {
	return selected.intersection(new Set(records.keys()));
}
function receiver(selected: ReadonlySet<number>, other: Set<number>) {
	return new Set(selected).union(other);
}
function result(selected: Set<number>, other: Set<number>) {
	return new Set(selected.union(other));
}
function argumentAs(selected: Set<number>, other: unknown) {
	return selected.union(new Set(other as Set<number>));
}
function receiverAs(selected: Set<number>, other: unknown) {
	return new Set(other as Set<number>).union(selected);
}
function receiverSatisfies(selected: Set<number>, other: Set<number>) {
	return new Set(other satisfies Set<number>).union(selected);
}
function receiverTypeAssertion(selected: Set<number>, other: unknown) {
	return new Set(<Set<number>>other).union(selected);
}
function receiverNonNull(selected: Set<number>, other: Set<number>) {
	return new Set(other!).union(selected);
}
function nonNullReceiver(selected: Set<number>, other: Set<number>) {
	return selected!.union(new Set(other));
}
function argumentSatisfies(selected: Set<number>, other: Set<number>) {
	return selected.union(new Set(other satisfies Set<number>));
}
function argumentAsTwice(selected: Set<number>, other: number[]) {
	return selected.union(new Set(other as unknown as Set<number>));
}
function resultAs(selected: Set<number>, other: Set<number>) {
	return new Set(selected.union(other)) as Set<number>;
}
function resultSatisfies(selected: Set<number>, other: Set<number>) {
	return new Set(selected.union(other)) satisfies Set<number>;
}
function defaultValue(selected: Set<number>, other: Set<number> = new Set()) {
	return selected.union(new Set(other));
}
function newCalleeNonNullCall(selected: Set<number>, other: Set<number>) {
	return new new Set(selected.union(other))!();
}
function newCalleeNonNull(selected: Set<number>, other: Set<number>) {
	return new new Set(selected.union(other))!.constructor();
}
function unionType(selected: Set<string> | (ReadonlySet<number>), other: Set<number>) {
	return selected.union(new Set(other));
}
function parenthesizedType(selected: (Set<number>), other: Set<number>) {
	return selected.union(new Set(other));
}
function readonlyType(selected: readonly Set<number>, other: Set<number>) {
	return selected.union(new Set(other));
}
function typedVariables() {
	let selected: Set<number> = getValues();
	const other: ReadonlySet<number> = getValues();
	selected.union(new Set(other));
}
const arrow = (selected: Set<number>, other: Set<number>) => selected.union(new Set(other));
class ParameterProperties {
	constructor(private selected: Set<number>, readonly other?: ReadonlySet<number>) {
		selected.union(new Set(other));
	}
}

// Optional parameters
function optionalParameter(selected: Set<number>, other?: Set<number>) {
	selected.union(new Set(other));
	new Set(other).union(selected);
	new Set(other.union(selected));
	new Set(other!).union(selected);
}

// A `const` variable or an asserted value with another type is a `Set` if its value is one
function otherTypes(selected: Set<number>, other: Set<number>, values: number[]) {
	const widened: Iterable<number> = new Set(values);
	selected.union(new Set(widened));
	selected.union(new Set(other as Iterable<number>));
	selected.union(new Set(<Iterable<number>>other));
	selected.union(new Set(other satisfies Iterable<number>));
	selected.union(new Set(other as any));
	selected.union(new Set(other as const));
	selected.union(new Set(values as Set<number> as Iterable<number>));
	new Set(widened).union(other);
	new Set(selected).union(other as Iterable<number>);
}
function otherTypesInUnion(selected: Set<number>) {
	const optional: Set<number> | undefined = new Set();
	selected.union(new Set(optional));
}
function userDefinedTypeWithSetValue(selected: Set<number>) {
	type ReadonlySet<T> = T[];
	const other: ReadonlySet<number> = new Set([1]);
	selected.union(new Set(other));
}

// A function that is only created can't change the `Set`
function createdFunction(selected: Set<number>) {
	new Set(selected).union((() => selected.clear()) as unknown as Set<number>);
}
