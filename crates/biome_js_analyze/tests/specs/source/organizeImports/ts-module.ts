declare module "module" {
	import type { B } from "b";
	import type { A } from "a";

	type X = [A, B];
}
