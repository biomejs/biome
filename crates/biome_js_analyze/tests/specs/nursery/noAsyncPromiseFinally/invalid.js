/* should generate diagnostics */
promise.finally(async () => {});
promise.finally(async () => cleanup());
promise.finally(async () => {
	await cleanup();
});
promise.finally(async function () {
	await cleanup();
});
Promise.resolve(value).finally(async () => cleanup());
new Promise((resolve) => resolve()).finally(async () => cleanup());
promise?.finally(async () => {});
promise.finally?.(async () => {});
promise["finally"](async () => {});
promise[`finally`](async () => {});
promise.finally((async () => {}));
(promise.finally)(async () => {});

async function cleanupDeclaration() {}
promise.finally(cleanupDeclaration);

const cleanupArrow = async () => {};
promise.finally(cleanupArrow);

const cleanupFunctionExpression = async function () {};
promise.finally(cleanupFunctionExpression);

promise.finally(hoistedCleanup);
async function hoistedCleanup() {}

const recursive = async function cleanupItself() {
	promise.finally(cleanupItself);
};

const method = "finally";
promise[method](async () => {});
