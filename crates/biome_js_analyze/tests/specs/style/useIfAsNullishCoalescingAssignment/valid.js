/* should not generate diagnostics */

// Already using ??=
foo ??= makeFoo();

// Has else clause
if (!foo) {
  foo = makeFoo();
} else {
  doSomething();
}

// Multiple statements in body
if (!foo) {
  console.log('Initializing');
  foo = makeFoo();
}

// Different variable in check vs assignment
if (!foo) {
  bar = makeFoo();
}

// Non-nullish condition
if (foo === 0) {
  foo = 1;
}

if (foo < 10) {
  foo = 10;
}

// Assignment with operator other than =
if (!foo) {
  foo += 1;
}

// No assignment in body
if (!foo) {
  return;
}

if (!foo) {
  throw new Error('Missing foo');
}

// Complex condition (not simple nullish check)
if (!foo && bar) {
  foo = makeFoo();
}

// Assignment is not to the same variable structure
if (!foo.bar) {
  foo.baz = value;
}
