/* should generate diagnostics */

// After `await`
element.addEventListener("click", async (event) => {
	await somePromise;
	event.preventDefault();
});
element.addEventListener("click", async (event) => {
	await somePromise;
	event.stopPropagation();
});
element.addEventListener("click", async (event) => {
	await somePromise;
	event.stopImmediatePropagation();
});

// After `yield` in a generator handler
element.onclick = function* (event) {
	yield somePromise;
	event.preventDefault();
};
element.onclick = function* (event) {
	event.stopPropagation(yield ready);
};

// The body of a generator handler doesn't run during the event
element.onclick = function* (event) {
	event.preventDefault();
	yield somePromise;
};

// Other parameter names that look like an event
element.onclick = async (e) => {
	await somePromise;
	e.preventDefault();
};
element.onclick = async (evt) => {
	await somePromise;
	evt.preventDefault();
};
element.onclick = async (mouseDownEvent) => {
	await somePromise;
	mouseDownEvent.stopPropagation();
};
element.onclick = async (key2Event) => {
	await somePromise;
	key2Event.stopPropagation();
};

// An earlier awaited argument pauses the handler first
element.onclick = async (event) => {
	log(await ready(), event.preventDefault());
};
element.onclick = async (event) => {
	event.preventDefault(await ready());
};

// Only the call after the `await` is reported
element.onclick = async (event) => {
	event.preventDefault();
	await somePromise;
	event.stopPropagation();
};

// Nested functions
element.onclick = (event) => setTimeout(() => event.preventDefault());
element.onclick = (event) => {
	setTimeout(() => {
		event.stopPropagation();
	}, 1);
};
element.onclick = (event) => {
	fetch(url).then(() => event.stopImmediatePropagation());
};
element.onclick = (event) => {
	setTimeout(() => {
		requestAnimationFrame(() => {
			event.preventDefault();
		});
	});
};
element.onclick = (event) => {
	items.forEach(function (item) {
		event.preventDefault();
	});
};

// A nested function with its own `await` is reported as a nested function
element.onclick = (event) => {
	button.addEventListener("click", async () => {
		await somePromise;
		event.preventDefault();
	});
};

// Optional chaining
async (event) => {
	await somePromise;
	event?.preventDefault();
};
async (event) => {
	await somePromise;
	event.preventDefault?.();
};

// Computed method names
async (event) => {
	await somePromise;
	event["preventDefault"]();
};
async (event) => {
	await somePromise;
	event[`preventDefault`]();
};
async (event) => {
	await somePromise;
	event?.["stopPropagation"]();
};

// Parenthesized callee
async (event) => {
	await somePromise;
	(event.preventDefault)();
};

// Function declarations and methods
async function handleClick(event) {
	await somePromise;
	event.preventDefault();
}
class Form {
	async onSubmit(event) {
		await this.save();
		event.preventDefault();
	}
	*onReset(event) {
		event.preventDefault();
	}
}
const handlers = {
	async onClick(event) {
		await save();
		event.preventDefault();
	},
};

// Destructured parameter
async ({ nativeEvent }) => {
	await somePromise;
	nativeEvent.preventDefault();
};

// After a `for await` loop
element.onclick = async (event) => {
	for await (const chunk of stream) {
		log(chunk);
	}
	event.preventDefault();
};

// Inside a `for await` loop body
element.onclick = async (event) => {
	for await (const chunk of stream) {
		event.preventDefault();
	}
};

// The iterated expression is awaited before the loop body runs
element.onclick = async (event) => {
	for (const item of await load()) {
		event.preventDefault();
	}
};

// The loop body runs again after the `await`
element.onclick = async (event) => {
	while (condition) {
		event.preventDefault();
		await promise;
	}
};
element.onclick = async (event) => {
	do {
		event.preventDefault();
		await promise;
	} while (condition);
};
element.onclick = async (event) => {
	for (let index = 0; event.preventDefault(); index++) {
		await promise;
	}
};
element.onclick = async (event) => {
	for (const item of items) {
		event.preventDefault();
		await render(item);
	}
};

// The loop condition runs again after the `await`
element.onclick = async (event) => {
	while (event.preventDefault()) {
		await promise;
	}
};

// After an `await` inside a `try` block
element.onclick = async (event) => {
	try {
		await somePromise;
	} catch {}
	event.preventDefault();
};
