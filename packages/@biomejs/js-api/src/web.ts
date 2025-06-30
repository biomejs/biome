import type { Configuration, Diagnostic } from "@biomejs/wasm-web";
import * as moduleWeb from "@biomejs/wasm-web";
import { BiomeCommon } from "./common";

export type * from "./common";
export type { Configuration, Diagnostic };

export class Biome extends BiomeCommon<Configuration, Diagnostic> {
	constructor() {
		super(moduleWeb);
	}
}
