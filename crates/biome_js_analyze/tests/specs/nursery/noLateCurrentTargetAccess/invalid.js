/* should generate diagnostics */

// After `await`
element.addEventListener("click", async (event) => {
	await somePromise;
	event.currentTarget.disabled = false;
});

// After `yield` in a generator handler
element.onclick = function* (event) {
	yield somePromise;
	event.currentTarget.disabled = false;
};

// Short parameter name
element.onclick = async (e) => {
	await somePromise;
	e.currentTarget.foo();
};

// `evt` parameter name
element.onclick = async (evt) => {
	await somePromise;
	evt.currentTarget.foo();
};

// camelCase parameter name ending with `Event`
element.onclick = async (mouseDownEvent) => {
	await somePromise;
	mouseDownEvent.currentTarget.foo();
};

// An earlier awaited argument pauses before a later argument in the same call
element.onclick = async (event) => {
	log(await ready(), event.currentTarget.value);
};

// Only the read after the `await` is reported
element.onclick = async (event) => {
	event.currentTarget.disabled = true;
	await somePromise;
	event.currentTarget.disabled = false;
};

// Nested function with an expression body
element.onclick = (event) => setTimeout(() => event.currentTarget.click());

// Nested function with a block body
element.onclick = (event) => {
	setTimeout(() => {
		event.currentTarget.remove();
	}, 1);
};

// Nested function in a `.then()` callback
element.onclick = (event) => {
	fetch(url).then(() => event.currentTarget.foo());
};

// Nested function expression
element.onclick = (event) => {
	setTimeout(function () {
		event.currentTarget.foo();
	});
};

// Optional chaining after `await`
async (event) => {
	await somePromise;
	event?.currentTarget.foo();
};

// Parenthesized object after `await`
async (event) => {
	await somePromise;
	(event).currentTarget.foo();
};

// Named function declaration
async function handleClick(event) {
	await somePromise;
	event.currentTarget.disabled = false;
}

// Class method
class Component {
	async onClick(event) {
		await this.save();
		event.currentTarget.disabled = false;
	}
}

// After a `for await` loop
element.onclick = async (event) => {
	for await (const chunk of stream) {
		log(chunk);
	}
	event.currentTarget.disabled = false;
};

// Inside a `for await` loop body
element.onclick = async (event) => {
	for await (const chunk of stream) {
		event.currentTarget.append(chunk);
	}
};

// The iterated expression of a `for...of` loop is awaited, so the loop body runs after the pause
element.onclick = async (event) => {
	for (const item of await load()) {
		event.currentTarget.append(item);
	}
};

// Before an `await` in a `while` loop body
element.onclick = async (event) => {
	while (condition) {
		event.currentTarget.click();
		await promise;
	}
};

// Before an `await` in a `do...while` loop body
element.onclick = async (event) => {
	do {
		event.currentTarget.click();
		await promise;
	} while (condition);
};

// Before an `await` in a `for` loop body
element.onclick = async (event) => {
	for (let i = 0; i < 3; i++) {
		event.currentTarget.click();
		await promise;
	}
};

// Before an `await` in a `for...in` loop body
element.onclick = async (event) => {
	for (const key in object) {
		event.currentTarget.click();
		await promise;
	}
};

// Before a `yield` in a generator handler's loop body
element.onclick = function* (event) {
	while (condition) {
		event.currentTarget.click();
		yield promise;
	}
};

// In a loop condition that runs again after an `await` in the body
element.onclick = async (event) => {
	while (event.currentTarget.isConnected) {
		await promise;
	}
};

// The assignment target is read after an earlier `await`
element.onclick = async (event) => {
	await step1;
	event.currentTarget.href = await getUrl();
};

// After an `await` inside a `try` block
element.onclick = async (event) => {
	try {
		await somePromise;
	} catch {}
	event.currentTarget.disabled = false;
};

// Deeply nested function
element.onclick = (event) => {
	setTimeout(() => {
		requestAnimationFrame(() => {
			event.currentTarget.focus();
		});
	});
};

// A nested async callback with its own `await` is reported as a nested function
element.onclick = (event) => {
	button.addEventListener("click", async () => {
		await somePromise;
		event.currentTarget.foo();
	});
};

// Destructured parameter renamed to an event-like name
element.onclick = async ({ detail: event }) => {
	await somePromise;
	event.currentTarget.foo();
};

// Parameter with a default value
element.onclick = async (event = fallback) => {
	await somePromise;
	event.currentTarget.foo();
};

// Parameter without parentheses, after `await`
element.onclick = async event => {
	await somePromise;
	event.currentTarget.foo();
};

// Parameter without parentheses, in a nested function
element.onclick = event => setTimeout(() => event.currentTarget.foo());
