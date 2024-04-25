interface Example {
	(): string;
}

function foo(example: { (): number }): number {
	return example();
}

// Interface with single call signature
interface SingleCall {
	(): number;
}

// Object type literal with single call signature
let obj: { (): string };

// Interface with a call signature and other properties
interface MixedInterface {
	(): void;
	prop: number;
}

// Nested object types with call signatures
let nestedObj: { inner: { (): boolean } };

// Object type with call signature as a type union member
type UnionWithCallSignature = { (): string } | string;

// Object type with call signature as a type intersection member
export type IntersectionCallSignature = { (): string } & string;

// Object type with call signature as a type array
export type ArrayCallSignature = readonly { (): string }[];

// Generic object type with a call signature
type GenericCallSignature<T> = { (arg: T): T };

// Object type with optional call signature
let optionalCall: { (): number | undefined };

// Generic interface with a call signature
interface GenericInterface<T> {
	(value: T): boolean;
}