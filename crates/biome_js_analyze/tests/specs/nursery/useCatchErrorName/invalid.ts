/* should generate diagnostics */
promise.catch(function (err: Error) {
	console.log(err);
});

promise.catch((err: unknown) => {
	console.log(err);
});

try {} catch (err: unknown) {
	console.log(err);
}
