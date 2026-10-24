/* should generate diagnostics */
for (const item of items) {
	items.push(item);
}

for (const item of items) {
	items.unshift(item);
}

for (const item of items) {
	items.pop();
}

for (const item of items) {
	items.shift();
}

for (const item of items) {
	items.splice(0, 1);
}

for (const item of items) {
	items.sort();
}

for (const item of items) {
	items.reverse();
}

for (const item of items) {
	items.fill(item);
}

for (const item of items) {
	items.copyWithin(0, 1);
}

for (const item of items) {
	items?.push(item);
}

for (const item of items) {
	items.push?.(item);
}

for (const item of items) {
	items["push"](item);
}

for (const item of items.values()) {
	items.push(item);
}

for (const item of items["values"]()) {
	items.push(item);
}

for (const index of items.keys()) {
	items.pop();
}

for (const [index, item] of items.entries()) {
	items.push(item);
}

for (const item of this.items) {
	this.items.push(item);
}

for (const item of object.items) {
	object.items.push(item);
}

for (const item of collections[index]) {
	collections[index].push(item);
}

for (const item of (items)) {
	(items).push(item);
}

for (const item of items) {
	if (condition) {
		items.push(item);
	}
}

for (const value of set) {
	set.add(value + 1);
}

for (const value of set) {
	set.clear();
}

for (const value of set.values()) {
	set.add(otherValue);
}

// Without a known `Set`, `values()` may be the values of a `Map`, deleted by key.
for (const value of set.values()) {
	set.delete(value);
}

{
	const set = new Set();
	for (const value of set.values()) {
		set.delete(value);
		set.add(value);
	}
}

for (let value of set) {
	set.add(value);
}

for (let value of set) {
	set.delete(value);
}

for (value of set) {
	set.add(value);
}

for (const value of set) {
	set.delete(value);
	set.add(value);
}

for (const value of set) {
	{
		set.delete(value);
	}
	set.add(value);
}

for (const value of set.keys()) {
	set.delete(value);
	set.add(value);
}

for (const [value] of set.entries()) {
	set.delete(value);
	set.add(value);
}

for (const value of set) {
	if (shouldDelete) {
		set.delete(value);
	}
	set.add(value);
}

for (const value of set) {
	if (condition) {
		foo();
	} else {
		set.delete(value);
	}
	set.add(value);
}

for (const value of set) {
	{
		const value = otherValue;
		set.add(value);
	}
}

for (const [key, value] of map) {
	map.set(otherKey, value);
}

{
	const map = new Map();
	for (const entry of map) {
		map.set(entry, value);
	}
}

{
	const set = new Set([["value"]]);
	for (const [value] of set) {
		set.delete(value);
	}
}

for (const key of map.keys()) {
	map.set(otherKey, value);
}

for (let key of map.keys()) {
	map.set(key, value);
}

for (const [key, value] of map) {
	map.delete(key);
	map.set(key, value);
}

for (const key of map.keys()) {
	map.delete(key);
	map.set(key, value);
}

for (const [key, value] of map.entries()) {
	map.delete(key);
	map.set(key, value);
}

for (const [key, value] of map) {
	switch (kind) {
		case 1:
			map.delete(key);
			map.set(key, value);
	}
}

for (const value of map.values()) {
	map.clear();
}

for (const value of map.values()) {
	map.delete(value);
}

for (const value of map.values()) {
	map.set(value, newValue);
}

for (const item of items) {
	for (const otherItem of otherItems) {
		items.push(otherItem);
	}
}

for (const item of items) {
	for (const otherItem of items) {
		items.push(otherItem);
	}
}

for (const value of set) {
	for (const innerValue of set) {
		set.delete(innerValue);
	}
}

for (const [key, value] of map) {
	for (const [innerKey] of map) {
		map.delete(innerKey);
	}
}

// A `break` only leaves the `switch`, so the code after it still runs.
for (const value of set) {
	switch (kind) {
		case "delete":
			set.delete(value);
			break;
	}
	set.add(value);
}

for (const value of set) {
	switch (kind) {
		case "delete": {
			set.delete(value);
			break;
		}
	}
	set.add(value);
}

for (const value of set) {
	switch (kind) {
		case "delete":
			set.delete(value);
		case "other":
			break;
	}
	set.add(value);
}

for (const value of set) {
	switch (kind) {
		case "delete":
			set.delete(value);
	}
	set.add(value);
}

for (const x of set) {
	try {
		set.delete(x);
	} catch {}
	set.add(x);
}

for (const x of set) {
	label: {
		set.delete(x);
	}
	set.add(x);
}

for (const x of set) {
	try {
		foo(x);
	} catch {
		set.delete(x);
	}
	set.add(x);
}

for (const x of set) {
	try {
		foo(x);
	} finally {
		set.delete(x);
	}
	set.add(x);
}

for (const x of set) {
	try {
		set.delete(x);
	} finally {
		events.push("final");
		while (true) {
			break;
		}
	}
	set.add(x);
}

for (const x of set) {
	label: set.delete(x);
	set.add(x);
}

// Only the `try` block jumps, so the `catch` block can still delete the item.
for (const x of set) {
	try {
		set.delete(x);
		continue;
	} catch {
		set.delete(x);
	}
	set.add(x);
}

for (const x of set) {
	try {
		set.delete(x);
	} catch {}
	if (condition) {
		set.add(x);
	}
}

for (const item of items) items.push(item);

// `Set` here is a local class, so its methods aren't known.
{
	class Set {}
	const set = new Set();
	for (const value of set.values()) {
		set.delete(value);
	}
}

// `break label` leaves only the labeled block, so the code after it still runs.
for (const x of set) {
	label: {
		set.delete(x);
		break label;
	}
	set.add(x);
}

// The `catch` block handles the error, so the code after the `try` statement runs.
for (const x of set) {
	try {
		set.delete(x);
		throw new Error("failure");
	} catch {}
	set.add(x);
}

// `continue` runs the `finally` clause first.
for (const x of set) {
	try {
		set.delete(x);
		continue;
	} finally {
		set.add(x);
	}
}

// The inner loop can run again after it deletes the item.
for (const x of set) {
	while (condition) {
		set.add(x);
		set.delete(x);
	}
}

for (const value of set) {
	set.delete(value), set.add(value);
}

for (const x of set) {
	if (set.delete(x)) set.add(x);
}

for (const x of set) {
	const removed = set.delete(x);
	set.add(x);
}

for (const x of set) {
	set.delete(x);
	for (const y of [set.add(x)]) {
	}
}

for (const x of set) {
	set.delete(x);
	const [first = set.add(x)] = list;
}

for (const x of set) {
	set.delete(x) ? set.add(x) : undefined;
}
