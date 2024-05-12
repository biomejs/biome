import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

import { createWorkspaceWithBinary } from "../dist/index.js";

describe("Workspace API", () => {
	it("should process remote requests", async () => {
		const extension = process.platform === "win32" ? ".exe" : "";
		const command = resolve(
			fileURLToPath(import.meta.url),
			"../../../../..",
			`target/release/biome${extension}`,
		);

		const workspace = await createWorkspaceWithBinary(command);
		workspace.registerProjectFolder({
			setAsCurrentWorkspace: true,
		});
		await workspace.openFile({
			path: {
				path: "test.js",
				id: 0,
			},
			content: "statement()",
			version: 0,
		});

		const printed = await workspace.formatFile({
			path: {
				path: "test.js",
				id: 0,
			},
		});

		expect(printed.code).toBe("statement();\n");

		await workspace.closeFile({
			path: {
				path: "test.js",
				id: 0,
			},
		});

		workspace.destroy();
	});
});
