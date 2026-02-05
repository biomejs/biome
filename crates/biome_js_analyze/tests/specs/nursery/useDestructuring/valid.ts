/* should not generate diagnostics */

// Type annotation on variable — should not suggest destructuring
{
	const foo: string = object.foo;
}
{
	let foo: number = array[0];
}
{
	var foo: string = object['foo'];
}
{
	const foo: Foo = object.bar.foo;
}

// Definite assignment annotation — should not suggest destructuring
{
	let foo!: string;
}

// as const with type annotation
{
	const y: number = x.y;
}
