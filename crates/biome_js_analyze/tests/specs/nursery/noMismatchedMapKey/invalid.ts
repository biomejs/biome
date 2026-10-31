/* should generate diagnostics */
if (map.has(key as string)) { map.get(anotherKey as string); }
if ((map as Map<string, number>).has(key)) { map!.get(anotherKey); }
function f(map: Map<string, number>) { if (map.has("a")) { return map.get("b"); } }
