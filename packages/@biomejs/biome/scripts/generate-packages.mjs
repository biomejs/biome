import * as fs from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { format } from "node:util";

const CLI_ROOT = resolve(fileURLToPath(import.meta.url), "../..");
const PACKAGES_ROOT = resolve(CLI_ROOT, "..");
const REPO_ROOT = resolve(PACKAGES_ROOT, "../..");
const MANIFEST_PATH = resolve(CLI_ROOT, "package.json");

const rootManifest = JSON.parse(
	fs.readFileSync(MANIFEST_PATH).toString("utf-8"),
);

function getName(platform, arch, prefix = "cli") {
	return format(`${prefix}-${platform}`, arch);
}

function copyBinaryToNativePackage(platform, arch) {
	const os = platform.split("-")[0];
	const buildName = getName(platform, arch);
	const packageRoot = resolve(PACKAGES_ROOT, buildName);
	const packageName = `@biomejs/${buildName}`;

	// Update the package.json manifest
	const { version, license, repository, engines, homepage } = rootManifest;

	const manifest = JSON.stringify(
		{
			name: packageName,
			version,
			license,
			repository,
			engines,
			homepage,
			os: [os],
			cpu: [arch],
			libc:
				os === "linux"
					? packageName.endsWith("musl")
						? ["musl"]
						: ["glibc"]
					: undefined,
		},
		null,
		2,
	);

	const manifestPath = resolve(packageRoot, "package.json");
	console.info(`Update manifest ${manifestPath}`);
	fs.writeFileSync(manifestPath, manifest);

	// Copy the CLI binary
	const ext = os === "win32" ? ".exe" : "";
	const binarySource = resolve(
		REPO_ROOT,
		`${getName(platform, arch, "biome")}${ext}`,
	);
	const binaryTarget = resolve(packageRoot, `biome${ext}`);

	if (!fs.existsSync(binarySource)) {
		console.error(
			`Source for binary for ${buildName} not found at: ${binarySource}`,
		);
		process.exit(1);
	}

	console.info(`Copy binary ${binaryTarget}`);
	fs.copyFileSync(binarySource, binaryTarget);
	fs.chmodSync(binaryTarget, 0o755);
}

/**
 * The wasm-pack binary changes the package name and version to use the ones coming from `biome_wasm/Cargo.toml`.
 * This function updates name and version of the `package.json` to match the ones of `@biomejs/biome`
 * @param target
 */
function updateWasmPackage(target) {
	const packageName = `@biomejs/wasm-${target}`;
	const packageRoot = resolve(PACKAGES_ROOT, `wasm-${target}`);

	const manifestPath = resolve(packageRoot, "package.json");
	const manifest = JSON.parse(fs.readFileSync(manifestPath).toString("utf-8"));

	const { version } = rootManifest;
	manifest.name = packageName;
	manifest.version = version;

	console.info(`Update manifest ${manifestPath}`);
	fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2));
}

const PLATFORMS = ["win32-%s", "darwin-%s", "linux-%s", "linux-%s-musl"];
const ARCHITECTURES = ["x64", "arm64"];
const WASM_TARGETS = ["bundler", "nodejs", "web"];

for (const target of WASM_TARGETS) {
	updateWasmPackage(target);
}

for (const platform of PLATFORMS) {
	for (const arch of ARCHITECTURES) {
		copyBinaryToNativePackage(platform, arch);
	}
}
