/**
 * The value returned by the `pathInfo` callback passed to
 * `Resolver.fromJsFileSystem()`.
 *
 * - `"file"` — the path exists and is a regular file.
 * - `"directory"` — the path exists and is a directory.
 * - `{ symlink: string }` — the path is a symlink; `symlink` must be the
 *   fully canonicalized real path (i.e. the result of `realpathSync` or
 *   equivalent). The resolver uses this to detect and break symlink cycles.
 * - `null` — the path does not exist or is not accessible.
 */
export type PathInfo = "file" | "directory" | { symlink: string } | null;

/**
 * Options for the module resolver.
 */
export interface ResolveOptions {
	/**
	 * Condition names to accept in `exports` / `imports` maps.
	 *
	 * Example: `["node", "import"]` for ESM, `["node", "require"]` for CJS.
	 */
	conditionNames?: string[];

	/**
	 * File extensions to try when resolving bare paths without an extension.
	 *
	 * Extensions must be provided **without a leading dot**. The resolver adds
	 * the dot itself when constructing candidate paths.
	 *
	 * Example: `["js", "ts", "json"]`
	 */
	extensions?: string[];

	/**
	 * Extension aliases: map an extension to one or more fallback extensions.
	 *
	 * When the resolver sees an import that ends with the key extension, it
	 * also tries the listed alias extensions in order. This is typically used
	 * to resolve `.ts` source files when the import specifier uses `.js`.
	 *
	 * Extensions must be provided **without a leading dot**.
	 *
	 * Example: `[{ extension: "js", aliases: ["ts", "js"] }]`
	 */
	extensionAliases?: Array<{ extension: string; aliases: string[] }>;

	/**
	 * Stem names to look for when resolving a directory. The resolver combines
	 * these with the `extensions` list to form candidate paths such as
	 * `index.js`, `index.ts`, etc.
	 *
	 * Provide the bare stem **without any extension**.
	 *
	 * Example: `["index"]` — combined with `extensions: ["js", "ts"]` this
	 * tries `index.js` then `index.ts`.
	 */
	defaultFiles?: string[];

	/**
	 * When `true`, Node.js built-in modules (e.g. `node:fs`) resolve to a
	 * `NodeBuiltIn` error instead of attempting further resolution.
	 */
	resolveNodeBuiltins?: boolean;

	/**
	 * When `true`, resolve TypeScript declaration files (`.d.ts`) instead of
	 * source files.
	 */
	resolveTypes?: boolean;
}

/**
 * Identifies the reason a resolution attempt failed.
 *
 * Returned as the `errorKind` field on a failed `resolve()` result. Use it
 * for programmatic branching; use the `error` string for display or logging.
 *
 * @example
 * ```ts
 * const result = resolver.resolve("./utils.js", "/project/src");
 * if (!result.path) {
 *   if (result.errorKind === ResolveErrorKind.ModuleNotFound) {
 *     // file is missing or extension not listed in options
 *   }
 * }
 * ```
 */
export enum ResolveErrorKind {
	/**
	 * The specifier could not be found anywhere the resolver looked.
	 *
	 * Common causes:
	 * - The file or package does not exist at the given path.
	 * - The package is not installed (`node_modules` is missing or stale).
	 * - The required extension is not listed in the `extensions` option.
	 * - The `conditionNames` option does not match any condition in the
	 *   package's `exports` map.
	 * - `baseDir` is not an absolute path to a directory.
	 */
	ModuleNotFound = 0,

	/**
	 * The specifier resolved to a directory but no index file was found inside it.
	 *
	 * Fix by providing both `defaultFiles` and `extensions` in the resolver
	 * options. For example, `defaultFiles: ["index"]` with
	 * `extensions: ["ts", "js"]` will try `index.ts` then `index.js`.
	 */
	DirectoryWithoutIndex = 1,

