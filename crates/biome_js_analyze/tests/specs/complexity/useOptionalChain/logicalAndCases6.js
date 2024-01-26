// case with a jump (i.e. a non-nullish prop)
foo != undefined && foo.bar != undefined && foo.bar.baz.buzz()
foo.bar != undefined && foo.bar.baz.buzz()

// case with a jump (i.e. a non-nullish prop)
foo != undefined && foo.bar != undefined && foo.bar.baz.buzz != undefined && foo.bar.baz.buzz()

// case with a call expr inside the chain for some inefficient reason
foo != undefined && foo.bar() != undefined && foo.bar().baz != undefined && foo.bar().baz.buzz != undefined && foo.bar().baz.buzz()

// chained calls with element access
foo != undefined && foo.bar != undefined && foo.bar.baz != undefined && foo.bar.baz[buzz]()
foo != undefined && foo.bar != undefined && foo.bar.baz != undefined && foo.bar.baz[buzz] != undefined && foo.bar.baz[buzz]()

// (partially) pre-optional chained
foo != undefined && foo?.bar != undefined && foo?.bar.baz != undefined && foo?.bar.baz[buzz] != undefined && foo?.bar.baz[buzz]()
foo != undefined && foo?.bar.baz != undefined && foo?.bar.baz[buzz]
foo != undefined && foo?.() != undefined && foo?.().bar
foo.bar != undefined && foo.bar?.() != undefined && foo.bar?.().baz

//private static member name
foo && foo.#bar
foo.#bar && foo.#bar.#baz
foo.#bar && foo.#bar()
foo && foo.#bar && foo.#bar.#baz && foo.#bar.#baz.#buzz
foo.#bar && foo.#bar.#baz && foo.#bar.#baz.#buzz

// two  errors
foo && foo.bar && foo.bar.baz || baz && baz.bar && baz.bar.foo

// case with inconsistent checks
foo && foo.bar != null && foo.bar.baz !== undefined && foo.bar.baz.buzz;

foo.bar && foo.bar.baz != null && foo.bar.baz.qux !== undefined && foo.bar.baz.qux.buzz;

// ensure essential whitespace isn't removed
foo && foo.bar(baz => <This Requires Spaces />);
foo && foo.bar(baz => typeof baz);
foo && foo["some long string"] && foo["some long string"].baz
foo && foo[`some long string`] && foo[`some long string`].baz
foo && foo['some long string'] && foo['some long string'].baz;

// other literal expressions
foo && foo[123] && foo[123].baz;
foo && foo[true] && foo[true].baz;
foo && foo[null] && foo[null].baz;
foo && foo[12n] && foo[12n].baz;
foo && foo[/\w+/] && foo[/\w+/].baz;


// should preserve comments in a call expression
foo && foo.bar(/* comment */a,
	// comment2
	b, );

// other weird cases
foo && foo?.();
foo.bar && foo.bar?.();

// comments
foo && foo.bar && /*0*/foo/*1*/./*2*/bar/*3*/./*4*/baz/*5*/;
foo && foo[bar] && /*0*/foo/*1*/[/*2*/bar/*3*/]/*4*/[/*5*/baz/*6*/]/*7*/;

foo && foo[bar] && /*0*/foo/*1*/?./*2*/[/*3*/bar/*4*/]/*5*/?./*6*/[/*7*/baz/*8*/]/*9*/;

// call expressions with the same member name but different arguments
foo && foo.bar('a') && foo.bar('b')
