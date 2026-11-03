/* should not generate diagnostics */

// Called while the handler is running
element.addEventListener("click", (event) => {
	event.preventDefault();
});
element.addEventListener("click", (event) => {
	event.stopPropagation();
});
element.addEventListener("click", (event) => {
	event.stopImmediatePropagation();
});

// Called before the `await`
element.addEventListener("click", async (event) => {
	event.preventDefault();
	await somePromise;
});

// The argument of an awaited call runs before the handler pauses
element.onclick = async (event) => {
	await foo(event.preventDefault());
};

// An argument runs before a later awaited argument of the same call
element.onclick = async (event) => {
	log(event.stopPropagation(), await ready());
};

// The iterated expression runs before the `for await` loop pauses
element.onclick = async (event) => {
	for await (const chunk of event.preventDefault()) {
		log(chunk);
	}
};

// The iterated expression of a `for...of` loop runs once, before the loop body
element.onclick = async (event) => {
	for (const child of event.preventDefault()) {
		await render(child);
	}
};

// The `for` loop initializer runs once, before the loop body
element.onclick = async (event) => {
	for (let index = event.preventDefault(); index < 3; index++) {
		await render(index);
	}
};

// The object is not a parameter
async function handler() {
	const event = getEvent();
	await somePromise;
	event.preventDefault();
}

// A method that doesn't control the event
async (event) => {
	await somePromise;
	event.composedPath();
};

// A computed method name that isn't known
async (event) => {
	await somePromise;
	event[method]();
};

// The method is stored in a variable first
async (event) => {
	const method = event.preventDefault;
	await somePromise;
	method();
};
async (event) => {
	const { preventDefault } = event;
	await somePromise;
	preventDefault();
};

// The global `event` is not a parameter
async () => {
	await somePromise;
	event.preventDefault();
};

// The inner handler declares its own `event` parameter
async function outer(event) {
	await somePromise;
	element.addEventListener("click", (event) => {
		event.preventDefault();
	});
}

// The `await` is inside a nested function, so it doesn't pause the handler
element.onclick = (event) => {
	promise.then(async () => {
		await somePromise;
	});
	event.preventDefault();
};

// The method is called on a property of the event
async function onClick(event) {
	await load();
	event.detail.preventDefault();
}

// The parameter name doesn't look like an event
async function update(data) {
	await load();
	data.preventDefault();
}
async function update(Event) {
	await load();
	Event.preventDefault();
}
async function update(my_event) {
	await load();
	my_event.preventDefault();
}
