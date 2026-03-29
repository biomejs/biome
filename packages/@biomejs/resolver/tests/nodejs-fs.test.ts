/**
 * Tests for the Node.js real-filesystem-backed resolver.
 *
 * These tests resolve against the actual filesystem of this repository, so
 * they require the Node.js WASM target to have been built beforehand.
 */

import * as path from "node:path";
import * as wasmModule from "@biomejs/wasm-resolver-nodejs";
import { beforeAll, describe, expect, it } from "vitest";
import { ensureInitialized, ResolveErrorKind, Resolver } from "../src/common";
import { nodePathInfo, nodeReadFileUtf8 } from "../src/nodejsFileSystem";

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
		try {
			const result = resolver.resolve("./nodejs-fs.test.ts", __dirname);
			expect(result).toEqual({
				path: path.join(__dirname, "nodejs-fs.test.ts"),
			});
		} finally {
			resolver.free();
		}
	});

	it("resolves a relative sibling file on disk", () => {
		const resolver = Resolver.fromJsFileSystem(
			wasmModule,
			nodePathInfo,
			nodeReadFileUtf8,
		);
		try {
			const result = resolver.resolve("./memory-fs.test.ts", __dirname);
			expect(result).toEqual({
				path: path.join(__dirname, "memory-fs.test.ts"),
			});
		} finally {
			resolver.free();
		}
	});

	it("returns an error for a module that does not exist", () => {
		const resolver = Resolver.fromJsFileSystem(
			wasmModule,
			nodePathInfo,
			nodeReadFileUtf8,
		);
		try {
			const result = resolver.resolve("this-package-does-not-exist", repoRoot);
			expect(result).toHaveProperty("error");
			expect(result).toHaveProperty(
				"errorKind",
				ResolveErrorKind.ModuleNotFound,
			);
		} finally {
			resolver.free();
		}
	});
});
