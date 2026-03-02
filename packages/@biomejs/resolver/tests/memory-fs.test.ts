/**
 * Tests for the `MemoryFileSystem`-backed resolver.
 *
 * These tests run against the Node.js WASM target because that is the only
 * one available in the Vitest environment. They exercise the same
 * `MemoryFileSystem` + `Resolver` code path used in browser / bundler builds.
 */

import * as wasmModule from "@biomejs/wasm-resolver-nodejs";
import { beforeAll, describe, expect, it } from "vitest";
import {
	MemoryFileSystem,
	Resolver,
	ResolveErrorKind,
	ensureInitialized,
} from "../src/common";

beforeAll(() => {
	ensureInitialized(wasmModule);
});

function makeMemFs(): MemoryFileSystem {
	return new MemoryFileSystem(new wasmModule.MemoryFileSystem());
}

describe("MemoryFileSystem resolver", () => {
	it("resolves a relative path to a JS file", () => {
		const fs = makeMemFs();
		fs.insertFile("/project/src/index.js", "");
		fs.insertFile(
			"/project/package.json",
			JSON.stringify({ name: "project", version: "1.0.0" }),
		);

		const resolver = Resolver.fromMemoryFileSystem(wasmModule, fs);

		const result = resolver.resolve("./index.js", "/project/src");
		expect(result).toEqual({ path: "/project/src/index.js" });

		resolver.free();
		fs.free();
	});

	it("returns an error for a non-existent specifier", () => {
		const fs = makeMemFs();
		fs.insertFile(
			"/project/package.json",
			JSON.stringify({ name: "project", version: "1.0.0" }),
		);

		const resolver = Resolver.fromMemoryFileSystem(wasmModule, fs);

		const result = resolver.resolve("./missing.js", "/project/src");
		expect(result).toHaveProperty("error");
		expect(result).toHaveProperty("errorKind", ResolveErrorKind.ModuleNotFound);

		resolver.free();
		fs.free();
	});

	it("resolves a package export when package.json is present", () => {
		const fs = makeMemFs();
		fs.insertFile(
			"/project/node_modules/my-pkg/package.json",
			JSON.stringify({
				name: "my-pkg",
				version: "1.0.0",
				exports: {
					".": "./dist/index.js",
				},
			}),
		);
		fs.insertFile("/project/node_modules/my-pkg/dist/index.js", "");
		fs.insertFile(
			"/project/package.json",
			JSON.stringify({ name: "project", version: "1.0.0" }),
		);

		const resolver = Resolver.fromMemoryFileSystem(wasmModule, fs, {
			conditionNames: ["require"],
		});

		const result = resolver.resolve("my-pkg", "/project/src");
		expect(result).toEqual({
			path: "/project/node_modules/my-pkg/dist/index.js",
		});

		resolver.free();
		fs.free();
	});

	it("resolves a directory index file", () => {
		const fs = makeMemFs();
		fs.insertFile("/project/src/utils/index.js", "");
		fs.insertFile(
			"/project/package.json",
			JSON.stringify({ name: "project", version: "1.0.0" }),
		);

		const resolver = Resolver.fromMemoryFileSystem(wasmModule, fs, {
			extensions: ["js"],
			defaultFiles: ["index"],
		});

		const result = resolver.resolve("./utils", "/project/src");
		expect(result).toEqual({ path: "/project/src/utils/index.js" });

		resolver.free();
		fs.free();
	});
});
