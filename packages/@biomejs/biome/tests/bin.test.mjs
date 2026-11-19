import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { test } from "node:test";
import { runInNewContext } from "node:vm";

const source = readFileSync(new URL("../bin/biome", import.meta.url), "utf8");
const nativeRequire = createRequire(import.meta.url);

function runWrapper({
	arch = "x64",
	platform = "linux",
	report,
	ldd = {},
	env = {},
}) {
	let launchedBinary;
	const childProcess = {
		execSync() {
			if (ldd.stdout !== undefined && !ldd.error) {
				return Buffer.from(ldd.stdout);
			}
			const error = new Error("ldd failed");
			error.stdout =
				ldd.stdout === undefined ? undefined : Buffer.from(ldd.stdout);
			error.stderr =
				ldd.stderr === undefined ? undefined : Buffer.from(ldd.stderr);
			throw error;
		},
		spawnSync(binary) {
			launchedBinary = binary;
			return { status: 0 };
		},
	};
	const require = (id) =>
		id === "child_process" || id === "node:child_process"
			? childProcess
			: nativeRequire(id);
	require.resolve = (id) => id;
	const process = {
		arch,
		platform,
		env,
		report,
		version: "v22.21.1",
		release: { name: "node" },
		argv: ["node", "biome", "--version"],
	};
	runInNewContext(source, { process, require, console });
	assert.equal(process.exitCode, 0);
	return launchedBinary;
}

for (const arch of ["x64", "arm64"]) {
	for (const musl of [false, true]) {
		test(`${arch}: selects ${musl ? "musl" : "glibc"} without ldd`, () => {
			assert.equal(
				runWrapper({
					arch,
					report: {
						getReport: () =>
							musl
								? {
										sharedObjects: [
											`/lib/ld-musl-${arch === "x64" ? "x86_64" : "aarch64"}.so.1`,
										],
									}
								: { header: { glibcVersionRuntime: "2.39" } },
					},
				}),
				`@biomejs/cli-linux-${arch}${musl ? "-musl" : ""}/biome`,
			);
		});
	}
}

for (const { name, report, ldd, musl } of [
	{
		name: "musl libc shared object without ldd",
		report: { sharedObjects: ["/lib/libc.musl-x86_64.so.1"] },
		musl: true,
	},
	{
		name: "glibc runtime with musl also loaded",
		report: {
			header: { glibcVersionRuntime: "2.39" },
			sharedObjects: ["/lib/ld-musl-x86_64.so.1"],
		},
	},
	{
		name: "static glibc Node with glibc ldd",
		report: { header: { glibcVersionCompiler: "2.39" } },
		ldd: { stdout: "ldd (GNU libc) 2.39" },
	},
	{
		name: "glibc compiler metadata in a musl environment",
		report: { header: { glibcVersionCompiler: "2.39" } },
		ldd: { stderr: "musl libc", error: true },
		musl: true,
	},
	{
		name: "musl ldd stdout with inconclusive report",
		report: { header: {}, sharedObjects: [] },
		ldd: { stdout: "musl libc" },
		musl: true,
	},
	{
		name: "musl ldd stderr on failure",
		report: {},
		ldd: { stderr: "musl libc", error: true },
		musl: true,
	},
	{
		name: "musl ldd stdout on failure",
		report: {},
		ldd: { stdout: "musl libc", stderr: "", error: true },
		musl: true,
	},
	{
		name: "inconclusive report without ldd defaults to glibc",
		report: { header: {}, sharedObjects: [] },
	},
	{
		name: "unknown libc defaults to glibc",
		report: {},
		ldd: { stdout: "unrecognized libc" },
	},
]) {
	test(name, () => {
		assert.equal(
			runWrapper({ report: { getReport: () => report }, ldd }),
			`@biomejs/cli-linux-x64${musl ? "-musl" : ""}/biome`,
		);
	});
}

for (const [name, report] of [
	["missing report API", undefined],
	[
		"throwing report API",
		{
			getReport() {
				throw new Error("report unavailable");
			},
		},
	],
]) {
	test(`${name} falls back to ldd`, () => {
		assert.equal(
			runWrapper({ report, ldd: { stdout: "musl libc" } }),
			"@biomejs/cli-linux-x64-musl/biome",
		);
	});
}

test("BIOME_BINARY overrides libc detection", () => {
	assert.equal(
		runWrapper({
			env: { BIOME_BINARY: "/custom/biome" },
			ldd: { stdout: "musl libc" },
		}),
		"/custom/biome",
	);
});

test("non-Linux platform ignores musl evidence", () => {
	assert.equal(
		runWrapper({
			platform: "darwin",
			report: {
				getReport: () => ({ sharedObjects: ["/lib/ld-musl-x86_64.so.1"] }),
			},
			ldd: { stdout: "musl libc" },
		}),
		"@biomejs/cli-darwin-x64/biome",
	);
});

for (const excludeNetwork of [false, true]) {
	for (const fails of [false, true]) {
		test(`preserves excludeNetwork=${excludeNetwork} after ${fails ? "failed" : "successful"} report`, () => {
			const report = {
				excludeNetwork,
				getReport() {
					if (fails || !this.excludeNetwork) {
						throw new Error("report unavailable");
					}
					return { sharedObjects: ["/lib/ld-musl-x86_64.so.1"] };
				},
			};
			assert.equal(
				runWrapper({
					report,
					ldd: fails ? { stdout: "musl libc" } : undefined,
				}),
				"@biomejs/cli-linux-x64-musl/biome",
			);
			assert.equal(report.excludeNetwork, excludeNetwork);
		});
	}
}
