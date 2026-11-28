/* should not generate diagnostics */
const A = 1;
type A = string;
export { A };

const B = 1;
export { B as C };
export type C = number;

export namespace X {
	export const x = 1;
}
export type { T as X } from "./t";

export declare function Y(): void;
export type { T as Y } from "./t";

export const D = 1;
export type D = string;

export const E = 1;
export interface E {}

export interface F {}
export { f as F } from "./f";

export class G {}
export interface G {}

export function h(value: string): void;
export function h(value: number): void;
export function h(value: unknown) {}

export class I {}
export namespace I {
	export const I = 1;
}

export enum J {}
export namespace J {}

export namespace K {
	export const L = 1;
}
export const L = 1;

export namespace M {
	export const N = 1;
	export namespace O {
		export const N = 2;
	}
}

declare module "p" {
	const Q = 1;
	export { Q as default };
}
declare module "r" {
	const S = 2;
	export { S as default };
}

export default interface T {}
const u = 1;
export { u as default };

export * as V from "./v";
export type * from "./w";

export type AA = string;
export type * as AA from "./aa";
