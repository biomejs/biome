declare module "test";


declare module "a.b.c"

type OptionsFlags<Type> = {
	[Property in keyof Type]: boolean;
};

declare function test(): string;

export declare function abcd(): string;

declare let a;
