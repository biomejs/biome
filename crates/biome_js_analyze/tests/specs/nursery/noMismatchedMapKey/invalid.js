/* should generate diagnostics */
map.has(key) ? map.get(anotherKey) : value;
map.has(key) ? map.set(anotherKey, value) : value;
map.has(key) ? value : map.get(anotherKey);
!map.has(key) ? value : map.get(anotherKey);
if (weakMap.has(key)) { weakMap.get(anotherKey); }
if (map.has(key)) { const value = map.get(anotherKey); }
if (map.has(key)) { map.set(anotherKey, value); }
if (!map.has(key)) { const value = map.get(anotherKey); }
if (!map.has(key)) { map.set(anotherKey, value); }
if (map.has(key)) {} else { map.get(anotherKey); }
if (!map.has(key)) {} else { map.get(anotherKey); }
if (map.has(key)) map.get(anotherKey);
if ((map.has(key))) { (map).get(anotherKey); }

// Nested code that still runs because of the check
if (map.has(key)) { for (const item of items) { map.get(anotherKey); } }
map.has(key) ? (map.has(key) ? map.get(anotherKey) : value) : value;
if (map.has(key)) { if (condition) { map.get(anotherKey); } }
if (map.has(key)) { condition ? map.get(anotherKey) : value; }
if (map.has(key)) { if (check(() => map.has(anotherKey))) { map.get(anotherKey); } }
if (map.has(key)) { map.get(map.get(anotherKey)); }

// Different keys
if (map.has(key)) { const key = anotherKey, value = map.get(key); }
if (map.has(object.key)) { const object = other, value = map.get(object.key); }
if (map.has("key")) { map.get("anotherKey"); }
if (map.has(1)) { map.get(2); }
if (map.has(1)) { map.get("1"); }
if (map.has(object.key)) { map.get(object.anotherKey); }
if (map.has(items[0])) { map.get(items[1]); }
if (this.map.has(this.key)) { this.map.get(this.anotherKey); }

// Checks with other conditions
if (condition && map.has(key)) { map.get(anotherKey); }
while (map.has(key)) { map.get(anotherKey); }
while (!map.has(key)) { map.set(anotherKey, value); }
while (map.has(key)) { const x = map.get(anotherKey); }
const a = map.has(key) && map.get(otherKey);
if (!map.has(key) && map.get(otherKey)) {}
const b = map.has(key) || map.get(otherKey);
if (map.has(key)) { map.has(otherKey) || map.get(anotherKey); }

// Assignments that don't replace the map
if (map.has(key)) { use([object.value] = values, map.get(anotherKey)); }
if (map.has(key)) { (count = 1, map.get(anotherKey)); }
if (map.has(key)) { (count++, map.get(anotherKey)); }
if (map.has(key)) { (map.size = 1, map.get(anotherKey)); }
