/* should not generate diagnostics */
// The check has to come before the call
const a = map.get(otherKey) && map.has(key);
const b = map.get(otherKey) || map.has(key);
do { map.get(anotherKey); } while (map.has(key));

// The check doesn't decide alone whether the branch runs
if (map.has(key) && condition) { map.get(anotherKey); }
if (condition || map.has(key)) { map.get(anotherKey); }
const c = map.has(key) ?? map.get(anotherKey);

// Same key
map.has(key) ? map.get(key) : value;
!map.has(key) ? map.set(key, value) : value;
const d = map.has(key) && map.get(key);
if (map.has(key)) { const value = map.get(key); }
if (!map.has(key)) { map.set(key, value); }
if (map.has(1)) { map.get(1.0); }
if (map.has("a")) { map.get('a'); }
if (map.has(object.key)) { map.get(object["key"]); }
if (map.has(NaN)) { map.get(NaN); }
if (this.map.has(this.key)) { this.map.get(this.key); }

// Different maps
if (map.has(key)) { otherMap.get(anotherKey); }
if (a.map.has(key)) { b.map.get(anotherKey); }

// Keys that can't be compared
if (map.has(key)) { map.get(getKey()); }
if (map.has(getKey())) { map.get(anotherKey); }
if (map.has(1 + 1)) { map.get(2); }
if (map.has(-0)) { map.get(0); }
if (map.has(String("key"))) { map.get("anotherKey"); }
if (map.has(true ? String("key") : "x")) { map.get("anotherKey"); }
if (map.has(key)) { map.get([anotherKey]); }
if (map.has(key)) { map.get(`anotherKey`); }
if (map.has(/a/)) { map.get(/b/); }
if (map.has(modes.size)) { map.get(1); }
if (map.has(key)) { map.get(items[getIndex()]); }
if (map.has(user?.id)) { map.get(user.id); }
if (map.has(user?.[0])) { map.get(user[0]); }

// Calls that aren't plain method calls
if (map.has(...key)) { map.get(anotherKey); }
if (map.has(key)) { map.set(anotherKey, ...values); }
if (map?.has(key)) { map.get(anotherKey); }
if (map.has?.(key)) { map.get(anotherKey); }
if (map[has](key)) { map.get(anotherKey); }
if (map.has(key)) { map[get](anotherKey); }
if (map.has(key)) { map?.get(anotherKey); }
if (map.has(key, other)) { map.get(anotherKey); }

// The map is a different variable inside the branch
if (map.has(key)) { const map = otherMap, value = map.get(anotherKey); }
if (cache.map.has(key)) { const cache = otherCache, value = cache.map.get(anotherKey); }
if (map.has(key)) { const {map} = cache, value = map.get(anotherKey); }
if (map.has(key)) { try {} catch (map) { map.get(anotherKey); } }
if (map.has(key)) {
	for (const map of maps) {
		map.get(anotherKey);
	}
}

// The branch assigns a new value to the map
if (map.has(key)) { for (map of maps) { map.get(anotherKey); } }
if (map.has(key)) { (map.get(anotherKey), map = otherMap); }
if (map.has(key)) { (map++, map.get(anotherKey)); }
if (cache.map.has(key)) { (cache.map = otherMap, cache.map.get(anotherKey)); }
if (cache.map.has(key)) { (cache = other, cache.map.get(anotherKey)); }
if (cache.map.has(key)) { for (cache.map of maps) { cache.map.get(anotherKey); } }
if (map.has(key)) { for ({map} of maps) { map.get(anotherKey); } }
if (map.has(key)) { use([, map] = values, map.get(anotherKey)); }
if (map.has(key)) { use({...map} = values, map.get(anotherKey)); }
if (map.has(key)) { use([map = fallback] = values, map.get(anotherKey)); }
if (map.has(key)) { use([...map] = values, map.get(anotherKey)); }
class A {
	f() {
		if (this.map.has(key)) { (this.map = other, this.map.get(anotherKey)); }
		if (this.map.has(key)) { for (this.map of maps) { this.map.get(anotherKey); } }
	}
}

// A nested check on the same map decides whether the call runs
if (map.has(key)) { if (map.has(anotherKey)) { map.get(anotherKey); } }
if (map.has(key)) { map.has(anotherKey) ? map.get(anotherKey) : value; }
if (map.has(key)) { if (!map.has(anotherKey)) { map.set(anotherKey, value); } }
if (map.has(key)) { if (condition && map.has(anotherKey)) { map.get(anotherKey); } }
if (map.has(key)) { condition && map.has(anotherKey) ? map.get(anotherKey) : value; }
if (map.has(key)) { while (map.has(anotherKey)) map.delete(map.get(anotherKey)); }
if (map.has(key)) { map.has(anotherKey) && map.get(anotherKey); }

// Code that may run later
if (map.has(key)) {
	function getValue() {
		return map.get(anotherKey);
	}
}
if (map.has(key)) {
	class Value {
		static value = map.get(anotherKey);
	}
}
const e = map.has(key) ? () => map.get(anotherKey) : value;
const f = map.has(key) ? { get value() { return map.get(anotherKey); } } : value;

// Branches that aren't checked
if (map.has(key)) {
	map.set(key, value);
	map.set(anotherKey, value);
}
if (map.has(key)) { map.get(key); } else if (condition) { map.get(anotherKey); }
while (map.has(key)) { map.get(anotherKey); next(); }
