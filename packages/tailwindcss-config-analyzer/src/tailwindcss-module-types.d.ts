declare module "tailwindcss/lib/lib/setupContextUtils" {
	export function createContext<Config extends import("tailwindcss").Config>(
		config: ReturnType<typeof import("tailwindcss/resolveConfig")<Config>>,
	): import("./types.ts").TailwindContext;
}
