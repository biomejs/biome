export interface I {
	["p1"]: number

	"p2": number

	get ["p3"](): number

	get "p4"(): number

	set ["p3"](x: number)

	set "p4"(x: number)

	["m1"](): void

	"m2"(): void

	[""]: number
}

export type T = {
	["p1"]: number

	"p2": number

	get ["p3"](): number

	get "p4"(): number

	set ["p3"](x: number)

	set "p4"(x: number)

	["m1"](): void

	"m2"(): void

	[""]: number
}
