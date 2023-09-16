type A = number;
type B = string extends number ? string : number;
type C = A extends (B extends A ? number : string) ? void : number;
type D<T> = T extends [infer S extends string, ...unknown[]] ? S : never;
type E<U, T> = T extends (infer U extends number ? U : T ) ? U : T
type F<T> = T extends { [P in infer U extends keyof T ? 1 : 0]: 1; } ? 1 : 0;
type G<T> = T extends [unknown, infer S extends string] ? S : never;
type H = A extends () => B extends C ? D : E ? F : G;
type J<T> = T extends ((...a: any[]) => infer R extends string) ? R : never;
type Equals = A extends (x: B extends C ? D : E) => 0 ? F : G;
type Curry<F extends ((...args: any) => any)> =
    <T extends any[]>(...args: Tools.Cast<Tools.Cast<T, Gaps<Parameters<F>>>, any[]>) =>
         GapsOf<T, Parameters<F>> extends [any, ...any[]]
         ? Curry<(...args: GapsOf<T, Parameters<F>> extends infer G ? Tools.Cast<G, any[]> : never) => ReturnType<F>>
         : ReturnType<F>;
interface GapsOfWorker<T1 extends any[], T2 extends any[], TN extends any[] = [], I extends any[] = []> {
    0: GapsOf<T1, T2, GapOf<T1, T2, TN, I> extends infer G ? Tools.Cast<G, any[]> : never, Tools.Next<I>>;
    1: Tools.Concat<TN, Tools.Drop<Tools.Pos<I>, T2> extends infer D ? Tools.Cast<D, any[]> : never>;
}
