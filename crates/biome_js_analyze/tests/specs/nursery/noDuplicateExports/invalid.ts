/* should generate diagnostics */
type A = string;
type B = number;
export type { A };
export type { B as A };

export type C = string;
export { type B as C };

export interface D {}
export type { D } from "./d";

export type * as E from "./e";
export type { E } from "./e";

export function f(value: string): void;
export function f(value: number): void;
export function f(value: unknown) {}
export { g as f } from "./g";

export enum H {}
export { H } from "./h";

export namespace I {
	export const x = 1;
}
export * as I from "./i";

export namespace J.K {
	export const x = 1;
}
export { j as J } from "./j";

export declare const L: string;
export { L } from "./l";

export import M = I;
export { M } from "./m";

export namespace N {
	const O = 1;
	export const P = 1;
	export { O as P };
}

declare module "q" {
	const R = 1;
	export { R };
	export { R };
}

const S = 1;
type T = number;
export { S as T };
export type { T };

export { type U } from "./u";
export { U } from "./u";

export * as V from "./v";
export type { V } from "./v";

export class W {}
export type { X as W } from "./x";

export enum Y {}
export type { Y } from "./y";

export const Z = 1;
export type { Z } from "./z";

export function AA() {}
export type { T as AA } from "./t";

export import AB = I;
export type { T as AB } from "./t";
