// Pattern 1: Explicit strict checks
if (foo === null || foo === undefined) {
  foo = makeFoo();
}

if (foo === undefined || foo === null) {
  foo = makeFoo();
}

// Pattern 2: Loose equality check
if (foo == null) {
  foo = makeFoo();
}

// Pattern 3: Multiline check
if (
  result === null ||
  result === undefined
) {
  result = computeResult();
}
