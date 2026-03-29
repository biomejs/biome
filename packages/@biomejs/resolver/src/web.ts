import * as wasmModule from "@biomejs/wasm-resolver-web";
import {
	ensureInitialized,
	MemoryFileSystem,
	type ResolveOptions,
	type ResolveResult,
	Resolver,
} from "./common";

export type * from "./common";

ensureInitialized(wasmModule);

export { MemoryFileSystem, Resolver };

/**
 * Creates an empty `MemoryFileSystem` for use with `createWebResolver()` or
 * `Resolver.fromMemoryFileSystem()`.
 *
 * This is the primary way to resolve modules in browser environments, since
 * direct filesystem access is not available there.
 */
export function createMemoryFileSystem(): MemoryFileSystem {
	return new MemoryFileSystem(new wasmModule.MemoryFileSystem());
}

/**
 * Creates a `Resolver` backed by the provided in-memory filesystem.
 *
 * This is a convenience wrapper around `Resolver.fromMemoryFileSystem()`.
 */
export function createWebResolver(
	fs: MemoryFileSystem,
	options?: ResolveOptions | null,
): Resolver {
	return Resolver.fromMemoryFileSystem(wasmModule, fs, options);
}

export type { ResolveOptions, ResolveResult };
