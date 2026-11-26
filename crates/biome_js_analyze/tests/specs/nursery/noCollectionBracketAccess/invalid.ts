/* should generate diagnostics */
function read(map: Map<string, number>) {
	return map["foo"];
}
function write(map: Map<string, number>) {
	map["foo"] = 1;
}
function writeReadonly(map: ReadonlyMap<string, number>, set: ReadonlySet<string>) {
	map["foo"];
	set["foo"];
}
function writeWeak(map: WeakMap<object, number>, set: WeakSet<object>) {
	map[key] = 1;
	set[key];
}
declare const declared: Map<string, number>;
(declared as Map<string, number>)["foo"];
declared!["foo"];
declared[("foo" as string)] = 1;
// A union with a weak collection type of another kind uses the initial value
const mixed: Set<string> | WeakSet<object> = new Set();
mixed["foo"];
