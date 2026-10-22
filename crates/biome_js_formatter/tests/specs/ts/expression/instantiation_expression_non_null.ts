// Without the parentheses, `make<T>!` and `extends B<T><T>` don't parse.
const a = (make<T>)!;
(make<T>)!.b;
(make<T>)!();
y = (make<T>)!.c<U>;
class D extends (B<T>)<T> {}
(class E extends (B<T>)<T> {});

// These don't need them.
class C extends (B<T>) {}
x = make<T>;
