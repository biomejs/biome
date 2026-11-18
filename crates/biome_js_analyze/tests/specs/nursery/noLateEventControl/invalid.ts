/* should generate diagnostics */
import ReactDefault from "react";
import type * as React from "react";
import type {
	BaseSyntheticEvent,
	ChangeEvent,
	InputEvent as Input,
	SyntheticEvent,
	SyntheticEvent as SE,
	ToggleEvent,
} from "react";

// Global event types
async function handleClick(event: Event) {
	await load();
	event.preventDefault();
}
async function handleClick(value: MouseEvent) {
	await load();
	value.stopPropagation();
}
async function handleClick(value: Event | MouseEvent) {
	await load();
	value.preventDefault();
}
async function handleClick(change: PictureInPictureEvent) {
	await load();
	change.stopPropagation();
}
async function handleClick(request: FetchEvent) {
	await load();
	request.preventDefault();
}

// Type wrappers around the event
async function handleClick(value: Event) {
	await load();
	value!.preventDefault();
}
async function handleClick(value: Event) {
	await load();
	(value as Event).stopPropagation();
}
async function handleClick(value: Event) {
	await load();
	(<Event>value).stopImmediatePropagation();
}

// The name looks like an event and the type is unknown
async function handleClick(mouseDownEvent: unknown) {
	await load();
	mouseDownEvent.preventDefault();
}

// React event types
async function handleClick(value: ReactDefault.SyntheticEvent) {
	await load();
	value.preventDefault();
}
async function handleClick(event: React.SyntheticEvent) {
	await load();
	event.stopImmediatePropagation();
}
async function handleClick(value: React.AnimationEvent) {
	await load();
	value.preventDefault();
}
async function handleClick(value: SyntheticEvent) {
	await load();
	value.preventDefault();
}
async function handleClick(value: BaseSyntheticEvent) {
	await load();
	value.preventDefault();
}
async function handleClick(value: SE) {
	await load();
	value.preventDefault();
}
async function handleClick(value: Input) {
	await load();
	value.preventDefault();
}
async function handleClick(value: ChangeEvent<HTMLInputElement>) {
	await load();
	value.preventDefault();
}
async function handleClick(value: ToggleEvent) {
	await load();
	value.preventDefault();
}

// Local types that extend an event type
function localTypes() {
	type ClickEvent = MouseEvent;
	interface DragPayload extends DragEvent {}
	class CustomClick extends MouseEvent {}

	async function handleClick(value: ClickEvent) {
		await load();
		value.preventDefault();
	}
	async function handleDrag(value: DragPayload) {
		await load();
		value.preventDefault();
	}
	async function handleCustom(value: CustomClick) {
		await load();
		value.preventDefault();
	}
	async function handleGeneric<T extends Event>(value: T) {
		await load();
		value.preventDefault();
	}
}

// A parameter property in a constructor
class Handler {
	constructor(private value: Event) {
		setTimeout(() => value.preventDefault());
	}
}
