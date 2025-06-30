import type { Configuration, Diagnostic } from "@biomejs/wasm-bundler";
import * as moduleBundler from "@biomejs/wasm-bundler";
import { BiomeCommon } from "./common";

export type * from "./common";
export type { Configuration, Diagnostic };

export class Biome extends BiomeCommon<Configuration, Diagnostic> {
	constructor() {
		super(moduleBundler);
	}
}
