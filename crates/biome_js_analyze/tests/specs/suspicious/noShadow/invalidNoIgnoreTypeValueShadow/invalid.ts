/* should generate diagnostics */

// With ignoreTypeValueShadow: false, these should be flagged
type Foo = number;
function f(Foo: string) {}

interface Bar {
    prop: number;
}
function g() {
    const Bar = "test";
}

type Baz = string;
{
    const Baz = 42;
}
