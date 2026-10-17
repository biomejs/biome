// should generate diagnostics

declare class DeclaredFactory {
	create<T>(): T;
}
declare const declared: DeclaredFactory;
declared.create<Promise<void>>();

class Factory {
	create<T>(): T {
		return undefined as T;
	}
}
const factory = new Factory();
factory.create<Promise<void>>();
