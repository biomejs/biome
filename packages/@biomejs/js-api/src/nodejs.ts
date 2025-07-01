import type { Configuration, Diagnostic } from "@biomejs/wasm-nodejs";
import * as moduleNodeJs from "@biomejs/wasm-nodejs";
import { BiomeCommon } from "./common";

export type * from "./common";
export type { Configuration, Diagnostic };

export class Biome extends BiomeCommon<Configuration, Diagnostic> {
	constructor() {
		super(moduleNodeJs);
	}
}
