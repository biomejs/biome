/* should generate diagnostics */
try {} catch (error) {
	console.log(error);
}

obj.catch((error) => error.stack);

obj.then(undefined, (error) => error.stack);

obj?.then(undefined, (error) => error.stack);

try {} catch (descriptiveError) {}
