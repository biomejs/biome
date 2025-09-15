type Foo<, A> = A;

type Foo<A, , B> = A & B;
