export type OtherType = {
	[K in keyof number]: number[K];
};

type Flatten<Type> = Type extends Array<infer Item> ? Item : Type;

class Foo<T> {
	id(x: T): T {
		return x;
	}
}