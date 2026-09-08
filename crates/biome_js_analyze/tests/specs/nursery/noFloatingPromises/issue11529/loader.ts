import type { Consumer } from "./consumer";

export function load(): Promise<void> {
	return Promise.resolve();
}

export type LoaderConsumer = Consumer;
