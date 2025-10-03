type Foo = Bar<A,>;
type Foo = Bar<A, >;

type Foo = Bar<1, 2,>;
type Foo = Bar<1, 2, >;

type C1 = Bar<A, /* tail */>;
type C2 = Bar<1, 2, /* tail */>;

type M1 = Bar<
    A, // tail
>;
type M2 = Bar<
    1,
    2, // tail
>;
