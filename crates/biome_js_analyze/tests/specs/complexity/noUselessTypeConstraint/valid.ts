interface FooAny0<T> {
	field: T;
}

interface FooNotAny0<T extends string> {
	field: T;
}

type Bar<T> = {};

type Bar2<T extends string> = {};
