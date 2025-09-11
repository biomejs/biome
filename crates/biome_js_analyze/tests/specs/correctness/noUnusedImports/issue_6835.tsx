import * as postcssModules from "postcss-modules"

type PostcssOptions = Parameters<postcssModules>[0]

export function f(options: PostcssOptions) {
	console.log(options)
}
