/* should not generate diagnostics */
import type { Event } from "not-dom";
import type { NotAnEvent as MouseEvent } from "not-dom";
import KeyboardEvent from "not-dom";
import type * as DOM from "not-dom";
import type { SyntheticEvent } from "not-react";
import NotReact from "not-react";

async function updateControl(event: Event) {
	await load();
	event.preventDefault();
}
async function updateControl(event: MouseEvent) {
	await load();
	event.preventDefault();
}
async function updateControl(event: KeyboardEvent) {
	await load();
	event.preventDefault();
}
async function updateControl(event: DOM.Event) {
	await load();
	event.preventDefault();
}
async function updateControl(value: SyntheticEvent) {
	await load();
	value.preventDefault();
}
async function updateControl(event: NotReact.SyntheticEvent) {
	await load();
	event.preventDefault();
}
