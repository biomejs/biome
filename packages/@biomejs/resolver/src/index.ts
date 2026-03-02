export type * from "./common";
export { MemoryFileSystem, Resolver, ResolveErrorKind } from "./common";

/**
 * Which WASM distribution to load.
 */
export enum Distribution {
	/** WASM built for Node.js */
	NODE = 0,
	/** WASM built for the browser (web) */
	WEB = 1,
}
