/* should not generate diagnostics */

// issue #7812: a runtime variable should not shadow a type-only declaration
type Story = unknown;
function render(Story: string) {}

// Type alias shadowed by function parameter
type Foo = number;
function f(Foo: string) {}

// Interface shadowed by variable
interface Bar {
    prop: number;
}
function g() {
    const Bar = "test";
}

// Type alias shadowed by variable in nested scope
type Baz = string;
{
    const Baz = 42;
}
