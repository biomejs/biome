import assert from "node:assert/strict";
import { existsSync } from "node:fs";
import { fileURLToPath } from "node:url";

// Tests run against the Biome binary built from this repository by default.
process.env.BIOME_BINARY ??= fileURLToPath(
	new URL(
		`../../../../target/release/biome${process.platform === "win32" ? ".exe" : ""}`,
		import.meta.url,
	),
);

assert(
	existsSync(process.env.BIOME_BINARY),
	`${process.env.BIOME_BINARY} does not exist; run \`cargo build -p biome_cli --release\` or set BIOME_BINARY`,
);
