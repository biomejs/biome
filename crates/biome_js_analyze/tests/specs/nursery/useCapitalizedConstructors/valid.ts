/* should not generate diagnostics */
const a = new Foo<string>();
const b = foo<string>();
const c = (Foo<string>)();
const d = Foo!();
