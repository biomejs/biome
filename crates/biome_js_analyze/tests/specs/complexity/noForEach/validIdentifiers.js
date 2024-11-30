Effect.forEach([1, 2, 3, 4, 5], (n) =>
	Console.log(`Current element: ${n}`).pipe(Effect.as(n * 2))
);

_.forEach([1, 2], function (value) {
	console.log(value);
});
