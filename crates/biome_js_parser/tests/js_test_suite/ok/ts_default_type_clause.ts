type A<X=string> = X;
type B<X extends number | string = string> = { a: X }
