/**
 * File watcher for Rust formatter crates with debounced WASM rebuilds.
 */

import chokidar from "chokidar";
import { spawn } from "child_process";
import { EventEmitter } from "events";
import { resolve } from "path";

/** Debounce delay in milliseconds */
const DEBOUNCE_MS = 500;

/** Enable debug logging via environment variable */
const DEBUG = process.env.DEBUG_WATCH === "1" || process.env.DEBUG === "1";

function debug(...args: unknown[]) {
	if (DEBUG) {
		console.log("[watch]", ...args);
	}
}

/**
 * Directories containing Rust files that affect the formatter.
 * Note: chokidar v4 doesn't support glob patterns, so we watch directories directly.
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
	let debounceTimer: NodeJS.Timeout | null = null;
	let isRebuilding = false;

	// Convert relative directories to absolute paths
	const absoluteDirs = WATCH_DIRS.map((dir) => resolve(rootDir, dir));

	debug("Creating watcher for directories:", absoluteDirs);

	const watcher = chokidar.watch(absoluteDirs, {
		ignoreInitial: true,
		// Ignore build artifacts and non-source files
		ignored: ["**/target/**", "**/node_modules/**"],
	});

	watcher.on("ready", () => {
		debug("Watcher ready");
		const watched = watcher.getWatched();
		debug("Watching", Object.keys(watched).length, "directories");
	});

	// Listen to all events (add, change, unlink) and filter for .rs files
	watcher.on("all", (event, path) => {
		// Only react to Rust file changes
		if (!path.endsWith(".rs")) {
			return;
		}

		debug(`File ${event}:`, path);

		// Clear any pending debounce timer
		if (debounceTimer) {
			clearTimeout(debounceTimer);
		}

		// Debounce rapid file changes
		debounceTimer = setTimeout(async () => {
			if (isRebuilding) {
				debug("Already rebuilding, skipping");
				return;
			}

			isRebuilding = true;
			emitter.emit("rebuilding", path);

			try {
				await rebuildWasm(rootDir);
				emitter.emit("rebuilt");
			} catch (err) {
				emitter.emit(
					"error",
					err instanceof Error ? err : new Error(String(err)),
				);
			} finally {
				isRebuilding = false;
			}
		}, DEBOUNCE_MS);
	});

	watcher.on("error", (err) => {
		debug("Watcher error:", err);
		emitter.emit("error", err);
	});

	return Object.assign(emitter, {
		close: () => watcher.close(),
	}) as Watcher;
}

/**
 * Rebuild the WASM module using `just build-wasm-node-dev`.
 *
 * @param rootDir - The root directory of the Biome repository
 * @returns A promise that resolves when the build completes
 */
export function rebuildWasm(rootDir: string): Promise<void> {
	debug("Starting WASM rebuild...");

	return new Promise((resolve, reject) => {
		const proc = spawn("just", ["build-wasm-node-dev"], {
			cwd: rootDir,
			stdio: "pipe",
		});

		let stderr = "";

		proc.stderr?.on("data", (data) => {
			stderr += data.toString();
		});

		proc.stdout?.on("data", (data) => {
			debug("[build]", data.toString().trim());
		});

		proc.on("close", (code) => {
			if (code === 0) {
				debug("WASM rebuild completed successfully");
				resolve();
			} else {
				const error = new Error(
					`WASM build failed with code ${code}:\n${stderr.trim()}`,
				);
				debug("WASM rebuild failed:", error.message);
				reject(error);
			}
		});

		proc.on("error", (err) => {
			const error = new Error(`Failed to start build process: ${err.message}`);
			debug("Failed to start build:", error.message);
			reject(error);
		});
	});
}
