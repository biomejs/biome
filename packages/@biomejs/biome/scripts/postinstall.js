const { platform, arch } = process;

const PLATFORMS = {
	win32: {
		x64: "@biomejs/cli-win32-x64/biome.exe",
		arm64: "@biomejs/cli-win32-arm64/biome.exe",
	},
	darwin: {
		x64: "@biomejs/cli-darwin-x64/biome",
		arm64: "@biomejs/cli-darwin-arm64/biome",
	},
	linux: {
		x64: "@biomejs/cli-linux-x64/biome",
		arm64: "@biomejs/cli-linux-arm64/biome",
	},
};

const binName = PLATFORMS?.[platform]?.[arch];
if (binName) {
	let binPath;
	try {
		binPath = require.resolve(binName);
	} catch {
		console.warn(
			`The Biome CLI postinstall script failed to resolve the binary file "${binName}". Running Biome from the npm package will probably not work correctly.`
		);
	}
} else {
	console.warn(
		"The Biome CLI package doesn't ship with prebuilt binaries for your platform yet. " +
			"You can still use the CLI by cloning the biomejs/biome repo from GitHub, " +
			"and follow the instructions there to build the CLI for your platform."
	);
}
