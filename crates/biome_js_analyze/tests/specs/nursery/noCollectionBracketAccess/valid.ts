/* should not generate diagnostics */
function read(map: Map<string, number>) {
	return map["size"];
}
function either(collection: Set<string> | Map<string, number>) {
	return collection["foo"];
}
function record(object: Record<string, number>) {
	return object["foo"];
}
declare const map: Map<string, number>;
map[("size" as string)];
