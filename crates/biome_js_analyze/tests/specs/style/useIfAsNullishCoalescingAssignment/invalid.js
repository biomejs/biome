// Pattern 1: Simple negation check
if (!foo) {
  foo = makeFoo();
}

// Pattern 2: Explicit strict checks
if (foo === null || foo === undefined) {
  foo = makeFoo();
}

if (foo === undefined || foo === null) {
  foo = makeFoo();
}

// Pattern 3: Loose equality check
if (foo == null) {
  foo = makeFoo();
}

// Pattern 4: Object property
if (!obj.prop) {
  obj.prop = getValue();
}

// Pattern 5: Nested property
if (!user.settings.theme) {
  user.settings.theme = 'light';
}

// Pattern 6: Array element
if (!arr[0]) {
  arr[0] = fallback;
}

// Pattern 7: Computed property
if (!obj[key]) {
  obj[key] = defaultValue;
}

// Pattern 8: Without braces (single statement)
if (!config) config = {};

// Pattern 9: Complex expressions in assignment
if (!cache.data) {
  cache.data = fetchData();
}

// Pattern 10: Multiline check
if (
  result === null ||
  result === undefined
) {
  result = computeResult();
}
