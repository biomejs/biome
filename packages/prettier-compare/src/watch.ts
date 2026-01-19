/**
 * File watcher for Rust formatter crates with debounced WASM rebuilds.
 */

// biome-ignore lint/style/useNodejsImportProtocol: auto-suppressed
import { spawn } from "child_process";
// biome-ignore lint/style/useNodejsImportProtocol: auto-suppressed
import { EventEmitter } from "events";
// biome-ignore lint/style/useNodejsImportProtocol: auto-suppressed
import { type FSWatcher, watch } from "fs";
// biome-ignore lint/style/useNodejsImportProtocol: auto-suppressed
import { readdir, stat } from "fs/promises";
// biome-ignore lint/style/useNodejsImportProtocol: auto-suppressed
import { resolve } from "path";

/** Debounce delay in milliseconds */
const DEBOUNCE_MS = 500;

/**
 * Directories containing Rust files that affect the formatter.
 * We watch directories directly because `fs.watch` can't subscribe to glob patterns.
 */
const WATCH_DIRS = [
	"crates/biome_formatter",
	"crates/biome_js_formatter",
	"crates/biome_json_formatter",
	"crates/biome_css_formatter",
	"crates/biome_html_formatter",
	"crates/biome_graphql_formatter",
	"crates/biome_wasm",
];

const IGNORED_DIRS = new Set(["target", "node_modules"]);

function isIgnoredPath(filePath: string): boolean {
	return filePath.split(/[/\\]+/).some((segment) => IGNORED_DIRS.has(segment));
}

function toError(err: unknown): Error {
	return err instanceof Error ? err : new Error(String(err));
}

export interface WatcherEvents {
	/** Emitted when a rebuild is starting */
	on(event: "rebuilding", listener: (changedFile: string) => void): this;
	/** Emitted when a rebuild completes successfully */
	on(event: "rebuilt", listener: () => void): this;
	/** Emitted when a rebuild fails */
	on(event: "error", listener: (err: Error) => void): this;
}

export interface Watcher extends WatcherEvents {
	/** Stop watching for changes */
	close(): Promise<void>;
}

/**
 * Create a file watcher for Rust formatter crates.
 *
 * @param rootDir - The root directory of the Biome repository
 * @returns A watcher instance that emits events on file changes
 */
export function createWatcher(rootDir: string): Watcher {
	const emitter = new EventEmitter();
	const watchers = new Map<string, FSWatcher>();
	let debounceTimer: NodeJS.Timeout | null = null;
	let isRebuilding = false;
	let lastChangedFile: string | null = null;

	const absoluteDirs = WATCH_DIRS.map((dir) => resolve(rootDir, dir));

	const scheduleRebuild = (changedFile: string) => {
		lastChangedFile = changedFile;

		if (debounceTimer) {
			clearTimeout(debounceTimer);
		}

		debounceTimer = setTimeout(async () => {
			debounceTimer = null;

			if (isRebuilding) {
				return;
			}

			if (!lastChangedFile) {
				return;
			}

			isRebuilding = true;
			emitter.emit("rebuilding", lastChangedFile);

			try {
				await rebuildWasm(rootDir);
				emitter.emit("rebuilt");
			} catch (err) {
				emitter.emit("error", toError(err));
			} finally {
				isRebuilding = false;
				lastChangedFile = null;
			}
		}, DEBOUNCE_MS);
	};

	const handleWatcherEvent = (
		baseDir: string,
		eventType: string,
		filename?: string | Buffer,
	) => {
		if (!filename) {
			return;
		}

		const fullPath = resolve(baseDir, filename.toString());

		if (isIgnoredPath(fullPath)) {
			return;
		}

		(async () => {
			if (eventType === "rename") {
				try {
					const stats = await stat(fullPath);
					if (stats.isDirectory()) {
						await visitDirectory(fullPath);
					}
				} catch {
					// File or directory might have been removed; ignore.
				}
			}

			if (fullPath.endsWith(".rs")) {
				scheduleRebuild(fullPath);
			}
		})().catch((err) => {
			const error = toError(err);
			emitter.emit("error", error);
		});
	};

	const startDirWatcher = async (dir: string): Promise<void> => {
		if (watchers.has(dir)) {
			return;
		}

		try {
			const watcher = watch(dir, { persistent: true }, (eventType, filename) =>
				handleWatcherEvent(dir, eventType, filename ?? undefined),
			);

			watcher.on("error", (err) => {
				const error = toError(err);
				emitter.emit("error", error);
				watcher.close();
				watchers.delete(dir);
			});

			watchers.set(dir, watcher);
		} catch (err) {
			const error = toError(err);
			emitter.emit("error", error);
		}
	};

	const visitDirectory = async (dir: string): Promise<void> => {
		const normalized = resolve(dir);

		if (isIgnoredPath(normalized) || watchers.has(normalized)) {
			return;
		}

		try {
			const stats = await stat(normalized);
			if (!stats.isDirectory()) {
				return;
			}
			// biome-ignore lint/correctness/noUnusedVariables: auto-suppressed
		} catch (err) {
			return;
		}

		await startDirWatcher(normalized);

		try {
			const entries = await readdir(normalized, { withFileTypes: true });
			await Promise.all(
				entries
					.filter((entry) => entry.isDirectory())
					.map((entry) => visitDirectory(resolve(normalized, entry.name))),
			);
		} catch (err) {
			console.error("Failed to read directory:", normalized, err);
		}
	};

	// biome-ignore lint/nursery/noFloatingPromises: its fine here
	(async () => {
		try {
			for (const dir of absoluteDirs) {
				await visitDirectory(dir);
			}
		} catch (err) {
			emitter.emit("error", toError(err));
		}
	})();

	// biome-ignore lint/plugin: our demo plugin rule flags this
	return Object.assign(emitter, {
		close: async () => {
			if (debounceTimer) {
				clearTimeout(debounceTimer);
				debounceTimer = null;
			}

			for (const watcher of watchers.values()) {
				watcher.close();
			}

			watchers.clear();
		},
	}) as Watcher;
}

/**
 * Rebuild the WASM module using `just build-wasm-node-dev`.
 *
 * @param rootDir - The root directory of the Biome repository
 * @returns A promise that resolves when the build completes
 */
export function rebuildWasm(rootDir: string): Promise<void> {
	console.info("Starting WASM rebuild...");

	return new Promise((resolve, reject) => {
		const proc = spawn("just", ["build-wasm-node-dev"], {
			cwd: rootDir,
			stdio: "pipe",
		});

		let stderr = "";

		proc.stderr?.on("data", (data) => {
			stderr += data.toString();
		});

		proc.on("close", (code) => {
			if (code === 0) {
				resolve();
			} else {
				const error = new Error(
					`WASM build failed with code ${code}:\n${stderr.trim()}`,
				);
				reject(error);
			}
		});

		proc.on("error", (err) => {
			const error = new Error(`Failed to start build process: ${err.message}`);
			reject(error);
		});
	});
}
