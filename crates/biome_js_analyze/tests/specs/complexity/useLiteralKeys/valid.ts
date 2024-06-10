export interface I {
	p1: number

	get p2(): number

	set p2(x: number)

	m1(): void

	"p3": number

	get "p4"(): number

	set "p4"(x: number)

	"m2"(): void
}

export type T = {
	p1: number

	get p2(): number

	set p2(x: number)

	m1(): void

	"p3": number

	get "p4"(): number

	set "p4"(x: number)

	"m2"(): void
}
