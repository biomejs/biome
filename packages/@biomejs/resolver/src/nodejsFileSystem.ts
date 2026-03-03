/**
 * Creates the two filesystem callbacks required by `JsFileSystem` using
 * Node.js built-in `node:fs` synchronous APIs.
 *
 * This module must only be imported in Node.js environments.
 */

import { lstatSync, readFileSync, realpathSync } from "node:fs";
import type { PathInfo } from "./common";

/**
 * Returns `"file"`, `"directory"`, `{ symlink: <canonicalized> }`, or `null`
 * for the given path without following symlinks.
 */
export function nodePathInfo(path: string): PathInfo {
	try {
		const stat = lstatSync(path);
		if (stat.isSymbolicLink()) {
			try {
				const real = realpathSync(path);
				return { symlink: real };
			} catch {
				return null;
			}
		}
		if (stat.isDirectory()) {
			return "directory";
		}
		if (stat.isFile()) {
			return "file";
		}
		return null;
	} catch {
		return null;
	}
}

/**
 * Reads the UTF-8 content of the file at `path`, or returns `null` if the
 * file does not exist or is not readable.
 */
export function nodeReadFileUtf8(path: string): string | null {
	try {
		return readFileSync(path, "utf8");
	} catch {
		return null;
	}
}
