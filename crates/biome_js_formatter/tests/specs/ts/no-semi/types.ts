type A = {   a: string ; b: number ; c: string }

type B = {
	a: string
	b: number
	c: string
}

interface C {   a: string ; b: number ; c: string }

type OptionsFlags<Type> = {
	[Property in keyof Type]: boolean;
};
