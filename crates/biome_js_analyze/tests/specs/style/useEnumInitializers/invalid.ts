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

const RED = 0;
export enum RgbColor {
	Red = RED,
	Green,
	Blue,
}

export enum RgbColor2 {
	Red = RED,
	Green = 5,
	Blue,
}

const GREEN = 0;
export enum RgbColor3 {
	Red,
	Green = GREEN,
	Blue,
}

// https://github.com/biomejs/biome/issues/1640
export enum WithComment {
  First = 1, // Comment1
  Second, // Comment2
  Third // Comment3
}