	/**
	 * The specifier names a Node.js built-in module such as `node:fs` or
	 * `node:path`.
	 *
	 * This is only returned when `resolveNodeBuiltins: true` is set. It is not
	 * a failure — it signals that the import refers to the runtime itself rather
	 * than a file on disk. Without that option, built-ins produce
	 * `ModuleNotFound` instead.
	 */
	NodeBuiltIn = 2,

	/**
	 * No `package.json` was found walking up from `baseDir`.
	 *
	 * Confirm that `baseDir` is inside a directory tree that contains a
	 * `package.json`. This error typically means `baseDir` points outside the
	 * project root or to a temporary directory.
	 */
	ManifestNotFound = 3,

	/**
	 * A `package.json` or `tsconfig.json` was found but could not be parsed.
	 *
	 * The file likely contains invalid JSON. Validate it with a JSON linter.
	 */
	ErrorLoadingManifest = 4,

	/**
	 * A symlink in the resolution chain points to a target that does not exist.
	 *
	 * This usually means a broken symlink in `node_modules` left behind by an
	 * interrupted package install. Re-running the package manager's install
	 * command normally fixes it.
	 */
	BrokenSymlink = 5,

	/**
	 * The matched condition in a `package.json` `exports` or `imports` map
	 * points to an invalid target.
	 *
	 * A valid target must be a string starting with `./`, an array of
	 * fallbacks, a conditions object, or `null`. This is a bug in the
	 * package's `package.json`. If you control the package, fix the manifest;
	 * otherwise check for a newer version.
	 */
	InvalidExportsTarget = 6,

	/**
	 * The specifier contains characters that are not valid in a package name.
	 *
	 * Check the specifier for typos such as uppercase letters in a scoped
	 * package name or a path segment that begins with `.`.
	 */
	InvalidPackageName = 7,
}

/**
 * The result of a successful resolution.
 */
export interface ResolveSuccess {
	/** The resolved absolute path. */
	path: string;
	error?: never;
	errorKind?: never;
}

/**
 * The result of a failed resolution.
 */
export interface ResolveFailure {
	path?: never;
	/** A human-readable description of why resolution failed. Suitable for display and logging. */
	error: string;
	/** A structured identifier for the failure reason. Use this for programmatic branching. */
	errorKind: ResolveErrorKind;
}

export type ResolveResult = ResolveSuccess | ResolveFailure;

/**
 * Minimal interface for the WASM `MemoryFileSystem` exported by any resolver
 * WASM package.
 */
export interface WasmMemoryFileSystem {
	insertFile(path: string, content: string): void;
	remove(path: string): void;
	free(): void;
}

/**
 * Minimal interface for the WASM `JsFileSystem` exported by any resolver WASM
 * package.
 */
export interface WasmJsFileSystem {
	free(): void;
}

/**
 * Minimal interface for the WASM `Resolver` exported by any resolver WASM
 * package.
 */
export interface WasmResolver {
	resolve(specifier: string, baseDir: string): ResolveResult;
	free(): void;
}

/**
 * The subset of the WASM module that `ResolverCommon` requires.
 */
export interface ResolverModule {
	main(): void;
	MemoryFileSystem: new () => WasmMemoryFileSystem;
	JsFileSystem: new (
		pathInfoFn: (path: string) => PathInfo,
		readFileUtf8Fn: (path: string) => string | null,
	) => WasmJsFileSystem;
	Resolver: {
		withJsFileSystem(
			fs: WasmJsFileSystem,
			options?: ResolveOptions | null,
		): WasmResolver;
		withMemoryFileSystem(
			fs: WasmMemoryFileSystem,
			options?: ResolveOptions | null,
		): WasmResolver;
	};
}

/**
 * List of modules that have been initialized.
 */
const initialized = new WeakSet<object>();

/**
 * An in-memory filesystem suitable for use in browser environments and tests.
 *
 * Populate with `insertFile()` before constructing a `Resolver`.
 */
