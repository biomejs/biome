/* should generate diagnostics */

// A parameter with an event type
async function f(event: Event) {
	await y();
	event.currentTarget;
}

// An event type decides, even when the name doesn't look like an event
async function f(click: MouseEvent) {
	await y();
	click.currentTarget;
}

// A React event type
import type { SyntheticEvent } from "react";
async function f(target: SyntheticEvent) {
	await y();
	target.currentTarget;
}

// A type assertion still refers to the event
el.addEventListener("click", async (e) => {
	await x;
	(e as Event).currentTarget;
});

// A non-null assertion still refers to the event
el.addEventListener("click", async (e) => {
	await x;
	e!.currentTarget;
});

// A `satisfies` expression still refers to the event
el.addEventListener("click", async (event) => {
	await x;
	(event satisfies Event).currentTarget;
});

// A type assertion with optional chaining
el.addEventListener("click", async (event) => {
	await x;
	(event as Event)?.currentTarget;
});

// A type assertion in a nested function
el.addEventListener("click", (e) => {
	setTimeout(() => (e as Event).currentTarget);
});
