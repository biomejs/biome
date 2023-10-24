interface I {
	get<T>(f: T): T;
	post<T>(g: T): T;
}

type A = {
	get<T>(f: T): T;
	post<T>(g: T): T;
}

declare class C {
	get<T>(f: T): T;
	post<T>(g: T): T;
}
