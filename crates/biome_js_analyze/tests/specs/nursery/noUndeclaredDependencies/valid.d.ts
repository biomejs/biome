declare module "jest";
declare module "*.scss" {
	const content: Record<string, string>;
	export default content;
}
