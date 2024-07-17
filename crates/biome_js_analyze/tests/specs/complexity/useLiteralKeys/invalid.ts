export interface I {
	["p1"]: number

	get ["p2"](): number

	set ["p2"](x: number)

	["m1"](): void

	[""]: number
}

export type T = {
	["p1"]: number

	get ["p2"](): number

	set ["p2"](x: number)

	["m1"](): void

	[""]: number
}

export enum E {
	["A"],
	["B"],
}
