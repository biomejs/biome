/* should not generate diagnostics */

// A parameter type that is not an event
async function f(event: number) {
	await y();
	event.currentTarget;
}
async function f(event: string) {
	await y();
	event.currentTarget;
}
async function f(event: number[]) {
	await y();
	event.currentTarget;
}
async function f(event: { currentTarget: HTMLElement }) {
	await y();
	event.currentTarget;
}

// A type assertion doesn't hide a parameter type that is not an event
el.addEventListener("click", async (e: number) => {
	await x;
	(e as any).currentTarget;
});

// Read with a type assertion before the `await`
el.addEventListener("click", async (e) => {
	(e as Event).currentTarget;
	await x;
});

// A type-only `Event` imported from somewhere other than React is not the DOM event
import type { MouseEvent } from "./my-events";
async function f(event: MouseEvent) {
	await y();
	event.currentTarget;
}
