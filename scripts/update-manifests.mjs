import * as fs from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { format } from "node:util";

const REPO_ROOT = resolve(fileURLToPath(import.meta.url), "../..");
const PACKAGES_ROOT = resolve(REPO_ROOT, "packages/@biomejs");
const BIOME_LIB_PATH = resolve(PACKAGES_ROOT, "biome");
const MANIFEST_PATH = resolve(BIOME_LIB_PATH, "package.json");

const PLATFORMS = ["win32-%s", "darwin-%s", "linux-%s", "linux-%s-musl"];
const ARCHITECTURES = ["x64", "arm64"];
const WASM_TARGETS = ["bundler", "nodejs", "web"];


const rootManifest = JSON.parse(
	fs.readFileSync(MANIFEST_PATH).toString("utf-8"),
);


for (const platform of PLATFORMS) {
	for (const arch of ARCHITECTURES) {
		updateOptionalDependencies(platform, arch);
	}
}

for (const target of WASM_TARGETS) {
	updateWasmPackage(target);
}


function getName(platform, arch, prefix = "cli") {
	return format(`${prefix}-${platform}`, arch);
}

function updateOptionalDependencies(platform, arch) {
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
			repository: {
				...repository,
				directory: repository.directory + "/" + buildName
			},
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
	console.log(`Update manifest ${manifestPath}`);
	fs.writeFileSync(manifestPath, manifest);

	// Copy the CLI binary
	const ext = os === "win32" ? ".exe" : "";
	const binarySource = resolve(
		REPO_ROOT,
		`${getName(platform, arch, "biome")}${ext}`,
	);
	const binaryTarget = resolve(packageRoot, `biome${ext}`);

	if (fs.existsSync(binaryTarget)) {
	console.log(`Copy binary ${binaryTarget}`);

	fs.copyFileSync(binarySource, binaryTarget);
	fs.chmodSync(binaryTarget, 0o755);
	}
}

function updateWasmPackage(target) {
	const packageName = `@biomejs/wasm-${target}`;
	const packageRoot = resolve(PACKAGES_ROOT, `wasm-${target}`);

	const manifestPath = resolve(packageRoot, "package.json");
	const manifest = JSON.parse(fs.readFileSync(manifestPath).toString("utf-8"));

	const { version, repository } = rootManifest;
	manifest.name = packageName;
	manifest.version = version;
	manifest.repository = {
		...repository,
		directory: repository.directory + `/wasm-${target}`
	}

	console.log(`Update manifest ${manifestPath}`);
	fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2));
}
