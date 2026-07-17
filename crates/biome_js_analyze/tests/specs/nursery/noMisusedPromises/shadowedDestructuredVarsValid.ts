/* should not generate diagnostics */

// Shadowed destructured variable with shorthand return in nested function.
// The inner `bar` from `const { bar, baz } = obj` must not be confused with
// the outer `bar` from `const { bar, ...params } = something()`.
export function foo(_arg: string) {
  const { bar, ...params } = something();
  return console.log(bar, params);

  function something() {
    const obj: Record<string, string> = { bar: "bar" };
    const { bar, baz } = obj;
    return { bar, baz };
  }
}

// Same scenario but with renamed destructuring (never triggered the bug).
export function fooRenamed(_arg: string) {
  const { bar: beer, ...params } = something();
  return console.log(beer, params);

  function something() {
    const obj: Record<string, string> = { bar: "bar" };
    const { bar, baz } = obj;
    return { bar, baz };
  }
}
