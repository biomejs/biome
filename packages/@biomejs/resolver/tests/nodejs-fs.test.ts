/**
 * Tests for the Node.js real-filesystem-backed resolver.
 *
 * These tests resolve against the actual filesystem of this repository, so
 * they require the Node.js WASM target to have been built beforehand.
 */

import * as wasmModule from "@biomejs/wasm-resolver-nodejs";
import * as path from "node:path";
import { beforeAll, describe, expect, it } from "vitest";
import { Resolver, ResolveErrorKind, ensureInitialized } from "../src/common";
import { nodePathInfo, nodeReadFileUtf8 } from "../src/nodejs-fs";

beforeAll(() => {
	ensureInitialized(wasmModule);
});

const repoRoot = path.resolve(__dirname, "../../../..");

describe("Node.js filesystem resolver", () => {
	it("resolves a file that exists on disk", () => {
		const resolver = Resolver.fromJsFileSystem(
			wasmModule,
			nodePathInfo,
			nodeReadFileUtf8,
		);

		// Resolve this test file itself relative to its own directory.
		const result = resolver.resolve("./nodejs-fs.test.ts", __dirname);
		expect(result).toEqual({ path: path.join(__dirname, "nodejs-fs.test.ts") });

		resolver.free();
	});

	it("resolves a relative sibling file on disk", () => {
		const resolver = Resolver.fromJsFileSystem(
			wasmModule,
			nodePathInfo,
			nodeReadFileUtf8,
		);

		// Resolve the memory-fs test file from the same directory.
		const result = resolver.resolve("./memory-fs.test.ts", __dirname);
		expect(result).toEqual({
			path: path.join(__dirname, "memory-fs.test.ts"),
		});

		resolver.free();
	});

	it("returns an error for a module that does not exist", () => {
		const resolver = Resolver.fromJsFileSystem(
			wasmModule,
			nodePathInfo,
			nodeReadFileUtf8,
		);

		const result = resolver.resolve(
			"this-package-does-not-exist",
			repoRoot,
		);
		expect(result).toHaveProperty("error");
		expect(result).toHaveProperty("errorKind", ResolveErrorKind.ModuleNotFound);

		resolver.free();
	});
});
