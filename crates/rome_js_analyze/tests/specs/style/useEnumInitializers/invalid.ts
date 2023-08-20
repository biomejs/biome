export enum Status {
	Close,
	MidClose = 1,
	MidOpen = 10,
	/* implicit */ Open /* 11 */,
}

export enum ComputedFlags {
	Flag1 = 1,
	Flag2 = 1 << 1,
	Flag3,
}

export enum Direction {
	Down,
	Left,
	Right,
	Up,
}

export enum Color {
	Red = "Red",
	Green = "Green",
	Blue,
}

export enum Exotic {
	A = 0.1,
	B,
	C = "Special",
	D,
}

export enum IndexedColor {
	Red = "0",
	Green = "1",
	Blue,
}

export namespace A {
    export namespace B {
        export enum Enum {
            A,
            B,
        }
    }
}
