import type {
	Configuration as ConfigurationBundler,
	Diagnostic as DiagnosticBundler,
} from "@biomejs/wasm-bundler";
import type {
	Configuration as ConfigurationNodejs,
	Diagnostic as DiagnosticNodeJs,
} from "@biomejs/wasm-nodejs";
import type {
	Configuration as ConfigurationWeb,
	Diagnostic as DiagnosticWeb,
} from "@biomejs/wasm-web";
import { BiomeCommon } from "./common";

export type * from "./common";
export type Configuration =
	| ConfigurationBundler
	| ConfigurationNodejs
	| ConfigurationWeb;
export type Diagnostic = DiagnosticBundler | DiagnosticNodeJs | DiagnosticWeb;

/**
 * What kind of client Biome should use to communicate with the binary
 */
export enum Distribution {
	/**
	 * Use this if you want to communicate with the WebAssembly client built for
	 * bundlers
	 */
	BUNDLER = 0,
	/**
	 * Use this if you want to communicate with the WebAssembly client built for
	 * Node.JS
	 */
	NODE = 1,
	/**
	 * Use this if you want to communicate with the WebAssembly client built for
	 * the Web
	 */
	WEB = 2,
}

export interface BiomeCreate {
	distribution: Distribution;
}

export class Biome extends BiomeCommon<Configuration, Diagnostic> {
	/**
	 * It creates a new instance of the class {Biome}.
	 */
	static async create({ distribution }: BiomeCreate): Promise<Biome> {
		switch (distribution) {
			case Distribution.BUNDLER:
				return new Biome(await import("@biomejs/wasm-bundler"));
			case Distribution.NODE:
				return new Biome(await import("@biomejs/wasm-nodejs"));
			case Distribution.WEB:
				return new Biome(await import("@biomejs/wasm-web"));
			default:
				throw new Error(`Unknown distribution: ${distribution}`);
		}
	}
}
