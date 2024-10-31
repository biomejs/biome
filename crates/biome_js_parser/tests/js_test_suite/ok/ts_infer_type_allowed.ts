type Args<F> = F extends (...args: infer A) => void ? A : never;
type A = T extends import("test").C<infer P> ? P : never
type A = T extends typeof Array<infer P> ? P : never
type A = T extends { set(a: infer P): number } ? P : never
type A = T extends { get(): infer P } ? P : never
type A = T extends { method(this: infer P): number } ? P : never
type valid9<T> = T extends Array<infer R> ? R : any;
type ContentBetweenBrackets<S> = S extends `[${infer T}]` ? T : never;
type WithSelectors<S> = S extends { getState: () => infer T }
    ? S & { use: { [K in keyof T]: () => T[K] } }
    : never;
type A = MyType extends (OtherType extends infer T ? infer U : InnerFalse) ? OuterTrue : OuterFalse
type Join<T extends unknown[], D extends string> =
     T extends [] ? '' :
     T extends [string | number | boolean | bigint] ? `${T[0]}` :
     T extends [string | number | boolean | bigint, ...infer U] ? `${T[0]}${D}${Join<U, D>}` :
     string;
type MatchPair<S extends string> = S extends `[${infer A},${infer B}]` ? [A, B] : unknown;
type FirstTwoAndRest<S extends string> = S extends `${infer A}${infer B}${infer R}` ? [`${A}${B}`, R] : unknown;
type Trim<S extends string> =
     S extends `${infer T} ` ? Trim<T> :
     S;
type Foo<T> = T extends `*${infer S}*` ? S : never;
type Unpacked<T> = T extends (infer U)[] ? U :
    T extends (...args: any[]) => infer U ? U :
    T extends Promise<infer U> ? U :
    T;
type ArgumentType<T extends (x: any) => any> = T extends (a: infer A) => any ? A : any;
type X3<T> = T extends { a: (x: infer U) => void, b: (x: infer U) => void } ? U : never;
type X1<T extends { x: any, y: any }> = T extends { x: infer X, y: infer Y } ? [X, Y] : any;
type T62<T> = U extends (infer U)[] ? U : U;
type T63<T> = T extends ((infer A) extends infer B ? infer C : infer D) ? string : number;
type T75<T> = T extends T74<infer U, infer U> ? T70<U> | T72<U> | T74<U, U> : never;
type Jsonified<T> = T extends string | number | boolean | null ? T
    : T extends undefined | Function ? never
    : T extends { toJSON(): infer R } ? R
    : T extends object ? JsonifiedObject<T>
: "what is this";
type T1 = F1 extends (...args: (infer T)) => void ? T : never;
type T2 = F1 extends (args: [...(infer T)]) => void ? T : never;
type T3<T> = T extends IsNumber<(infer N)> ? true : false;
type T4 = F1 extends (...args: ((infer T))) => void ? T : never;
type T5 = F1 extends (args: [...((infer T))]) => void ? T : never;
type T6<T> = T extends IsNumber<((infer N))> ? true : false;
type T7 = F1 extends (...args: ((((infer T))))) => void ? T : never;
type T8 = F1 extends (args: [...((((infer T))))]) => void ? T : never;
type T9<T> = T extends IsNumber<((((infer N))))> ? true : false;
type Prepend<E, T extends any[]> =
    ((head: E, ...args: T) => any) extends ((...args: infer U) => any)
    ? U
    : T;
