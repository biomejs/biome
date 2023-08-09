/**
 * Gets the path of the Rome binary for the current platform
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
			x64: "@rometools/cli-win32-x64/rome.exe",
			arm64: "@rometools/cli-win32-arm64/rome.exe",
		},
		darwin: {
			x64: "@rometools/cli-darwin-x64/rome",
			arm64: "@rometools/cli-darwin-arm64/rome",
		},
		linux: {
			x64: "@rometools/cli-linux-x64/rome",
			arm64: "@rometools/cli-linux-arm64/rome",
		},
	};

	const binPath = PLATFORMS?.[platform]?.[arch];
	if (!binPath) {
		return null;
	}

	return require.resolve(binPath);
}
