/**
 * Gets the path of the Biome binary for the current platform
 *
 * @returns Filesystem path to the binary, or null if no prebuilt distribution exists for the current platform
 */
export function getCommand(): string | null {
	const { platform, arch } = process;

	type PlatformPaths = {
		[P in NodeJS.Platform]?: {
			[A in NodeJS.Architecture]?: string;
		};
	};

	const PLATFORMS: PlatformPaths = {
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

	const binPath = PLATFORMS?.[platform]?.[arch];
	if (!binPath) {
		return null;
	}

	return require.resolve(binPath);
}
