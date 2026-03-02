import * as wasmModule from "@biomejs/wasm-resolver-nodejs";
import {
	MemoryFileSystem,
	Resolver,
	ensureInitialized,
	type ResolveOptions,
	type ResolveResult,
} from "./common";
import { nodePathInfo, nodeReadFileUtf8 } from "./nodejsFileSystem";

export type * from "./common";

ensureInitialized(wasmModule);

export { MemoryFileSystem, Resolver };

/**
 * Creates a `Resolver` backed by the real Node.js filesystem.
 *
 * Uses `lstatSync`, `realpathSync`, and `readFileSync` from `node:fs`.
 */
export function createNodeResolver(options?: ResolveOptions | null): Resolver {
	return Resolver.fromJsFileSystem(
		wasmModule,
		nodePathInfo,
		nodeReadFileUtf8,
		options,
	);
}

/**
 * Creates an empty `MemoryFileSystem` for use with `Resolver.fromMemoryFileSystem()`.
 */
export function createMemoryFileSystem(): MemoryFileSystem {
	return new MemoryFileSystem(new wasmModule.MemoryFileSystem());
}

export type { ResolveOptions, ResolveResult };
