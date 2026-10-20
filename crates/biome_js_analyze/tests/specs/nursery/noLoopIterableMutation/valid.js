/* should not generate diagnostics */
for (const item of items) {
	otherItems.push(item);
}

for (const item of [...items]) {
	items.push(item);
}

for (const item of Array.from(items)) {
	items.push(item);
}

for (const key of Object.keys(object)) {
	delete object[key];
}

for (const [key] of Object.entries(object)) {
	delete object[key];
}

for (const item of items) {
	function later() {
		items.push(item);
	}
}

for (const item of items) {
	const later = () => {
		items.push(item);
	};
}

for (const item of items) {
	const handlers = {
		run() {
			items.push(item);
		},
	};
}

for (const item of items) {
	class Later {
		method() {
			items.push(item);
		}
	}
}

for (const item of items) {
	{
		const items = [];
		items.push(item);
	}
}

for (const item of object.items) {
	{
		const object = { items: [] };
		object.items.push(item);
	}
}

for (const item of object.items) {
	{
		const object = { items: [] };
		object["items"].push(item);
	}
}

for (const item of collections[index]) {
	{
		const index = otherIndex;
		collections[index].push(item);
	}
}

for (const item of items) {
	items[method](item);
}

for (const item of items) {
	items.notMutating(item);
}

{
	const items = [];
	for (const item of items) {
		items.add(other);
	}
}

{
	const set = new Set();
	for (const value of set) {
		set.push(other);
	}
}

{
	const map = new Map();
	for (const [key] of map) {
		map.add(other);
	}
}

for (const item of items.values(argument)) {
	items.push(item);
}

for (const item of items?.values()) {
	items.push(item);
}

for (const item of items.values?.()) {
	items.push(item);
}

async function iterate() {
	for await (const item of items) {
		items.push(item);
	}
}

for (const item of items) {
	items = [];
}

for (const value of set) {
	set.add(value);
}

for (const value of set) {
	set.delete(value);
}

for (const value of set.values()) {
	set.add(value);
}

{
	const set = new Set();
	for (const value of set.values()) {
		set.delete(value);
	}
}

for (const value of set["values"]()) {
	set.add(value);
}

for (const value of set.keys()) {
	set.add(value);
}

for (const [value] of set.entries()) {
	set.add(value);
}

for (const [value] of set.entries()) {
	set.delete(value);
}

for (const key of set.keys()) {
	set.delete(key);
}

for (const [value] of set) {
	set.delete(value);
}

for (const value of set) {
	if (shouldDelete) {
		set.delete(value);
	} else {
		set.add(value);
	}
}

for (const value of set) {
	if (shouldDelete) {
		set.delete(value);
		continue;
	}
	set.add(value);
}

for (const value of set) {
	set.add(value), set.delete(value);
}

for (const key of map.keys()) {
	map.set(key, value);
}

for (const key of map.keys()) {
	map.delete(key);
}

for (const [key] of map) {
	map.set(key, value);
}

for (const [key] of map) {
	map.delete(key);
}

for (const [key] of map.entries()) {
	map.set(key, value);
}

for (const [key] of map["entries"]()) {
	map.delete(key);
}

// The whole `[key, value]` pair is never a key, so deleting it removes nothing.
for (const entry of map) {
	map.delete(entry);
}

{
	const map = new Map();
	for (const entry of map.entries()) {
		map.delete(entry);
	}
}

for (const key of map.keys()) {
	if (shouldDelete) {
		map.delete(key);
	} else {
		map.set(key, value);
	}
}

for (const [key, value] of map) {
	if (shouldDelete) {
		map.delete(key);
		continue;
	}
	map.set(key, value);
}

for (const value of set) {
	switch (kind) {
		case "delete":
			set.delete(value);
			continue;
	}
	set.add(value);
}

for (const value of set) {
	switch (kind) {
		case "delete":
			set.delete(value);
		case "other":
			continue;
	}
	set.add(value);
}

{
	const iterator = items.values();
	for (const item of iterator) {
		items.push(item);
	}
}

for (const value of set) {
	for (const innerValue of set) {
		set.add(innerValue);
	}
}

for (const [key, value] of map) {
	for (const [innerKey, innerValue] of map) {
		map.set(innerKey, innerValue);
	}
}

for (const value of foo.toArray()) {
	foo.push(value);
}

for (const value of getSet().keys()) {
	getSet().add(value);
}

for (const value of array) {
	foo(value);
}

for (const value of set) set.add(value);

for (const value of set) {
	let count;
	count = 1;
	set.add(value);
}

for (const value of set) {
	if (condition) {
		foo();
	} else {
		continue;
	}
	set.add(value);
}

for (const value of set) {
	try {
		set.delete(value);
	} finally {
		continue;
	}
	set.add(value);
}

for (const value of set) {
	try {
		throw 1;
	} catch {
		set.delete(value);
	} finally {
		continue;
	}
	set.add(value);
}

for (const x of set) {
	try {
		foo(x);
	} catch {
		set.delete(x);
		continue;
	}
	set.add(x);
}

for (const x of set) {
	label: {
		set.delete(x);
		continue;
	}
	set.add(x);
}

for (const x of set) {
	try {
		foo(x);
	} catch (error) {
		set.delete(x);
		throw error;
	}
	set.add(x);
}

for (const x of set) {
	try {
		set.delete(x);
		break;
	} finally {
		foo(x);
	}
	set.add(x);
}

for (const x of set) {
	try {
		set.delete(x);
	} catch {}
}

function leave(set) {
	for (const x of set) {
		if (condition) {
			set.delete(x);
			return;
		}
		set.add(x);
	}
}

for (const x of set) {
	switch (kind) {
		case "delete":
			set.delete(x);
			if (condition) {
				continue;
			} else {
				throw new Error("failure");
			}
	}
	set.add(x);
}

for (const x of set) {
	try {
		set.delete(x);
		continue;
	} catch {}
	set.add(x);
}

for (const x of set) {
	try {
		set.delete(x);
		break;
	} finally {
		foo(x);
	}
	set.add(x);
}

// The next pass of the loop has a different current item.
for (const x of set) {
	set.add(x);
	set.delete(x);
}

for (const x of set) {
	shouldRemove(x) ? set.delete(x) : set.add(x);
}

for (const [key, value] of map) {
	const next = isStale(key) ? map.delete(key) : map.set(key, value + 1);
}

// The inner loop reads its collection once, before it deletes anything.
for (const x of set) {
	for (const y of [set.add(x)]) {
		set.delete(x);
	}
}
