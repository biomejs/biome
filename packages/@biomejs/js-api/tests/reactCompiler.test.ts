import { MemoryFileSystem, Workspace } from "@biomejs/wasm-nodejs";
import { afterEach, beforeEach, describe, expect, it } from "vitest";

describe("React Compiler in WebAssembly", () => {
	let filesystem: MemoryFileSystem;
	let workspace: Workspace;
	let projectKey: number;

	beforeEach(() => {
		filesystem = new MemoryFileSystem();
		filesystem.insert(
			"/package.json",
			new TextEncoder().encode('{"dependencies":{"react":"^19.0.0"}}'),
		);
		workspace = Workspace.withFileSystem(filesystem);
		projectKey = workspace.openProject({
			path: "/",
			openUninitialized: true,
		}).projectKey;
		workspace.updateSettings({
			projectKey,
			configuration: {
				linter: {
					enabled: true,
					rules: { recommended: false, nursery: { useReactCompiler: "error" } },
				},
			},
		});
		workspace.scanProject({
			projectKey,
			scanKind: "project",
			force: true,
			watch: false,
		});
	});

	afterEach(() => {
		workspace.free();
		filesystem.free();
	});

	function lint(source: string) {
		workspace.openFile({
			projectKey,
			path: "/App.tsx",
			content: { type: "fromClient", content: source, version: 0 },
		});
		return workspace.pullDiagnostics({
			projectKey,
			path: "/App.tsx",
			categories: ["lint"],
		}).diagnostics;
	}

	it("reports conditional hook calls", () => {
		const diagnostics = lint(`import { useState } from "react";
export function App({ enabled }) {
  if (enabled) useState(0);
  return <div />;
}`);
		expect(diagnostics).toEqual([
			expect.objectContaining({
				category: "lint/nursery/useReactCompiler",
				description: "This hook usage does not follow React's rules.",
			}),
		]);
	});

	it("accepts unconditional hook calls", () => {
		const diagnostics = lint(`import { useState } from "react";
export function App() {
  const [value] = useState(0);
  return <div>{value}</div>;
}`);
		expect(diagnostics).toEqual([]);
	});

	it("reports hook violations with a deeply nested expression", () => {
		const expression = Array.from({ length: 100 }, () => "value").join(" + ");
		const diagnostics = lint(`import { useState } from "react";
export function App({ value }) {
  if (value) useState(0);
  return <div>{${expression}}</div>;
}`);
		expect(diagnostics).toEqual([
			expect.objectContaining({
				category: "lint/nursery/useReactCompiler",
				description: "This hook usage does not follow React's rules.",
			}),
		]);
	});

	it("accepts a deeply nested expression with unconditional hooks", () => {
		const expression = Array.from({ length: 100 }, () => "value").join(" + ");
		const diagnostics = lint(`import { useState } from "react";
export function App() {
  const [value] = useState(0);
  return <div>{${expression}}</div>;
}`);
		expect(diagnostics).toEqual([]);
	});
});