export class MemoryFileSystem {
	constructor(private readonly inner: WasmMemoryFileSystem) {}

	/**
	 * Inserts a file at `path` with the given UTF-8 string content.
	 */
	insertFile(path: string, content: string): void {
		this.inner.insertFile(path, content);
	}

	/**
	 * Removes the file at `path`.
	 */
	remove(path: string): void {
		this.inner.remove(path);
	}

	/**
	 * Frees the underlying WASM memory.
	 *
	 * After calling this, the object must not be used. Calling `free()` more
	 * than once on the same instance throws an error.
	 */
	free(): void {
		this.inner.free();
	}

	/** @internal */
	get wasmInner(): WasmMemoryFileSystem {
		return this.inner;
	}
}

/**
 * A module resolver.
 *
 * Create with the static factory methods `Resolver.fromJsFileSystem()` or
 * `Resolver.fromMemoryFileSystem()`.
 */
export class Resolver {
	private constructor(private readonly inner: WasmResolver) {}

	/**
	 * Creates a resolver backed by two JavaScript filesystem callbacks.
	 *
	 * This is the low-level constructor for environments that have synchronous
	 * filesystem access but are not Node.js — for example Bun, Deno, or any
	 * runtime that exposes its own `fs`-like APIs. Pass callbacks that
	 * implement `pathInfo` and `readFileUtf8` for your target runtime.
	 *
	 * For Node.js specifically, prefer `createNodeResolver()` from
	 * `@biomejs/resolver/nodejs`, which wires these callbacks automatically.
	 *
	 * @param module - The loaded WASM module.
	 * @param pathInfoFn - Returns the kind of the filesystem entry at `path`
	 *   without following symlinks. See {@link PathInfo} for the expected return
	 *   values. Must be synchronous.
	 * @param readFileUtf8Fn - Returns the UTF-8 content of the file at `path`,
	 *   or `null` if it does not exist or is not readable. Must be synchronous.
	 * @param options - Optional resolver options.
	 */
	static fromJsFileSystem(
		module: ResolverModule,
		pathInfoFn: (path: string) => PathInfo,
		readFileUtf8Fn: (path: string) => string | null,
		options?: ResolveOptions | null,
	): Resolver {
		const fs = new module.JsFileSystem(pathInfoFn, readFileUtf8Fn);
		// `withJsFileSystem` takes ownership of `fs`, transferring it into Rust.
		// Do NOT call `fs.free()` afterwards — the pointer has been moved.
		const inner = module.Resolver.withJsFileSystem(fs, options ?? null);
		return new Resolver(inner);
	}

	/**
	 * Creates a resolver backed by the provided in-memory filesystem.
	 */
	static fromMemoryFileSystem(
		module: ResolverModule,
		fs: MemoryFileSystem,
		options?: ResolveOptions | null,
	): Resolver {
		const inner = module.Resolver.withMemoryFileSystem(
			fs.wasmInner,
			options ?? null,
		);
		return new Resolver(inner);
	}

	/**
	 * Resolves `specifier` starting from `baseDir`. Never throws.
	 *
	 * `baseDir` must be an absolute path to a **directory** (not a file).
	 *
	 * Returns `{ path: string }` on success, or
	 * `{ error: string; errorKind: ResolveErrorKind }` on failure.
	 * Use `error` for display and logging; use `errorKind` for programmatic
	 * branching.
	 */
	resolve(specifier: string, baseDir: string): ResolveResult {
		return this.inner.resolve(specifier, baseDir);
	}

	/**
	 * Frees the underlying WASM memory.
	 *
	 * After calling this, the object must not be used. Calling `free()` more
	 * than once on the same instance throws an error.
	 */
	free(): void {
		this.inner.free();
	}
}

/**
 * @internal
 * Initialises a WASM module at most once.
 */
export function ensureInitialized(module: ResolverModule): void {
	if (!initialized.has(module as object)) {
		module.main();
		initialized.add(module as object);
	}
}
