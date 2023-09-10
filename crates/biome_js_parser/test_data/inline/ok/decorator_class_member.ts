class Foo {
// properties
@dec foo = 2;
@dec @(await dec) @dec() foo = 2;
@dec public foo = 1;
@dec @(await dec) @dec() public foo = 1;
@dec static foo = 2;
@dec @(await dec) @dec() static foo = 2;
@dec accessor foo = 2;
@dec @(await dec) @dec() accessor foo = 2;
@dec readonly foo = 2;
@dec @(await dec) @dec() readonly foo = 2;
@dec override foo = 2;
@dec @(await dec) @dec() override foo = 2;
// methods
@dec foo() {}
@dec @(await dec) @dec() foo() {}
@dec public foo() {}
@dec @(await dec) @dec() public foo() {}
@dec static foo() {}
@dec @(await dec) @dec() static foo() {}
@dec override foo() {}
@dec @(await dec) @dec() override foo() {}
// getters
@dec get foo() {}
@dec @(await dec) @dec() get foo() {}
@dec public get foo() {}
@dec @(await dec) @dec() public get foo() {}
@dec static get foo() {}
@dec @(await dec) @dec() static get foo() {}
@dec override get foo() {}
@dec @(await dec) @dec() override get foo() {}
// setters
@dec set foo(val) {}
@dec @(await dec) @dec() set foo(val) {}
@dec public set foo(val) {}
@dec @(await dec) @dec() public set foo(val) {}
@dec static set foo(val) {}
@dec @(await dec) @dec() static set foo(val) {}
@dec override set foo(val) {}
@dec @(await dec) @dec() override set foo(val) {}
}
