// =====================
// Single type argument (inline branch)
// =====================

// Test 1: Single type argument - reference type
foo<T>();

// Test 2: Single type argument - primitive type
foo<number>();

// Test 3: Single type argument - object type
foo<{ a: T }>();

// Test 4: Single type argument - mapped type
foo<{ [K in keyof T]: U }>();

// Test 5: Method call with type argument
a.foo<T>();

// Test 6: Chained generic calls
a.foo<T>().bar<U>();

// Test 7: Class instantiation
new Foo<T>();

// Test 8: Nested generic - outer multi-line, inner gets spaces
foo<Bar<T>>();

// Test 9: Boundary - long object type that causes line break
foo<{ aaaaaaaaaaaaaaaaaaaaa: T; bbbbbbbbbbbbbbbbbbbbb: U; ccccccccccccccccccccc: V }>();

// Test 10: Boundary - object type that fits with spaces
foo<{ a: T; b: U }>();

// =====================
// Multiple type arguments (multi-line branch)
// =====================

// Test 1: Multiple type arguments - spaces added
foo<T, U>();

// Test 2: Method call with type arguments - spaces added
a.foo<T, U, V>();

// Test 3: Class instantiation with type arguments - spaces added
new Foo<T, U>();

// Test 4: Long type arguments that break - no spaces in multi-line
foo<Loooooooooooooooooooooooooong, Loooooooooooooooooooooooooong, Loooooooooooooooooooooooooong>();

// Test 5: Arrow function with type annotation - spaces added
const a: Foo<T, U, V> = () => {};

// Test 6: Arrow function with long types that breaks - no spaces
const b: Foo<Loooooooooooooooooooooooooong, Loooooooooooooooooooooooooong> = () => {};

// Test 7: Boundary - fits on line (79 chars, becomes 81 with spaces -> breaks)
foo<Aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa, Aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa>();

// Test 8: Boundary - under limit (78 chars, becomes 80 with spaces -> fits)
foo<Aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa, Aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa>();

// Test 9: Chained generic calls
a.foo<T, U>().bar<V, R>();
