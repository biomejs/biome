/* should generate diagnostics */
{
	var foo = array[0];
}
{
	foo = array[0];
}
{
	var foo = object.foo;
}
{
	var foo = (a, b).foo;
}
{
	var length = (() => {}).length;
}
{
	var foo = (a = b).foo;
}
{
	var foo = (a || b).foo;
}
{
	var foo = f().foo;
}
{
	var foo = object.bar.foo;
}
{
	var foo = object['foo'];
}
{
	foo = object.foo;
}
{
	foo = object['foo'];
}
{
	class Foo extends Bar {
		static foo() {
			var bar = super.foo.bar;
		}
	}
}

{
	var /* comment */ foo = object.foo;
}
{
	var a,
		/* comment */ foo = object.foo;
}
{
	var foo /* comment */ = object.foo;
}
{
	var a,
		foo /* comment */ = object.foo;
}
{
	var foo /* comment */ = object.foo,
		a;
}
{
	var foo = object.foo; /* comment */
}
{
	var foo = object.foo,
		/* comment */ a;
}
{
	var foo = bar(/* comment */).foo;
}
{
	var foo = bar /* comment */.baz.foo;
}
{
	var foo = bar[baz].foo;
}
{
	var foo = object.foo /* comment */,
		a;
}
{
	var foo = object.foo,
		/* comment */ a;
}
