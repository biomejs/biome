/* should generate diagnostics */
Promise.all([await promise]);
Promise.allSettled([await promise]);
Promise.any([await promise]);
Promise.race([await promise]);
Promise.all([, await promise]);
Promise.all([await promise,]);
Promise.all([await promise],);
Promise.all([await (0, promise)]);
Promise.all([await (( promise ))]);
Promise.all([await await promise]);
Promise.all([...foo, await promise1, await promise2]);
Promise.any([await a, await b, await c]);
Promise.all([await /* comment */ promise]);
Promise.all([/* before */ await promise /* after */]);
Promise.all([await // comment
	promise]);
Promise.all([
	await
		promise,
	anotherPromise,
]);
Promise.all([(await promise)]);
Promise.all(([await promise]));
(Promise.all)([await promise]);
Promise["all"]([await promise]);
globalThis.Promise.all([await promise]);

// Nested `await`
Promise.all([(await promise, 0)]);
Promise.all([(await promise).foo, anotherPromise]);
Promise.all([foo ? await promise : anotherPromise, anotherPromise]);
Promise.all([foo && await promise, anotherPromise]);
Promise.all([fetch(await getUrl()), anotherPromise]);
Promise.all([...await promises, anotherPromise]);
Promise.all([{foo: await promise}]);
Promise.all([[await promise]]);
Promise.all([(await promise1) + (await promise2)]);
Promise.all([await (await promise).foo]);
Promise.all([Promise.all([await promise]), anotherPromise]);
Promise.all([async () => {}, (await promise).foo]);
Promise.all([(await promise)?.foo, anotherPromise]);
Promise.all([tag`${await promise}`, anotherPromise]);
Promise.all([`${await promise}`, anotherPromise]);
Promise.all([new Foo(await bar)]);
Promise.all([foo[await bar]]);
Promise.all([(await promise)()]);
Promise.all([foo ?? await bar]);
Promise.all([!await foo, anotherPromise]);
Promise.all([(foo = await bar)]);
Promise.all([import(await getPath()), anotherPromise]);
Promise.all([{[await key]: value}]);
Promise.all([{async [await key]() {}}]);
Promise.all([class { [await key]() {} }]);
Promise.all([class extends (await Base) {}]);
Promise.all([foo([await promise])]);

// Only the outer `await` is reported
Promise.all([await foo(await promise)]);

// Each `Promise` method call is reported
Promise.all([await Promise.all([await promise])]);

async function foo() {
	return Promise.all([(await promise).foo, anotherPromise]);
}
