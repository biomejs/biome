/* should generate diagnostics */
export default function foo(value: string): void;
export default function foo(value: unknown) {}
const bar = 1;
export { bar as default };

declare module "z" {
	export default class Z {}
	export type { Y as default } from "./y";
}

declare module "aa" {
	const a: number;
	export default a;
	export type { T as default } from "./t";
}
