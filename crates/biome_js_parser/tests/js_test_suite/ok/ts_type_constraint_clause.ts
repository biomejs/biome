type A<X extends number> = X;
type B<X extends number | string> = { a: X }
