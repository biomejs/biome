/* should not generate diagnostics */
declare module "foo" {
	type Private = 1;
	export type Public = 2;
	export {};
}
