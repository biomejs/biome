/* should generate diagnostics */
const selected = new Set([1, 2]);
const other = new Set([2, 3]);
const records = new Map([[2, "value"]]);

// Copies of the argument
selected.union(new Set(new Set([1, 2])));
selected.union((new Set(other)));
(selected.union)(new Set(other));
(selected).union(new Set((other)));
selected.union(new (Set)(other));
selected.union(new Set(other),);
selected.union(new Set(other,));
new Set([1, 2]).union(new Set(other));
selected.union(
	new Set(other),
);

// Copies of the `Set` the method is called on
new Set(condition ? other : selected).union(records);
(new Set(selected)).union(other);
(new Set(selected).union)(other);
new Set(new Set).union(other);
new Set(selected).union(condition ? other : selected);
const unionOfCopy = new Set(selected).union(other);

// The argument of the method has nothing that the rule treats as a change to the `Set`
new Set(selected).union(tag`${selected}` ? other : selected);
new Set(selected).union(`${selected}` ? other : selected);
new Set(selected).union(typeof counter ? other : selected);
new Set(selected).union((() => selected.clear()) ? other : selected);
new Set(selected).union((function () { selected.clear(); }) ? other : selected);
new Set(selected).union(((value = selected.clear()) => value) ? other : selected);
new Set(selected).union({ clear() { selected.clear(); } } ? other : selected);
new Set(selected).union({ get size() { return selected.clear(); } } ? other : selected);
new Set(selected).union({ set size(value = selected.clear()) {} } ? other : selected);
new Set(selected).union(class { clear(value = selected.clear()) { selected.clear(); } } ? other : selected);
new Set(selected).union(class { constructor(value = selected.clear()) { selected.clear(); } } ? other : selected);

// Copies of the result
new Set(selected.union(other)).has(2);
new Set((selected.union(other)));
new Set((selected).union(other));

// Copies of the result in the callee of `new`
new new Set(selected.union(other))["constructor"]();
new new Set((selected.union(other))).constructor();

// Variables that hold another variable
const alias = other;
const aliasOfAlias = alias;
new Set(aliasOfAlias).union(other);
new Set(aliasOfAlias.union(other));
const either = condition ? selected : aliasOfAlias;
either.union(new Set(other));
usedBeforeDeclaration.union(new Set(other));
const usedBeforeDeclaration = new Set();

// Comments
/* before */ new Set(selected.union(other)) /* after */;
selected.union(/* before copy */ new Set(other) /* after copy */);
