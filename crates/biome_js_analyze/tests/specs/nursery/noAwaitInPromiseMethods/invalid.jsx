/* should generate diagnostics */
Promise.all([<div>{await promise}</div>, anotherPromise]);
Promise.all([<Foo bar={await promise} />, anotherPromise]);
