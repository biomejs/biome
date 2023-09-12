// array
(action) =>
	// eslint-disable-next-line react/no-array-index-key
	[<li />];

// function body
(action) =>
	// eslint-disable-next-line react/no-array-index-key
{
	return <li />;
}

// object expression
(action) =>
	// eslint-disable-next-line react/no-array-index-key
	({					a: 10
	});

	(action) => /* comment */ `
${test}
multiline`;
