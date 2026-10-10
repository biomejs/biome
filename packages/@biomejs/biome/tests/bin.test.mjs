import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { chmodSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { constants, tmpdir } from "node:os";
import { join } from "node:path";
import { after, before, describe, it } from "node:test";
import { fileURLToPath } from "node:url";

const launcher = fileURLToPath(new URL("../bin/biome", import.meta.url));

// Runs the npm launcher with `BIOME_BINARY` pointing to a fake binary that
// executes the given shell script.
function runLauncher(dir, name, script) {
	const binary = join(dir, name);
	writeFileSync(binary, `#!/bin/sh\n${script}\n`);
	chmodSync(binary, 0o755);
	return spawnSync(process.execPath, [launcher, "check"], {
		env: { ...process.env, BIOME_BINARY: binary },
		stdio: "ignore",
	});
}

describe("npm launcher", { skip: process.platform === "win32" }, () => {
	let dir;

	before(() => {
		dir = mkdtempSync(join(tmpdir(), "biome-launcher-"));
	});

	after(() => {
		rmSync(dir, { recursive: true, force: true });
	});

	it("forwards the exit code of the binary", () => {
		const result = runLauncher(dir, "exit", "exit 3");
		assert.equal(result.signal, null);
		assert.equal(result.status, 3);
	});

	for (const signal of ["SIGABRT", "SIGKILL"]) {
		it(`exits with a failure when the binary is terminated by ${signal}`, () => {
			const result = runLauncher(dir, signal, `kill -${signal.slice(3)} $$`);
			assert.equal(result.signal, null);
			assert.equal(result.status, 128 + constants.signals[signal]);
		});
	}
});
