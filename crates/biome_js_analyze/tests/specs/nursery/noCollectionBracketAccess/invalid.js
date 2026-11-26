/* should generate diagnostics */

// Read
const map = new Map();
map["foo"];
map[key];
map[0];
const value = map["foo"];
if (map["foo"]) {}
map?.["foo"];
(map)["foo"];
map["foo"].bar;
map[`${prefix}foo`];
map[`foo`];

const set = new Set();
set["foo"];
const weakMap = new WeakMap();
weakMap[key];
const weakSet = new WeakSet();
weakSet[key];
weakMap[Symbol.iterator];

// Keys that may not be a property
const symbol = Symbol("x");
map[symbol];
map[Symbol.for("foo")];
map[Symbol.asyncIterator];
const fooKey = "foo";
map[fooKey];
set[condition ? "add" : "foo"]("foo");
map[key || "get"];
map[(0, "foo")];
map["fo\x6f"];
map[+(condition ? "size" : "get")];

// `Symbol` is a local variable, not the global one
function shadowedSymbol(Symbol) {
	map[Symbol.iterator];
}

// Write
map["foo"] = "bar";
map[key] = value;
weakMap[key] = value;
map["foo"] += 1;
map["foo"]++;
++map["foo"];
map["foo"] ??= "bar";
[map["foo"]] = array;
({ a: map["foo"] } = object);
for (map["foo"] of array) {}
for (map["foo"] in object) {}
map["foo"] = (a, b);
map[a, b] = d;
map[(key)] = value;
set["foo"] = 1;
weakSet[key] = 1;

// Delete
delete map["foo"];
delete set["foo"];
delete weakMap[key];
delete weakSet[key];
delete map[a, b];
delete map?.["foo"];
delete (map["foo"]);

// No fix: the value of the assignment or `delete` is used
const x = (map["foo"] = "bar");
if (delete map["foo"]) {}
(delete map["foo"], 1);

// No fix: comments inside
map["foo"] = /* keep */ "bar";
delete map[/* keep */ "foo"];
map /* keep */ ["foo"] = "bar";

// No fix: the object isn't a variable
(condition ? map : map)["foo"] = "bar";
new Map()["foo"] = "bar";

// Nested: both maps are reported
const keys = new Map();
map[keys["foo"]];

// The fix keeps the comments around the statement
/* before */ map["foo"] = "bar"; // after
