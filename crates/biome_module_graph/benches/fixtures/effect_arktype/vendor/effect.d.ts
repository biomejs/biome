type UnionToIntersection<T> = (T extends any ? (x: T) => any : never) extends (x: infer R) => any ? R : never;
type Simplify<A> = {
    [K in keyof A]: A[K];
} extends infer B ? B : never;
type EqualsWith<A, B, Y, N> = (<T>() => T extends A ? 1 : 2) extends (<T>() => T extends B ? 1 : 2) ? Y : N;
type Concurrency = number | "unbounded" | "inherit";
type NoInfer$1<A> = [
    A
][A extends any ? 0 : never];
type Invariant<A> = (_: A) => A;
declare namespace Invariant {
    type Type<A> = A extends Invariant<infer U> ? U : never;
}
type Covariant<A> = (_: never) => A;
declare namespace Covariant {
    type Type<A> = A extends Covariant<infer U> ? U : never;
}
type Contravariant<A> = (_: A) => void;
declare namespace Contravariant {
    type Type<A> = A extends Contravariant<infer U> ? U : never;
}
type NotFunction<T> = T extends Function ? never : T;
type NoExcessProperties<T, U> = T & {
    readonly [K in Exclude<keyof U, keyof T>]: never;
};
type VoidIfEmpty<S> = keyof S extends never ? void : S;
interface TypeLambda {
    readonly In: unknown;
    readonly Out2: unknown;
    readonly Out1: unknown;
    readonly Target: unknown;
}
type Kind<F extends TypeLambda, In, Out2, Out1, Target> = F extends {
    readonly type: unknown;
} ? (F & {
    readonly In: In;
    readonly Out2: Out2;
    readonly Out1: Out1;
    readonly Target: Target;
})["type"] : {
    readonly F: F;
    readonly In: Contravariant<In>;
    readonly Out2: Covariant<Out2>;
    readonly Out1: Covariant<Out1>;
    readonly Target: Invariant<Target>;
};
interface LazyArg<A> {
    (): A;
}
declare function pipe<A>(a: A): A;
declare function pipe<A, B = never>(a: A, ab: (a: A) => B): B;
declare function pipe<A, B = never, C = never>(a: A, ab: (a: A) => B, bc: (b: B) => C): C;
declare function pipe<A, B = never, C = never, D = never>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D): D;
declare function pipe<A, B = never, C = never, D = never, E = never>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E): E;
declare function pipe<A, B = never, C = never, D = never, E = never, F = never>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F): F;
declare function pipe<A, B = never, C = never, D = never, E = never, F = never, G = never>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G): G;
declare function pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H): H;
declare function pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never, I = never>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I): I;
declare function pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never, I = never, J = never>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J): J;
declare function pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never, I = never, J = never, K = never>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K): K;
declare function pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never, I = never, J = never, K = never, L = never>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L): L;
declare function pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never, I = never, J = never, K = never, L = never, M = never>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L, lm: (l: L) => M): M;
declare function pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never, I = never, J = never, K = never, L = never, M = never, N = never>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L, lm: (l: L) => M, mn: (m: M) => N): N;
declare function pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never, I = never, J = never, K = never, L = never, M = never, N = never, O = never>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L, lm: (l: L) => M, mn: (m: M) => N, no: (n: N) => O): O;
declare function pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never, I = never, J = never, K = never, L = never, M = never, N = never, O = never, P = never>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L, lm: (l: L) => M, mn: (m: M) => N, no: (n: N) => O, op: (o: O) => P): P;
declare function pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never, I = never, J = never, K = never, L = never, M = never, N = never, O = never, P = never, Q = never>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L, lm: (l: L) => M, mn: (m: M) => N, no: (n: N) => O, op: (o: O) => P, pq: (p: P) => Q): Q;
declare function pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never, I = never, J = never, K = never, L = never, M = never, N = never, O = never, P = never, Q = never, R = never>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L, lm: (l: L) => M, mn: (m: M) => N, no: (n: N) => O, op: (o: O) => P, pq: (p: P) => Q, qr: (q: Q) => R): R;
declare function pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never, I = never, J = never, K = never, L = never, M = never, N = never, O = never, P = never, Q = never, R = never, S = never>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L, lm: (l: L) => M, mn: (m: M) => N, no: (n: N) => O, op: (o: O) => P, pq: (p: P) => Q, qr: (q: Q) => R, rs: (r: R) => S): S;
declare function pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never, I = never, J = never, K = never, L = never, M = never, N = never, O = never, P = never, Q = never, R = never, S = never, T = never>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L, lm: (l: L) => M, mn: (m: M) => N, no: (n: N) => O, op: (o: O) => P, pq: (p: P) => Q, qr: (q: Q) => R, rs: (r: R) => S, st: (s: S) => T): T;
interface Equivalence$1<in A> {
    (self: A, that: A): boolean;
}
declare const symbol$2: unique symbol;
interface Hash {
    [symbol$2](): number;
}
declare const symbol$1: unique symbol;
interface Equal extends Hash {
    [symbol$1](that: Equal): boolean;
}
declare const nonEmpty: unique symbol;
interface NonEmptyIterable<out A> extends Iterable<A> {
    readonly [nonEmpty]: A;
}
interface Order$1<in A> {
    (self: A, that: A): -1 | 0 | 1;
}
interface Pipeable {
    pipe<A>(this: A): A;
    pipe<A, B = never>(this: A, ab: (_: A) => B): B;
    pipe<A, B = never, C = never>(this: A, ab: (_: A) => B, bc: (_: B) => C): C;
    pipe<A, B = never, C = never, D = never>(this: A, ab: (_: A) => B, bc: (_: B) => C, cd: (_: C) => D): D;
    pipe<A, B = never, C = never, D = never, E = never>(this: A, ab: (_: A) => B, bc: (_: B) => C, cd: (_: C) => D, de: (_: D) => E): E;
    pipe<A, B = never, C = never, D = never, E = never, F = never>(this: A, ab: (_: A) => B, bc: (_: B) => C, cd: (_: C) => D, de: (_: D) => E, ef: (_: E) => F): F;
    pipe<A, B = never, C = never, D = never, E = never, F = never, G = never>(this: A, ab: (_: A) => B, bc: (_: B) => C, cd: (_: C) => D, de: (_: D) => E, ef: (_: E) => F, fg: (_: F) => G): G;
    pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never>(this: A, ab: (_: A) => B, bc: (_: B) => C, cd: (_: C) => D, de: (_: D) => E, ef: (_: E) => F, fg: (_: F) => G, gh: (_: G) => H): H;
    pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never, I = never>(this: A, ab: (_: A) => B, bc: (_: B) => C, cd: (_: C) => D, de: (_: D) => E, ef: (_: E) => F, fg: (_: F) => G, gh: (_: G) => H, hi: (_: H) => I): I;
    pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never, I = never, J = never>(this: A, ab: (_: A) => B, bc: (_: B) => C, cd: (_: C) => D, de: (_: D) => E, ef: (_: E) => F, fg: (_: F) => G, gh: (_: G) => H, hi: (_: H) => I, ij: (_: I) => J): J;
    pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never, I = never, J = never, K = never>(this: A, ab: (_: A) => B, bc: (_: B) => C, cd: (_: C) => D, de: (_: D) => E, ef: (_: E) => F, fg: (_: F) => G, gh: (_: G) => H, hi: (_: H) => I, ij: (_: I) => J, jk: (_: J) => K): K;
    pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never, I = never, J = never, K = never, L = never>(this: A, ab: (_: A) => B, bc: (_: B) => C, cd: (_: C) => D, de: (_: D) => E, ef: (_: E) => F, fg: (_: F) => G, gh: (_: G) => H, hi: (_: H) => I, ij: (_: I) => J, jk: (_: J) => K, kl: (_: K) => L): L;
    pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never, I = never, J = never, K = never, L = never, M = never>(this: A, ab: (_: A) => B, bc: (_: B) => C, cd: (_: C) => D, de: (_: D) => E, ef: (_: E) => F, fg: (_: F) => G, gh: (_: G) => H, hi: (_: H) => I, ij: (_: I) => J, jk: (_: J) => K, kl: (_: K) => L, lm: (_: L) => M): M;
    pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never, I = never, J = never, K = never, L = never, M = never, N = never>(this: A, ab: (_: A) => B, bc: (_: B) => C, cd: (_: C) => D, de: (_: D) => E, ef: (_: E) => F, fg: (_: F) => G, gh: (_: G) => H, hi: (_: H) => I, ij: (_: I) => J, jk: (_: J) => K, kl: (_: K) => L, lm: (_: L) => M, mn: (_: M) => N): N;
    pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never, I = never, J = never, K = never, L = never, M = never, N = never, O = never>(this: A, ab: (_: A) => B, bc: (_: B) => C, cd: (_: C) => D, de: (_: D) => E, ef: (_: E) => F, fg: (_: F) => G, gh: (_: G) => H, hi: (_: H) => I, ij: (_: I) => J, jk: (_: J) => K, kl: (_: K) => L, lm: (_: L) => M, mn: (_: M) => N, no: (_: N) => O): O;
    pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never, I = never, J = never, K = never, L = never, M = never, N = never, O = never, P = never>(this: A, ab: (_: A) => B, bc: (_: B) => C, cd: (_: C) => D, de: (_: D) => E, ef: (_: E) => F, fg: (_: F) => G, gh: (_: G) => H, hi: (_: H) => I, ij: (_: I) => J, jk: (_: J) => K, kl: (_: K) => L, lm: (_: L) => M, mn: (_: M) => N, no: (_: N) => O, op: (_: O) => P): P;
    pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never, I = never, J = never, K = never, L = never, M = never, N = never, O = never, P = never, Q = never>(this: A, ab: (_: A) => B, bc: (_: B) => C, cd: (_: C) => D, de: (_: D) => E, ef: (_: E) => F, fg: (_: F) => G, gh: (_: G) => H, hi: (_: H) => I, ij: (_: I) => J, jk: (_: J) => K, kl: (_: K) => L, lm: (_: L) => M, mn: (_: M) => N, no: (_: N) => O, op: (_: O) => P, pq: (_: P) => Q): Q;
    pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never, I = never, J = never, K = never, L = never, M = never, N = never, O = never, P = never, Q = never, R = never>(this: A, ab: (_: A) => B, bc: (_: B) => C, cd: (_: C) => D, de: (_: D) => E, ef: (_: E) => F, fg: (_: F) => G, gh: (_: G) => H, hi: (_: H) => I, ij: (_: I) => J, jk: (_: J) => K, kl: (_: K) => L, lm: (_: L) => M, mn: (_: M) => N, no: (_: N) => O, op: (_: O) => P, pq: (_: P) => Q, qr: (_: Q) => R): R;
    pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never, I = never, J = never, K = never, L = never, M = never, N = never, O = never, P = never, Q = never, R = never, S = never>(this: A, ab: (_: A) => B, bc: (_: B) => C, cd: (_: C) => D, de: (_: D) => E, ef: (_: E) => F, fg: (_: F) => G, gh: (_: G) => H, hi: (_: H) => I, ij: (_: I) => J, jk: (_: J) => K, kl: (_: K) => L, lm: (_: L) => M, mn: (_: M) => N, no: (_: N) => O, op: (_: O) => P, pq: (_: P) => Q, qr: (_: Q) => R, rs: (_: R) => S): S;
    pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never, I = never, J = never, K = never, L = never, M = never, N = never, O = never, P = never, Q = never, R = never, S = never, T = never>(this: A, ab: (_: A) => B, bc: (_: B) => C, cd: (_: C) => D, de: (_: D) => E, ef: (_: E) => F, fg: (_: F) => G, gh: (_: G) => H, hi: (_: H) => I, ij: (_: I) => J, jk: (_: J) => K, kl: (_: K) => L, lm: (_: L) => M, mn: (_: M) => N, no: (_: N) => O, op: (_: O) => P, pq: (_: P) => Q, qr: (_: Q) => R, rs: (_: R) => S, st: (_: S) => T): T;
    pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never, I = never, J = never, K = never, L = never, M = never, N = never, O = never, P = never, Q = never, R = never, S = never, T = never, U = never>(this: A, ab: (_: A) => B, bc: (_: B) => C, cd: (_: C) => D, de: (_: D) => E, ef: (_: E) => F, fg: (_: F) => G, gh: (_: G) => H, hi: (_: H) => I, ij: (_: I) => J, jk: (_: J) => K, kl: (_: K) => L, lm: (_: L) => M, mn: (_: M) => N, no: (_: N) => O, op: (_: O) => P, pq: (_: P) => Q, qr: (_: Q) => R, rs: (_: R) => S, st: (_: S) => T, tu: (_: T) => U): U;
    pipe<A, B = never, C = never, D = never, E = never, F = never, G = never, H = never, I = never, J = never, K = never, L = never, M = never, N = never, O = never, P = never, Q = never, R = never, S = never, T = never, U = never>(this: A, ab: (_: A) => B, bc: (_: B) => C, cd: (_: C) => D, de: (_: D) => E, ef: (_: E) => F, fg: (_: F) => G, gh: (_: G) => H, hi: (_: H) => I, ij: (_: I) => J, jk: (_: J) => K, kl: (_: K) => L, lm: (_: L) => M, mn: (_: M) => N, no: (_: N) => O, op: (_: O) => P, pq: (_: P) => Q, qr: (_: Q) => R, rs: (_: R) => S, st: (_: S) => T, tu: (_: T) => U): U;
}
interface Predicate<in A> {
    (a: A): boolean;
}
declare namespace Predicate {
    type In<T extends Any> = [
        T
    ] extends [
        Predicate<infer _A>
    ] ? _A : never;
    type Any = Predicate<never>;
}
interface Refinement<in A, out B extends A> {
    (a: A): a is B;
}
declare namespace Refinement {
    type In<T extends Any> = [
        T
    ] extends [
        Refinement<infer _A, infer _>
    ] ? _A : never;
    type Out<T extends Any> = [
        T
    ] extends [
        Refinement<infer _, infer _B>
    ] ? _B : never;
    type Any = Refinement<any, any>;
}
declare const unifySymbol: unique symbol;
type unifySymbol = typeof unifySymbol;
declare const typeSymbol: unique symbol;
type typeSymbol = typeof typeSymbol;
declare const ignoreSymbol: unique symbol;
type ignoreSymbol = typeof ignoreSymbol;
type MaybeReturn<F> = F extends () => infer R ? R : NonNullable<F>;
type Values<X extends [
    any,
    any
]> = X extends [
    infer A,
    infer Ignore
] ? Exclude<keyof A, Ignore> extends infer k ? k extends keyof A ? MaybeReturn<A[k]> : never : never : never;
type Ignore<X> = X extends {
    [ignoreSymbol]?: infer Obj;
} ? keyof NonNullable<Obj> : never;
type ExtractTypes<X> = X extends {
    [typeSymbol]?: infer _Type;
    [unifySymbol]?: infer _Unify;
} ? [
    NonNullable<_Unify>,
    Ignore<X>
] : never;
type FilterIn<A> = A extends any ? typeSymbol extends keyof A ? A : never : never;
type FilterOut<A> = A extends any ? typeSymbol extends keyof A ? never : A : never;
type Unify<A> = Values<ExtractTypes<(FilterIn<A> & {
    [typeSymbol]: A;
})>> extends infer Z ? Z | Exclude<A, Z> | FilterOut<A> : never;
declare const GenKindTypeId: unique symbol;
type GenKindTypeId = typeof GenKindTypeId;
interface GenKind<F extends TypeLambda, R, O, E, A> extends Variance$1<F, R, O, E> {
    readonly value: Kind<F, R, O, E, A>;
    [Symbol.iterator](): IterableIterator<GenKind<F, R, O, E, A>, A>;
}
interface Variance$1<in out F extends TypeLambda, in R, out O, out E> {
    readonly [GenKindTypeId]: GenKindTypeId;
    readonly _F: Invariant<F>;
    readonly _R: Contravariant<R>;
    readonly _O: Covariant<O>;
    readonly _E: Covariant<E>;
}
interface Gen<F extends TypeLambda, Z> {
    <Self, K extends Variance$1<F, any, any, any> | YieldWrap<Kind<F, any, any, any, any>>, A>(...args: [
        self: Self,
        body: (this: Self, resume: Z) => Generator<K, A, never>
    ] | [
        body: (resume: Z) => Generator<K, A, never>
    ]): Kind<F, [
        K
    ] extends [
        Variance$1<F, infer R, any, any>
    ] ? R : [
        K
    ] extends [
        YieldWrap<Kind<F, infer R, any, any, any>>
    ] ? R : never, [
        K
    ] extends [
        Variance$1<F, any, infer O, any>
    ] ? O : [
        K
    ] extends [
        YieldWrap<Kind<F, any, infer O, any, any>>
    ] ? O : never, [
        K
    ] extends [
        Variance$1<F, any, any, infer E>
    ] ? E : [
        K
    ] extends [
        YieldWrap<Kind<F, any, any, infer E, any>>
    ] ? E : never, A>;
}
interface Adapter$1<Z extends TypeLambda> {
    <_R, _O, _E, _A>(self: Kind<Z, _R, _O, _E, _A>): GenKind<Z, _R, _O, _E, _A>;
    <A, _R, _O, _E, _A>(a: A, ab: (a: A) => Kind<Z, _R, _O, _E, _A>): GenKind<Z, _R, _O, _E, _A>;
    <A, B, _R, _O, _E, _A>(a: A, ab: (a: A) => B, bc: (b: B) => Kind<Z, _R, _O, _E, _A>): GenKind<Z, _R, _O, _E, _A>;
    <A, B, C, _R, _O, _E, _A>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => Kind<Z, _R, _O, _E, _A>): GenKind<Z, _R, _O, _E, _A>;
    <A, B, C, D, _R, _O, _E, _A>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => Kind<Z, _R, _O, _E, _A>): GenKind<Z, _R, _O, _E, _A>;
    <A, B, C, D, E, _R, _O, _E, _A>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => Kind<Z, _R, _O, _E, _A>): GenKind<Z, _R, _O, _E, _A>;
    <A, B, C, D, E, F, _R, _O, _E, _A>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => Kind<Z, _R, _O, _E, _A>): GenKind<Z, _R, _O, _E, _A>;
    <A, B, C, D, E, F, G, _R, _O, _E, _A>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: F) => Kind<Z, _R, _O, _E, _A>): GenKind<Z, _R, _O, _E, _A>;
    <A, B, C, D, E, F, G, H, _R, _O, _E, _A>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (g: H) => Kind<Z, _R, _O, _E, _A>): GenKind<Z, _R, _O, _E, _A>;
    <A, B, C, D, E, F, G, H, I, _R, _O, _E, _A>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => Kind<Z, _R, _O, _E, _A>): GenKind<Z, _R, _O, _E, _A>;
    <A, B, C, D, E, F, G, H, I, J, _R, _O, _E, _A>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => Kind<Z, _R, _O, _E, _A>): GenKind<Z, _R, _O, _E, _A>;
    <A, B, C, D, E, F, G, H, I, J, K, _R, _O, _E, _A>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => Kind<Z, _R, _O, _E, _A>): GenKind<Z, _R, _O, _E, _A>;
    <A, B, C, D, E, F, G, H, I, J, K, L, _R, _O, _E, _A>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L, lm: (l: L) => Kind<Z, _R, _O, _E, _A>): GenKind<Z, _R, _O, _E, _A>;
    <A, B, C, D, E, F, G, H, I, J, K, L, M, _R, _O, _E, _A>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L, lm: (l: L) => M, mn: (m: M) => Kind<Z, _R, _O, _E, _A>): GenKind<Z, _R, _O, _E, _A>;
    <A, B, C, D, E, F, G, H, I, J, K, L, M, N, _R, _O, _E, _A>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L, lm: (l: L) => M, mn: (m: M) => N, no: (n: N) => Kind<Z, _R, _O, _E, _A>): GenKind<Z, _R, _O, _E, _A>;
    <A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, _R, _O, _E, _A>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L, lm: (l: L) => M, mn: (m: M) => N, no: (n: N) => O, op: (o: O) => Kind<Z, _R, _O, _E, _A>): GenKind<Z, _R, _O, _E, _A>;
    <A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, _R, _O, _E, _A>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L, lm: (l: L) => M, mn: (m: M) => N, no: (n: N) => O, op: (o: O) => P, pq: (p: P) => Kind<Z, _R, _O, _E, _A>): GenKind<Z, _R, _O, _E, _A>;
    <A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, _R, _O, _E, _A>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L, lm: (l: L) => M, mn: (m: M) => N, no: (n: N) => O, op: (o: O) => P, pq: (p: P) => Q, qr: (q: Q) => Kind<Z, _R, _O, _E, _A>): GenKind<Z, _R, _O, _E, _A>;
    <A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, _R, _O, _E, _A>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L, lm: (l: L) => M, mn: (m: M) => N, no: (n: N) => O, op: (o: O) => P, pq: (p: P) => Q, qr: (q: Q) => R, rs: (r: R) => Kind<Z, _R, _O, _E, _A>): GenKind<Z, _R, _O, _E, _A>;
    <A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, _R, _O, _E, _A>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L, lm: (l: L) => M, mn: (m: M) => N, no: (n: N) => O, op: (o: O) => P, pq: (p: P) => Q, qr: (q: Q) => R, rs: (r: R) => S, st: (s: S) => Kind<Z, _R, _O, _E, _A>): GenKind<Z, _R, _O, _E, _A>;
    <A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, _R, _O, _E, _A>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L, lm: (l: L) => M, mn: (m: M) => N, no: (n: N) => O, op: (o: O) => P, pq: (p: P) => Q, qr: (q: Q) => R, rs: (r: R) => S, st: (s: S) => T, tu: (s: T) => Kind<Z, _R, _O, _E, _A>): GenKind<Z, _R, _O, _E, _A>;
}
declare const YieldWrapTypeId: unique symbol;
declare class YieldWrap<T> {
    #private;
    constructor(value: T);
    [YieldWrapTypeId](): T;
}
declare const TypeId$b: unique symbol;
type TypeId$b = typeof TypeId$b;
interface None$2<out A> extends Pipeable, Inspectable {
    readonly _tag: "None";
    readonly _op: "None";
    readonly [TypeId$b]: {
        readonly _A: Covariant<A>;
    };
    [typeSymbol]?: unknown;
    [unifySymbol]?: OptionUnify<this>;
    [ignoreSymbol]?: OptionUnifyIgnore;
}
interface Some<out A> extends Pipeable, Inspectable {
    readonly _tag: "Some";
    readonly _op: "Some";
    readonly value: A;
    readonly [TypeId$b]: {
        readonly _A: Covariant<A>;
    };
    [typeSymbol]?: unknown;
    [unifySymbol]?: OptionUnify<this>;
    [ignoreSymbol]?: OptionUnifyIgnore;
}
interface OptionUnify<A extends {
    [typeSymbol]?: any;
}> {
    Option?: () => A[typeSymbol] extends Option<infer A0> | infer _ ? Option<A0> : never;
}
type Option<A> = None$2<A> | Some<A>;
declare namespace Option {
    type Value<T extends Option<any>> = [
        T
    ] extends [
        Option<infer _A>
    ] ? _A : never;
}
interface OptionUnifyIgnore {
}
interface OptionTypeLambda extends TypeLambda {
    readonly type: Option<this["Target"]>;
}
declare const none$1: <A = never>() => Option<A>;
declare const some: <A>(value: A) => Option<A>;
declare const isOption: (input: unknown) => input is Option<unknown>;
declare const isNone: <A>(self: Option<A>) => self is None$2<A>;
declare const isSome: <A>(self: Option<A>) => self is Some<A>;
declare const match$6: {
    <B, A, C = B>(options: {
        readonly onNone: LazyArg<B>;
        readonly onSome: (a: A) => C;
    }): (self: Option<A>) => B | C;
    <A, B, C = B>(self: Option<A>, options: {
        readonly onNone: LazyArg<B>;
        readonly onSome: (a: A) => C;
    }): B | C;
};
declare const toRefinement: <A, B extends A>(f: (a: A) => Option<B>) => (a: A) => a is B;
declare const fromIterable: <A>(collection: Iterable<A>) => Option<A>;
declare const getRight$1: <R, L>(self: Either<R, L>) => Option<R>;
declare const getLeft$1: <R, L>(self: Either<R, L>) => Option<L>;
declare const getOrElse$3: {
    <B>(onNone: LazyArg<B>): <A>(self: Option<A>) => B | A;
    <A, B>(self: Option<A>, onNone: LazyArg<B>): A | B;
};
declare const orElse$4: {
    <B>(that: LazyArg<Option<B>>): <A>(self: Option<A>) => Option<B | A>;
    <A, B>(self: Option<A>, that: LazyArg<Option<B>>): Option<A | B>;
};
declare const orElseSome: {
    <B>(onNone: LazyArg<B>): <A>(self: Option<A>) => Option<B | A>;
    <A, B>(self: Option<A>, onNone: LazyArg<B>): Option<A | B>;
};
declare const orElseEither: {
    <B>(that: LazyArg<Option<B>>): <A>(self: Option<A>) => Option<Either<B, A>>;
    <A, B>(self: Option<A>, that: LazyArg<Option<B>>): Option<Either<B, A>>;
};
declare const firstSomeOf: <T, C extends Iterable<Option<T>> = Iterable<Option<T>>>(collection: C) => [
    C
] extends [
    Iterable<Option<infer A>>
] ? Option<A> : never;
declare const fromNullable$2: <A>(nullableValue: A) => Option<NonNullable<A>>;
declare const liftNullable: <A extends ReadonlyArray<unknown>, B>(f: (...a: A) => B | null | undefined) => (...a: A) => Option<NonNullable<B>>;
declare const getOrNull$1: <A>(self: Option<A>) => A | null;
declare const getOrUndefined$1: <A>(self: Option<A>) => A | undefined;
declare const liftThrowable: <A extends ReadonlyArray<unknown>, B>(f: (...a: A) => B) => (...a: A) => Option<B>;
declare const getOrThrowWith$1: {
    (onNone: () => unknown): <A>(self: Option<A>) => A;
    <A>(self: Option<A>, onNone: () => unknown): A;
};
declare const getOrThrow$1: <A>(self: Option<A>) => A;
declare const map$6: {
    <A, B>(f: (a: A) => B): (self: Option<A>) => Option<B>;
    <A, B>(self: Option<A>, f: (a: A) => B): Option<B>;
};
declare const as$4: {
    <B>(b: B): <X>(self: Option<X>) => Option<B>;
    <X, B>(self: Option<X>, b: B): Option<B>;
};
declare const asVoid$3: <_>(self: Option<_>) => Option<void>;
declare const void_$2: Option<void>;
declare const flatMap$5: {
    <A, B>(f: (a: A) => Option<B>): (self: Option<A>) => Option<B>;
    <A, B>(self: Option<A>, f: (a: A) => Option<B>): Option<B>;
};
declare const andThen$4: {
    <A, B>(f: (a: A) => Option<B>): (self: Option<A>) => Option<B>;
    <B>(f: Option<B>): <A>(self: Option<A>) => Option<B>;
    <A, B>(f: (a: A) => B): (self: Option<A>) => Option<B>;
    <B>(f: NotFunction<B>): <A>(self: Option<A>) => Option<B>;
    <A, B>(self: Option<A>, f: (a: A) => Option<B>): Option<B>;
    <A, B>(self: Option<A>, f: Option<B>): Option<B>;
    <A, B>(self: Option<A>, f: (a: A) => B): Option<B>;
    <A, B>(self: Option<A>, f: NotFunction<B>): Option<B>;
};
declare const flatMapNullable: {
    <A, B>(f: (a: A) => B | null | undefined): (self: Option<A>) => Option<NonNullable<B>>;
    <A, B>(self: Option<A>, f: (a: A) => B | null | undefined): Option<NonNullable<B>>;
};
declare const flatten$4: <A>(self: Option<Option<A>>) => Option<A>;
declare const zipRight$3: {
    <B>(that: Option<B>): <_>(self: Option<_>) => Option<B>;
    <X, B>(self: Option<X>, that: Option<B>): Option<B>;
};
declare const zipLeft$3: {
    <_>(that: Option<_>): <A>(self: Option<A>) => Option<A>;
    <A, X>(self: Option<A>, that: Option<X>): Option<A>;
};
declare const composeK: {
    <B, C>(bfc: (b: B) => Option<C>): <A>(afb: (a: A) => Option<B>) => (a: A) => Option<C>;
    <A, B, C>(afb: (a: A) => Option<B>, bfc: (b: B) => Option<C>): (a: A) => Option<C>;
};
declare const tap$2: {
    <A, X>(f: (a: A) => Option<X>): (self: Option<A>) => Option<A>;
    <A, X>(self: Option<A>, f: (a: A) => Option<X>): Option<A>;
};
declare const product: <A, B>(self: Option<A>, that: Option<B>) => Option<[
    A,
    B
]>;
declare const productMany: <A>(self: Option<A>, collection: Iterable<Option<A>>) => Option<[
    A,
    ...Array<A>
]>;
declare const all$3: <const I extends Iterable<Option<any>> | Record<string, Option<any>>>(input: I) => [
    I
] extends [
    ReadonlyArray<Option<any>>
] ? Option<{
    -readonly [K in keyof I]: [
        I[K]
    ] extends [
        Option<infer A>
    ] ? A : never;
}> : [
    I
] extends [
    Iterable<Option<infer A>>
] ? Option<Array<A>> : Option<{
    -readonly [K in keyof I]: [
        I[K]
    ] extends [
        Option<infer A>
    ] ? A : never;
}>;
declare const zipWith$5: {
    <B, A, C>(that: Option<B>, f: (a: A, b: B) => C): (self: Option<A>) => Option<C>;
    <A, B, C>(self: Option<A>, that: Option<B>, f: (a: A, b: B) => C): Option<C>;
};
declare const ap$2: {
    <A>(that: Option<A>): <B>(self: Option<(a: A) => B>) => Option<B>;
    <A, B>(self: Option<(a: A) => B>, that: Option<A>): Option<B>;
};
declare const reduceCompact: {
    <B, A>(b: B, f: (b: B, a: A) => B): (self: Iterable<Option<A>>) => B;
    <A, B>(self: Iterable<Option<A>>, b: B, f: (b: B, a: A) => B): B;
};
declare const toArray: <A>(self: Option<A>) => Array<A>;
declare const partitionMap: {
    <A, B, C>(f: (a: A) => Either<C, B>): (self: Option<A>) => [
        left: Option<B>,
        right: Option<C>
    ];
    <A, B, C>(self: Option<A>, f: (a: A) => Either<C, B>): [
        left: Option<B>,
        right: Option<C>
    ];
};
declare const filterMap$1: {
    <A, B>(f: (a: A) => Option<B>): (self: Option<A>) => Option<B>;
    <A, B>(self: Option<A>, f: (a: A) => Option<B>): Option<B>;
};
declare const filter$2: {
    <A, B extends A>(refinement: Refinement<NoInfer$1<A>, B>): (self: Option<A>) => Option<B>;
    <A>(predicate: Predicate<NoInfer$1<A>>): (self: Option<A>) => Option<A>;
    <A, B extends A>(self: Option<A>, refinement: Refinement<A, B>): Option<B>;
    <A>(self: Option<A>, predicate: Predicate<A>): Option<A>;
};
declare const getEquivalence$1: <A>(isEquivalent: Equivalence$1<A>) => Equivalence$1<Option<A>>;
declare const getOrder: <A>(O: Order$1<A>) => Order$1<Option<A>>;
declare const lift2: <A, B, C>(f: (a: A, b: B) => C) => {
    (that: Option<B>): (self: Option<A>) => Option<C>;
    (self: Option<A>, that: Option<B>): Option<C>;
};
declare const liftPredicate$2: {
    <A, B extends A>(refinement: Refinement<A, B>): (a: A) => Option<B>;
    <B extends A, A = B>(predicate: Predicate<A>): (b: B) => Option<B>;
    <A, B extends A>(self: A, refinement: Refinement<A, B>): Option<B>;
    <B extends A, A = B>(self: B, predicate: Predicate<A>): Option<B>;
};
declare const containsWith: <A>(isEquivalent: (self: A, that: A) => boolean) => {
    (a: A): (self: Option<A>) => boolean;
    (self: Option<A>, a: A): boolean;
};
declare const contains$1: {
    <A>(a: A): (self: Option<A>) => boolean;
    <A>(self: Option<A>, a: A): boolean;
};
declare const exists$2: {
    <A, B extends A>(refinement: Refinement<NoInfer$1<A>, B>): (self: Option<A>) => self is Option<B>;
    <A>(predicate: Predicate<NoInfer$1<A>>): (self: Option<A>) => boolean;
    <A, B extends A>(self: Option<A>, refinement: Refinement<A, B>): self is Option<B>;
    <A>(self: Option<A>, predicate: Predicate<A>): boolean;
};
declare const bindTo$2: {
    <N extends string>(name: N): <A>(self: Option<A>) => Option<{
        [K in N]: A;
    }>;
    <A, N extends string>(self: Option<A>, name: N): Option<{
        [K in N]: A;
    }>;
};
declare const let_$2: {
    <N extends string, A extends object, B>(name: Exclude<N, keyof A>, f: (a: NoInfer$1<A>) => B): (self: Option<A>) => Option<{
        [K in N | keyof A]: K extends keyof A ? A[K] : B;
    }>;
    <A extends object, N extends string, B>(self: Option<A>, name: Exclude<N, keyof A>, f: (a: NoInfer$1<A>) => B): Option<{
        [K in N | keyof A]: K extends keyof A ? A[K] : B;
    }>;
};
declare const bind$2: {
    <N extends string, A extends object, B>(name: Exclude<N, keyof A>, f: (a: NoInfer$1<A>) => Option<B>): (self: Option<A>) => Option<{
        [K in N | keyof A]: K extends keyof A ? A[K] : B;
    }>;
    <A extends object, N extends string, B>(self: Option<A>, name: Exclude<N, keyof A>, f: (a: NoInfer$1<A>) => Option<B>): Option<{
        [K in N | keyof A]: K extends keyof A ? A[K] : B;
    }>;
};
declare const Do$2: Option<{}>;
declare const gen$2: Gen<OptionTypeLambda, Adapter$1<OptionTypeLambda>>;
import Option_d_Option = Option;
type Option_d_OptionTypeLambda = OptionTypeLambda;
type Option_d_OptionUnify<A extends {
    [typeSymbol]?: any;
}> = OptionUnify<A>;
type Option_d_OptionUnifyIgnore = OptionUnifyIgnore;
type Option_d_Some<out A> = Some<A>;
declare const Option_d_composeK: typeof composeK;
declare const Option_d_containsWith: typeof containsWith;
declare const Option_d_firstSomeOf: typeof firstSomeOf;
declare const Option_d_flatMapNullable: typeof flatMapNullable;
declare const Option_d_fromIterable: typeof fromIterable;
declare const Option_d_getOrder: typeof getOrder;
declare const Option_d_isNone: typeof isNone;
declare const Option_d_isOption: typeof isOption;
declare const Option_d_isSome: typeof isSome;
declare const Option_d_lift2: typeof lift2;
declare const Option_d_liftNullable: typeof liftNullable;
declare const Option_d_liftThrowable: typeof liftThrowable;
declare const Option_d_orElseEither: typeof orElseEither;
declare const Option_d_orElseSome: typeof orElseSome;
declare const Option_d_partitionMap: typeof partitionMap;
declare const Option_d_product: typeof product;
declare const Option_d_productMany: typeof productMany;
declare const Option_d_reduceCompact: typeof reduceCompact;
declare const Option_d_some: typeof some;
declare const Option_d_toArray: typeof toArray;
declare const Option_d_toRefinement: typeof toRefinement;
declare namespace Option_d {
    export { Do$2 as Do, Option_d_Option as Option, TypeId$b as TypeId, all$3 as all, andThen$4 as andThen, ap$2 as ap, as$4 as as, asVoid$3 as asVoid, bind$2 as bind, bindTo$2 as bindTo, Option_d_composeK as composeK, contains$1 as contains, Option_d_containsWith as containsWith, exists$2 as exists, filter$2 as filter, filterMap$1 as filterMap, Option_d_firstSomeOf as firstSomeOf, flatMap$5 as flatMap, Option_d_flatMapNullable as flatMapNullable, flatten$4 as flatten, Option_d_fromIterable as fromIterable, fromNullable$2 as fromNullable, gen$2 as gen, getEquivalence$1 as getEquivalence, getLeft$1 as getLeft, getOrElse$3 as getOrElse, getOrNull$1 as getOrNull, getOrThrow$1 as getOrThrow, getOrThrowWith$1 as getOrThrowWith, getOrUndefined$1 as getOrUndefined, Option_d_getOrder as getOrder, getRight$1 as getRight, Option_d_isNone as isNone, Option_d_isOption as isOption, Option_d_isSome as isSome, let_$2 as let, Option_d_lift2 as lift2, Option_d_liftNullable as liftNullable, liftPredicate$2 as liftPredicate, Option_d_liftThrowable as liftThrowable, map$6 as map, match$6 as match, none$1 as none, orElse$4 as orElse, Option_d_orElseEither as orElseEither, Option_d_orElseSome as orElseSome, Option_d_partitionMap as partitionMap, Option_d_product as product, Option_d_productMany as productMany, Option_d_reduceCompact as reduceCompact, Option_d_some as some, tap$2 as tap, Option_d_toArray as toArray, Option_d_toRefinement as toRefinement, void_$2 as void, zipLeft$3 as zipLeft, zipRight$3 as zipRight, zipWith$5 as zipWith };
    export type { None$2 as None, Option_d_OptionTypeLambda as OptionTypeLambda, Option_d_OptionUnify as OptionUnify, Option_d_OptionUnifyIgnore as OptionUnifyIgnore, Option_d_Some as Some };
}
declare const TypeId$a: unique symbol;
type TypeId$a = typeof TypeId$a;
interface NonEmptyChunk<out A> extends Chunk<A>, NonEmptyIterable<A> {
}
interface Chunk<out A> extends Iterable<A>, Equal, Pipeable, Inspectable {
    readonly [TypeId$a]: {
        readonly _A: Covariant<A>;
    };
    readonly length: number;
}
declare namespace Chunk {
    type Infer<S extends Chunk<any>> = S extends Chunk<infer A> ? A : never;
    type With<S extends Chunk<any>, A> = S extends NonEmptyChunk<any> ? NonEmptyChunk<A> : Chunk<A>;
    type OrNonEmpty<S extends Chunk<any>, T extends Chunk<any>, A> = S extends NonEmptyChunk<any> ? NonEmptyChunk<A> : T extends NonEmptyChunk<any> ? NonEmptyChunk<A> : Chunk<A>;
    type AndNonEmpty<S extends Chunk<any>, T extends Chunk<any>, A> = S extends NonEmptyChunk<any> ? T extends NonEmptyChunk<any> ? NonEmptyChunk<A> : Chunk<A> : Chunk<A>;
    type Flatten<T extends Chunk<Chunk<any>>> = T extends NonEmptyChunk<NonEmptyChunk<infer A>> ? NonEmptyChunk<A> : T extends Chunk<Chunk<infer A>> ? Chunk<A> : never;
}
declare const TagTypeId: unique symbol;
type TagTypeId = typeof TagTypeId;
interface ReadonlyTag<in out Id, out Value> extends Pipeable, Inspectable, Effect<Value, never, Id> {
    readonly _op: "Tag";
    readonly Service: Value;
    readonly Identifier: Id;
    readonly [TagTypeId]: {
        readonly _Service: Covariant<Value>;
        readonly _Identifier: Invariant<Id>;
    };
    readonly stack?: string | undefined;
    readonly key: string;
}
declare const ReferenceTypeId: unique symbol;
type ReferenceTypeId = typeof ReferenceTypeId;
interface TagClassShape<Id, Shape> {
    readonly [TagTypeId]: TagTypeId;
    readonly Type: Shape;
    readonly Id: Id;
}
interface TagClass<Self, Id extends string, Type> extends Tag$1<Self, Type> {
    new (_: never): TagClassShape<Id, Type>;
    readonly key: Id;
}
interface ReferenceClass<Self, Id extends string, Type> extends Reference<Self, Type> {
    new (_: never): TagClassShape<Id, Type>;
    readonly key: Id;
}
interface TagUnify<A extends {
    [typeSymbol]?: any;
}> {
    Tag?: () => Extract<A[typeSymbol], Tag$1<any, any>>;
}
interface TagUnifyIgnore {
}
declare const GenericTag: <Identifier, Service = Identifier>(key: string) => Tag$1<Identifier, Service>;
declare const TypeId$9: unique symbol;
type TypeId$9 = typeof TypeId$9;
type ValidTagsById<R> = R extends infer S ? Tag$1<S, any> : never;
interface Context<in Services> extends Equal, Pipeable, Inspectable {
    readonly [TypeId$9]: {
        readonly _Services: Contravariant<Services>;
    };
    readonly unsafeMap: Map<string, any>;
}
declare const unsafeMake$1: <Services>(unsafeMap: Map<string, any>) => Context<Services>;
declare const isContext: (input: unknown) => input is Context<never>;
declare const isTag: (input: unknown) => input is Tag$1<any, any>;
declare const isReference: (u: unknown) => u is Reference<any, any>;
declare const empty$2: () => Context<never>;
declare const make$1: <I, S>(tag: Tag$1<I, S>, service: NoInfer$1<S>) => Context<I>;
declare const add: {
    <I, S>(tag: Tag$1<I, S>, service: NoInfer$1<S>): <Services>(self: Context<Services>) => Context<Services | I>;
    <Services, I, S>(self: Context<Services>, tag: Tag$1<I, S>, service: NoInfer$1<S>): Context<Services | I>;
};
declare const get$1: {
    <I, S>(tag: Reference<I, S>): <Services>(self: Context<Services>) => S;
    <Services, I extends Services, S>(tag: Tag$1<I, S>): (self: Context<Services>) => S;
    <Services, I, S>(self: Context<Services>, tag: Reference<I, S>): S;
    <Services, I extends Services, S>(self: Context<Services>, tag: Tag$1<I, S>): S;
};
declare const getOrElse$2: {
    <S, I, B>(tag: Tag$1<I, S>, orElse: LazyArg<B>): <Services>(self: Context<Services>) => S | B;
    <Services, S, I, B>(self: Context<Services>, tag: Tag$1<I, S>, orElse: LazyArg<B>): S | B;
};
declare const unsafeGet: {
    <S, I>(tag: Tag$1<I, S>): <Services>(self: Context<Services>) => S;
    <Services, S, I>(self: Context<Services>, tag: Tag$1<I, S>): S;
};
declare const getOption: {
    <S, I>(tag: Tag$1<I, S>): <Services>(self: Context<Services>) => Option<S>;
    <Services, S, I>(self: Context<Services>, tag: Tag$1<I, S>): Option<S>;
};
declare const merge$3: {
    <R1>(that: Context<R1>): <Services>(self: Context<Services>) => Context<R1 | Services>;
    <Services, R1>(self: Context<Services>, that: Context<R1>): Context<Services | R1>;
};
declare const mergeAll$2: <T extends Array<unknown>>(...ctxs: [
    ...{
        [K in keyof T]: Context<T[K]>;
    }
]) => Context<T[number]>;
declare const pick: <Tags extends ReadonlyArray<Tag$1<any, any>>>(...tags: Tags) => <Services>(self: Context<Services>) => Context<Services & Tag$1.Identifier<Tags[number]>>;
declare const omit: <Tags extends ReadonlyArray<Tag$1<any, any>>>(...tags: Tags) => <Services>(self: Context<Services>) => Context<Exclude<Services, Tag$1.Identifier<Tags[number]>>>;
interface Tag$1<in out Id, in out Value> extends Pipeable, Inspectable, ReadonlyTag<Id, Value> {
    readonly _op: "Tag";
    readonly Service: Value;
    readonly Identifier: Id;
    readonly [TagTypeId]: {
        readonly _Service: Invariant<Value>;
        readonly _Identifier: Invariant<Id>;
    };
    of(self: Value): Value;
    context(self: Value): Context<Id>;
    readonly stack?: string | undefined;
    readonly key: string;
    [typeSymbol]?: unknown;
    [unifySymbol]?: TagUnify<this>;
    [ignoreSymbol]?: TagUnifyIgnore;
}
declare namespace Tag$1 {
    type Service<T extends Tag$1<any, any> | TagClassShape<any, any>> = T extends Tag$1<any, any> ? T["Service"] : T extends TagClassShape<any, infer A> ? A : never;
    type Identifier<T extends Tag$1<any, any> | TagClassShape<any, any>> = T extends Tag$1<any, any> ? T["Identifier"] : T extends TagClassShape<any, any> ? T : never;
}
declare const Tag$1: <const Id extends string>(id: Id) => <Self, Shape>() => TagClass<Self, Id, Shape>;
interface Reference<in out Id, in out Value> extends Pipeable, Inspectable {
    readonly [ReferenceTypeId]: ReferenceTypeId;
    readonly defaultValue: () => Value;
    readonly _op: "Tag";
    readonly Service: Value;
    readonly Identifier: Id;
    readonly [TagTypeId]: {
        readonly _Service: Invariant<Value>;
        readonly _Identifier: Invariant<Id>;
    };
    of(self: Value): Value;
    context(self: Value): Context<Id>;
    readonly stack?: string | undefined;
    readonly key: string;
    [typeSymbol]?: unknown;
    [unifySymbol]?: TagUnify<this>;
    [ignoreSymbol]?: TagUnifyIgnore;
}
declare const Reference: <Self>() => <const Id extends string, Service>(id: Id, options: {
    readonly defaultValue: () => Service;
}) => ReferenceClass<Self, Id, Service>;
type Context_d_Context<in Services> = Context<Services>;
declare const Context_d_GenericTag: typeof GenericTag;
type Context_d_ReadonlyTag<in out Id, out Value> = ReadonlyTag<Id, Value>;
declare const Context_d_Reference: typeof Reference;
type Context_d_ReferenceClass<Self, Id extends string, Type> = ReferenceClass<Self, Id, Type>;
type Context_d_ReferenceTypeId = ReferenceTypeId;
type Context_d_TagClass<Self, Id extends string, Type> = TagClass<Self, Id, Type>;
type Context_d_TagClassShape<Id, Shape> = TagClassShape<Id, Shape>;
type Context_d_TagTypeId = TagTypeId;
type Context_d_TagUnify<A extends {
    [typeSymbol]?: any;
}> = TagUnify<A>;
type Context_d_TagUnifyIgnore = TagUnifyIgnore;
type Context_d_ValidTagsById<R> = ValidTagsById<R>;
declare const Context_d_add: typeof add;
declare const Context_d_getOption: typeof getOption;
declare const Context_d_isContext: typeof isContext;
declare const Context_d_isReference: typeof isReference;
declare const Context_d_isTag: typeof isTag;
declare const Context_d_omit: typeof omit;
declare const Context_d_pick: typeof pick;
declare const Context_d_unsafeGet: typeof unsafeGet;
declare namespace Context_d {
    export { Context_d_GenericTag as GenericTag, Context_d_Reference as Reference, Tag$1 as Tag, TypeId$9 as TypeId, Context_d_add as add, empty$2 as empty, get$1 as get, Context_d_getOption as getOption, getOrElse$2 as getOrElse, Context_d_isContext as isContext, Context_d_isReference as isReference, Context_d_isTag as isTag, make$1 as make, merge$3 as merge, mergeAll$2 as mergeAll, Context_d_omit as omit, Context_d_pick as pick, Context_d_unsafeGet as unsafeGet, unsafeMake$1 as unsafeMake };
    export type { Context_d_Context as Context, Context_d_ReadonlyTag as ReadonlyTag, Context_d_ReferenceClass as ReferenceClass, Context_d_ReferenceTypeId as ReferenceTypeId, Context_d_TagClass as TagClass, Context_d_TagClassShape as TagClassShape, Context_d_TagTypeId as TagTypeId, Context_d_TagUnify as TagUnify, Context_d_TagUnifyIgnore as TagUnifyIgnore, Context_d_ValidTagsById as ValidTagsById };
}
declare const TypeId$8: unique symbol;
type TypeId$8 = typeof TypeId$8;
interface HashSet<out A> extends Iterable<A>, Equal, Pipeable, Inspectable {
    readonly [TypeId$8]: TypeId$8;
}
declare const FiberIdTypeId: unique symbol;
type FiberIdTypeId = typeof FiberIdTypeId;
type Single$1 = None$1 | Runtime$1;
type FiberId = Single$1 | Composite;
interface None$1 extends Equal, Inspectable {
    readonly [FiberIdTypeId]: FiberIdTypeId;
    readonly _tag: "None";
    readonly id: -1;
    readonly startTimeMillis: -1;
}
interface Runtime$1 extends Equal, Inspectable {
    readonly [FiberIdTypeId]: FiberIdTypeId;
    readonly _tag: "Runtime";
    readonly id: number;
    readonly startTimeMillis: number;
}
interface Composite extends Equal, Inspectable {
    readonly [FiberIdTypeId]: FiberIdTypeId;
    readonly _tag: "Composite";
    readonly left: FiberId;
    readonly right: FiberId;
}
type Exit<A, E = never> = Success<A, E> | Failure<A, E>;
interface Failure<out A, out E> extends Effect<A, E>, Pipeable, Inspectable {
    readonly _tag: "Failure";
    readonly _op: "Failure";
    readonly cause: Cause<E>;
    [typeSymbol]?: unknown;
    [unifySymbol]?: ExitUnify<this>;
    [ignoreSymbol]?: ExitUnifyIgnore;
}
interface ExitUnify<A extends {
    [typeSymbol]?: any;
}> extends EffectUnify<A> {
    Exit?: () => A[typeSymbol] extends Exit<infer A0, infer E0> | infer _ ? Exit<A0, E0> : never;
}
interface ExitUnifyIgnore extends EffectUnifyIgnore {
    Effect?: true;
}
interface Success<out A, out E> extends Effect<A, E>, Pipeable, Inspectable {
    readonly _tag: "Success";
    readonly _op: "Success";
    readonly value: A;
    [typeSymbol]?: unknown;
    [unifySymbol]?: ExitUnify<this>;
    [ignoreSymbol]?: ExitUnifyIgnore;
}
declare const isExit: (u: unknown) => u is Exit<unknown, unknown>;
declare const isFailure$2: <A, E>(self: Exit<A, E>) => self is Failure<A, E>;
declare const isSuccess$1: <A, E>(self: Exit<A, E>) => self is Success<A, E>;
declare const isInterrupted$1: <A, E>(self: Exit<A, E>) => boolean;
declare const as$3: {
    <A2>(value: A2): <A, E>(self: Exit<A, E>) => Exit<A2, E>;
    <A, E, A2>(self: Exit<A, E>, value: A2): Exit<A2, E>;
};
declare const asVoid$2: <A, E>(self: Exit<A, E>) => Exit<void, E>;
declare const causeOption: <A, E>(self: Exit<A, E>) => Option<Cause<E>>;
declare const all$2: <A, E>(exits: Iterable<Exit<A, E>>, options?: {
    readonly parallel?: boolean | undefined;
} | undefined) => Option<Exit<Array<A>, E>>;
declare const die$3: (defect: unknown) => Exit<never>;
declare const exists$1: {
    <A, B extends A>(refinement: Refinement<NoInfer$1<A>, B>): <E>(self: Exit<A, E>) => self is Exit<B>;
    <A>(predicate: Predicate<NoInfer$1<A>>): <E>(self: Exit<A, E>) => boolean;
    <A, E, B extends A>(self: Exit<A, E>, refinement: Refinement<A, B>): self is Exit<B>;
    <A, E>(self: Exit<A, E>, predicate: Predicate<A>): boolean;
};
declare const fail$3: <E>(error: E) => Exit<never, E>;
declare const failCause$2: <E>(cause: Cause<E>) => Exit<never, E>;
declare const flatMap$4: {
    <A, A2, E2>(f: (a: A) => Exit<A2, E2>): <E>(self: Exit<A, E>) => Exit<A2, E2 | E>;
    <A, E, E2, A2>(self: Exit<A, E>, f: (a: A) => Exit<A2, E2>): Exit<A2, E | E2>;
};
declare const flatMapEffect: {
    <A, E, A2, E2, R>(f: (a: A) => Effect<Exit<A2, E>, E2, R>): (self: Exit<A, E>) => Effect<Exit<A2, E>, E2, R>;
    <A, E, A2, E2, R>(self: Exit<A, E>, f: (a: A) => Effect<Exit<A2, E>, E2, R>): Effect<Exit<A2, E>, E2, R>;
};
declare const flatten$3: <A, E, E2>(self: Exit<Exit<A, E>, E2>) => Exit<A, E | E2>;
declare const forEachEffect: {
    <A, B, E2, R>(f: (a: A) => Effect<B, E2, R>): <E>(self: Exit<A, E>) => Effect<Exit<B, E2 | E>, never, R>;
    <A, E, B, E2, R>(self: Exit<A, E>, f: (a: A) => Effect<B, E2, R>): Effect<Exit<B, E | E2>, never, R>;
};
declare const fromEither: <R, L>(either: Either<R, L>) => Exit<R, L>;
declare const fromOption$1: <A>(option: Option<A>) => Exit<A, void>;
declare const getOrElse$1: {
    <E, A2>(orElse: (cause: Cause<E>) => A2): <A>(self: Exit<A, E>) => A2 | A;
    <A, E, A2>(self: Exit<A, E>, orElse: (cause: Cause<E>) => A2): A | A2;
};
declare const interrupt$2: (fiberId: FiberId) => Exit<never>;
declare const map$5: {
    <A, B>(f: (a: A) => B): <E>(self: Exit<A, E>) => Exit<B, E>;
    <A, E, B>(self: Exit<A, E>, f: (a: A) => B): Exit<B, E>;
};
declare const mapBoth$3: {
    <E, A, E2, A2>(options: {
        readonly onFailure: (e: E) => E2;
        readonly onSuccess: (a: A) => A2;
    }): (self: Exit<A, E>) => Exit<A2, E2>;
    <A, E, E2, A2>(self: Exit<A, E>, options: {
        readonly onFailure: (e: E) => E2;
        readonly onSuccess: (a: A) => A2;
    }): Exit<A2, E2>;
};
declare const mapError$2: {
    <E, E2>(f: (e: E) => E2): <A>(self: Exit<A, E>) => Exit<A, E2>;
    <A, E, E2>(self: Exit<A, E>, f: (e: E) => E2): Exit<A, E2>;
};
declare const mapErrorCause$1: {
    <E, E2>(f: (cause: Cause<E>) => Cause<E2>): <A>(self: Exit<A, E>) => Exit<A, E2>;
    <E, A, E2>(self: Exit<A, E>, f: (cause: Cause<E>) => Cause<E2>): Exit<A, E2>;
};
declare const match$5: {
    <E, A, Z1, Z2>(options: {
        readonly onFailure: (cause: Cause<E>) => Z1;
        readonly onSuccess: (a: A) => Z2;
    }): (self: Exit<A, E>) => Z1 | Z2;
    <A, E, Z1, Z2>(self: Exit<A, E>, options: {
        readonly onFailure: (cause: Cause<E>) => Z1;
        readonly onSuccess: (a: A) => Z2;
    }): Z1 | Z2;
};
declare const matchEffect$1: {
    <E, A2, E2, R, A, A3, E3, R2>(options: {
        readonly onFailure: (cause: Cause<E>) => Effect<A2, E2, R>;
        readonly onSuccess: (a: A) => Effect<A3, E3, R2>;
    }): (self: Exit<A, E>) => Effect<A2 | A3, E2 | E3, R | R2>;
    <A, E, A2, E2, R, A3, E3, R2>(self: Exit<A, E>, options: {
        readonly onFailure: (cause: Cause<E>) => Effect<A2, E2, R>;
        readonly onSuccess: (a: A) => Effect<A3, E3, R2>;
    }): Effect<A2 | A3, E2 | E3, R | R2>;
};
declare const succeed$3: <A>(value: A) => Exit<A>;
declare const void_$1: Exit<void>;
declare const zip$1: {
    <A2, E2>(that: Exit<A2, E2>): <A, E>(self: Exit<A, E>) => Exit<[
        A,
        A2
    ], E2 | E>;
    <A, E, A2, E2>(self: Exit<A, E>, that: Exit<A2, E2>): Exit<[
        A,
        A2
    ], E | E2>;
};
declare const zipLeft$2: {
    <A2, E2>(that: Exit<A2, E2>): <A, E>(self: Exit<A, E>) => Exit<A, E2 | E>;
    <A, E, A2, E2>(self: Exit<A, E>, that: Exit<A2, E2>): Exit<A, E | E2>;
};
declare const zipRight$2: {
    <A2, E2>(that: Exit<A2, E2>): <A, E>(self: Exit<A, E>) => Exit<A2, E2 | E>;
    <A, E, A2, E2>(self: Exit<A, E>, that: Exit<A2, E2>): Exit<A2, E | E2>;
};
declare const zipPar: {
    <A2, E2>(that: Exit<A2, E2>): <A, E>(self: Exit<A, E>) => Exit<[
        A,
        A2
    ], E2 | E>;
    <A, E, A2, E2>(self: Exit<A, E>, that: Exit<A2, E2>): Exit<[
        A,
        A2
    ], E | E2>;
};
declare const zipParLeft: {
    <A2, E2>(that: Exit<A2, E2>): <A, E>(self: Exit<A, E>) => Exit<A, E2 | E>;
    <A, E, A2, E2>(self: Exit<A, E>, that: Exit<A2, E2>): Exit<A, E | E2>;
};
declare const zipParRight: {
    <A2, E2>(that: Exit<A2, E2>): <A, E>(self: Exit<A, E>) => Exit<A2, E2 | E>;
    <A, E, A2, E2>(self: Exit<A, E>, that: Exit<A2, E2>): Exit<A2, E | E2>;
};
declare const zipWith$4: {
    <B, E2, A, C, E>(that: Exit<B, E2>, options: {
        readonly onSuccess: (a: A, b: B) => C;
        readonly onFailure: (cause: Cause<E>, cause2: Cause<E2>) => Cause<any>;
    }): (self: Exit<A, E>) => Exit<C, any>;
    <A, E, B, E2, C>(self: Exit<A, E>, that: Exit<B, E2>, options: {
        readonly onSuccess: (a: A, b: B) => C;
        readonly onFailure: (cause: Cause<E>, cause2: Cause<E2>) => Cause<E | E2>;
    }): Exit<C, E | E2>;
};
type Exit_d_Exit<A, E = never> = Exit<A, E>;
type Exit_d_ExitUnify<A extends {
    [typeSymbol]?: any;
}> = ExitUnify<A>;
type Exit_d_ExitUnifyIgnore = ExitUnifyIgnore;
type Exit_d_Failure<out A, out E> = Failure<A, E>;
type Exit_d_Success<out A, out E> = Success<A, E>;
declare const Exit_d_causeOption: typeof causeOption;
declare const Exit_d_flatMapEffect: typeof flatMapEffect;
declare const Exit_d_forEachEffect: typeof forEachEffect;
declare const Exit_d_fromEither: typeof fromEither;
declare const Exit_d_isExit: typeof isExit;
declare const Exit_d_zipPar: typeof zipPar;
declare const Exit_d_zipParLeft: typeof zipParLeft;
declare const Exit_d_zipParRight: typeof zipParRight;
declare namespace Exit_d {
    export { all$2 as all, as$3 as as, asVoid$2 as asVoid, Exit_d_causeOption as causeOption, die$3 as die, exists$1 as exists, fail$3 as fail, failCause$2 as failCause, flatMap$4 as flatMap, Exit_d_flatMapEffect as flatMapEffect, flatten$3 as flatten, Exit_d_forEachEffect as forEachEffect, Exit_d_fromEither as fromEither, fromOption$1 as fromOption, getOrElse$1 as getOrElse, interrupt$2 as interrupt, Exit_d_isExit as isExit, isFailure$2 as isFailure, isInterrupted$1 as isInterrupted, isSuccess$1 as isSuccess, map$5 as map, mapBoth$3 as mapBoth, mapError$2 as mapError, mapErrorCause$1 as mapErrorCause, match$5 as match, matchEffect$1 as matchEffect, succeed$3 as succeed, void_$1 as void, zip$1 as zip, zipLeft$2 as zipLeft, Exit_d_zipPar as zipPar, Exit_d_zipParLeft as zipParLeft, Exit_d_zipParRight as zipParRight, zipRight$2 as zipRight, zipWith$4 as zipWith };
    export type { Exit_d_Exit as Exit, Exit_d_ExitUnify as ExitUnify, Exit_d_ExitUnifyIgnore as ExitUnifyIgnore, Exit_d_Failure as Failure, Exit_d_Success as Success };
}
declare const DeferredTypeId: unique symbol;
type DeferredTypeId = typeof DeferredTypeId;
interface DeferredUnify<A extends {
    [typeSymbol]?: any;
}> extends EffectUnify<A> {
    Deferred?: () => Extract<A[typeSymbol], Deferred<any, any>>;
}
interface DeferredUnifyIgnore extends EffectUnifyIgnore {
    Effect?: true;
}
interface Deferred<in out A, in out E = never> extends Effect<A, E>, Deferred.Variance<A, E> {
    readonly [typeSymbol]?: unknown;
    readonly [unifySymbol]?: DeferredUnify<this>;
    readonly [ignoreSymbol]?: DeferredUnifyIgnore;
}
declare namespace Deferred {
    interface Variance<in out A, in out E> {
        readonly [DeferredTypeId]: {
            readonly _A: Invariant<A>;
            readonly _E: Invariant<E>;
        };
    }
}
declare const TypeId$7: unique symbol;
type TypeId$7 = typeof TypeId$7;
interface Duration extends Equal, Pipeable, Inspectable {
    readonly [TypeId$7]: TypeId$7;
    readonly value: DurationValue;
}
type DurationValue = {
    readonly _tag: "Millis";
    readonly millis: number;
} | {
    readonly _tag: "Nanos";
    readonly nanos: bigint;
} | {
    readonly _tag: "Infinity";
};
type Unit = "nano" | "nanos" | "micro" | "micros" | "milli" | "millis" | "second" | "seconds" | "minute" | "minutes" | "hour" | "hours" | "day" | "days" | "week" | "weeks";
type DurationInput = Duration | number | bigint | readonly [
    seconds: number,
    nanos: number
] | `${number} ${Unit}`;
declare const decode: (input: DurationInput) => Duration;
declare const decodeUnknown: (u: unknown) => Option<Duration>;
declare const isDuration: (u: unknown) => u is Duration;
declare const isFinite: (self: Duration) => boolean;
declare const isZero: (self: Duration) => boolean;
declare const zero: Duration;
declare const infinity: Duration;
declare const nanos: (nanos: bigint) => Duration;
declare const micros: (micros: bigint) => Duration;
declare const millis: (millis: number) => Duration;
declare const seconds: (seconds: number) => Duration;
declare const minutes: (minutes: number) => Duration;
declare const hours: (hours: number) => Duration;
declare const days: (days: number) => Duration;
declare const weeks: (weeks: number) => Duration;
declare const toMillis: (self: DurationInput) => number;
declare const toSeconds: (self: DurationInput) => number;
declare const toMinutes: (self: DurationInput) => number;
declare const toHours: (self: DurationInput) => number;
declare const toDays: (self: DurationInput) => number;
declare const toWeeks: (self: DurationInput) => number;
declare const toNanos: (self: DurationInput) => Option<bigint>;
declare const unsafeToNanos: (self: DurationInput) => bigint;
declare const toHrTime: (self: DurationInput) => [
    seconds: number,
    nanos: number
];
declare const match$4: {
    <A, B>(options: {
        readonly onMillis: (millis: number) => A;
        readonly onNanos: (nanos: bigint) => B;
    }): (self: DurationInput) => A | B;
    <A, B>(self: DurationInput, options: {
        readonly onMillis: (millis: number) => A;
        readonly onNanos: (nanos: bigint) => B;
    }): A | B;
};
declare const matchWith: {
    <A, B>(that: DurationInput, options: {
        readonly onMillis: (self: number, that: number) => A;
        readonly onNanos: (self: bigint, that: bigint) => B;
    }): (self: DurationInput) => A | B;
    <A, B>(self: DurationInput, that: DurationInput, options: {
        readonly onMillis: (self: number, that: number) => A;
        readonly onNanos: (self: bigint, that: bigint) => B;
    }): A | B;
};
declare const Order: Order$1<Duration>;
declare const between: {
    (options: {
        minimum: DurationInput;
        maximum: DurationInput;
    }): (self: DurationInput) => boolean;
    (self: DurationInput, options: {
        minimum: DurationInput;
        maximum: DurationInput;
    }): boolean;
};
declare const Equivalence: Equivalence$1<Duration>;
declare const min: {
    (that: DurationInput): (self: DurationInput) => Duration;
    (self: DurationInput, that: DurationInput): Duration;
};
declare const max: {
    (that: DurationInput): (self: DurationInput) => Duration;
    (self: DurationInput, that: DurationInput): Duration;
};
declare const clamp: {
    (options: {
        minimum: DurationInput;
        maximum: DurationInput;
    }): (self: DurationInput) => Duration;
    (self: DurationInput, options: {
        minimum: DurationInput;
        maximum: DurationInput;
    }): Duration;
};
declare const divide: {
    (by: number): (self: DurationInput) => Option<Duration>;
    (self: DurationInput, by: number): Option<Duration>;
};
declare const unsafeDivide: {
    (by: number): (self: DurationInput) => Duration;
    (self: DurationInput, by: number): Duration;
};
declare const times: {
    (times: number): (self: DurationInput) => Duration;
    (self: DurationInput, times: number): Duration;
};
declare const subtract: {
    (that: DurationInput): (self: DurationInput) => Duration;
    (self: DurationInput, that: DurationInput): Duration;
};
declare const sum: {
    (that: DurationInput): (self: DurationInput) => Duration;
    (self: DurationInput, that: DurationInput): Duration;
};
declare const lessThan: {
    (that: DurationInput): (self: DurationInput) => boolean;
    (self: DurationInput, that: DurationInput): boolean;
};
declare const lessThanOrEqualTo: {
    (that: DurationInput): (self: DurationInput) => boolean;
    (self: DurationInput, that: DurationInput): boolean;
};
declare const greaterThan: {
    (that: DurationInput): (self: DurationInput) => boolean;
    (self: DurationInput, that: DurationInput): boolean;
};
declare const greaterThanOrEqualTo: {
    (that: DurationInput): (self: DurationInput) => boolean;
    (self: DurationInput, that: DurationInput): boolean;
};
declare const equals: {
    (that: DurationInput): (self: DurationInput) => boolean;
    (self: DurationInput, that: DurationInput): boolean;
};
declare const parts: (self: DurationInput) => {
    days: number;
    hours: number;
    minutes: number;
    seconds: number;
    millis: number;
    nanos: number;
};
declare const format: (self: DurationInput) => string;
declare const unsafeFormatIso: (self: DurationInput) => string;
declare const formatIso: (self: DurationInput) => Option<string>;
declare const fromIso: (iso: string) => Option<Duration>;
type Duration_d_Duration = Duration;
type Duration_d_DurationInput = DurationInput;
type Duration_d_DurationValue = DurationValue;
declare const Duration_d_Equivalence: typeof Equivalence;
declare const Duration_d_Order: typeof Order;
type Duration_d_Unit = Unit;
declare const Duration_d_between: typeof between;
declare const Duration_d_clamp: typeof clamp;
declare const Duration_d_days: typeof days;
declare const Duration_d_decode: typeof decode;
declare const Duration_d_decodeUnknown: typeof decodeUnknown;
declare const Duration_d_divide: typeof divide;
declare const Duration_d_equals: typeof equals;
declare const Duration_d_format: typeof format;
declare const Duration_d_formatIso: typeof formatIso;
declare const Duration_d_fromIso: typeof fromIso;
declare const Duration_d_greaterThan: typeof greaterThan;
declare const Duration_d_greaterThanOrEqualTo: typeof greaterThanOrEqualTo;
declare const Duration_d_hours: typeof hours;
declare const Duration_d_infinity: typeof infinity;
declare const Duration_d_isDuration: typeof isDuration;
declare const Duration_d_isFinite: typeof isFinite;
declare const Duration_d_isZero: typeof isZero;
declare const Duration_d_lessThan: typeof lessThan;
declare const Duration_d_lessThanOrEqualTo: typeof lessThanOrEqualTo;
declare const Duration_d_matchWith: typeof matchWith;
declare const Duration_d_max: typeof max;
declare const Duration_d_micros: typeof micros;
declare const Duration_d_millis: typeof millis;
declare const Duration_d_min: typeof min;
declare const Duration_d_minutes: typeof minutes;
declare const Duration_d_nanos: typeof nanos;
declare const Duration_d_parts: typeof parts;
declare const Duration_d_seconds: typeof seconds;
declare const Duration_d_subtract: typeof subtract;
declare const Duration_d_sum: typeof sum;
declare const Duration_d_times: typeof times;
declare const Duration_d_toDays: typeof toDays;
declare const Duration_d_toHours: typeof toHours;
declare const Duration_d_toHrTime: typeof toHrTime;
declare const Duration_d_toMillis: typeof toMillis;
declare const Duration_d_toMinutes: typeof toMinutes;
declare const Duration_d_toNanos: typeof toNanos;
declare const Duration_d_toSeconds: typeof toSeconds;
declare const Duration_d_toWeeks: typeof toWeeks;
declare const Duration_d_unsafeDivide: typeof unsafeDivide;
declare const Duration_d_unsafeFormatIso: typeof unsafeFormatIso;
declare const Duration_d_unsafeToNanos: typeof unsafeToNanos;
declare const Duration_d_weeks: typeof weeks;
declare const Duration_d_zero: typeof zero;
declare namespace Duration_d {
    export { Duration_d_Equivalence as Equivalence, Duration_d_Order as Order, TypeId$7 as TypeId, Duration_d_between as between, Duration_d_clamp as clamp, Duration_d_days as days, Duration_d_decode as decode, Duration_d_decodeUnknown as decodeUnknown, Duration_d_divide as divide, Duration_d_equals as equals, Duration_d_format as format, Duration_d_formatIso as formatIso, Duration_d_fromIso as fromIso, Duration_d_greaterThan as greaterThan, Duration_d_greaterThanOrEqualTo as greaterThanOrEqualTo, Duration_d_hours as hours, Duration_d_infinity as infinity, Duration_d_isDuration as isDuration, Duration_d_isFinite as isFinite, Duration_d_isZero as isZero, Duration_d_lessThan as lessThan, Duration_d_lessThanOrEqualTo as lessThanOrEqualTo, match$4 as match, Duration_d_matchWith as matchWith, Duration_d_max as max, Duration_d_micros as micros, Duration_d_millis as millis, Duration_d_min as min, Duration_d_minutes as minutes, Duration_d_nanos as nanos, Duration_d_parts as parts, Duration_d_seconds as seconds, Duration_d_subtract as subtract, Duration_d_sum as sum, Duration_d_times as times, Duration_d_toDays as toDays, Duration_d_toHours as toHours, Duration_d_toHrTime as toHrTime, Duration_d_toMillis as toMillis, Duration_d_toMinutes as toMinutes, Duration_d_toNanos as toNanos, Duration_d_toSeconds as toSeconds, Duration_d_toWeeks as toWeeks, Duration_d_unsafeDivide as unsafeDivide, Duration_d_unsafeFormatIso as unsafeFormatIso, Duration_d_unsafeToNanos as unsafeToNanos, Duration_d_weeks as weeks, Duration_d_zero as zero };
    export type { Duration_d_Duration as Duration, Duration_d_DurationInput as DurationInput, Duration_d_DurationValue as DurationValue, Duration_d_Unit as Unit };
}
declare const ClockTypeId: unique symbol;
type ClockTypeId = typeof ClockTypeId;
interface Clock {
    readonly [ClockTypeId]: ClockTypeId;
    unsafeCurrentTimeMillis(): number;
    readonly currentTimeMillis: Effect<number>;
    unsafeCurrentTimeNanos(): bigint;
    readonly currentTimeNanos: Effect<bigint>;
    sleep(duration: Duration): Effect<void>;
}
declare const Clock: Tag$1<Clock, Clock>;
declare const ConfigErrorTypeId: unique symbol;
type ConfigErrorTypeId = typeof ConfigErrorTypeId;
type ConfigError = And | Or | InvalidData | MissingData | SourceUnavailable | Unsupported;
declare namespace ConfigError {
    interface Proto {
        readonly _tag: "ConfigError";
        readonly [ConfigErrorTypeId]: ConfigErrorTypeId;
    }
    type Reducer<C, Z> = ConfigErrorReducer<C, Z>;
}
interface ConfigErrorReducer<in C, in out Z> {
    andCase(context: C, left: Z, right: Z): Z;
    orCase(context: C, left: Z, right: Z): Z;
    invalidDataCase(context: C, path: Array<string>, message: string): Z;
    missingDataCase(context: C, path: Array<string>, message: string): Z;
    sourceUnavailableCase(context: C, path: Array<string>, message: string, cause: Cause<unknown>): Z;
    unsupportedCase(context: C, path: Array<string>, message: string): Z;
}
interface Options {
    readonly pathDelim: string;
}
interface And extends ConfigError.Proto {
    readonly _op: "And";
    readonly left: ConfigError;
    readonly right: ConfigError;
    readonly message: string;
}
declare const And: (self: ConfigError, that: ConfigError) => ConfigError;
interface Or extends ConfigError.Proto {
    readonly _op: "Or";
    readonly left: ConfigError;
    readonly right: ConfigError;
    readonly message: string;
}
declare const Or: (self: ConfigError, that: ConfigError) => ConfigError;
interface MissingData extends ConfigError.Proto {
    readonly _op: "MissingData";
    readonly path: Array<string>;
    readonly message: string;
}
declare const MissingData: (path: Array<string>, message: string, options?: Options) => ConfigError;
interface InvalidData extends ConfigError.Proto {
    readonly _op: "InvalidData";
    readonly path: Array<string>;
    readonly message: string;
}
declare const InvalidData: (path: Array<string>, message: string, options?: Options) => ConfigError;
interface SourceUnavailable extends ConfigError.Proto {
    readonly _op: "SourceUnavailable";
    readonly path: Array<string>;
    readonly message: string;
    readonly cause: Cause<unknown>;
}
declare const SourceUnavailable: (path: Array<string>, message: string, cause: Cause<unknown>, options?: Options) => ConfigError;
interface Unsupported extends ConfigError.Proto {
    readonly _op: "Unsupported";
    readonly path: Array<string>;
    readonly message: string;
}
declare const Unsupported: (path: Array<string>, message: string, options?: Options) => ConfigError;
declare const TypeId$6: unique symbol;
type TypeId$6 = typeof TypeId$6;
interface HashMap<out Key, out Value> extends Iterable<[
    Key,
    Value
]>, Equal, Pipeable, Inspectable {
    readonly [TypeId$6]: TypeId$6;
}
declare namespace HashMap {
    type UpdateFn<V> = (option: Option<V>) => Option<V>;
    type Key<T extends HashMap<any, any>> = [
        T
    ] extends [
        HashMap<infer _K, infer _V>
    ] ? _K : never;
    type Value<T extends HashMap<any, any>> = [
        T
    ] extends [
        HashMap<infer _K, infer _V>
    ] ? _V : never;
    type Entry<T extends HashMap<any, any>> = [
        Key<T>,
        Value<T>
    ];
}
type LogLevel = All$1 | Fatal | Error$2 | Warning | Info | Debug | Trace | None;
type Literal = LogLevel["_tag"];
interface All$1 extends Pipeable {
    readonly _tag: "All";
    readonly label: "ALL";
    readonly syslog: 0;
    readonly ordinal: number;
}
declare const All$1: LogLevel;
interface Fatal extends Pipeable {
    readonly _tag: "Fatal";
    readonly label: "FATAL";
    readonly syslog: 2;
    readonly ordinal: number;
}
declare const Fatal: LogLevel;
interface Error$2 extends Pipeable {
    readonly _tag: "Error";
    readonly label: "ERROR";
    readonly syslog: 3;
    readonly ordinal: number;
}
declare const Error$2: LogLevel;
interface Warning extends Pipeable {
    readonly _tag: "Warning";
    readonly label: "WARN";
    readonly syslog: 4;
    readonly ordinal: number;
}
declare const Warning: LogLevel;
interface Info extends Pipeable {
    readonly _tag: "Info";
    readonly label: "INFO";
    readonly syslog: 6;
    readonly ordinal: number;
}
declare const Info: LogLevel;
interface Debug extends Pipeable {
    readonly _tag: "Debug";
    readonly label: "DEBUG";
    readonly syslog: 7;
    readonly ordinal: number;
}
declare const Debug: LogLevel;
interface Trace extends Pipeable {
    readonly _tag: "Trace";
    readonly label: "TRACE";
    readonly syslog: 7;
    readonly ordinal: number;
}
declare const Trace: LogLevel;
interface None extends Pipeable {
    readonly _tag: "None";
    readonly label: "OFF";
    readonly syslog: 7;
    readonly ordinal: number;
}
declare const None: LogLevel;
declare const ConfigTypeId: unique symbol;
type ConfigTypeId = typeof ConfigTypeId;
interface Config<out A> extends Config.Variance<A>, Effect<A, ConfigError> {
}
declare namespace Config {
    interface Variance<out A> {
        readonly [ConfigTypeId]: {
            readonly _A: Covariant<A>;
        };
    }
    type Success<T extends Config<any>> = [
        T
    ] extends [
        Config<infer _A>
    ] ? _A : never;
    interface Primitive<out A> extends Config<A> {
        readonly description: string;
        parse(text: string): Either<A, ConfigError>;
    }
    type Wrap<A> = [
        NonNullable<A>
    ] extends [
        infer T
    ] ? [
        IsPlainObject<T>
    ] extends [
        true
    ] ? {
        readonly [K in keyof A]: Wrap<A[K]>;
    } | Config<A> : Config<A> : Config<A>;
    type IsPlainObject<A> = [
        A
    ] extends [
        Record<string, any>
    ] ? [
        keyof A
    ] extends [
        never
    ] ? false : [
        keyof A
    ] extends [
        string
    ] ? true : false : false;
}
type PathPatch = Empty$3 | AndThen$1 | MapName | Nested | Unnested;
interface Empty$3 {
    readonly _tag: "Empty";
}
interface AndThen$1 {
    readonly _tag: "AndThen";
    readonly first: PathPatch;
    readonly second: PathPatch;
}
interface MapName {
    readonly _tag: "MapName";
    f(string: string): string;
}
interface Nested {
    readonly _tag: "Nested";
    readonly name: string;
}
interface Unnested {
    readonly _tag: "Unnested";
    readonly name: string;
}
declare const ConfigProviderTypeId: unique symbol;
type ConfigProviderTypeId = typeof ConfigProviderTypeId;
declare const FlatConfigProviderTypeId: unique symbol;
type FlatConfigProviderTypeId = typeof FlatConfigProviderTypeId;
interface ConfigProvider extends ConfigProvider.Proto, Pipeable {
    load<A>(config: Config<A>): Effect<A, ConfigError>;
    readonly flattened: ConfigProvider.Flat;
}
declare namespace ConfigProvider {
    interface Proto {
        readonly [ConfigProviderTypeId]: ConfigProviderTypeId;
    }
    interface Flat {
        readonly [FlatConfigProviderTypeId]: FlatConfigProviderTypeId;
        readonly patch: PathPatch;
        load<A>(path: ReadonlyArray<string>, config: Config.Primitive<A>, split?: boolean): Effect<Array<A>, ConfigError>;
        enumerateChildren(path: ReadonlyArray<string>): Effect<HashSet<string>, ConfigError>;
    }
    interface FromMapConfig {
        readonly pathDelim: string;
        readonly seqDelim: string;
    }
    interface FromEnvConfig {
        readonly pathDelim: string;
        readonly seqDelim: string;
    }
    type KeyComponent = KeyName | KeyIndex;
    interface KeyName {
        readonly _tag: "KeyName";
        readonly name: string;
    }
    interface KeyIndex {
        readonly _tag: "KeyIndex";
        readonly index: number;
    }
}
declare const ConfigProvider: Tag$1<ConfigProvider, ConfigProvider>;
type ExecutionStrategy = Sequential$1 | Parallel$1 | ParallelN;
interface Sequential$1 {
    readonly _tag: "Sequential";
}
interface Parallel$1 {
    readonly _tag: "Parallel";
}
interface ParallelN {
    readonly _tag: "ParallelN";
    readonly parallelism: number;
}
declare const ScopeTypeId: unique symbol;
type ScopeTypeId = typeof ScopeTypeId;
declare const CloseableScopeTypeId: unique symbol;
type CloseableScopeTypeId = typeof CloseableScopeTypeId;
interface CloseableScope extends Scope, Pipeable {
    readonly [CloseableScopeTypeId]: CloseableScopeTypeId;
}
interface Scope extends Pipeable {
    readonly [ScopeTypeId]: ScopeTypeId;
    readonly strategy: ExecutionStrategy;
}
declare const Scope: Tag$1<Scope, Scope>;
declare namespace Scope {
    type Finalizer = (exit: Exit<unknown, unknown>) => Effect<void>;
    type Closeable = CloseableScope;
}
declare const MetricLabelTypeId: unique symbol;
type MetricLabelTypeId = typeof MetricLabelTypeId;
interface MetricLabel extends Equal, Pipeable {
    readonly [MetricLabelTypeId]: MetricLabelTypeId;
    readonly key: string;
    readonly value: string;
}
declare const CacheTypeId: unique symbol;
type CacheTypeId = typeof CacheTypeId;
declare const ConsumerCacheTypeId: unique symbol;
type ConsumerCacheTypeId = typeof ConsumerCacheTypeId;
interface ConsumerCache<in out Key, out Value, out Error = never> extends Cache$1.ConsumerVariance<Key, Value, Error> {
    getOption(key: Key): Effect<Option<Value>, Error>;
    getOptionComplete(key: Key): Effect<Option<Value>>;
    readonly cacheStats: Effect<CacheStats>;
    contains(key: Key): Effect<boolean>;
    entryStats(key: Key): Effect<Option<EntryStats>>;
    invalidate(key: Key): Effect<void>;
    invalidateWhen(key: Key, predicate: Predicate<Value>): Effect<void>;
    readonly invalidateAll: Effect<void>;
    readonly size: Effect<number>;
    readonly keys: Effect<Array<Key>>;
    readonly values: Effect<Array<Value>>;
    readonly entries: Effect<Array<[
        Key,
        Value
    ]>>;
}
interface Cache$1<in out Key, in out Value, out Error = never> extends ConsumerCache<Key, Value, Error>, Cache$1.Variance<Key, Value, Error> {
    get(key: Key): Effect<Value, Error>;
    getEither(key: Key): Effect<Either<Value, Value>, Error>;
    refresh(key: Key): Effect<void, Error>;
    set(key: Key, value: Value): Effect<void>;
}
declare namespace Cache$1 {
    interface Variance<in out Key, in out Value, out Error> {
        readonly [CacheTypeId]: {
            readonly _Key: Invariant<Key>;
            readonly _Error: Covariant<Error>;
            readonly _Value: Invariant<Value>;
        };
    }
    interface ConsumerVariance<in out Key, out Value, out Error> {
        readonly [ConsumerCacheTypeId]: {
            readonly _Key: Invariant<Key>;
            readonly _Error: Covariant<Error>;
            readonly _Value: Covariant<Value>;
        };
    }
}
interface CacheStats {
    readonly hits: number;
    readonly misses: number;
    readonly size: number;
}
interface EntryStats {
    readonly loadedMillis: number;
}
declare const RequestTypeId: unique symbol;
type RequestTypeId = typeof RequestTypeId;
interface Request<out A, out E = never> extends Request.Variance<A, E> {
}
declare namespace Request {
    interface Variance<out A, out E> {
        readonly [RequestTypeId]: {
            readonly _A: Covariant<A>;
            readonly _E: Covariant<E>;
        };
    }
    interface Constructor<R extends Request<any, any>, T extends keyof R = never> {
        (args: Omit<R, T | keyof (Request.Variance<Request.Success<R>, Request.Error<R>>)>): R;
    }
    type Error<T extends Request<any, any>> = [
        T
    ] extends [
        Request<infer _A, infer _E>
    ] ? _E : never;
    type Success<T extends Request<any, any>> = [
        T
    ] extends [
        Request<infer _A, infer _E>
    ] ? _A : never;
    type Result<T extends Request<any, any>> = T extends Request<infer A, infer E> ? Exit<A, E> : never;
    type OptionalResult<T extends Request<any, any>> = T extends Request<infer A, infer E> ? Exit<Option<A>, E> : never;
}
interface Listeners {
    readonly count: number;
    readonly observers: Set<(count: number) => void>;
    interrupted: boolean;
    addObserver(f: (count: number) => void): void;
    removeObserver(f: (count: number) => void): void;
    increment(): void;
    decrement(): void;
}
interface Cache extends ConsumerCache<Request<any, any>, {
    listeners: Listeners;
    handle: Deferred<unknown, unknown>;
}> {
}
declare const EntryTypeId: unique symbol;
type EntryTypeId = typeof EntryTypeId;
interface Entry<out R> extends Entry.Variance<R> {
    readonly request: R;
    readonly result: Deferred<[
        R
    ] extends [
        Request<infer _A, infer _E>
    ] ? _A : never, [
        R
    ] extends [
        Request<infer _A, infer _E>
    ] ? _E : never>;
    readonly listeners: Listeners;
    readonly ownerId: FiberId;
    readonly state: {
        completed: boolean;
    };
}
declare namespace Entry {
    interface Variance<out R> {
        readonly [EntryTypeId]: {
            readonly _R: Covariant<R>;
        };
    }
}
type RuntimeFlagsPatch = number & {
    readonly RuntimeFlagsPatch: unique symbol;
};
type RuntimeFlags = number & {
    readonly RuntimeFlags: unique symbol;
};
declare const TypeId$5: unique symbol;
type TypeId$5 = typeof TypeId$5;
interface UnsafeConsole {
    assert(condition: boolean, ...args: ReadonlyArray<any>): void;
    clear(): void;
    count(label?: string): void;
    countReset(label?: string): void;
    debug(...args: ReadonlyArray<any>): void;
    dir(item: any, options?: any): void;
    dirxml(...args: ReadonlyArray<any>): void;
    error(...args: ReadonlyArray<any>): void;
    group(...args: ReadonlyArray<any>): void;
    groupCollapsed(...args: ReadonlyArray<any>): void;
    groupEnd(): void;
    info(...args: ReadonlyArray<any>): void;
    log(...args: ReadonlyArray<any>): void;
    table(tabularData: any, properties?: ReadonlyArray<string>): void;
    time(label?: string): void;
    timeEnd(label?: string): void;
    timeLog(label?: string, ...args: ReadonlyArray<any>): void;
    trace(...args: ReadonlyArray<any>): void;
    warn(...args: ReadonlyArray<any>): void;
}
interface Console {
    readonly [TypeId$5]: TypeId$5;
    assert(condition: boolean, ...args: ReadonlyArray<any>): Effect<void>;
    readonly clear: Effect<void>;
    count(label?: string): Effect<void>;
    countReset(label?: string): Effect<void>;
    debug(...args: ReadonlyArray<any>): Effect<void>;
    dir(item: any, options?: any): Effect<void>;
    dirxml(...args: ReadonlyArray<any>): Effect<void>;
    error(...args: ReadonlyArray<any>): Effect<void>;
    group(options?: {
        readonly label?: string | undefined;
        readonly collapsed?: boolean | undefined;
    }): Effect<void>;
    readonly groupEnd: Effect<void>;
    info(...args: ReadonlyArray<any>): Effect<void>;
    log(...args: ReadonlyArray<any>): Effect<void>;
    table(tabularData: any, properties?: ReadonlyArray<string>): Effect<void>;
    time(label?: string): Effect<void>;
    timeEnd(label?: string): Effect<void>;
    timeLog(label?: string, ...args: ReadonlyArray<any>): Effect<void>;
    trace(...args: ReadonlyArray<any>): Effect<void>;
    warn(...args: ReadonlyArray<any>): Effect<void>;
    readonly unsafe: UnsafeConsole;
}
declare const Console: Tag$1<Console, Console>;
declare const RandomTypeId: unique symbol;
type RandomTypeId = typeof RandomTypeId;
interface Random {
    readonly [RandomTypeId]: RandomTypeId;
    readonly next: Effect<number>;
    readonly nextBoolean: Effect<boolean>;
    readonly nextInt: Effect<number>;
    nextRange(min: number, max: number): Effect<number>;
    nextIntBetween(min: number, max: number): Effect<number>;
    shuffle<A>(elements: Iterable<A>): Effect<Chunk<A>>;
}
declare const Random: Tag$1<Random, Random>;
declare const TracerTypeId: unique symbol;
type TracerTypeId = typeof TracerTypeId;
type SpanStatus = {
    _tag: "Started";
    startTime: bigint;
} | {
    _tag: "Ended";
    startTime: bigint;
    endTime: bigint;
    exit: Exit<unknown, unknown>;
};
type AnySpan = Span | ExternalSpan;
interface ParentSpan {
    readonly _: unique symbol;
}
declare const ParentSpan: Tag$1<ParentSpan, AnySpan>;
interface ExternalSpan {
    readonly _tag: "ExternalSpan";
    readonly spanId: string;
    readonly traceId: string;
    readonly sampled: boolean;
    readonly context: Context<never>;
}
interface SpanOptions {
    readonly attributes?: Record<string, unknown> | undefined;
    readonly links?: ReadonlyArray<SpanLink> | undefined;
    readonly parent?: AnySpan | undefined;
    readonly root?: boolean | undefined;
    readonly context?: Context<never> | undefined;
    readonly kind?: SpanKind | undefined;
    readonly captureStackTrace?: boolean | LazyArg<string | undefined> | undefined;
}
type SpanKind = "internal" | "server" | "client" | "producer" | "consumer";
interface Span {
    readonly _tag: "Span";
    readonly name: string;
    readonly spanId: string;
    readonly traceId: string;
    readonly parent: Option<AnySpan>;
    readonly context: Context<never>;
    readonly status: SpanStatus;
    readonly attributes: ReadonlyMap<string, unknown>;
    readonly links: ReadonlyArray<SpanLink>;
    readonly sampled: boolean;
    readonly kind: SpanKind;
    end(endTime: bigint, exit: Exit<unknown, unknown>): void;
    attribute(key: string, value: unknown): void;
    event(name: string, startTime: bigint, attributes?: Record<string, unknown>): void;
    addLinks(links: ReadonlyArray<SpanLink>): void;
}
interface SpanLink {
    readonly _tag: "SpanLink";
    readonly span: AnySpan;
    readonly attributes: Readonly<Record<string, unknown>>;
}
interface Tracer {
    readonly [TracerTypeId]: TracerTypeId;
    span(name: string, parent: Option<AnySpan>, context: Context<never>, links: ReadonlyArray<SpanLink>, startTime: bigint, kind: SpanKind, options?: SpanOptions): Span;
    context<X>(f: () => X, fiber: RuntimeFiber<any, any>): X;
}
declare const Tracer: Tag$1<Tracer, Tracer>;
type DefaultServices = Clock | Console | Random | ConfigProvider | Tracer;
declare const FiberStatusTypeId: unique symbol;
type FiberStatusTypeId = typeof FiberStatusTypeId;
type FiberStatus = Done$1 | Running | Suspended;
interface Done$1 extends Equal {
    readonly _tag: "Done";
    readonly [FiberStatusTypeId]: FiberStatusTypeId;
}
interface Running extends Equal {
    readonly _tag: "Running";
    readonly [FiberStatusTypeId]: FiberStatusTypeId;
    readonly runtimeFlags: RuntimeFlags;
}
interface Suspended extends Equal {
    readonly _tag: "Suspended";
    readonly [FiberStatusTypeId]: FiberStatusTypeId;
    readonly runtimeFlags: RuntimeFlags;
    readonly blockingOn: FiberId;
}
declare const SupervisorTypeId: unique symbol;
type SupervisorTypeId = typeof SupervisorTypeId;
interface Supervisor<out T> extends Supervisor.Variance<T> {
    readonly value: Effect<T>;
    onStart<A, E, R>(context: Context<R>, effect: Effect<A, E, R>, parent: Option<RuntimeFiber<any, any>>, fiber: RuntimeFiber<A, E>): void;
    onEnd<A, E>(value: Exit<A, E>, fiber: RuntimeFiber<A, E>): void;
    onEffect<A, E>(fiber: RuntimeFiber<A, E>, effect: Effect<any, any, any>): void;
    onSuspend<A, E>(fiber: RuntimeFiber<A, E>): void;
    onResume<A, E>(fiber: RuntimeFiber<A, E>): void;
    map<B>(f: (a: T) => B): Supervisor<B>;
    zip<A>(right: Supervisor<A>): Supervisor<[
        T,
        A
    ]>;
}
declare namespace Supervisor {
    interface Variance<out T> {
        readonly [SupervisorTypeId]: {
            readonly _T: Covariant<T>;
        };
    }
}
declare const FiberTypeId: unique symbol;
type FiberTypeId = typeof FiberTypeId;
declare const RuntimeFiberTypeId: unique symbol;
type RuntimeFiberTypeId = typeof RuntimeFiberTypeId;
interface FiberUnify<A extends {
    [typeSymbol]?: any;
}> extends EffectUnify<A> {
    Fiber?: () => A[typeSymbol] extends Fiber<infer A0, infer E0> | infer _ ? Fiber<A0, E0> : never;
}
interface FiberUnifyIgnore extends EffectUnifyIgnore {
    Effect?: true;
}
interface RuntimeFiber<out A, out E = never> extends Fiber<A, E>, Fiber.RuntimeVariance<A, E> {
    get currentOpCount(): number;
    getFiberRef<X>(fiberRef: FiberRef<X>): X;
    id(): Runtime$1;
    readonly status: Effect<FiberStatus>;
    readonly runtimeFlags: Effect<RuntimeFlags>;
    addObserver(observer: (exit: Exit<A, E>) => void): void;
    removeObserver(observer: (exit: Exit<A, E>) => void): void;
    getFiberRefs(): FiberRefs;
    unsafePoll(): Exit<A, E> | null;
    unsafeInterruptAsFork(fiberId: FiberId): void;
    get currentContext(): Context<never>;
    get currentDefaultServices(): Context<DefaultServices>;
    get currentScheduler(): Scheduler;
    get currentTracer(): Tracer;
    get currentSpan(): AnySpan | undefined;
    get currentSupervisor(): Supervisor<unknown>;
    readonly [typeSymbol]?: unknown;
    readonly [unifySymbol]?: RuntimeFiberUnify<this>;
    readonly [ignoreSymbol]?: RuntimeFiberUnifyIgnore;
}
interface RuntimeFiberUnify<A extends {
    [typeSymbol]?: any;
}> extends FiberUnify<A> {
    RuntimeFiber?: () => A[typeSymbol] extends RuntimeFiber<infer A0, infer E0> | infer _ ? RuntimeFiber<A0, E0> : never;
}
interface RuntimeFiberUnifyIgnore extends FiberUnifyIgnore {
    Fiber?: true;
}
interface Fiber<out A, out E = never> extends Effect<A, E>, Fiber.Variance<A, E> {
    id(): FiberId;
    readonly await: Effect<Exit<A, E>>;
    readonly children: Effect<Array<Fiber.Runtime<any, any>>>;
    readonly inheritAll: Effect<void>;
    readonly poll: Effect<Option<Exit<A, E>>>;
    interruptAsFork(fiberId: FiberId): Effect<void>;
    readonly [typeSymbol]?: unknown;
    readonly [unifySymbol]?: FiberUnify<this>;
    readonly [ignoreSymbol]?: FiberUnifyIgnore;
}
declare namespace Fiber {
    type Runtime<A, E = never> = RuntimeFiber<A, E>;
    interface Variance<out A, out E> {
        readonly [FiberTypeId]: {
            readonly _A: Covariant<A>;
            readonly _E: Covariant<E>;
        };
    }
    interface RuntimeVariance<out A, out E> {
        readonly [RuntimeFiberTypeId]: {
            readonly _A: Covariant<A>;
            readonly _E: Covariant<E>;
        };
    }
    interface Dump {
        readonly id: Runtime$1;
        readonly status: FiberStatus;
    }
    interface Descriptor {
        readonly id: FiberId;
        readonly status: FiberStatus;
        readonly interruptors: HashSet<FiberId>;
    }
}
type Task = () => void;
interface Scheduler {
    shouldYield(fiber: RuntimeFiber<unknown, unknown>): number | false;
    scheduleTask(task: Task, priority: number, fiber?: RuntimeFiber<unknown, unknown>): void;
}
declare const FiberRefTypeId: unique symbol;
type FiberRefTypeId = typeof FiberRefTypeId;
interface FiberRef<in out A> extends Effect<A>, Variance<A> {
    readonly [typeSymbol]?: unknown;
    readonly [unifySymbol]?: FiberRefUnify<this>;
    readonly [ignoreSymbol]?: FiberRefUnifyIgnore;
}
interface FiberRefUnify<A extends {
    [typeSymbol]?: any;
}> extends EffectUnify<A> {
    FiberRef?: () => Extract<A[typeSymbol], FiberRef<any>>;
}
interface FiberRefUnifyIgnore extends EffectUnifyIgnore {
    Effect?: true;
}
interface Variance<in out A> {
    readonly [FiberRefTypeId]: {
        readonly _A: Invariant<A>;
    };
}
interface Cancel<out A, out E = never> {
    (fiberId?: FiberId, options?: RunCallbackOptions<A, E> | undefined): void;
}
interface Runtime<in R> extends Pipeable {
    readonly context: Context<R>;
    readonly runtimeFlags: RuntimeFlags;
    readonly fiberRefs: FiberRefs;
}
declare namespace Runtime {
    type Context<T extends Runtime<never>> = [
        T
    ] extends [
        Runtime<infer R>
    ] ? R : never;
}
interface RunForkOptions {
    readonly scheduler?: Scheduler | undefined;
    readonly updateRefs?: ((refs: FiberRefs, fiberId: Runtime$1) => FiberRefs) | undefined;
    readonly immediate?: boolean;
    readonly scope?: Scope;
}
interface RunCallbackOptions<in A, in E = never> extends RunForkOptions {
    readonly onExit?: ((exit: Exit<A, E>) => void) | undefined;
}
declare const TimeZoneTypeId: unique symbol;
type TimeZoneTypeId = typeof TimeZoneTypeId;
type TimeZone = TimeZone.Offset | TimeZone.Named;
declare namespace TimeZone {
    interface Proto extends Inspectable {
        readonly [TimeZoneTypeId]: TimeZoneTypeId;
    }
    interface Offset extends Proto {
        readonly _tag: "Offset";
        readonly offset: number;
    }
    interface Named extends Proto {
        readonly _tag: "Named";
        readonly id: string;
    }
}
declare const TypeId$4: unique symbol;
type TypeId$4 = typeof TypeId$4;
interface Cron extends Pipeable, Equal, Inspectable {
    readonly [TypeId$4]: TypeId$4;
    readonly tz: Option<TimeZone>;
    readonly seconds: ReadonlySet<number>;
    readonly minutes: ReadonlySet<number>;
    readonly hours: ReadonlySet<number>;
    readonly days: ReadonlySet<number>;
    readonly months: ReadonlySet<number>;
    readonly weekdays: ReadonlySet<number>;
}
declare const TypeId$3: unique symbol;
type TypeId$3 = typeof TypeId$3;
interface Readable<A, E = never, R = never> extends Pipeable {
    readonly [TypeId$3]: TypeId$3;
    readonly get: Effect<A, E, R>;
}
declare const RefTypeId: unique symbol;
type RefTypeId = typeof RefTypeId;
interface RefUnify<A extends {
    [typeSymbol]?: any;
}> extends EffectUnify<A> {
    Ref?: () => Extract<A[typeSymbol], Ref<any>>;
}
interface RefUnifyIgnore extends EffectUnifyIgnore {
    Effect?: true;
}
interface Ref<in out A> extends Ref.Variance<A>, Effect<A>, Readable<A> {
    modify<B>(f: (a: A) => readonly [
        B,
        A
    ]): Effect<B>;
    readonly [typeSymbol]?: unknown;
    readonly [unifySymbol]?: RefUnify<this>;
    readonly [ignoreSymbol]?: RefUnifyIgnore;
}
declare namespace Ref {
    interface Variance<in out A> {
        readonly [RefTypeId]: {
            readonly _A: Invariant<A>;
        };
    }
}
declare const make: <A>(value: A) => Effect<Ref<A>>;
declare const get: <A>(self: Ref<A>) => Effect<A>;
declare const getAndSet: {
    <A>(value: A): (self: Ref<A>) => Effect<A>;
    <A>(self: Ref<A>, value: A): Effect<A>;
};
declare const getAndUpdate: {
    <A>(f: (a: A) => A): (self: Ref<A>) => Effect<A>;
    <A>(self: Ref<A>, f: (a: A) => A): Effect<A>;
};
declare const getAndUpdateSome: {
    <A>(pf: (a: A) => Option<A>): (self: Ref<A>) => Effect<A>;
    <A>(self: Ref<A>, pf: (a: A) => Option<A>): Effect<A>;
};
declare const modify: {
    <A, B>(f: (a: A) => readonly [
        B,
        A
    ]): (self: Ref<A>) => Effect<B>;
    <A, B>(self: Ref<A>, f: (a: A) => readonly [
        B,
        A
    ]): Effect<B>;
};
declare const modifySome: {
    <B, A>(fallback: B, pf: (a: A) => Option<readonly [
        B,
        A
    ]>): (self: Ref<A>) => Effect<B>;
    <A, B>(self: Ref<A>, fallback: B, pf: (a: A) => Option<readonly [
        B,
        A
    ]>): Effect<B>;
};
declare const set: {
    <A>(value: A): (self: Ref<A>) => Effect<void>;
    <A>(self: Ref<A>, value: A): Effect<void>;
};
declare const setAndGet: {
    <A>(value: A): (self: Ref<A>) => Effect<A>;
    <A>(self: Ref<A>, value: A): Effect<A>;
};
declare const update: {
    <A>(f: (a: A) => A): (self: Ref<A>) => Effect<void>;
    <A>(self: Ref<A>, f: (a: A) => A): Effect<void>;
};
declare const updateAndGet: {
    <A>(f: (a: A) => A): (self: Ref<A>) => Effect<A>;
    <A>(self: Ref<A>, f: (a: A) => A): Effect<A>;
};
declare const updateSome: {
    <A>(f: (a: A) => Option<A>): (self: Ref<A>) => Effect<void>;
    <A>(self: Ref<A>, f: (a: A) => Option<A>): Effect<void>;
};
declare const updateSomeAndGet: {
    <A>(pf: (a: A) => Option<A>): (self: Ref<A>) => Effect<A>;
    <A>(self: Ref<A>, pf: (a: A) => Option<A>): Effect<A>;
};
declare const unsafeMake: <A>(value: A) => Ref<A>;
import Ref_d_Ref = Ref;
type Ref_d_RefTypeId = RefTypeId;
type Ref_d_RefUnify<A extends {
    [typeSymbol]?: any;
}> = RefUnify<A>;
type Ref_d_RefUnifyIgnore = RefUnifyIgnore;
declare const Ref_d_get: typeof get;
declare const Ref_d_getAndSet: typeof getAndSet;
declare const Ref_d_getAndUpdate: typeof getAndUpdate;
declare const Ref_d_getAndUpdateSome: typeof getAndUpdateSome;
declare const Ref_d_make: typeof make;
declare const Ref_d_modify: typeof modify;
declare const Ref_d_modifySome: typeof modifySome;
declare const Ref_d_set: typeof set;
declare const Ref_d_setAndGet: typeof setAndGet;
declare const Ref_d_unsafeMake: typeof unsafeMake;
declare const Ref_d_update: typeof update;
declare const Ref_d_updateAndGet: typeof updateAndGet;
declare const Ref_d_updateSome: typeof updateSome;
declare const Ref_d_updateSomeAndGet: typeof updateSomeAndGet;
declare namespace Ref_d {
    export { Ref_d_Ref as Ref, Ref_d_get as get, Ref_d_getAndSet as getAndSet, Ref_d_getAndUpdate as getAndUpdate, Ref_d_getAndUpdateSome as getAndUpdateSome, Ref_d_make as make, Ref_d_modify as modify, Ref_d_modifySome as modifySome, Ref_d_set as set, Ref_d_setAndGet as setAndGet, Ref_d_unsafeMake as unsafeMake, Ref_d_update as update, Ref_d_updateAndGet as updateAndGet, Ref_d_updateSome as updateSome, Ref_d_updateSomeAndGet as updateSomeAndGet };
    export type { Ref_d_RefTypeId as RefTypeId, Ref_d_RefUnify as RefUnify, Ref_d_RefUnifyIgnore as RefUnifyIgnore };
}
declare const IntervalTypeId: unique symbol;
type IntervalTypeId = typeof IntervalTypeId;
interface Interval {
    readonly [IntervalTypeId]: IntervalTypeId;
    readonly startMillis: number;
    readonly endMillis: number;
}
declare const IntervalsTypeId: unique symbol;
type IntervalsTypeId = typeof IntervalsTypeId;
interface Intervals {
    readonly [IntervalsTypeId]: IntervalsTypeId;
    readonly intervals: Chunk<Interval>;
}
type ScheduleDecision = Continue | Done;
interface Continue {
    readonly _tag: "Continue";
    readonly intervals: Intervals;
}
interface Done {
    readonly _tag: "Done";
}
declare const ScheduleTypeId: unique symbol;
type ScheduleTypeId = typeof ScheduleTypeId;
declare const ScheduleDriverTypeId: unique symbol;
type ScheduleDriverTypeId = typeof ScheduleDriverTypeId;
interface Schedule<out Out, in In = unknown, out R = never> extends Schedule.Variance<Out, In, R>, Pipeable {
    readonly initial: any;
    step(now: number, input: In, state: any): Effect<readonly [
        any,
        Out,
        ScheduleDecision
    ], never, R>;
}
declare namespace Schedule {
    interface Variance<out Out, in In, out R> {
        readonly [ScheduleTypeId]: {
            readonly _Out: Covariant<Out>;
            readonly _In: Contravariant<In>;
            readonly _R: Covariant<R>;
        };
    }
    interface DriverVariance<out Out, in In, out R> {
        readonly [ScheduleDriverTypeId]: {
            readonly _Out: Covariant<Out>;
            readonly _In: Contravariant<In>;
            readonly _R: Covariant<R>;
        };
    }
}
interface ScheduleDriver<out Out, in In = unknown, out R = never> extends Schedule.DriverVariance<Out, In, R> {
    readonly state: Effect<unknown>;
    readonly iterationMeta: Ref<IterationMetadata>;
    readonly last: Effect<Out, NoSuchElementException>;
    readonly reset: Effect<void>;
    next(input: In): Effect<Out, Option<never>, R>;
}
declare const makeWithState: <S, In, Out, R = never>(initial: S, step: (now: number, input: In, state: S) => Effect<readonly [
    S,
    Out,
    ScheduleDecision
], never, R>) => Schedule<Out, In, R>;
declare const isSchedule: (u: unknown) => u is Schedule<unknown, never, unknown>;
declare const addDelay: {
    <Out>(f: (out: Out) => DurationInput): <In, R>(self: Schedule<Out, In, R>) => Schedule<Out, In, R>;
    <Out, In, R>(self: Schedule<Out, In, R>, f: (out: Out) => DurationInput): Schedule<Out, In, R>;
};
declare const addDelayEffect: {
    <Out, R2>(f: (out: Out) => Effect<DurationInput, never, R2>): <In, R>(self: Schedule<Out, In, R>) => Schedule<Out, In, R2 | R>;
    <Out, In, R, R2>(self: Schedule<Out, In, R>, f: (out: Out) => Effect<DurationInput, never, R2>): Schedule<Out, In, R | R2>;
};
declare const andThen$3: {
    <Out2, In2, R2>(that: Schedule<Out2, In2, R2>): <Out, In, R>(self: Schedule<Out, In, R>) => Schedule<Out2 | Out, In & In2, R2 | R>;
    <Out, In, R, Out2, In2, R2>(self: Schedule<Out, In, R>, that: Schedule<Out2, In2, R2>): Schedule<Out | Out2, In & In2, R | R2>;
};
declare const andThenEither: {
    <Out2, In2, R2>(that: Schedule<Out2, In2, R2>): <Out, In, R>(self: Schedule<Out, In, R>) => Schedule<Either<Out2, Out>, In & In2, R2 | R>;
    <Out, In, R, Out2, In2, R2>(self: Schedule<Out, In, R>, that: Schedule<Out2, In2, R2>): Schedule<Either<Out2, Out>, In & In2, R | R2>;
};
declare const as$2: {
    <Out2>(out: Out2): <Out, In, R>(self: Schedule<Out, In, R>) => Schedule<Out2, In, R>;
    <Out, In, R, Out2>(self: Schedule<Out, In, R>, out: Out2): Schedule<Out2, In, R>;
};
declare const asVoid$1: <Out, In, R>(self: Schedule<Out, In, R>) => Schedule<void, In, R>;
declare const bothInOut: {
    <Out2, In2, R2>(that: Schedule<Out2, In2, R2>): <Out, In, R>(self: Schedule<Out, In, R>) => Schedule<[
        Out,
        Out2
    ], readonly [
        In,
        In2
    ], R2 | R>;
    <Out, In, R, Out2, In2, R2>(self: Schedule<Out, In, R>, that: Schedule<Out2, In2, R2>): Schedule<[
        Out,
        Out2
    ], readonly [
        In,
        In2
    ], R | R2>;
};
declare const check: {
    <In, Out>(test: (input: In, output: Out) => boolean): <R>(self: Schedule<Out, In, R>) => Schedule<Out, In, R>;
    <Out, In, R>(self: Schedule<Out, In, R>, test: (input: In, output: Out) => boolean): Schedule<Out, In, R>;
};
declare const checkEffect: {
    <In, Out, R2>(test: (input: In, output: Out) => Effect<boolean, never, R2>): <R>(self: Schedule<Out, In, R>) => Schedule<Out, In, R2 | R>;
    <Out, In, R, R2>(self: Schedule<Out, In, R>, test: (input: In, output: Out) => Effect<boolean, never, R2>): Schedule<Out, In, R | R2>;
};
declare const collectAllInputs: <A>() => Schedule<Chunk<A>, A>;
declare const collectAllOutputs: <Out, In, R>(self: Schedule<Out, In, R>) => Schedule<Chunk<Out>, In, R>;
declare const collectUntil: <A>(f: Predicate<A>) => Schedule<Chunk<A>, A>;
declare const collectUntilEffect: <A, R>(f: (a: A) => Effect<boolean, never, R>) => Schedule<Chunk<A>, A, R>;
declare const collectWhile: <A>(f: Predicate<A>) => Schedule<Chunk<A>, A>;
declare const collectWhileEffect: <A, R>(f: (a: A) => Effect<boolean, never, R>) => Schedule<Chunk<A>, A, R>;
declare const compose: {
    <Out2, Out, R2>(that: Schedule<Out2, Out, R2>): <In, R>(self: Schedule<Out, In, R>) => Schedule<Out2, In, R2 | R>;
    <Out, In, R, Out2, R2>(self: Schedule<Out, In, R>, that: Schedule<Out2, Out, R2>): Schedule<Out2, In, R | R2>;
};
declare const mapInput: {
    <In, In2>(f: (in2: In2) => In): <Out, R>(self: Schedule<Out, In, R>) => Schedule<Out, In2, R>;
    <Out, In, R, In2>(self: Schedule<Out, In, R>, f: (in2: In2) => In): Schedule<Out, In2, R>;
};
declare const mapInputEffect: {
    <In2, In, R2>(f: (in2: In2) => Effect<In, never, R2>): <Out, R>(self: Schedule<Out, In, R>) => Schedule<Out, In2, R2 | R>;
    <Out, In, R, In2, R2>(self: Schedule<Out, In, R>, f: (in2: In2) => Effect<In, never, R2>): Schedule<Out, In2, R | R2>;
};
declare const mapInputContext$1: {
    <R0, R>(f: (env0: Context<R0>) => Context<R>): <Out, In>(self: Schedule<Out, In, R>) => Schedule<Out, In, R0>;
    <Out, In, R, R0>(self: Schedule<Out, In, R>, f: (env0: Context<R0>) => Context<R>): Schedule<Out, In, R0>;
};
declare const count: Schedule<number>;
declare const cron: {
    (cron: Cron): Schedule<[
        number,
        number
    ]>;
    (expression: string, tz?: TimeZone | string): Schedule<[
        number,
        number
    ]>;
};
declare const secondOfMinute: (second: number) => Schedule<number>;
declare const minuteOfHour: (minute: number) => Schedule<number>;
declare const hourOfDay: (hour: number) => Schedule<number>;
declare const dayOfMonth: (day: number) => Schedule<number>;
declare const dayOfWeek: (day: number) => Schedule<number>;
declare const delayed: {
    (f: (duration: Duration) => DurationInput): <Out, In, R>(self: Schedule<Out, In, R>) => Schedule<Out, In, R>;
    <Out, In, R>(self: Schedule<Out, In, R>, f: (duration: Duration) => DurationInput): Schedule<Out, In, R>;
};
declare const delayedEffect: {
    <R2>(f: (duration: Duration) => Effect<DurationInput, never, R2>): <Out, In, R>(self: Schedule<Out, In, R>) => Schedule<Out, In, R2 | R>;
    <Out, In, R, R2>(self: Schedule<Out, In, R>, f: (duration: Duration) => Effect<DurationInput, never, R2>): Schedule<Out, In, R | R2>;
};
declare const delayedSchedule: <In, R>(schedule: Schedule<Duration, In, R>) => Schedule<Duration, In, R>;
declare const delays: <Out, In, R>(self: Schedule<Out, In, R>) => Schedule<Duration, In, R>;
declare const mapBoth$2: {
    <In2, In, Out, Out2>(options: {
        readonly onInput: (in2: In2) => In;
        readonly onOutput: (out: Out) => Out2;
    }): <R>(self: Schedule<Out, In, R>) => Schedule<Out2, In2, R>;
    <Out, In, R, In2, Out2>(self: Schedule<Out, In, R>, options: {
        readonly onInput: (in2: In2) => In;
        readonly onOutput: (out: Out) => Out2;
    }): Schedule<Out2, In2, R>;
};
declare const mapBothEffect: {
    <In2, In, R2, Out, R3, Out2>(options: {
        readonly onInput: (input: In2) => Effect<In, never, R2>;
        readonly onOutput: (out: Out) => Effect<Out2, never, R3>;
    }): <R>(self: Schedule<Out, In, R>) => Schedule<Out2, In2, R2 | R3 | R>;
    <Out, In, R, In2, R2, Out2, R3>(self: Schedule<Out, In, R>, options: {
        readonly onInput: (input: In2) => Effect<In, never, R2>;
        readonly onOutput: (out: Out) => Effect<Out2, never, R3>;
    }): Schedule<Out2, In2, R | R2 | R3>;
};
declare const driver: <Out, In, R>(self: Schedule<Out, In, R>) => Effect<ScheduleDriver<Out, In, R>>;
declare const duration: (duration: DurationInput) => Schedule<Duration>;
declare const either$2: {
    <Out2, In2, R2>(that: Schedule<Out2, In2, R2>): <Out, In, R>(self: Schedule<Out, In, R>) => Schedule<[
        Out,
        Out2
    ], In & In2, R2 | R>;
    <Out, In, R, Out2, In2, R2>(self: Schedule<Out, In, R>, that: Schedule<Out2, In2, R2>): Schedule<[
        Out,
        Out2
    ], In & In2, R | R2>;
};
declare const eitherWith: {
    <Out2, In2, R2>(that: Schedule<Out2, In2, R2>, f: (x: Intervals, y: Intervals) => Intervals): <Out, In, R>(self: Schedule<Out, In, R>) => Schedule<[
        Out,
        Out2
    ], In & In2, R2 | R>;
    <Out, In, R, Out2, In2, R2>(self: Schedule<Out, In, R>, that: Schedule<Out2, In2, R2>, f: (x: Intervals, y: Intervals) => Intervals): Schedule<[
        Out,
        Out2
    ], In & In2, R | R2>;
};
declare const elapsed: Schedule<Duration>;
declare const ensuring$1: {
    <X>(finalizer: Effect<X, never, never>): <Out, In, R>(self: Schedule<Out, In, R>) => Schedule<Out, In, R>;
    <Out, In, R, X>(self: Schedule<Out, In, R>, finalizer: Effect<X, never, never>): Schedule<Out, In, R>;
};
declare const exponential: (base: DurationInput, factor?: number) => Schedule<Duration>;
declare const fibonacci: (one: DurationInput) => Schedule<Duration>;
declare const fixed: (interval: DurationInput) => Schedule<number>;
declare const forever$1: Schedule<number>;
declare const fromDelay: (delay: DurationInput) => Schedule<Duration>;
declare const fromDelays: (delay: DurationInput, ...delays: Array<DurationInput>) => Schedule<Duration>;
declare const fromFunction$1: <A, B>(f: (a: A) => B) => Schedule<B, A>;
declare const identity: <A>() => Schedule<A, A>;
declare const passthrough$1: <Out, In, R>(self: Schedule<Out, In, R>) => Schedule<In, In, R>;
declare const intersect: {
    <Out2, In2, R2>(that: Schedule<Out2, In2, R2>): <Out, In, R>(self: Schedule<Out, In, R>) => Schedule<[
        Out,
        Out2
    ], In & In2, R2 | R>;
    <Out, In, R, Out2, In2, R2>(self: Schedule<Out, In, R>, that: Schedule<Out2, In2, R2>): Schedule<[
        Out,
        Out2
    ], In & In2, R | R2>;
};
declare const intersectWith: {
    <Out2, In2, R2>(that: Schedule<Out2, In2, R2>, f: (x: Intervals, y: Intervals) => Intervals): <Out, In, R>(self: Schedule<Out, In, R>) => Schedule<[
        Out,
        Out2
    ], In & In2, R2 | R>;
    <Out, In, R, Out2, In2, R2>(self: Schedule<Out, In, R>, that: Schedule<Out2, In2, R2>, f: (x: Intervals, y: Intervals) => Intervals): Schedule<[
        Out,
        Out2
    ], In & In2, R | R2>;
};
declare const jittered: <Out, In, R>(self: Schedule<Out, In, R>) => Schedule<Out, In, R>;
declare const jitteredWith: {
    (options: {
        min?: number | undefined;
        max?: number | undefined;
    }): <Out, In, R>(self: Schedule<Out, In, R>) => Schedule<Out, In, R>;
    <Out, In, R>(self: Schedule<Out, In, R>, options: {
        min?: number | undefined;
        max?: number | undefined;
    }): Schedule<Out, In, R>;
};
declare const linear: (base: DurationInput) => Schedule<Duration>;
declare const map$4: {
    <Out, Out2>(f: (out: Out) => Out2): <In, R>(self: Schedule<Out, In, R>) => Schedule<Out2, In, R>;
    <Out, In, R, Out2>(self: Schedule<Out, In, R>, f: (out: Out) => Out2): Schedule<Out2, In, R>;
};
declare const mapEffect: {
    <Out, Out2, R2>(f: (out: Out) => Effect<Out2, never, R2>): <In, R>(self: Schedule<Out, In, R>) => Schedule<Out2, In, R2 | R>;
    <Out, In, R, Out2, R2>(self: Schedule<Out, In, R>, f: (out: Out) => Effect<Out2, never, R2>): Schedule<Out2, In, R | R2>;
};
declare const modifyDelay: {
    <Out>(f: (out: Out, duration: Duration) => DurationInput): <In, R>(self: Schedule<Out, In, R>) => Schedule<Out, In, R>;
    <Out, In, R>(self: Schedule<Out, In, R>, f: (out: Out, duration: Duration) => DurationInput): Schedule<Out, In, R>;
};
declare const modifyDelayEffect: {
    <Out, R2>(f: (out: Out, duration: Duration) => Effect<DurationInput, never, R2>): <In, R>(self: Schedule<Out, In, R>) => Schedule<Out, In, R2 | R>;
    <Out, In, R, R2>(self: Schedule<Out, In, R>, f: (out: Out, duration: Duration) => Effect<DurationInput, never, R2>): Schedule<Out, In, R | R2>;
};
declare const onDecision: {
    <Out, X, R2>(f: (out: Out, decision: ScheduleDecision) => Effect<X, never, R2>): <In, R>(self: Schedule<Out, In, R>) => Schedule<Out, In, R2 | R>;
    <Out, In, R, X, R2>(self: Schedule<Out, In, R>, f: (out: Out, decision: ScheduleDecision) => Effect<X, never, R2>): Schedule<Out, In, R | R2>;
};
declare const once$1: Schedule<void>;
declare const provideContext: {
    <R>(context: Context<R>): <Out, In>(self: Schedule<Out, In, R>) => Schedule<Out, In, never>;
    <Out, In, R>(self: Schedule<Out, In, R>, context: Context<R>): Schedule<Out, In, never>;
};
declare const provideService$1: {
    <I, S>(tag: Tag$1<I, S>, service: NoInfer$1<S>): <Out, In, R>(self: Schedule<Out, In, R>) => Schedule<Out, In, Exclude<R, I>>;
    <Out, In, R, I, S>(self: Schedule<Out, In, R>, tag: Tag$1<I, S>, service: NoInfer$1<S>): Schedule<Out, In, Exclude<R, I>>;
};
declare const recurUntil: <A>(f: Predicate<A>) => Schedule<A, A>;
declare const recurUntilEffect: <A, R>(f: (a: A) => Effect<boolean, never, R>) => Schedule<A, A, R>;
declare const recurUntilOption: <A, B>(pf: (a: A) => Option<B>) => Schedule<Option<B>, A>;
declare const recurUpTo: (duration: DurationInput) => Schedule<Duration>;
declare const recurWhile: <A>(f: Predicate<A>) => Schedule<A, A>;
declare const recurWhileEffect: <A, R>(f: (a: A) => Effect<boolean, never, R>) => Schedule<A, A, R>;
declare const recurs: (n: number) => Schedule<number>;
declare const reduce$2: {
    <Out, Z>(zero: Z, f: (z: Z, out: Out) => Z): <In, R>(self: Schedule<Out, In, R>) => Schedule<Z, In, R>;
    <Out, In, R, Z>(self: Schedule<Out, In, R>, zero: Z, f: (z: Z, out: Out) => Z): Schedule<Z, In, R>;
};
declare const reduceEffect$1: {
    <Z, Out, R2>(zero: Z, f: (z: Z, out: Out) => Effect<Z, never, R2>): <In, R>(self: Schedule<Out, In, R>) => Schedule<Z, In, R2 | R>;
    <Out, In, R, Z, R2>(self: Schedule<Out, In, R>, zero: Z, f: (z: Z, out: Out) => Effect<Z, never, R2>): Schedule<Z, In, R | R2>;
};
declare const repeatForever: Schedule<number>;
declare const repetitions: <Out, In, R>(self: Schedule<Out, In, R>) => Schedule<number, In, R>;
declare const resetAfter: {
    (duration: DurationInput): <Out, In, R>(self: Schedule<Out, In, R>) => Schedule<Out, In, R>;
    <Out, In, R>(self: Schedule<Out, In, R>, duration: DurationInput): Schedule<Out, In, R>;
};
declare const resetWhen: {
    <Out>(f: Predicate<Out>): <In, R>(self: Schedule<Out, In, R>) => Schedule<Out, In, R>;
    <Out, In, R>(self: Schedule<Out, In, R>, f: Predicate<Out>): Schedule<Out, In, R>;
};
declare const run: {
    <In>(now: number, input: Iterable<In>): <Out, R>(self: Schedule<Out, In, R>) => Effect<Chunk<Out>, never, R>;
    <Out, In, R>(self: Schedule<Out, In, R>, now: number, input: Iterable<In>): Effect<Chunk<Out>, never, R>;
};
declare const spaced: (duration: DurationInput) => Schedule<number>;
declare const stop: Schedule<void>;
declare const succeed$2: <A>(value: A) => Schedule<A>;
declare const sync$2: <A>(evaluate: LazyArg<A>) => Schedule<A>;
declare const tapInput: {
    <In2, X, R2>(f: (input: In2) => Effect<X, never, R2>): <Out, In, R>(self: Schedule<Out, In, R>) => Schedule<Out, In & In2, R2 | R>;
    <Out, In, R, In2, X, R2>(self: Schedule<Out, In, R>, f: (input: In2) => Effect<X, never, R2>): Schedule<Out, In & In2, R | R2>;
};
declare const tapOutput: {
    <X, R2, Out>(f: (out: NoInfer$1<Out>) => Effect<X, never, R2>): <In, R>(self: Schedule<Out, In, R>) => Schedule<Out, In, R2 | R>;
    <Out, In, R, X, R2>(self: Schedule<Out, In, R>, f: (out: Out) => Effect<X, never, R2>): Schedule<Out, In, R | R2>;
};
declare const unfold: <A>(initial: A, f: (a: A) => A) => Schedule<A>;
declare const union: {
    <Out2, In2, R2>(that: Schedule<Out2, In2, R2>): <Out, In, R>(self: Schedule<Out, In, R>) => Schedule<[
        Out,
        Out2
    ], In & In2, R2 | R>;
    <Out, In, R, Out2, In2, R2>(self: Schedule<Out, In, R>, that: Schedule<Out2, In2, R2>): Schedule<[
        Out,
        Out2
    ], In & In2, R | R2>;
};
declare const unionWith: {
    <Out2, In2, R2>(that: Schedule<Out2, In2, R2>, f: (x: Intervals, y: Intervals) => Intervals): <Out, In, R>(self: Schedule<Out, In, R>) => Schedule<[
        Out,
        Out2
    ], In & In2, R2 | R>;
    <Out, In, R, Out2, In2, R2>(self: Schedule<Out, In, R>, that: Schedule<Out2, In2, R2>, f: (x: Intervals, y: Intervals) => Intervals): Schedule<[
        Out,
        Out2
    ], In & In2, R | R2>;
};
declare const untilInput: {
    <In>(f: Predicate<In>): <Out, R>(self: Schedule<Out, In, R>) => Schedule<Out, In, R>;
    <Out, In, R>(self: Schedule<Out, In, R>, f: Predicate<In>): Schedule<Out, In, R>;
};
declare const untilInputEffect: {
    <In, R2>(f: (input: In) => Effect<boolean, never, R2>): <Out, R>(self: Schedule<Out, In, R>) => Schedule<Out, In, R2 | R>;
    <Out, In, R, R2>(self: Schedule<Out, In, R>, f: (input: In) => Effect<boolean, never, R2>): Schedule<Out, In, R | R2>;
};
declare const untilOutput: {
    <Out>(f: Predicate<Out>): <In, R>(self: Schedule<Out, In, R>) => Schedule<Out, In, R>;
    <Out, In, R>(self: Schedule<Out, In, R>, f: Predicate<Out>): Schedule<Out, In, R>;
};
declare const untilOutputEffect: {
    <Out, R2>(f: (out: Out) => Effect<boolean, never, R2>): <In, R>(self: Schedule<Out, In, R>) => Schedule<Out, In, R2 | R>;
    <Out, In, R, R2>(self: Schedule<Out, In, R>, f: (out: Out) => Effect<boolean, never, R2>): Schedule<Out, In, R | R2>;
};
declare const upTo: {
    (duration: DurationInput): <Out, In, R>(self: Schedule<Out, In, R>) => Schedule<Out, In, R>;
    <Out, In, R>(self: Schedule<Out, In, R>, duration: DurationInput): Schedule<Out, In, R>;
};
declare const whileInput: {
    <In>(f: Predicate<In>): <Out, R>(self: Schedule<Out, In, R>) => Schedule<Out, In, R>;
    <Out, In, R>(self: Schedule<Out, In, R>, f: Predicate<In>): Schedule<Out, In, R>;
};
declare const whileInputEffect: {
    <In, R2>(f: (input: In) => Effect<boolean, never, R2>): <Out, R>(self: Schedule<Out, In, R>) => Schedule<Out, In, R2 | R>;
    <Out, In, R, R2>(self: Schedule<Out, In, R>, f: (input: In) => Effect<boolean, never, R2>): Schedule<Out, In, R | R2>;
};
declare const whileOutput: {
    <Out>(f: Predicate<Out>): <In, R>(self: Schedule<Out, In, R>) => Schedule<Out, In, R>;
    <Out, In, R>(self: Schedule<Out, In, R>, f: Predicate<Out>): Schedule<Out, In, R>;
};
declare const whileOutputEffect: {
    <Out, R2>(f: (out: Out) => Effect<boolean, never, R2>): <In, R>(self: Schedule<Out, In, R>) => Schedule<Out, In, R2 | R>;
    <Out, In, R, R2>(self: Schedule<Out, In, R>, f: (out: Out) => Effect<boolean, never, R2>): Schedule<Out, In, R | R2>;
};
declare const windowed: (interval: DurationInput) => Schedule<number>;
declare const zipLeft$1: {
    <Out2, In2, R2>(that: Schedule<Out2, In2, R2>): <Out, In, R>(self: Schedule<Out, In, R>) => Schedule<Out, In & In2, R2 | R>;
    <Out, In, R, Out2, In2, R2>(self: Schedule<Out, In, R>, that: Schedule<Out2, In2, R2>): Schedule<Out, In & In2, R | R2>;
};
declare const zipRight$1: {
    <Out2, In2, R2>(that: Schedule<Out2, In2, R2>): <Out, In, R>(self: Schedule<Out, In, R>) => Schedule<Out2, In & In2, R2 | R>;
    <Out, In, R, Out2, In2, R2>(self: Schedule<Out, In, R>, that: Schedule<Out2, In2, R2>): Schedule<Out2, In & In2, R | R2>;
};
declare const zipWith$3: {
    <Out2, In2, R2, Out, Out3>(that: Schedule<Out2, In2, R2>, f: (out: Out, out2: Out2) => Out3): <In, R>(self: Schedule<Out, In, R>) => Schedule<Out3, In & In2, R2 | R>;
    <Out, In, R, Out2, In2, R2, Out3>(self: Schedule<Out, In, R>, that: Schedule<Out2, In2, R2>, f: (out: Out, out2: Out2) => Out3): Schedule<Out3, In & In2, R | R2>;
};
interface IterationMetadata {
    readonly input: unknown;
    readonly output: unknown;
    readonly recurrence: number;
    readonly start: number;
    readonly now: number;
    readonly elapsed: Duration;
    readonly elapsedSincePrevious: Duration;
}
interface CurrentIterationMetadata {
    readonly _: unique symbol;
}
declare const CurrentIterationMetadata: Reference<CurrentIterationMetadata, IterationMetadata>;
declare const Schedule_d_CurrentIterationMetadata: typeof CurrentIterationMetadata;
type Schedule_d_IterationMetadata = IterationMetadata;
import Schedule_d_Schedule = Schedule;
type Schedule_d_ScheduleDriver<out Out, in In = unknown, out R = never> = ScheduleDriver<Out, In, R>;
type Schedule_d_ScheduleDriverTypeId = ScheduleDriverTypeId;
type Schedule_d_ScheduleTypeId = ScheduleTypeId;
declare const Schedule_d_addDelay: typeof addDelay;
declare const Schedule_d_addDelayEffect: typeof addDelayEffect;
declare const Schedule_d_andThenEither: typeof andThenEither;
declare const Schedule_d_bothInOut: typeof bothInOut;
declare const Schedule_d_check: typeof check;
declare const Schedule_d_checkEffect: typeof checkEffect;
declare const Schedule_d_collectAllInputs: typeof collectAllInputs;
declare const Schedule_d_collectAllOutputs: typeof collectAllOutputs;
declare const Schedule_d_collectUntil: typeof collectUntil;
declare const Schedule_d_collectUntilEffect: typeof collectUntilEffect;
declare const Schedule_d_collectWhile: typeof collectWhile;
declare const Schedule_d_collectWhileEffect: typeof collectWhileEffect;
declare const Schedule_d_compose: typeof compose;
declare const Schedule_d_count: typeof count;
declare const Schedule_d_cron: typeof cron;
declare const Schedule_d_dayOfMonth: typeof dayOfMonth;
declare const Schedule_d_dayOfWeek: typeof dayOfWeek;
declare const Schedule_d_delayed: typeof delayed;
declare const Schedule_d_delayedEffect: typeof delayedEffect;
declare const Schedule_d_delayedSchedule: typeof delayedSchedule;
declare const Schedule_d_delays: typeof delays;
declare const Schedule_d_driver: typeof driver;
declare const Schedule_d_duration: typeof duration;
declare const Schedule_d_eitherWith: typeof eitherWith;
declare const Schedule_d_elapsed: typeof elapsed;
declare const Schedule_d_exponential: typeof exponential;
declare const Schedule_d_fibonacci: typeof fibonacci;
declare const Schedule_d_fixed: typeof fixed;
declare const Schedule_d_fromDelay: typeof fromDelay;
declare const Schedule_d_fromDelays: typeof fromDelays;
declare const Schedule_d_hourOfDay: typeof hourOfDay;
declare const Schedule_d_identity: typeof identity;
declare const Schedule_d_intersect: typeof intersect;
declare const Schedule_d_intersectWith: typeof intersectWith;
declare const Schedule_d_isSchedule: typeof isSchedule;
declare const Schedule_d_jittered: typeof jittered;
declare const Schedule_d_jitteredWith: typeof jitteredWith;
declare const Schedule_d_linear: typeof linear;
declare const Schedule_d_makeWithState: typeof makeWithState;
declare const Schedule_d_mapBothEffect: typeof mapBothEffect;
declare const Schedule_d_mapEffect: typeof mapEffect;
declare const Schedule_d_mapInput: typeof mapInput;
declare const Schedule_d_mapInputEffect: typeof mapInputEffect;
declare const Schedule_d_minuteOfHour: typeof minuteOfHour;
declare const Schedule_d_modifyDelay: typeof modifyDelay;
declare const Schedule_d_modifyDelayEffect: typeof modifyDelayEffect;
declare const Schedule_d_onDecision: typeof onDecision;
declare const Schedule_d_provideContext: typeof provideContext;
declare const Schedule_d_recurUntil: typeof recurUntil;
declare const Schedule_d_recurUntilEffect: typeof recurUntilEffect;
declare const Schedule_d_recurUntilOption: typeof recurUntilOption;
declare const Schedule_d_recurUpTo: typeof recurUpTo;
declare const Schedule_d_recurWhile: typeof recurWhile;
declare const Schedule_d_recurWhileEffect: typeof recurWhileEffect;
declare const Schedule_d_recurs: typeof recurs;
declare const Schedule_d_repeatForever: typeof repeatForever;
declare const Schedule_d_repetitions: typeof repetitions;
declare const Schedule_d_resetAfter: typeof resetAfter;
declare const Schedule_d_resetWhen: typeof resetWhen;
declare const Schedule_d_run: typeof run;
declare const Schedule_d_secondOfMinute: typeof secondOfMinute;
declare const Schedule_d_spaced: typeof spaced;
declare const Schedule_d_stop: typeof stop;
declare const Schedule_d_tapInput: typeof tapInput;
declare const Schedule_d_tapOutput: typeof tapOutput;
declare const Schedule_d_unfold: typeof unfold;
declare const Schedule_d_union: typeof union;
declare const Schedule_d_unionWith: typeof unionWith;
declare const Schedule_d_untilInput: typeof untilInput;
declare const Schedule_d_untilInputEffect: typeof untilInputEffect;
declare const Schedule_d_untilOutput: typeof untilOutput;
declare const Schedule_d_untilOutputEffect: typeof untilOutputEffect;
declare const Schedule_d_upTo: typeof upTo;
declare const Schedule_d_whileInput: typeof whileInput;
declare const Schedule_d_whileInputEffect: typeof whileInputEffect;
declare const Schedule_d_whileOutput: typeof whileOutput;
declare const Schedule_d_whileOutputEffect: typeof whileOutputEffect;
declare const Schedule_d_windowed: typeof windowed;
declare namespace Schedule_d {
    export { Schedule_d_CurrentIterationMetadata as CurrentIterationMetadata, Schedule_d_Schedule as Schedule, Schedule_d_addDelay as addDelay, Schedule_d_addDelayEffect as addDelayEffect, andThen$3 as andThen, Schedule_d_andThenEither as andThenEither, as$2 as as, asVoid$1 as asVoid, Schedule_d_bothInOut as bothInOut, Schedule_d_check as check, Schedule_d_checkEffect as checkEffect, Schedule_d_collectAllInputs as collectAllInputs, Schedule_d_collectAllOutputs as collectAllOutputs, Schedule_d_collectUntil as collectUntil, Schedule_d_collectUntilEffect as collectUntilEffect, Schedule_d_collectWhile as collectWhile, Schedule_d_collectWhileEffect as collectWhileEffect, Schedule_d_compose as compose, Schedule_d_count as count, Schedule_d_cron as cron, Schedule_d_dayOfMonth as dayOfMonth, Schedule_d_dayOfWeek as dayOfWeek, Schedule_d_delayed as delayed, Schedule_d_delayedEffect as delayedEffect, Schedule_d_delayedSchedule as delayedSchedule, Schedule_d_delays as delays, Schedule_d_driver as driver, Schedule_d_duration as duration, either$2 as either, Schedule_d_eitherWith as eitherWith, Schedule_d_elapsed as elapsed, ensuring$1 as ensuring, Schedule_d_exponential as exponential, Schedule_d_fibonacci as fibonacci, Schedule_d_fixed as fixed, forever$1 as forever, Schedule_d_fromDelay as fromDelay, Schedule_d_fromDelays as fromDelays, fromFunction$1 as fromFunction, Schedule_d_hourOfDay as hourOfDay, Schedule_d_identity as identity, Schedule_d_intersect as intersect, Schedule_d_intersectWith as intersectWith, Schedule_d_isSchedule as isSchedule, Schedule_d_jittered as jittered, Schedule_d_jitteredWith as jitteredWith, Schedule_d_linear as linear, Schedule_d_makeWithState as makeWithState, map$4 as map, mapBoth$2 as mapBoth, Schedule_d_mapBothEffect as mapBothEffect, Schedule_d_mapEffect as mapEffect, Schedule_d_mapInput as mapInput, mapInputContext$1 as mapInputContext, Schedule_d_mapInputEffect as mapInputEffect, Schedule_d_minuteOfHour as minuteOfHour, Schedule_d_modifyDelay as modifyDelay, Schedule_d_modifyDelayEffect as modifyDelayEffect, Schedule_d_onDecision as onDecision, once$1 as once, passthrough$1 as passthrough, Schedule_d_provideContext as provideContext, provideService$1 as provideService, Schedule_d_recurUntil as recurUntil, Schedule_d_recurUntilEffect as recurUntilEffect, Schedule_d_recurUntilOption as recurUntilOption, Schedule_d_recurUpTo as recurUpTo, Schedule_d_recurWhile as recurWhile, Schedule_d_recurWhileEffect as recurWhileEffect, Schedule_d_recurs as recurs, reduce$2 as reduce, reduceEffect$1 as reduceEffect, Schedule_d_repeatForever as repeatForever, Schedule_d_repetitions as repetitions, Schedule_d_resetAfter as resetAfter, Schedule_d_resetWhen as resetWhen, Schedule_d_run as run, Schedule_d_secondOfMinute as secondOfMinute, Schedule_d_spaced as spaced, Schedule_d_stop as stop, succeed$2 as succeed, sync$2 as sync, Schedule_d_tapInput as tapInput, Schedule_d_tapOutput as tapOutput, Schedule_d_unfold as unfold, Schedule_d_union as union, Schedule_d_unionWith as unionWith, Schedule_d_untilInput as untilInput, Schedule_d_untilInputEffect as untilInputEffect, Schedule_d_untilOutput as untilOutput, Schedule_d_untilOutputEffect as untilOutputEffect, Schedule_d_upTo as upTo, Schedule_d_whileInput as whileInput, Schedule_d_whileInputEffect as whileInputEffect, Schedule_d_whileOutput as whileOutput, Schedule_d_whileOutputEffect as whileOutputEffect, Schedule_d_windowed as windowed, zipLeft$1 as zipLeft, zipRight$1 as zipRight, zipWith$3 as zipWith };
    export type { Schedule_d_IterationMetadata as IterationMetadata, Schedule_d_ScheduleDriver as ScheduleDriver, Schedule_d_ScheduleDriverTypeId as ScheduleDriverTypeId, Schedule_d_ScheduleTypeId as ScheduleTypeId };
}
declare const TypeId$2: unique symbol;
type TypeId$2 = typeof TypeId$2;
interface ExecutionPlan<Types extends {
    provides: any;
    input: any;
    error: any;
    requirements: any;
}> extends Pipeable {
    readonly [TypeId$2]: TypeId$2;
    readonly steps: NonEmptyReadonlyArray<{
        readonly provide: Context<Types["provides"]> | Layer<Types["provides"], Types["error"], Types["requirements"]>;
        readonly attempts?: number | undefined;
        readonly while?: ((input: Types["input"]) => Effect<boolean, Types["error"], Types["requirements"]>) | undefined;
        readonly schedule?: Schedule<any, Types["input"], Types["requirements"]> | undefined;
    }>;
    readonly withRequirements: Effect<ExecutionPlan<{
        provides: Types["provides"];
        input: Types["input"];
        error: Types["error"];
        requirements: never;
    }>, never, Types["requirements"]>;
}
declare const SinkTypeId: unique symbol;
type SinkTypeId = typeof SinkTypeId;
interface Sink<out A, in In = unknown, out L = never, out E = never, out R = never> extends Sink.Variance<A, In, L, E, R>, Pipeable {
}
declare namespace Sink {
    interface Variance<out A, in In, out L, out E, out R> {
        readonly [SinkTypeId]: VarianceStruct<A, In, L, E, R>;
    }
    interface VarianceStruct<out A, in In, out L, out E, out R> {
        _A: Covariant<A>;
        _In: Contravariant<In>;
        _L: Covariant<L>;
        _E: Covariant<E>;
        _R: Covariant<R>;
    }
}
declare const STMTypeId: unique symbol;
type STMTypeId = typeof STMTypeId;
interface STMUnify<A extends {
    [typeSymbol]?: any;
}> extends EffectUnify<A> {
    STM?: () => A[typeSymbol] extends STM<infer A0, infer E0, infer R0> | infer _ ? STM<A0, E0, R0> : never;
}
interface STMUnifyIgnore extends EffectUnifyIgnore {
    Effect?: true;
}
interface STM<out A, out E = never, out R = never> extends Effect<A, E, R>, STM.Variance<A, E, R>, Pipeable {
    [typeSymbol]?: unknown;
    [unifySymbol]?: STMUnify<this>;
    [ignoreSymbol]?: STMUnifyIgnore;
    [Symbol.iterator](): EffectGenerator<STM<A, E, R>>;
}
declare namespace STM {
    interface Variance<out A, out E, out R> {
        readonly [STMTypeId]: {
            readonly _A: Covariant<A>;
            readonly _E: Covariant<E>;
            readonly _R: Covariant<R>;
        };
    }
}
declare const StreamTypeId: unique symbol;
type StreamTypeId = typeof StreamTypeId;
interface StreamUnify<A extends {
    [typeSymbol]?: any;
}> extends EffectUnify<A> {
    Stream?: () => A[typeSymbol] extends Stream<infer A0, infer E0, infer R0> | infer _ ? Stream<A0, E0, R0> : never;
}
interface StreamUnifyIgnore extends EffectUnifyIgnore {
    Effect?: true;
}
interface Stream<out A, out E = never, out R = never> extends Stream.Variance<A, E, R>, Pipeable {
    [typeSymbol]?: unknown;
    [unifySymbol]?: StreamUnify<this>;
    [ignoreSymbol]?: StreamUnifyIgnore;
}
declare namespace Stream {
    interface Variance<out A, out E, out R> {
        readonly [StreamTypeId]: VarianceStruct<A, E, R>;
    }
    interface VarianceStruct<out A, out E, out R> {
        readonly _A: Covariant<A>;
        readonly _E: Covariant<E>;
        readonly _R: Covariant<R>;
    }
    type Success<T extends Stream<any, any, any>> = [
        T
    ] extends [
        Stream<infer _A, infer _E, infer _R>
    ] ? _A : never;
    type Error<T extends Stream<any, any, any>> = [
        T
    ] extends [
        Stream<infer _A, infer _E, infer _R>
    ] ? _E : never;
    type Context<T extends Stream<any, any, any>> = [
        T
    ] extends [
        Stream<infer _A, infer _E, infer _R>
    ] ? _R : never;
    type DynamicTuple<T, N extends number> = N extends N ? number extends N ? Array<T> : DynamicTupleOf<T, N, [
    ]> : never;
    type DynamicTupleOf<T, N extends number, R extends Array<unknown>> = R["length"] extends N ? R : DynamicTupleOf<T, N, [
        T,
        ...R
    ]>;
}
declare const LayerTypeId: unique symbol;
type LayerTypeId = typeof LayerTypeId;
interface Layer<in ROut, out E = never, out RIn = never> extends Layer.Variance<ROut, E, RIn>, Pipeable {
}
declare namespace Layer {
    interface Variance<in ROut, out E, out RIn> {
        readonly [LayerTypeId]: {
            readonly _ROut: Contravariant<ROut>;
            readonly _E: Covariant<E>;
            readonly _RIn: Covariant<RIn>;
        };
    }
    interface Any {
        readonly [LayerTypeId]: {
            readonly _ROut: Contravariant<never>;
            readonly _E: Covariant<any>;
            readonly _RIn: Covariant<any>;
        };
    }
    type Context<T extends Any> = [
        T
    ] extends [
        Layer<infer _ROut, infer _E, infer _RIn>
    ] ? _RIn : never;
    type Error<T extends Any> = [
        T
    ] extends [
        Layer<infer _ROut, infer _E, infer _RIn>
    ] ? _E : never;
    type Success<T extends Any> = [
        T
    ] extends [
        Layer<infer _ROut, infer _E, infer _RIn>
    ] ? _ROut : never;
}
declare const MemoMapTypeId: unique symbol;
type MemoMapTypeId = typeof MemoMapTypeId;
interface MemoMap {
    readonly [MemoMapTypeId]: MemoMapTypeId;
}
interface CurrentMemoMap {
    readonly _: unique symbol;
}
declare const CurrentMemoMap: Reference<CurrentMemoMap, MemoMap>;
declare const isLayer: (u: unknown) => u is Layer<unknown, unknown, unknown>;
declare const isFresh: <RIn, E, ROut>(self: Layer<ROut, E, RIn>) => boolean;
declare const annotateLogs$1: {
    (key: string, value: unknown): <A, E, R>(self: Layer<A, E, R>) => Layer<A, E, R>;
    (values: Record<string, unknown>): <A, E, R>(self: Layer<A, E, R>) => Layer<A, E, R>;
    <A, E, R>(self: Layer<A, E, R>, key: string, value: unknown): Layer<A, E, R>;
    <A, E, R>(self: Layer<A, E, R>, values: Record<string, unknown>): Layer<A, E, R>;
};
declare const annotateSpans$1: {
    (key: string, value: unknown): <A, E, R>(self: Layer<A, E, R>) => Layer<A, E, R>;
    (values: Record<string, unknown>): <A, E, R>(self: Layer<A, E, R>) => Layer<A, E, R>;
    <A, E, R>(self: Layer<A, E, R>, key: string, value: unknown): Layer<A, E, R>;
    <A, E, R>(self: Layer<A, E, R>, values: Record<string, unknown>): Layer<A, E, R>;
};
declare const build: <RIn, E, ROut>(self: Layer<ROut, E, RIn>) => Effect<Context<ROut>, E, Scope | RIn>;
declare const buildWithScope: {
    (scope: Scope): <RIn, E, ROut>(self: Layer<ROut, E, RIn>) => Effect<Context<ROut>, E, RIn>;
    <RIn, E, ROut>(self: Layer<ROut, E, RIn>, scope: Scope): Effect<Context<ROut>, E, RIn>;
};
declare const catchAll$1: {
    <E, RIn2, E2, ROut2>(onError: (error: E) => Layer<ROut2, E2, RIn2>): <RIn, ROut>(self: Layer<ROut, E, RIn>) => Layer<ROut & ROut2, E2, RIn2 | RIn>;
    <RIn, E, ROut, RIn2, E2, ROut2>(self: Layer<ROut, E, RIn>, onError: (error: E) => Layer<ROut2, E2, RIn2>): Layer<ROut & ROut2, E2, RIn | RIn2>;
};
declare const catchAllCause$1: {
    <E, RIn2, E2, ROut2>(onError: (cause: Cause<E>) => Layer<ROut2, E2, RIn2>): <RIn, ROut>(self: Layer<ROut, E, RIn>) => Layer<ROut & ROut2, E2, RIn2 | RIn>;
    <RIn, E, ROut, RIn2, E2, ROut22>(self: Layer<ROut, E, RIn>, onError: (cause: Cause<E>) => Layer<ROut22, E2, RIn2>): Layer<ROut & ROut22, E2, RIn | RIn2>;
};
declare const context$1: <R>() => Layer<R, never, R>;
declare const die$2: (defect: unknown) => Layer<unknown>;
declare const dieSync$1: (evaluate: LazyArg<unknown>) => Layer<unknown>;
declare const discard: <RIn, E, ROut>(self: Layer<ROut, E, RIn>) => Layer<never, E, RIn>;
declare const effect: {
    <I, S>(tag: Tag$1<I, S>): <E, R>(effect: Effect<NoInfer$1<S>, E, R>) => Layer<I, E, R>;
    <I, S, E, R>(tag: Tag$1<I, S>, effect: Effect<NoInfer$1<S>, E, R>): Layer<I, E, R>;
};
declare const effectDiscard: <X, E, R>(effect: Effect<X, E, R>) => Layer<never, E, R>;
declare const effectContext: <A, E, R>(effect: Effect<Context<A>, E, R>) => Layer<A, E, R>;
declare const empty$1: Layer<never>;
declare const extendScope: <RIn, E, ROut>(self: Layer<ROut, E, RIn>) => Layer<ROut, E, Scope | RIn>;
declare const fail$2: <E>(error: E) => Layer<unknown, E>;
declare const failSync$1: <E>(evaluate: LazyArg<E>) => Layer<unknown, E>;
declare const failCause$1: <E>(cause: Cause<E>) => Layer<unknown, E>;
declare const failCauseSync$1: <E>(evaluate: LazyArg<Cause<E>>) => Layer<unknown, E>;
declare const flatMap$3: {
    <A, A2, E2, R2>(f: (context: Context<A>) => Layer<A2, E2, R2>): <E, R>(self: Layer<A, E, R>) => Layer<A2, E2 | E, R2 | R>;
    <A, E, R, A2, E2, R2>(self: Layer<A, E, R>, f: (context: Context<A>) => Layer<A2, E2, R2>): Layer<A2, E | E2, R | R2>;
};
declare const flatten$2: {
    <I, A, E2, R2>(tag: Tag$1<I, Layer<A, E2, R2>>): <E, R>(self: Layer<I, E, R>) => Layer<A, E2 | E, R2 | R>;
    <I, E, R, A, E2, R2>(self: Layer<I, E, R>, tag: Tag$1<I, Layer<A, E2, R2>>): Layer<A, E | E2, R | R2>;
};
declare const fresh: <A, E, R>(self: Layer<A, E, R>) => Layer<A, E, R>;
type PartialEffectful<A extends object> = Simplify<{
    [K in keyof A as A[K] extends Effect<any, any, any> | Stream<any, any, any> | ((...args: any) => Effect<any, any, any> | Stream<any, any, any>) ? K : never]?: A[K];
} & {
    [K in keyof A as A[K] extends Effect<any, any, any> | Stream<any, any, any> | ((...args: any) => Effect<any, any, any> | Stream<any, any, any>) ? never : K]: A[K];
}>;
declare const mock: {
    <I, S extends object>(tag: Tag$1<I, S>): (service: PartialEffectful<S>) => Layer<I>;
    <I, S extends object>(tag: Tag$1<I, S>, service: PartialEffectful<S>): Layer<I>;
};
declare const fromFunction: <I1, S1, I2, S2>(tagA: Tag$1<I1, S1>, tagB: Tag$1<I2, S2>, f: (a: NoInfer$1<S1>) => NoInfer$1<S2>) => Layer<I2, never, I1>;
declare const launch: <RIn, E, ROut>(self: Layer<ROut, E, RIn>) => Effect<never, E, RIn>;
declare const map$3: {
    <A, B>(f: (context: Context<A>) => Context<B>): <E, R>(self: Layer<A, E, R>) => Layer<B, E, R>;
    <A, E, R, B>(self: Layer<A, E, R>, f: (context: Context<A>) => Context<B>): Layer<B, E, R>;
};
declare const mapError$1: {
    <E, E2>(f: (error: E) => E2): <A, R>(self: Layer<A, E, R>) => Layer<A, E2, R>;
    <A, E, R, E2>(self: Layer<A, E, R>, f: (error: E) => E2): Layer<A, E2, R>;
};
declare const match$3: {
    <E, A2, E2, R2, A, A3, E3, R3>(options: {
        readonly onFailure: (error: E) => Layer<A2, E2, R2>;
        readonly onSuccess: (context: Context<A>) => Layer<A3, E3, R3>;
    }): <R>(self: Layer<A, E, R>) => Layer<A2 & A3, E2 | E3, R2 | R3 | R>;
    <A, E, R, A2, E2, R2, A3, E3, R3>(self: Layer<A, E, R>, options: {
        readonly onFailure: (error: E) => Layer<A2, E2, R2>;
        readonly onSuccess: (context: Context<A>) => Layer<A3, E3, R3>;
    }): Layer<A2 & A3, E2 | E3, R | R2 | R3>;
};
declare const matchCause$1: {
    <E, A2, E2, R2, A, A3, E3, R3>(options: {
        readonly onFailure: (cause: Cause<E>) => Layer<A2, E2, R2>;
        readonly onSuccess: (context: Context<A>) => Layer<A3, E3, R3>;
    }): <R>(self: Layer<A, E, R>) => Layer<A2 & A3, E2 | E3, R2 | R3 | R>;
    <A, E, R, A2, E2, R2, A3, E3, R3>(self: Layer<A, E, R>, options: {
        readonly onFailure: (cause: Cause<E>) => Layer<A2, E2, R2>;
        readonly onSuccess: (context: Context<A>) => Layer<A3, E3, R3>;
    }): Layer<A2 & A3, E2 | E3, R | R2 | R3>;
};
declare const memoize: <RIn, E, ROut>(self: Layer<ROut, E, RIn>) => Effect<Layer<ROut, E, RIn>, never, Scope>;
declare const merge$2: {
    <RIn2, E2, ROut2>(that: Layer<ROut2, E2, RIn2>): <RIn, E1, ROut>(self: Layer<ROut, E1, RIn>) => Layer<ROut2 | ROut, E2 | E1, RIn2 | RIn>;
    <RIn, E1, ROut, RIn2, E2, ROut2>(self: Layer<ROut, E1, RIn>, that: Layer<ROut2, E2, RIn2>): Layer<ROut | ROut2, E1 | E2, RIn | RIn2>;
};
declare const mergeAll$1: <Layers extends readonly [
    Layer<never, any, any>,
    ...Array<Layer<never, any, any>>
]>(...layers: Layers) => Layer<{
    [k in keyof Layers]: Layer.Success<Layers[k]>;
}[number], {
    [k in keyof Layers]: Layer.Error<Layers[k]>;
}[number], {
    [k in keyof Layers]: Layer.Context<Layers[k]>;
}[number]>;
declare const orDie$1: <A, E, R>(self: Layer<A, E, R>) => Layer<A, never, R>;
declare const orElse$3: {
    <A2, E2, R2>(that: LazyArg<Layer<A2, E2, R2>>): <A, E, R>(self: Layer<A, E, R>) => Layer<A & A2, E2 | E, R2 | R>;
    <A, E, R, A2, E2, R2>(self: Layer<A, E, R>, that: LazyArg<Layer<A2, E2, R2>>): Layer<A & A2, E | E2, R | R2>;
};
declare const passthrough: <RIn, E, ROut>(self: Layer<ROut, E, RIn>) => Layer<RIn | ROut, E, RIn>;
declare const project: {
    <I1, S1, I2, S2>(tagA: Tag$1<I1, S1>, tagB: Tag$1<I2, S2>, f: (a: NoInfer$1<S1>) => NoInfer$1<S2>): <RIn, E>(self: Layer<I1, E, RIn>) => Layer<I2, E, RIn>;
    <RIn, E, I1, S1, I2, S2>(self: Layer<I1, E, RIn>, tagA: Tag$1<I1, S1>, tagB: Tag$1<I2, S2>, f: (a: NoInfer$1<S1>) => NoInfer$1<S2>): Layer<I2, E, RIn>;
};
declare const locallyEffect: {
    <RIn, E, ROut, RIn2, E2, ROut2>(f: (_: Effect<RIn, E, Context<ROut>>) => Effect<RIn2, E2, Context<ROut2>>): (self: Layer<ROut, E, RIn>) => Layer<ROut2, E2, RIn2>;
    <RIn, E, ROut, RIn2, E2, ROut2>(self: Layer<ROut, E, RIn>, f: (_: Effect<RIn, E, Context<ROut>>) => Effect<RIn2, E2, Context<ROut2>>): Layer<ROut2, E2, RIn2>;
};
declare const locally$1: {
    <X>(ref: FiberRef<X>, value: X): <A, E, R>(self: Layer<A, E, R>) => Layer<A, E, R>;
    <A, E, R, X>(self: Layer<A, E, R>, ref: FiberRef<X>, value: X): Layer<A, E, R>;
};
declare const locallyWith$1: {
    <X>(ref: FiberRef<X>, value: (_: X) => X): <A, E, R>(self: Layer<A, E, R>) => Layer<A, E, R>;
    <A, E, R, X>(self: Layer<A, E, R>, ref: FiberRef<X>, value: (_: X) => X): Layer<A, E, R>;
};
declare const locallyScoped$1: <A>(self: FiberRef<A>, value: A) => Layer<never>;
declare const fiberRefLocallyScopedWith: <A>(self: FiberRef<A>, value: (_: A) => A) => Layer<never>;
declare const retry$1: {
    <X, E, RIn2>(schedule: Schedule<X, NoInfer<E>, RIn2>): <ROut, RIn>(self: Layer<ROut, E, RIn>) => Layer<ROut, E, RIn2 | RIn>;
    <ROut, E, RIn, X, RIn2>(self: Layer<ROut, E, RIn>, schedule: Schedule<X, E, RIn2>): Layer<ROut, E, RIn | RIn2>;
};
declare const scope$1: Layer<Scope>;
declare const scoped$1: {
    <I, S>(tag: Tag$1<I, S>): <E, R>(effect: Effect<NoInfer$1<S>, E, R>) => Layer<I, E, Exclude<R, Scope>>;
    <I, S, E, R>(tag: Tag$1<I, S>, effect: Effect<NoInfer$1<S>, E, R>): Layer<I, E, Exclude<R, Scope>>;
};
declare const scopedDiscard: <X, E, R>(effect: Effect<X, E, R>) => Layer<never, E, Exclude<R, Scope>>;
declare const scopedContext: <A, E, R>(effect: Effect<Context<A>, E, R>) => Layer<A, E, Exclude<R, Scope>>;
declare const service: <I, S>(tag: Tag$1<I, S>) => Layer<I, never, I>;
declare const succeed$1: {
    <I, S>(tag: Tag$1<I, S>): (resource: NoInfer$1<S>) => Layer<I>;
    <I, S>(tag: Tag$1<I, S>, resource: NoInfer$1<S>): Layer<I>;
};
declare const succeedContext: <A>(context: Context<A>) => Layer<A>;
declare const suspend$1: <RIn, E, ROut>(evaluate: LazyArg<Layer<ROut, E, RIn>>) => Layer<ROut, E, RIn>;
declare const sync$1: {
    <I, S>(tag: Tag$1<I, S>): (evaluate: LazyArg<NoInfer$1<S>>) => Layer<I>;
    <I, S>(tag: Tag$1<I, S>, evaluate: LazyArg<NoInfer$1<S>>): Layer<I>;
};
declare const syncContext: <A>(evaluate: LazyArg<Context<A>>) => Layer<A>;
declare const tap$1: {
    <ROut, XR extends ROut, RIn2, E2, X>(f: (context: Context<XR>) => Effect<X, E2, RIn2>): <RIn, E>(self: Layer<ROut, E, RIn>) => Layer<ROut, E2 | E, RIn2 | RIn>;
    <RIn, E, ROut, XR extends ROut, RIn2, E2, X>(self: Layer<ROut, E, RIn>, f: (context: Context<XR>) => Effect<X, E2, RIn2>): Layer<ROut, E | E2, RIn | RIn2>;
};
declare const tapError$1: {
    <E, XE extends E, RIn2, E2, X>(f: (e: XE) => Effect<X, E2, RIn2>): <RIn, ROut>(self: Layer<ROut, E, RIn>) => Layer<ROut, E | E2, RIn2 | RIn>;
    <RIn, E, XE extends E, ROut, RIn2, E2, X>(self: Layer<ROut, E, RIn>, f: (e: XE) => Effect<X, E2, RIn2>): Layer<ROut, E | E2, RIn | RIn2>;
};
declare const tapErrorCause$1: {
    <E, XE extends E, RIn2, E2, X>(f: (cause: Cause<XE>) => Effect<X, E2, RIn2>): <RIn, ROut>(self: Layer<ROut, E, RIn>) => Layer<ROut, E | E2, RIn2 | RIn>;
    <RIn, E, XE extends E, ROut, RIn2, E2, X>(self: Layer<ROut, E, RIn>, f: (cause: Cause<XE>) => Effect<X, E2, RIn2>): Layer<ROut, E | E2, RIn | RIn2>;
};
declare const toRuntime: <RIn, E, ROut>(self: Layer<ROut, E, RIn>) => Effect<Runtime<ROut>, E, Scope | RIn>;
declare const toRuntimeWithMemoMap: {
    (memoMap: MemoMap): <RIn, E, ROut>(self: Layer<ROut, E, RIn>) => Effect<Runtime<ROut>, E, Scope | RIn>;
    <RIn, E, ROut>(self: Layer<ROut, E, RIn>, memoMap: MemoMap): Effect<Runtime<ROut>, E, Scope | RIn>;
};
declare const provide$1: {
    <RIn, E, ROut>(that: Layer<ROut, E, RIn>): <RIn2, E2, ROut2>(self: Layer<ROut2, E2, RIn2>) => Layer<ROut2, E | E2, RIn | Exclude<RIn2, ROut>>;
    <const Layers extends readonly [
        Layer.Any,
        ...Array<Layer.Any>
    ]>(that: Layers): <A, E, R>(self: Layer<A, E, R>) => Layer<A, E | {
        [k in keyof Layers]: Layer.Error<Layers[k]>;
    }[number], {
        [k in keyof Layers]: Layer.Context<Layers[k]>;
    }[number] | Exclude<R, {
        [k in keyof Layers]: Layer.Success<Layers[k]>;
    }[number]>>;
    <RIn2, E2, ROut2, RIn, E, ROut>(self: Layer<ROut2, E2, RIn2>, that: Layer<ROut, E, RIn>): Layer<ROut2, E | E2, RIn | Exclude<RIn2, ROut>>;
    <A, E, R, const Layers extends readonly [
        Layer.Any,
        ...Array<Layer.Any>
    ]>(self: Layer<A, E, R>, that: Layers): Layer<A, E | {
        [k in keyof Layers]: Layer.Error<Layers[k]>;
    }[number], {
        [k in keyof Layers]: Layer.Context<Layers[k]>;
    }[number] | Exclude<R, {
        [k in keyof Layers]: Layer.Success<Layers[k]>;
    }[number]>>;
};
declare const provideMerge: {
    <RIn, E, ROut>(self: Layer<ROut, E, RIn>): <RIn2, E2, ROut2>(that: Layer<ROut2, E2, RIn2>) => Layer<ROut | ROut2, E | E2, RIn | Exclude<RIn2, ROut>>;
    <RIn2, E2, ROut2, RIn, E, ROut>(that: Layer<ROut2, E2, RIn2>, self: Layer<ROut, E, RIn>): Layer<ROut2 | ROut, E2 | E, RIn | Exclude<RIn2, ROut>>;
};
declare const zipWith$2: {
    <B, E2, R2, A, C>(that: Layer<B, E2, R2>, f: (a: Context<A>, b: Context<B>) => Context<C>): <E, R>(self: Layer<A, E, R>) => Layer<C, E2 | E, R2 | R>;
    <A, E, R, B, E2, R2, C>(self: Layer<A, E, R>, that: Layer<B, E2, R2>, f: (a: Context<A>, b: Context<B>) => Context<C>): Layer<C, E | E2, R | R2>;
};
declare const unwrapEffect: <A, E1, R1, E, R>(self: Effect<Layer<A, E1, R1>, E, R>) => Layer<A, E | E1, R | R1>;
declare const unwrapScoped: <A, E1, R1, E, R>(self: Effect<Layer<A, E1, R1>, E, R>) => Layer<A, E | E1, R1 | Exclude<R, Scope>>;
declare const setClock: <A extends Clock>(clock: A) => Layer<never>;
declare const setConfigProvider: (configProvider: ConfigProvider) => Layer<never>;
declare const parentSpan: (span: AnySpan) => Layer<ParentSpan>;
declare const setRandom: <A extends Random>(random: A) => Layer<never>;
declare const setRequestBatching: (requestBatching: boolean) => Layer<never>;
declare const setRequestCaching: (requestCaching: boolean) => Layer<never>;
declare const setRequestCache: {
    <E, R>(cache: Effect<Cache, E, R>): Layer<never, E, Exclude<R, Scope>>;
    (cache: Cache): Layer<never>;
};
declare const setScheduler: (scheduler: Scheduler) => Layer<never>;
declare const span: (name: string, options?: SpanOptions & {
    readonly onEnd?: ((span: Span, exit: Exit<unknown, unknown>) => Effect<void>) | undefined;
}) => Layer<ParentSpan>;
declare const setTracer: (tracer: Tracer) => Layer<never>;
declare const setTracerEnabled: (enabled: boolean) => Layer<never>;
declare const setTracerTiming: (enabled: boolean) => Layer<never>;
declare const setUnhandledErrorLogLevel: (level: Option<LogLevel>) => Layer<never>;
declare const setVersionMismatchErrorLogLevel: (level: Option<LogLevel>) => Layer<never>;
declare const withSpan$1: {
    (name: string, options?: SpanOptions & {
        readonly onEnd?: ((span: Span, exit: Exit<unknown, unknown>) => Effect<void>) | undefined;
    }): <A, E, R>(self: Layer<A, E, R>) => Layer<A, E, Exclude<R, ParentSpan>>;
    <A, E, R>(self: Layer<A, E, R>, name: string, options?: SpanOptions & {
        readonly onEnd?: ((span: Span, exit: Exit<unknown, unknown>) => Effect<void>) | undefined;
    }): Layer<A, E, Exclude<R, ParentSpan>>;
};
declare const withParentSpan$1: {
    (span: AnySpan): <A, E, R>(self: Layer<A, E, R>) => Layer<A, E, Exclude<R, ParentSpan>>;
    <A, E, R>(self: Layer<A, E, R>, span: AnySpan): Layer<A, E, Exclude<R, ParentSpan>>;
};
declare const makeMemoMap: Effect<MemoMap>;
declare const buildWithMemoMap: {
    (memoMap: MemoMap, scope: Scope): <RIn, E, ROut>(self: Layer<ROut, E, RIn>) => Effect<Context<ROut>, E, RIn>;
    <RIn, E, ROut>(self: Layer<ROut, E, RIn>, memoMap: MemoMap, scope: Scope): Effect<Context<ROut>, E, RIn>;
};
declare const updateService$1: (<I, A>(tag: Tag$1<I, A>, f: (a: A) => A) => <A1, E1, R1>(layer: Layer<A1, E1, R1>) => Layer<A1, E1, I | R1>) & (<A1, E1, R1, I, A>(layer: Layer<A1, E1, R1>, tag: Tag$1<I, A>, f: (a: A) => A) => Layer<A1, E1, I | R1>);
declare const ensureSuccessType$1: <ROut>() => <ROut2 extends ROut, E, RIn>(layer: Layer<ROut2, E, RIn>) => Layer<ROut2, E, RIn>;
declare const ensureErrorType$1: <E>() => <ROut, E2 extends E, RIn>(layer: Layer<ROut, E2, RIn>) => Layer<ROut, E2, RIn>;
declare const ensureRequirementsType$1: <RIn>() => <ROut, E, RIn2 extends RIn>(layer: Layer<ROut, E, RIn2>) => Layer<ROut, E, RIn2>;
declare const Layer_d_CurrentMemoMap: typeof CurrentMemoMap;
import Layer_d_Layer = Layer;
type Layer_d_LayerTypeId = LayerTypeId;
type Layer_d_MemoMap = MemoMap;
type Layer_d_MemoMapTypeId = MemoMapTypeId;
type Layer_d_PartialEffectful<A extends object> = PartialEffectful<A>;
declare const Layer_d_build: typeof build;
declare const Layer_d_buildWithMemoMap: typeof buildWithMemoMap;
declare const Layer_d_buildWithScope: typeof buildWithScope;
declare const Layer_d_discard: typeof discard;
declare const Layer_d_effect: typeof effect;
declare const Layer_d_effectContext: typeof effectContext;
declare const Layer_d_effectDiscard: typeof effectDiscard;
declare const Layer_d_extendScope: typeof extendScope;
declare const Layer_d_fiberRefLocallyScopedWith: typeof fiberRefLocallyScopedWith;
declare const Layer_d_fresh: typeof fresh;
declare const Layer_d_isFresh: typeof isFresh;
declare const Layer_d_isLayer: typeof isLayer;
declare const Layer_d_launch: typeof launch;
declare const Layer_d_locallyEffect: typeof locallyEffect;
declare const Layer_d_makeMemoMap: typeof makeMemoMap;
declare const Layer_d_memoize: typeof memoize;
declare const Layer_d_mock: typeof mock;
declare const Layer_d_parentSpan: typeof parentSpan;
declare const Layer_d_passthrough: typeof passthrough;
declare const Layer_d_project: typeof project;
declare const Layer_d_provideMerge: typeof provideMerge;
declare const Layer_d_scopedContext: typeof scopedContext;
declare const Layer_d_scopedDiscard: typeof scopedDiscard;
declare const Layer_d_service: typeof service;
declare const Layer_d_setClock: typeof setClock;
declare const Layer_d_setConfigProvider: typeof setConfigProvider;
declare const Layer_d_setRandom: typeof setRandom;
declare const Layer_d_setRequestBatching: typeof setRequestBatching;
declare const Layer_d_setRequestCache: typeof setRequestCache;
declare const Layer_d_setRequestCaching: typeof setRequestCaching;
declare const Layer_d_setScheduler: typeof setScheduler;
declare const Layer_d_setTracer: typeof setTracer;
declare const Layer_d_setTracerEnabled: typeof setTracerEnabled;
declare const Layer_d_setTracerTiming: typeof setTracerTiming;
declare const Layer_d_setUnhandledErrorLogLevel: typeof setUnhandledErrorLogLevel;
declare const Layer_d_setVersionMismatchErrorLogLevel: typeof setVersionMismatchErrorLogLevel;
declare const Layer_d_span: typeof span;
declare const Layer_d_succeedContext: typeof succeedContext;
declare const Layer_d_syncContext: typeof syncContext;
declare const Layer_d_toRuntime: typeof toRuntime;
declare const Layer_d_toRuntimeWithMemoMap: typeof toRuntimeWithMemoMap;
declare const Layer_d_unwrapEffect: typeof unwrapEffect;
declare const Layer_d_unwrapScoped: typeof unwrapScoped;
declare namespace Layer_d {
    export { Layer_d_CurrentMemoMap as CurrentMemoMap, Layer_d_Layer as Layer, annotateLogs$1 as annotateLogs, annotateSpans$1 as annotateSpans, Layer_d_build as build, Layer_d_buildWithMemoMap as buildWithMemoMap, Layer_d_buildWithScope as buildWithScope, catchAll$1 as catchAll, catchAllCause$1 as catchAllCause, context$1 as context, die$2 as die, dieSync$1 as dieSync, Layer_d_discard as discard, Layer_d_effect as effect, Layer_d_effectContext as effectContext, Layer_d_effectDiscard as effectDiscard, empty$1 as empty, ensureErrorType$1 as ensureErrorType, ensureRequirementsType$1 as ensureRequirementsType, ensureSuccessType$1 as ensureSuccessType, Layer_d_extendScope as extendScope, fail$2 as fail, failCause$1 as failCause, failCauseSync$1 as failCauseSync, failSync$1 as failSync, Layer_d_fiberRefLocallyScopedWith as fiberRefLocallyScopedWith, flatMap$3 as flatMap, flatten$2 as flatten, Layer_d_fresh as fresh, fromFunction as function, Layer_d_isFresh as isFresh, Layer_d_isLayer as isLayer, Layer_d_launch as launch, locally$1 as locally, Layer_d_locallyEffect as locallyEffect, locallyScoped$1 as locallyScoped, locallyWith$1 as locallyWith, Layer_d_makeMemoMap as makeMemoMap, map$3 as map, mapError$1 as mapError, match$3 as match, matchCause$1 as matchCause, Layer_d_memoize as memoize, merge$2 as merge, mergeAll$1 as mergeAll, Layer_d_mock as mock, orDie$1 as orDie, orElse$3 as orElse, Layer_d_parentSpan as parentSpan, Layer_d_passthrough as passthrough, Layer_d_project as project, provide$1 as provide, Layer_d_provideMerge as provideMerge, retry$1 as retry, scope$1 as scope, scoped$1 as scoped, Layer_d_scopedContext as scopedContext, Layer_d_scopedDiscard as scopedDiscard, Layer_d_service as service, Layer_d_setClock as setClock, Layer_d_setConfigProvider as setConfigProvider, Layer_d_setRandom as setRandom, Layer_d_setRequestBatching as setRequestBatching, Layer_d_setRequestCache as setRequestCache, Layer_d_setRequestCaching as setRequestCaching, Layer_d_setScheduler as setScheduler, Layer_d_setTracer as setTracer, Layer_d_setTracerEnabled as setTracerEnabled, Layer_d_setTracerTiming as setTracerTiming, Layer_d_setUnhandledErrorLogLevel as setUnhandledErrorLogLevel, Layer_d_setVersionMismatchErrorLogLevel as setVersionMismatchErrorLogLevel, Layer_d_span as span, succeed$1 as succeed, Layer_d_succeedContext as succeedContext, suspend$1 as suspend, sync$1 as sync, Layer_d_syncContext as syncContext, tap$1 as tap, tapError$1 as tapError, tapErrorCause$1 as tapErrorCause, Layer_d_toRuntime as toRuntime, Layer_d_toRuntimeWithMemoMap as toRuntimeWithMemoMap, Layer_d_unwrapEffect as unwrapEffect, Layer_d_unwrapScoped as unwrapScoped, updateService$1 as updateService, withParentSpan$1 as withParentSpan, withSpan$1 as withSpan, zipWith$2 as zipWith };
    export type { Layer_d_LayerTypeId as LayerTypeId, Layer_d_MemoMap as MemoMap, Layer_d_MemoMapTypeId as MemoMapTypeId, Layer_d_PartialEffectful as PartialEffectful };
}
declare const ChannelTypeId: unique symbol;
type ChannelTypeId = typeof ChannelTypeId;
interface ChannelUnify<A extends {
    [typeSymbol]?: any;
}> extends EffectUnify<A> {
    Channel?: () => A[typeSymbol] extends Channel<infer OutElem, infer InElem, infer OutErr, infer InErr, infer OutDone, infer InDone, infer Env> | infer _ ? Channel<OutElem, InElem, OutErr, InErr, OutDone, InDone, Env> : never;
}
interface ChannelUnifyIgnore extends EffectUnifyIgnore {
    Channel?: true;
}
interface Channel<out OutElem, in InElem = unknown, out OutErr = never, in InErr = unknown, out OutDone = void, in InDone = unknown, out Env = never> extends Channel.Variance<OutElem, InElem, OutErr, InErr, OutDone, InDone, Env>, Pipeable {
    [typeSymbol]?: unknown;
    [unifySymbol]?: ChannelUnify<this>;
    [ignoreSymbol]?: ChannelUnifyIgnore;
}
declare namespace Channel {
    interface Variance<out OutElem, in InElem, out OutErr, in InErr, out OutDone, in InDone, out Env> {
        readonly [ChannelTypeId]: VarianceStruct<OutElem, InElem, OutErr, InErr, OutDone, InDone, Env>;
    }
    interface VarianceStruct<out OutElem, in InElem, out OutErr, in InErr, out OutDone, in InDone, out Env> {
        _Env: Covariant<Env>;
        _InErr: Contravariant<InErr>;
        _InElem: Contravariant<InElem>;
        _InDone: Contravariant<InDone>;
        _OutErr: Covariant<OutErr>;
        _OutElem: Covariant<OutElem>;
        _OutDone: Covariant<OutDone>;
    }
}
declare const CauseTypeId: unique symbol;
type CauseTypeId = typeof CauseTypeId;
declare const RuntimeExceptionTypeId: unique symbol;
type RuntimeExceptionTypeId = typeof RuntimeExceptionTypeId;
declare const InterruptedExceptionTypeId: unique symbol;
type InterruptedExceptionTypeId = typeof InterruptedExceptionTypeId;
declare const IllegalArgumentExceptionTypeId: unique symbol;
type IllegalArgumentExceptionTypeId = typeof IllegalArgumentExceptionTypeId;
declare const NoSuchElementExceptionTypeId: unique symbol;
type NoSuchElementExceptionTypeId = typeof NoSuchElementExceptionTypeId;
declare const InvalidPubSubCapacityExceptionTypeId: unique symbol;
type InvalidPubSubCapacityExceptionTypeId = typeof InvalidPubSubCapacityExceptionTypeId;
declare const ExceededCapacityExceptionTypeId: unique symbol;
type ExceededCapacityExceptionTypeId = typeof ExceededCapacityExceptionTypeId;
declare const TimeoutExceptionTypeId: unique symbol;
type TimeoutExceptionTypeId = typeof TimeoutExceptionTypeId;
declare const UnknownExceptionTypeId: unique symbol;
type UnknownExceptionTypeId = typeof UnknownExceptionTypeId;
type Cause<E> = Empty$2 | Fail$1<E> | Die | Interrupt | Sequential<E> | Parallel<E>;
declare namespace Cause {
    interface Variance<out E> {
        readonly [CauseTypeId]: {
            readonly _E: Covariant<E>;
        };
    }
}
interface CauseReducer<in C, in E, in out Z> {
    emptyCase(context: C): Z;
    failCase(context: C, error: E): Z;
    dieCase(context: C, defect: unknown): Z;
    interruptCase(context: C, fiberId: FiberId): Z;
    sequentialCase(context: C, left: Z, right: Z): Z;
    parallelCase(context: C, left: Z, right: Z): Z;
}
interface YieldableError extends Pipeable, Inspectable, Error {
    readonly [EffectTypeId]: Effect.VarianceStruct<never, this, never>;
    readonly [StreamTypeId]: Stream.VarianceStruct<never, this, never>;
    readonly [SinkTypeId]: Sink.VarianceStruct<never, unknown, never, this, never>;
    readonly [ChannelTypeId]: Channel.VarianceStruct<never, unknown, this, unknown, never, unknown, never>;
    [Symbol.iterator](): EffectGenerator<Effect<never, this, never>>;
}
declare const YieldableError: new (message?: string | undefined) => YieldableError;
interface InvalidPubSubCapacityException extends YieldableError {
    readonly _tag: "InvalidPubSubCapacityException";
    readonly [InvalidPubSubCapacityExceptionTypeId]: InvalidPubSubCapacityExceptionTypeId;
}
interface Empty$2 extends Cause.Variance<never>, Equal, Pipeable, Inspectable {
    readonly _tag: "Empty";
}
interface Fail$1<out E> extends Cause.Variance<E>, Equal, Pipeable, Inspectable {
    readonly _tag: "Fail";
    readonly error: E;
}
interface Die extends Cause.Variance<never>, Equal, Pipeable, Inspectable {
    readonly _tag: "Die";
    readonly defect: unknown;
}
interface Interrupt extends Cause.Variance<never>, Equal, Pipeable, Inspectable {
    readonly _tag: "Interrupt";
    readonly fiberId: FiberId;
}
interface Parallel<out E> extends Cause.Variance<E>, Equal, Pipeable, Inspectable {
    readonly _tag: "Parallel";
    readonly left: Cause<E>;
    readonly right: Cause<E>;
}
interface Sequential<out E> extends Cause.Variance<E>, Equal, Pipeable, Inspectable {
    readonly _tag: "Sequential";
    readonly left: Cause<E>;
    readonly right: Cause<E>;
}
declare const empty: Cause<never>;
declare const fail$1: <E>(error: E) => Cause<E>;
declare const die$1: (defect: unknown) => Cause<never>;
declare const interrupt$1: (fiberId: FiberId) => Cause<never>;
declare const parallel: <E, E2>(left: Cause<E>, right: Cause<E2>) => Cause<E | E2>;
declare const sequential: <E, E2>(left: Cause<E>, right: Cause<E2>) => Cause<E | E2>;
declare const isCause: (u: unknown) => u is Cause<unknown>;
declare const isEmptyType: <E>(self: Cause<E>) => self is Empty$2;
declare const isFailType: <E>(self: Cause<E>) => self is Fail$1<E>;
declare const isDieType: <E>(self: Cause<E>) => self is Die;
declare const isInterruptType: <E>(self: Cause<E>) => self is Interrupt;
declare const isSequentialType: <E>(self: Cause<E>) => self is Sequential<E>;
declare const isParallelType: <E>(self: Cause<E>) => self is Parallel<E>;
declare const size: <E>(self: Cause<E>) => number;
declare const isEmpty: <E>(self: Cause<E>) => boolean;
declare const isFailure$1: <E>(self: Cause<E>) => boolean;
declare const isDie: <E>(self: Cause<E>) => boolean;
declare const isInterrupted: <E>(self: Cause<E>) => boolean;
declare const isInterruptedOnly: <E>(self: Cause<E>) => boolean;
declare const failures: <E>(self: Cause<E>) => Chunk<E>;
declare const defects: <E>(self: Cause<E>) => Chunk<unknown>;
declare const interruptors: <E>(self: Cause<E>) => HashSet<FiberId>;
declare const failureOption: <E>(self: Cause<E>) => Option<E>;
declare const failureOrCause: <E>(self: Cause<E>) => Either<Cause<never>, E>;
declare const flipCauseOption: <E>(self: Cause<Option<E>>) => Option<Cause<E>>;
declare const dieOption: <E>(self: Cause<E>) => Option<unknown>;
declare const interruptOption: <E>(self: Cause<E>) => Option<FiberId>;
declare const keepDefects: <E>(self: Cause<E>) => Option<Cause<never>>;
declare const linearize: <E>(self: Cause<E>) => HashSet<Cause<E>>;
declare const stripFailures: <E>(self: Cause<E>) => Cause<never>;
declare const stripSomeDefects: {
    (pf: (defect: unknown) => Option<unknown>): <E>(self: Cause<E>) => Option<Cause<E>>;
    <E>(self: Cause<E>, pf: (defect: unknown) => Option<unknown>): Option<Cause<E>>;
};
declare const as$1: {
    <E2>(error: E2): <E>(self: Cause<E>) => Cause<E2>;
    <E, E2>(self: Cause<E>, error: E2): Cause<E2>;
};
declare const map$2: {
    <E, E2>(f: (e: E) => E2): (self: Cause<E>) => Cause<E2>;
    <E, E2>(self: Cause<E>, f: (e: E) => E2): Cause<E2>;
};
declare const flatMap$2: {
    <E, E2>(f: (e: E) => Cause<E2>): (self: Cause<E>) => Cause<E2>;
    <E, E2>(self: Cause<E>, f: (e: E) => Cause<E2>): Cause<E2>;
};
declare const andThen$2: {
    <E, E2>(f: (e: E) => Cause<E2>): (self: Cause<E>) => Cause<E2>;
    <E2>(f: Cause<E2>): <E>(self: Cause<E>) => Cause<E2>;
    <E, E2>(self: Cause<E>, f: (e: E) => Cause<E2>): Cause<E2>;
    <E, E2>(self: Cause<E>, f: Cause<E2>): Cause<E2>;
};
declare const flatten$1: <E>(self: Cause<Cause<E>>) => Cause<E>;
declare const contains: {
    <E2>(that: Cause<E2>): <E>(self: Cause<E>) => boolean;
    <E, E2>(self: Cause<E>, that: Cause<E2>): boolean;
};
declare const squash: <E>(self: Cause<E>) => unknown;
declare const squashWith: {
    <E>(f: (error: E) => unknown): (self: Cause<E>) => unknown;
    <E>(self: Cause<E>, f: (error: E) => unknown): unknown;
};
declare const find: {
    <E, Z>(pf: (cause: Cause<E>) => Option<Z>): (self: Cause<E>) => Option<Z>;
    <E, Z>(self: Cause<E>, pf: (cause: Cause<E>) => Option<Z>): Option<Z>;
};
declare const filter$1: {
    <E, EB extends E>(refinement: Refinement<Cause<NoInfer$1<E>>, Cause<EB>>): (self: Cause<E>) => Cause<EB>;
    <E>(predicate: Predicate<Cause<NoInfer$1<E>>>): (self: Cause<E>) => Cause<E>;
    <E, EB extends E>(self: Cause<E>, refinement: Refinement<Cause<E>, Cause<EB>>): Cause<EB>;
    <E>(self: Cause<E>, predicate: Predicate<Cause<E>>): Cause<E>;
};
declare const match$2: {
    <Z, E>(options: {
        readonly onEmpty: Z;
        readonly onFail: (error: E) => Z;
        readonly onDie: (defect: unknown) => Z;
        readonly onInterrupt: (fiberId: FiberId) => Z;
        readonly onSequential: (left: Z, right: Z) => Z;
        readonly onParallel: (left: Z, right: Z) => Z;
    }): (self: Cause<E>) => Z;
    <Z, E>(self: Cause<E>, options: {
        readonly onEmpty: Z;
        readonly onFail: (error: E) => Z;
        readonly onDie: (defect: unknown) => Z;
        readonly onInterrupt: (fiberId: FiberId) => Z;
        readonly onSequential: (left: Z, right: Z) => Z;
        readonly onParallel: (left: Z, right: Z) => Z;
    }): Z;
};
declare const reduce$1: {
    <Z, E>(zero: Z, pf: (accumulator: Z, cause: Cause<E>) => Option<Z>): (self: Cause<E>) => Z;
    <Z, E>(self: Cause<E>, zero: Z, pf: (accumulator: Z, cause: Cause<E>) => Option<Z>): Z;
};
declare const reduceWithContext: {
    <C, E, Z>(context: C, reducer: CauseReducer<C, E, Z>): (self: Cause<E>) => Z;
    <C, E, Z>(self: Cause<E>, context: C, reducer: CauseReducer<C, E, Z>): Z;
};
interface InterruptedException extends YieldableError {
    readonly _tag: "InterruptedException";
    readonly [InterruptedExceptionTypeId]: InterruptedExceptionTypeId;
}
declare const InterruptedException: new (message?: string | undefined) => InterruptedException;
declare const isInterruptedException: (u: unknown) => u is InterruptedException;
interface IllegalArgumentException extends YieldableError {
    readonly _tag: "IllegalArgumentException";
    readonly [IllegalArgumentExceptionTypeId]: IllegalArgumentExceptionTypeId;
}
declare const IllegalArgumentException: new (message?: string | undefined) => IllegalArgumentException;
declare const isIllegalArgumentException: (u: unknown) => u is IllegalArgumentException;
interface NoSuchElementException extends YieldableError {
    readonly _tag: "NoSuchElementException";
    readonly [NoSuchElementExceptionTypeId]: NoSuchElementExceptionTypeId;
}
declare const NoSuchElementException: new (message?: string | undefined) => NoSuchElementException;
declare const isNoSuchElementException: (u: unknown) => u is NoSuchElementException;
interface RuntimeException extends YieldableError {
    readonly _tag: "RuntimeException";
    readonly [RuntimeExceptionTypeId]: RuntimeExceptionTypeId;
}
declare const RuntimeException: new (message?: string | undefined) => RuntimeException;
declare const isRuntimeException: (u: unknown) => u is RuntimeException;
interface TimeoutException extends YieldableError {
    readonly _tag: "TimeoutException";
    readonly [TimeoutExceptionTypeId]: TimeoutExceptionTypeId;
}
declare const TimeoutException: new (message?: string | undefined) => TimeoutException;
declare const isTimeoutException: (u: unknown) => u is TimeoutException;
interface UnknownException extends YieldableError {
    readonly _tag: "UnknownException";
    readonly [UnknownExceptionTypeId]: UnknownExceptionTypeId;
    readonly error: unknown;
}
declare const UnknownException: new (error: unknown, message?: string | undefined) => UnknownException;
declare const isUnknownException: (u: unknown) => u is UnknownException;
interface ExceededCapacityException extends YieldableError {
    readonly _tag: "ExceededCapacityException";
    readonly [ExceededCapacityExceptionTypeId]: ExceededCapacityExceptionTypeId;
}
declare const ExceededCapacityException: new (message?: string | undefined) => ExceededCapacityException;
declare const isExceededCapacityException: (u: unknown) => u is ExceededCapacityException;
declare const pretty: <E>(cause: Cause<E>, options?: {
    readonly renderErrorCause?: boolean | undefined;
}) => string;
interface PrettyError extends Error {
    readonly span: Span | undefined;
}
declare const prettyErrors: <E>(cause: Cause<E>) => Array<PrettyError>;
declare const originalError: <E>(obj: E) => E;
import Cause_d_Cause = Cause;
type Cause_d_CauseReducer<in C, in E, in out Z> = CauseReducer<C, E, Z>;
type Cause_d_CauseTypeId = CauseTypeId;
type Cause_d_Die = Die;
declare const Cause_d_ExceededCapacityException: typeof ExceededCapacityException;
type Cause_d_ExceededCapacityExceptionTypeId = ExceededCapacityExceptionTypeId;
declare const Cause_d_IllegalArgumentException: typeof IllegalArgumentException;
type Cause_d_IllegalArgumentExceptionTypeId = IllegalArgumentExceptionTypeId;
type Cause_d_Interrupt = Interrupt;
declare const Cause_d_InterruptedException: typeof InterruptedException;
type Cause_d_InterruptedExceptionTypeId = InterruptedExceptionTypeId;
type Cause_d_InvalidPubSubCapacityException = InvalidPubSubCapacityException;
type Cause_d_InvalidPubSubCapacityExceptionTypeId = InvalidPubSubCapacityExceptionTypeId;
declare const Cause_d_NoSuchElementException: typeof NoSuchElementException;
type Cause_d_NoSuchElementExceptionTypeId = NoSuchElementExceptionTypeId;
type Cause_d_Parallel<out E> = Parallel<E>;
type Cause_d_PrettyError = PrettyError;
declare const Cause_d_RuntimeException: typeof RuntimeException;
type Cause_d_RuntimeExceptionTypeId = RuntimeExceptionTypeId;
type Cause_d_Sequential<out E> = Sequential<E>;
declare const Cause_d_TimeoutException: typeof TimeoutException;
type Cause_d_TimeoutExceptionTypeId = TimeoutExceptionTypeId;
declare const Cause_d_UnknownException: typeof UnknownException;
type Cause_d_UnknownExceptionTypeId = UnknownExceptionTypeId;
declare const Cause_d_YieldableError: typeof YieldableError;
declare const Cause_d_contains: typeof contains;
declare const Cause_d_defects: typeof defects;
declare const Cause_d_dieOption: typeof dieOption;
declare const Cause_d_empty: typeof empty;
declare const Cause_d_failureOption: typeof failureOption;
declare const Cause_d_failureOrCause: typeof failureOrCause;
declare const Cause_d_failures: typeof failures;
declare const Cause_d_find: typeof find;
declare const Cause_d_flipCauseOption: typeof flipCauseOption;
declare const Cause_d_interruptOption: typeof interruptOption;
declare const Cause_d_interruptors: typeof interruptors;
declare const Cause_d_isCause: typeof isCause;
declare const Cause_d_isDie: typeof isDie;
declare const Cause_d_isDieType: typeof isDieType;
declare const Cause_d_isEmpty: typeof isEmpty;
declare const Cause_d_isEmptyType: typeof isEmptyType;
declare const Cause_d_isExceededCapacityException: typeof isExceededCapacityException;
declare const Cause_d_isFailType: typeof isFailType;
declare const Cause_d_isIllegalArgumentException: typeof isIllegalArgumentException;
declare const Cause_d_isInterruptType: typeof isInterruptType;
declare const Cause_d_isInterrupted: typeof isInterrupted;
declare const Cause_d_isInterruptedException: typeof isInterruptedException;
declare const Cause_d_isInterruptedOnly: typeof isInterruptedOnly;
declare const Cause_d_isNoSuchElementException: typeof isNoSuchElementException;
declare const Cause_d_isParallelType: typeof isParallelType;
declare const Cause_d_isRuntimeException: typeof isRuntimeException;
declare const Cause_d_isSequentialType: typeof isSequentialType;
declare const Cause_d_isTimeoutException: typeof isTimeoutException;
declare const Cause_d_isUnknownException: typeof isUnknownException;
declare const Cause_d_keepDefects: typeof keepDefects;
declare const Cause_d_linearize: typeof linearize;
declare const Cause_d_originalError: typeof originalError;
declare const Cause_d_parallel: typeof parallel;
declare const Cause_d_pretty: typeof pretty;
declare const Cause_d_prettyErrors: typeof prettyErrors;
declare const Cause_d_reduceWithContext: typeof reduceWithContext;
declare const Cause_d_sequential: typeof sequential;
declare const Cause_d_size: typeof size;
declare const Cause_d_squash: typeof squash;
declare const Cause_d_squashWith: typeof squashWith;
declare const Cause_d_stripFailures: typeof stripFailures;
declare const Cause_d_stripSomeDefects: typeof stripSomeDefects;
declare namespace Cause_d {
    export { Cause_d_Cause as Cause, Cause_d_ExceededCapacityException as ExceededCapacityException, Cause_d_IllegalArgumentException as IllegalArgumentException, Cause_d_InterruptedException as InterruptedException, Cause_d_NoSuchElementException as NoSuchElementException, Cause_d_RuntimeException as RuntimeException, Cause_d_TimeoutException as TimeoutException, Cause_d_UnknownException as UnknownException, Cause_d_YieldableError as YieldableError, andThen$2 as andThen, as$1 as as, Cause_d_contains as contains, Cause_d_defects as defects, die$1 as die, Cause_d_dieOption as dieOption, Cause_d_empty as empty, fail$1 as fail, Cause_d_failureOption as failureOption, Cause_d_failureOrCause as failureOrCause, Cause_d_failures as failures, filter$1 as filter, Cause_d_find as find, flatMap$2 as flatMap, flatten$1 as flatten, Cause_d_flipCauseOption as flipCauseOption, interrupt$1 as interrupt, Cause_d_interruptOption as interruptOption, Cause_d_interruptors as interruptors, Cause_d_isCause as isCause, Cause_d_isDie as isDie, Cause_d_isDieType as isDieType, Cause_d_isEmpty as isEmpty, Cause_d_isEmptyType as isEmptyType, Cause_d_isExceededCapacityException as isExceededCapacityException, Cause_d_isFailType as isFailType, isFailure$1 as isFailure, Cause_d_isIllegalArgumentException as isIllegalArgumentException, Cause_d_isInterruptType as isInterruptType, Cause_d_isInterrupted as isInterrupted, Cause_d_isInterruptedException as isInterruptedException, Cause_d_isInterruptedOnly as isInterruptedOnly, Cause_d_isNoSuchElementException as isNoSuchElementException, Cause_d_isParallelType as isParallelType, Cause_d_isRuntimeException as isRuntimeException, Cause_d_isSequentialType as isSequentialType, Cause_d_isTimeoutException as isTimeoutException, Cause_d_isUnknownException as isUnknownException, Cause_d_keepDefects as keepDefects, Cause_d_linearize as linearize, map$2 as map, match$2 as match, Cause_d_originalError as originalError, Cause_d_parallel as parallel, Cause_d_pretty as pretty, Cause_d_prettyErrors as prettyErrors, reduce$1 as reduce, Cause_d_reduceWithContext as reduceWithContext, Cause_d_sequential as sequential, Cause_d_size as size, Cause_d_squash as squash, Cause_d_squashWith as squashWith, Cause_d_stripFailures as stripFailures, Cause_d_stripSomeDefects as stripSomeDefects };
    export type { Cause_d_CauseReducer as CauseReducer, Cause_d_CauseTypeId as CauseTypeId, Cause_d_Die as Die, Empty$2 as Empty, Cause_d_ExceededCapacityExceptionTypeId as ExceededCapacityExceptionTypeId, Fail$1 as Fail, Cause_d_IllegalArgumentExceptionTypeId as IllegalArgumentExceptionTypeId, Cause_d_Interrupt as Interrupt, Cause_d_InterruptedExceptionTypeId as InterruptedExceptionTypeId, Cause_d_InvalidPubSubCapacityException as InvalidPubSubCapacityException, Cause_d_InvalidPubSubCapacityExceptionTypeId as InvalidPubSubCapacityExceptionTypeId, Cause_d_NoSuchElementExceptionTypeId as NoSuchElementExceptionTypeId, Cause_d_Parallel as Parallel, Cause_d_PrettyError as PrettyError, Cause_d_RuntimeExceptionTypeId as RuntimeExceptionTypeId, Cause_d_Sequential as Sequential, Cause_d_TimeoutExceptionTypeId as TimeoutExceptionTypeId, Cause_d_UnknownExceptionTypeId as UnknownExceptionTypeId };
}
type FiberRefsPatch = Empty$1 | Add | Remove | Update | AndThen;
interface Empty$1 {
    readonly _tag: "Empty";
}
interface Add {
    readonly _tag: "Add";
    readonly fiberRef: FiberRef<unknown>;
    readonly value: unknown;
}
interface Remove {
    readonly _tag: "Remove";
    readonly fiberRef: FiberRef<unknown>;
}
interface Update {
    readonly _tag: "Update";
    readonly fiberRef: FiberRef<unknown>;
    readonly patch: unknown;
}
interface AndThen {
    readonly _tag: "AndThen";
    readonly first: FiberRefsPatch;
    readonly second: FiberRefsPatch;
}
declare const TypeId$1: unique symbol;
type TypeId$1 = typeof TypeId$1;
declare namespace ManagedRuntime {
    type Context<T extends ManagedRuntime<never, any>> = [
        T
    ] extends [
        ManagedRuntime<infer R, infer _E>
    ] ? R : never;
    type Error<T extends ManagedRuntime<never, any>> = [
        T
    ] extends [
        ManagedRuntime<infer _R, infer E>
    ] ? E : never;
}
interface ManagedRuntime<in R, out ER> extends Effect<Runtime<R>, ER> {
    readonly [TypeId$1]: TypeId$1;
    readonly memoMap: MemoMap;
    readonly runtimeEffect: Effect<Runtime<R>, ER>;
    readonly runtime: () => Promise<Runtime<R>>;
    readonly runFork: <A, E>(self: Effect<A, E, R>, options?: RunForkOptions) => RuntimeFiber<A, E | ER>;
    readonly runSyncExit: <A, E>(effect: Effect<A, E, R>) => Exit<A, ER | E>;
    readonly runSync: <A, E>(effect: Effect<A, E, R>) => A;
    readonly runCallback: <A, E>(effect: Effect<A, E, R>, options?: RunCallbackOptions<A, E | ER> | undefined) => Cancel<A, E | ER>;
    readonly runPromise: <A, E>(effect: Effect<A, E, R>, options?: {
        readonly signal?: AbortSignal | undefined;
    }) => Promise<A>;
    readonly runPromiseExit: <A, E>(effect: Effect<A, E, R>, options?: {
        readonly signal?: AbortSignal | undefined;
    }) => Promise<Exit<A, ER | E>>;
    readonly dispose: () => Promise<void>;
    readonly disposeEffect: Effect<void, never, never>;
    readonly [typeSymbol]?: unknown;
    readonly [unifySymbol]?: ManagedRuntimeUnify<this>;
    readonly [ignoreSymbol]?: ManagedRuntimeUnifyIgnore;
}
interface ManagedRuntimeUnify<A extends {
    [typeSymbol]?: any;
}> extends EffectUnify<A> {
    ManagedRuntime?: () => Extract<A[typeSymbol], ManagedRuntime<any, any>>;
}
interface ManagedRuntimeUnifyIgnore extends EffectUnifyIgnore {
    Effect?: true;
}
declare const MetricBoundariesTypeId: unique symbol;
type MetricBoundariesTypeId = typeof MetricBoundariesTypeId;
interface MetricBoundaries extends Equal, Pipeable {
    readonly [MetricBoundariesTypeId]: MetricBoundariesTypeId;
    readonly values: ReadonlyArray<number>;
}
declare const MetricStateTypeId: unique symbol;
type MetricStateTypeId = typeof MetricStateTypeId;
declare const CounterStateTypeId: unique symbol;
type CounterStateTypeId = typeof CounterStateTypeId;
declare const FrequencyStateTypeId: unique symbol;
type FrequencyStateTypeId = typeof FrequencyStateTypeId;
declare const GaugeStateTypeId: unique symbol;
type GaugeStateTypeId = typeof GaugeStateTypeId;
declare const HistogramStateTypeId: unique symbol;
type HistogramStateTypeId = typeof HistogramStateTypeId;
declare const SummaryStateTypeId: unique symbol;
type SummaryStateTypeId = typeof SummaryStateTypeId;
interface MetricState<in A> extends MetricState.Variance<A>, Equal, Pipeable {
}
declare namespace MetricState {
    interface Untyped extends MetricState<any> {
    }
    interface Counter<in out A extends (number | bigint)> extends MetricState<MetricKeyType.Counter<A>> {
        readonly [CounterStateTypeId]: CounterStateTypeId;
        readonly count: A;
    }
    interface Frequency extends MetricState<MetricKeyType.Frequency> {
        readonly [FrequencyStateTypeId]: FrequencyStateTypeId;
        readonly occurrences: ReadonlyMap<string, number>;
    }
    interface Gauge<in out A extends (number | bigint)> extends MetricState<MetricKeyType.Gauge<A>> {
        readonly [GaugeStateTypeId]: GaugeStateTypeId;
        readonly value: A;
    }
    interface Histogram extends MetricState<MetricKeyType.Histogram> {
        readonly [HistogramStateTypeId]: HistogramStateTypeId;
        readonly buckets: ReadonlyArray<readonly [
            number,
            number
        ]>;
        readonly count: number;
        readonly min: number;
        readonly max: number;
        readonly sum: number;
    }
    interface Summary extends MetricState<MetricKeyType.Summary> {
        readonly [SummaryStateTypeId]: SummaryStateTypeId;
        readonly error: number;
        readonly quantiles: ReadonlyArray<readonly [
            number,
            Option<number>
        ]>;
        readonly count: number;
        readonly min: number;
        readonly max: number;
        readonly sum: number;
    }
    interface Variance<in A> {
        readonly [MetricStateTypeId]: {
            readonly _A: Contravariant<A>;
        };
    }
}
declare const MetricKeyTypeTypeId: unique symbol;
type MetricKeyTypeTypeId = typeof MetricKeyTypeTypeId;
declare const CounterKeyTypeTypeId: unique symbol;
type CounterKeyTypeTypeId = typeof CounterKeyTypeTypeId;
declare const FrequencyKeyTypeTypeId: unique symbol;
type FrequencyKeyTypeTypeId = typeof FrequencyKeyTypeTypeId;
declare const GaugeKeyTypeTypeId: unique symbol;
type GaugeKeyTypeTypeId = typeof GaugeKeyTypeTypeId;
declare const HistogramKeyTypeTypeId: unique symbol;
type HistogramKeyTypeTypeId = typeof HistogramKeyTypeTypeId;
declare const SummaryKeyTypeTypeId: unique symbol;
type SummaryKeyTypeTypeId = typeof SummaryKeyTypeTypeId;
interface MetricKeyType<in In, out Out> extends MetricKeyType.Variance<In, Out>, Equal, Pipeable {
}
declare namespace MetricKeyType {
    type Untyped = MetricKeyType<any, any>;
    type Counter<A extends (number | bigint)> = MetricKeyType<A, MetricState.Counter<A>> & {
        readonly [CounterKeyTypeTypeId]: CounterKeyTypeTypeId;
        readonly incremental: boolean;
        readonly bigint: boolean;
    };
    type Frequency = MetricKeyType<string, MetricState.Frequency> & {
        readonly [FrequencyKeyTypeTypeId]: FrequencyKeyTypeTypeId;
        readonly preregisteredWords: ReadonlyArray<string>;
    };
    type Gauge<A extends (number | bigint)> = MetricKeyType<A, MetricState.Gauge<A>> & {
        readonly [GaugeKeyTypeTypeId]: GaugeKeyTypeTypeId;
        readonly bigint: boolean;
    };
    type Histogram = MetricKeyType<number, MetricState.Histogram> & {
        readonly [HistogramKeyTypeTypeId]: HistogramKeyTypeTypeId;
        readonly boundaries: MetricBoundaries;
    };
    type Summary = MetricKeyType<readonly [
        number,
        number
    ], MetricState.Summary> & {
        readonly [SummaryKeyTypeTypeId]: SummaryKeyTypeTypeId;
        readonly maxAge: Duration;
        readonly maxSize: number;
        readonly error: number;
        readonly quantiles: ReadonlyArray<number>;
    };
    interface Variance<in In, out Out> {
        readonly [MetricKeyTypeTypeId]: {
            readonly _In: Contravariant<In>;
            readonly _Out: Covariant<Out>;
        };
    }
    type InType<Type extends MetricKeyType<any, any>> = [
        Type
    ] extends [
        {
            readonly [MetricKeyTypeTypeId]: {
                readonly _In: (_: infer In) => void;
            };
        }
    ] ? In : never;
    type OutType<Type extends MetricKeyType<any, any>> = [
        Type
    ] extends [
        {
            readonly [MetricKeyTypeTypeId]: {
                readonly _Out: (_: never) => infer Out;
            };
        }
    ] ? Out : never;
}
declare const MetricTypeId: unique symbol;
type MetricTypeId = typeof MetricTypeId;
interface Metric<in out Type, in In, out Out> extends Metric.Variance<Type, In, Out>, Pipeable {
    readonly keyType: Type;
    unsafeUpdate(input: In, extraTags: ReadonlyArray<MetricLabel>): void;
    unsafeValue(extraTags: ReadonlyArray<MetricLabel>): Out;
    unsafeModify(input: In, extraTags: ReadonlyArray<MetricLabel>): void;
    register(): this;
    <A extends In, E, R>(effect: Effect<A, E, R>): Effect<A, E, R>;
}
declare namespace Metric {
    interface Counter<In extends number | bigint> extends Metric<MetricKeyType.Counter<In>, In, MetricState.Counter<In>> {
    }
    interface Gauge<In extends number | bigint> extends Metric<MetricKeyType.Gauge<In>, In, MetricState.Gauge<In>> {
    }
    interface Frequency<In> extends Metric<MetricKeyType.Frequency, In, MetricState.Frequency> {
    }
    interface Histogram<In> extends Metric<MetricKeyType.Histogram, In, MetricState.Histogram> {
    }
    interface Summary<In> extends Metric<MetricKeyType.Summary, In, MetricState.Summary> {
    }
    interface Variance<in out Type, in In, out Out> {
        readonly [MetricTypeId]: {
            readonly _Type: Invariant<Type>;
            readonly _In: Contravariant<In>;
            readonly _Out: Covariant<Out>;
        };
    }
}
declare const RequestResolverTypeId: unique symbol;
type RequestResolverTypeId = typeof RequestResolverTypeId;
interface RequestResolver<in A, out R = never> extends RequestResolver.Variance<A, R>, Equal, Pipeable {
    runAll(requests: Array<Array<Entry<A>>>): Effect<void, never, R>;
    identified(...identifiers: Array<unknown>): RequestResolver<A, R>;
}
declare namespace RequestResolver {
    interface Variance<in A, out R> {
        readonly [RequestResolverTypeId]: {
            readonly _A: Contravariant<A>;
            readonly _R: Covariant<R>;
        };
    }
}
type RequestBlock = Empty | Par | Seq | Single;
declare namespace RequestBlock {
    interface Reducer<in out Z> {
        emptyCase(): Z;
        parCase(left: Z, right: Z): Z;
        singleCase(dataSource: RequestResolver<unknown>, blockedRequest: Entry<unknown>): Z;
        seqCase(left: Z, right: Z): Z;
    }
}
interface Empty {
    readonly _tag: "Empty";
}
interface Par {
    readonly _tag: "Par";
    readonly left: RequestBlock;
    readonly right: RequestBlock;
}
interface Seq {
    readonly _tag: "Seq";
    readonly left: RequestBlock;
    readonly right: RequestBlock;
}
interface Single {
    readonly _tag: "Single";
    readonly dataSource: RequestResolver<unknown>;
    readonly blockedRequest: Entry<unknown>;
}
declare const EffectTypeId: unique symbol;
type EffectTypeId = typeof EffectTypeId;
interface EffectGenerator<T extends Effect<any, any, any>> {
    next(...args: ReadonlyArray<any>): IteratorResult<YieldWrap<T>, Effect.Success<T>>;
}
interface EffectUnify<A extends {
    [typeSymbol]?: any;
}> extends EitherUnify<A>, OptionUnify<A>, TagUnify<A> {
    Effect?: () => A[typeSymbol] extends Effect<infer A0, infer E0, infer R0> | infer _ ? Effect<A0, E0, R0> : never;
}
interface EffectUnifyIgnore {
    Tag?: true;
    Option?: true;
    Either?: true;
}
interface EffectTypeLambda extends TypeLambda {
    readonly type: Effect<this["Target"], this["Out1"], this["Out2"]>;
}
interface Blocked<out A, out E> extends Effect<A, E> {
    readonly _op: "Blocked";
    readonly effect_instruction_i0: RequestBlock;
    readonly effect_instruction_i1: Effect<A, E>;
}
interface Effect<out A, out E = never, out R = never> extends Effect.Variance<A, E, R>, Pipeable {
    readonly [typeSymbol]?: unknown;
    readonly [unifySymbol]?: EffectUnify<this>;
    readonly [ignoreSymbol]?: EffectUnifyIgnore;
    [Symbol.iterator](): EffectGenerator<Effect<A, E, R>>;
}
declare namespace Effect {
    interface Variance<out A, out E, out R> {
        readonly [EffectTypeId]: VarianceStruct<A, E, R>;
    }
    interface VarianceStruct<out A, out E, out R> {
        readonly _V: string;
        readonly _A: Covariant<A>;
        readonly _E: Covariant<E>;
        readonly _R: Covariant<R>;
    }
    type Context<T extends Effect<any, any, any>> = [
        T
    ] extends [
        Effect<infer _A, infer _E, infer _R>
    ] ? _R : never;
    type Error<T extends Effect<any, any, any>> = [
        T
    ] extends [
        Effect<infer _A, infer _E, infer _R>
    ] ? _E : never;
    type Success<T extends Effect<any, any, any>> = [
        T
    ] extends [
        Effect<infer _A, infer _E, infer _R>
    ] ? _A : never;
    type AsEffect<T extends Effect<any, any, any>> = Effect<T extends Effect<infer _A, infer _E, infer _R> ? _A : never, T extends Effect<infer _A, infer _E, infer _R> ? _E : never, T extends Effect<infer _A, infer _E, infer _R> ? _R : never> extends infer Q ? Q : never;
}
declare const isEffect: (u: unknown) => u is Effect<unknown, unknown, unknown>;
declare const cachedWithTTL: {
    (timeToLive: DurationInput): <A, E, R>(self: Effect<A, E, R>) => Effect<Effect<A, E>, never, R>;
    <A, E, R>(self: Effect<A, E, R>, timeToLive: DurationInput): Effect<Effect<A, E>, never, R>;
};
declare const cachedInvalidateWithTTL: {
    (timeToLive: DurationInput): <A, E, R>(self: Effect<A, E, R>) => Effect<[
        Effect<A, E>,
        Effect<void>
    ], never, R>;
    <A, E, R>(self: Effect<A, E, R>, timeToLive: DurationInput): Effect<[
        Effect<A, E>,
        Effect<void>
    ], never, R>;
};
declare const cached: <A, E, R>(self: Effect<A, E, R>) => Effect<Effect<A, E, R>>;
declare const cachedFunction: <A, B, E, R>(f: (a: A) => Effect<B, E, R>, eq?: Equivalence$1<A>) => Effect<(a: A) => Effect<B, E, R>>;
declare const once: <A, E, R>(self: Effect<A, E, R>) => Effect<Effect<void, E, R>>;
declare const all$1: <const Arg extends Iterable<Effect<any, any, any>> | Record<string, Effect<any, any, any>>, O extends NoExcessProperties<{
    readonly concurrency?: Concurrency | undefined;
    readonly batching?: boolean | "inherit" | undefined;
    readonly discard?: boolean | undefined;
    readonly mode?: "default" | "validate" | "either" | undefined;
    readonly concurrentFinalizers?: boolean | undefined;
}, O>>(arg: Arg, options?: O) => All.Return<Arg, O>;
declare const allWith: <O extends NoExcessProperties<{
    readonly concurrency?: Concurrency | undefined;
    readonly batching?: boolean | "inherit" | undefined;
    readonly discard?: boolean | undefined;
    readonly mode?: "default" | "validate" | "either" | undefined;
    readonly concurrentFinalizers?: boolean | undefined;
}, O>>(options?: O) => <const Arg extends Iterable<Effect<any, any, any>> | Record<string, Effect<any, any, any>>>(arg: Arg) => All.Return<Arg, O>;
declare namespace All {
    type EffectAny = Effect<any, any, any>;
    type ReturnIterable<T extends Iterable<EffectAny>, Discard extends boolean, Mode> = [
        T
    ] extends [
        Iterable<Effect.Variance<infer R0, infer L0, infer R>>
    ] ? Effect<Discard extends true ? void : Mode extends "either" ? Array<Either<R0, L0>> : Array<R0>, Mode extends "either" ? never : Mode extends "validate" ? Array<Option<L0>> : L0, R> : never;
    type ReturnTuple<T extends ReadonlyArray<unknown>, Discard extends boolean, Mode> = Effect<Discard extends true ? void : T[number] extends never ? [
    ] : Mode extends "either" ? {
        -readonly [K in keyof T]: [
            T[K]
        ] extends [
            Effect.Variance<infer _A, infer _E, infer _R>
        ] ? Either<_A, _E> : never;
    } : {
        -readonly [K in keyof T]: [
            T[K]
        ] extends [
            Effect.Variance<infer _A, infer _E, infer _R>
        ] ? _A : never;
    }, Mode extends "either" ? never : T[number] extends never ? never : Mode extends "validate" ? {
        -readonly [K in keyof T]: [
            T[K]
        ] extends [
            Effect.Variance<infer _A, infer _E, infer _R>
        ] ? Option<_E> : never;
    } : [
        T[number]
    ] extends [
        {
            [EffectTypeId]: {
                _E: (_: never) => infer E;
            };
        }
    ] ? E : never, T[number] extends never ? never : [
        T[number]
    ] extends [
        {
            [EffectTypeId]: {
                _R: (_: never) => infer R;
            };
        }
    ] ? R : never> extends infer X ? X : never;
    type ReturnObject<T, Discard extends boolean, Mode> = [
        T
    ] extends [
        {
            [K: string]: EffectAny;
        }
    ] ? Effect<Discard extends true ? void : Mode extends "either" ? {
        -readonly [K in keyof T]: [
            T[K]
        ] extends [
            Effect.Variance<infer _A, infer _E, infer _R>
        ] ? Either<_A, _E> : never;
    } : {
        -readonly [K in keyof T]: [
            T[K]
        ] extends [
            Effect.Variance<infer _A, infer _E, infer _R>
        ] ? _A : never;
    }, Mode extends "either" ? never : keyof T extends never ? never : Mode extends "validate" ? {
        -readonly [K in keyof T]: [
            T[K]
        ] extends [
            Effect.Variance<infer _A, infer _E, infer _R>
        ] ? Option<_E> : never;
    } : [
        T[keyof T]
    ] extends [
        {
            [EffectTypeId]: {
                _E: (_: never) => infer E;
            };
        }
    ] ? E : never, keyof T extends never ? never : [
        T[keyof T]
    ] extends [
        {
            [EffectTypeId]: {
                _R: (_: never) => infer R;
            };
        }
    ] ? R : never> : never;
    type IsDiscard<A> = [
        Extract<A, {
            readonly discard: true;
        }>
    ] extends [
        never
    ] ? false : true;
    type ExtractMode<A> = [
        A
    ] extends [
        {
            mode: infer M;
        }
    ] ? M : "default";
    type Return<Arg extends Iterable<EffectAny> | Record<string, EffectAny>, O extends NoExcessProperties<{
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly discard?: boolean | undefined;
        readonly mode?: "default" | "validate" | "either" | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    }, O>> = [
        Arg
    ] extends [
        ReadonlyArray<EffectAny>
    ] ? ReturnTuple<Arg, IsDiscard<O>, ExtractMode<O>> : [
        Arg
    ] extends [
        Iterable<EffectAny>
    ] ? ReturnIterable<Arg, IsDiscard<O>, ExtractMode<O>> : [
        Arg
    ] extends [
        Record<string, EffectAny>
    ] ? ReturnObject<Arg, IsDiscard<O>, ExtractMode<O>> : never;
}
declare const allSuccesses: <X extends Effect<any, any, any>>(elements: Iterable<X>, options?: {
    readonly concurrency?: Concurrency | undefined;
    readonly batching?: boolean | "inherit" | undefined;
    readonly concurrentFinalizers?: boolean | undefined;
} | undefined) => Effect<Array<Effect.Success<X>>, never, Effect.Context<X>>;
declare const dropUntil: {
    <A, E, R>(predicate: (a: NoInfer$1<A>, i: number) => Effect<boolean, E, R>): (elements: Iterable<A>) => Effect<Array<A>, E, R>;
    <A, E, R>(elements: Iterable<A>, predicate: (a: A, i: number) => Effect<boolean, E, R>): Effect<Array<A>, E, R>;
};
declare const dropWhile: {
    <A, E, R>(predicate: (a: NoInfer$1<A>, i: number) => Effect<boolean, E, R>): (elements: Iterable<A>) => Effect<Array<A>, E, R>;
    <A, E, R>(elements: Iterable<A>, predicate: (a: A, i: number) => Effect<boolean, E, R>): Effect<Array<A>, E, R>;
};
declare const takeUntil: {
    <A, R, E>(predicate: (a: NoInfer$1<A>, i: number) => Effect<boolean, E, R>): (elements: Iterable<A>) => Effect<Array<A>, E, R>;
    <A, E, R>(elements: Iterable<A>, predicate: (a: NoInfer$1<A>, i: number) => Effect<boolean, E, R>): Effect<Array<A>, E, R>;
};
declare const takeWhile: {
    <A, E, R>(predicate: (a: NoInfer$1<A>, i: number) => Effect<boolean, E, R>): (elements: Iterable<A>) => Effect<Array<A>, E, R>;
    <A, E, R>(elements: Iterable<A>, predicate: (a: NoInfer$1<A>, i: number) => Effect<boolean, E, R>): Effect<Array<A>, E, R>;
};
declare const every: {
    <A, E, R>(predicate: (a: A, i: number) => Effect<boolean, E, R>): (elements: Iterable<A>) => Effect<boolean, E, R>;
    <A, E, R>(elements: Iterable<A>, predicate: (a: A, i: number) => Effect<boolean, E, R>): Effect<boolean, E, R>;
};
declare const exists: {
    <A, E, R>(predicate: (a: A, i: number) => Effect<boolean, E, R>, options?: {
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    } | undefined): (elements: Iterable<A>) => Effect<boolean, E, R>;
    <A, E, R>(elements: Iterable<A>, predicate: (a: A, i: number) => Effect<boolean, E, R>, options?: {
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    } | undefined): Effect<boolean, E, R>;
};
declare const filter: {
    <A, E, R>(predicate: (a: NoInfer$1<A>, i: number) => Effect<boolean, E, R>, options?: {
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly negate?: boolean | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    } | undefined): (elements: Iterable<A>) => Effect<Array<A>, E, R>;
    <A, E, R>(elements: Iterable<A>, predicate: (a: NoInfer$1<A>, i: number) => Effect<boolean, E, R>, options?: {
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly negate?: boolean | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    } | undefined): Effect<Array<A>, E, R>;
};
declare const filterMap: {
    <Eff extends Effect<any, any, any>, B>(pf: (a: Effect.Success<Eff>) => Option<B>): (elements: Iterable<Eff>) => Effect<Array<B>, Effect.Error<Eff>, Effect.Context<Eff>>;
    <Eff extends Effect<any, any, any>, B>(elements: Iterable<Eff>, pf: (a: Effect.Success<Eff>) => Option<B>): Effect<Array<B>, Effect.Error<Eff>, Effect.Context<Eff>>;
};
declare const findFirst: {
    <A, E, R>(predicate: (a: NoInfer$1<A>, i: number) => Effect<boolean, E, R>): (elements: Iterable<A>) => Effect<Option<A>, E, R>;
    <A, E, R>(elements: Iterable<A>, predicate: (a: NoInfer$1<A>, i: number) => Effect<boolean, E, R>): Effect<Option<A>, E, R>;
};
declare const forEach: {
    <B, E, R, S extends Iterable<any>>(f: (a: ReadonlyArray$1.Infer<S>, i: number) => Effect<B, E, R>, options?: {
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly discard?: false | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    } | undefined): (self: S) => Effect<ReadonlyArray$1.With<S, B>, E, R>;
    <A, B, E, R>(f: (a: A, i: number) => Effect<B, E, R>, options: {
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly discard: true;
        readonly concurrentFinalizers?: boolean | undefined;
    }): (self: Iterable<A>) => Effect<void, E, R>;
    <B, E, R, S extends Iterable<any>>(self: S, f: (a: ReadonlyArray$1.Infer<S>, i: number) => Effect<B, E, R>, options?: {
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly discard?: false | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    } | undefined): Effect<ReadonlyArray$1.With<S, B>, E, R>;
    <A, B, E, R>(self: Iterable<A>, f: (a: A, i: number) => Effect<B, E, R>, options: {
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly discard: true;
        readonly concurrentFinalizers?: boolean | undefined;
    }): Effect<void, E, R>;
};
declare const head: <A, E, R>(self: Effect<Iterable<A>, E, R>) => Effect<A, NoSuchElementException | E, R>;
declare const mergeAll: {
    <Z, Eff extends Effect<any, any, any>>(zero: Z, f: (z: Z, a: Effect.Success<Eff>, i: number) => Z, options?: {
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    } | undefined): (elements: Iterable<Eff>) => Effect<Z, Effect.Error<Eff>, Effect.Context<Eff>>;
    <Eff extends Effect<any, any, any>, Z>(elements: Iterable<Eff>, zero: Z, f: (z: Z, a: Effect.Success<Eff>, i: number) => Z, options?: {
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    } | undefined): Effect<Z, Effect.Error<Eff>, Effect.Context<Eff>>;
};
declare const partition: {
    <A, B, E, R>(f: (a: A, i: number) => Effect<B, E, R>, options?: {
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    } | undefined): (elements: Iterable<A>) => Effect<[
        excluded: Array<E>,
        satisfying: Array<B>
    ], never, R>;
    <A, B, E, R>(elements: Iterable<A>, f: (a: A, i: number) => Effect<B, E, R>, options?: {
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    } | undefined): Effect<[
        excluded: Array<E>,
        satisfying: Array<B>
    ], never, R>;
};
declare const reduce: {
    <Z, A, E, R>(zero: Z, f: (z: Z, a: A, i: number) => Effect<Z, E, R>): (elements: Iterable<A>) => Effect<Z, E, R>;
    <A, Z, E, R>(elements: Iterable<A>, zero: Z, f: (z: Z, a: A, i: number) => Effect<Z, E, R>): Effect<Z, E, R>;
};
declare const reduceWhile: {
    <Z, A, E, R>(zero: Z, options: {
        readonly while: Predicate<Z>;
        readonly body: (s: Z, a: A, i: number) => Effect<Z, E, R>;
    }): (elements: Iterable<A>) => Effect<Z, E, R>;
    <A, Z, E, R>(elements: Iterable<A>, zero: Z, options: {
        readonly while: Predicate<Z>;
        readonly body: (s: Z, a: A, i: number) => Effect<Z, E, R>;
    }): Effect<Z, E, R>;
};
declare const reduceRight: {
    <A, Z, R, E>(zero: Z, f: (a: A, z: Z, i: number) => Effect<Z, E, R>): (elements: Iterable<A>) => Effect<Z, E, R>;
    <A, Z, R, E>(elements: Iterable<A>, zero: Z, f: (a: A, z: Z, i: number) => Effect<Z, E, R>): Effect<Z, E, R>;
};
declare const reduceEffect: {
    <Z, E, R, Eff extends Effect<any, any, any>>(zero: Effect<Z, E, R>, f: (z: NoInfer$1<Z>, a: Effect.Success<Eff>, i: number) => Z, options?: {
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    } | undefined): (elements: Iterable<Eff>) => Effect<Z, E | Effect.Error<Eff>, R | Effect.Context<Eff>>;
    <Eff extends Effect<any, any, any>, Z, E, R>(elements: Iterable<Eff>, zero: Effect<Z, E, R>, f: (z: NoInfer$1<Z>, a: Effect.Success<Eff>, i: number) => Z, options?: {
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    } | undefined): Effect<Z, E | Effect.Error<Eff>, R | Effect.Context<Eff>>;
};
declare const replicate: {
    (n: number): <A, E, R>(self: Effect<A, E, R>) => Array<Effect<A, E, R>>;
    <A, E, R>(self: Effect<A, E, R>, n: number): Array<Effect<A, E, R>>;
};
declare const replicateEffect: {
    (n: number, options?: {
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly discard?: false | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    }): <A, E, R>(self: Effect<A, E, R>) => Effect<Array<A>, E, R>;
    (n: number, options: {
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly discard: true;
        readonly concurrentFinalizers?: boolean | undefined;
    }): <A, E, R>(self: Effect<A, E, R>) => Effect<void, E, R>;
    <A, E, R>(self: Effect<A, E, R>, n: number, options?: {
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly discard?: false | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    }): Effect<Array<A>, E, R>;
    <A, E, R>(self: Effect<A, E, R>, n: number, options: {
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly discard: true;
        readonly concurrentFinalizers?: boolean | undefined;
    }): Effect<void, E, R>;
};
declare const validateAll: {
    <A, B, E, R>(f: (a: A, i: number) => Effect<B, E, R>, options?: {
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly discard?: false | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    } | undefined): (elements: Iterable<A>) => Effect<Array<B>, NonEmptyArray<E>, R>;
    <A, B, E, R>(f: (a: A, i: number) => Effect<B, E, R>, options: {
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly discard: true;
        readonly concurrentFinalizers?: boolean | undefined;
    }): (elements: Iterable<A>) => Effect<void, NonEmptyArray<E>, R>;
    <A, B, E, R>(elements: Iterable<A>, f: (a: A, i: number) => Effect<B, E, R>, options?: {
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly discard?: false | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    } | undefined): Effect<Array<B>, NonEmptyArray<E>, R>;
    <A, B, E, R>(elements: Iterable<A>, f: (a: A, i: number) => Effect<B, E, R>, options: {
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly discard: true;
        readonly concurrentFinalizers?: boolean | undefined;
    }): Effect<void, NonEmptyArray<E>, R>;
};
declare const validateFirst: {
    <A, B, E, R>(f: (a: A, i: number) => Effect<B, E, R>, options?: {
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    } | undefined): (elements: Iterable<A>) => Effect<B, Array<E>, R>;
    <A, B, E, R>(elements: Iterable<A>, f: (a: A, i: number) => Effect<B, E, R>, options?: {
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    } | undefined): Effect<B, Array<E>, R>;
};
declare const async: <A, E = never, R = never>(resume: (callback: (_: Effect<A, E, R>) => void, signal: AbortSignal) => void | Effect<void, never, R>, blockingOn?: FiberId) => Effect<A, E, R>;
declare const asyncEffect: <A, E, R, R3, E2, R2>(register: (callback: (_: Effect<A, E, R>) => void) => Effect<Effect<void, never, R3> | void, E2, R2>) => Effect<A, E | E2, R | R2 | R3>;
declare const custom: {
    <X, A, E, R>(i0: X, body: (this: {
        effect_instruction_i0: X;
    }) => Effect<A, E, R>): Effect<A, E, R>;
    <X, Y, A, E, R>(i0: X, i1: Y, body: (this: {
        effect_instruction_i0: X;
        effect_instruction_i1: Y;
    }) => Effect<A, E, R>): Effect<A, E, R>;
    <X, Y, Z, A, E, R>(i0: X, i1: Y, i2: Z, body: (this: {
        effect_instruction_i0: X;
        effect_instruction_i1: Y;
        effect_instruction_i2: Z;
    }) => Effect<A, E, R>): Effect<A, E, R>;
};
declare const withFiberRuntime: <A, E = never, R = never>(withRuntime: (fiber: RuntimeFiber<A, E>, status: Running) => Effect<A, E, R>) => Effect<A, E, R>;
declare const fail: <E>(error: E) => Effect<never, E>;
declare const failSync: <E>(evaluate: LazyArg<E>) => Effect<never, E>;
declare const failCause: <E>(cause: Cause<E>) => Effect<never, E>;
declare const failCauseSync: <E>(evaluate: LazyArg<Cause<E>>) => Effect<never, E>;
declare const die: (defect: unknown) => Effect<never>;
declare const dieMessage: (message: string) => Effect<never>;
declare const dieSync: (evaluate: LazyArg<unknown>) => Effect<never>;
declare const gen$1: {
    <Eff extends YieldWrap<Effect<any, any, any>>, AEff>(f: (resume: Adapter) => Generator<Eff, AEff, never>): Effect<AEff, [
        Eff
    ] extends [
        never
    ] ? never : [
        Eff
    ] extends [
        YieldWrap<Effect<infer _A, infer E, infer _R>>
    ] ? E : never, [
        Eff
    ] extends [
        never
    ] ? never : [
        Eff
    ] extends [
        YieldWrap<Effect<infer _A, infer _E, infer R>>
    ] ? R : never>;
    <Self, Eff extends YieldWrap<Effect<any, any, any>>, AEff>(self: Self, f: (this: Self, resume: Adapter) => Generator<Eff, AEff, never>): Effect<AEff, [
        Eff
    ] extends [
        never
    ] ? never : [
        Eff
    ] extends [
        YieldWrap<Effect<infer _A, infer E, infer _R>>
    ] ? E : never, [
        Eff
    ] extends [
        never
    ] ? never : [
        Eff
    ] extends [
        YieldWrap<Effect<infer _A, infer _E, infer R>>
    ] ? R : never>;
};
interface Adapter {
    <A, E, R>(self: Effect<A, E, R>): Effect<A, E, R>;
    <A, _A, _E, _R>(a: A, ab: (a: A) => Effect<_A, _E, _R>): Effect<_A, _E, _R>;
    <A, B, _A, _E, _R>(a: A, ab: (a: A) => B, bc: (b: B) => Effect<_A, _E, _R>): Effect<_A, _E, _R>;
    <A, B, C, _A, _E, _R>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => Effect<_A, _E, _R>): Effect<_A, _E, _R>;
    <A, B, C, D, _A, _E, _R>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => Effect<_A, _E, _R>): Effect<_A, _E, _R>;
    <A, B, C, D, E, _A, _E, _R>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => Effect<_A, _E, _R>): Effect<_A, _E, _R>;
    <A, B, C, D, E, F, _A, _E, _R>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => Effect<_A, _E, _R>): Effect<_A, _E, _R>;
    <A, B, C, D, E, F, G, _A, _E, _R>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => Effect<_A, _E, _R>): Effect<_A, _E, _R>;
    <A, B, C, D, E, F, G, H, _A, _E, _R>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (g: H) => Effect<_A, _E, _R>): Effect<_A, _E, _R>;
    <A, B, C, D, E, F, G, H, I, _A, _E, _R>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => Effect<_A, _E, _R>): Effect<_A, _E, _R>;
    <A, B, C, D, E, F, G, H, I, J, _A, _E, _R>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => Effect<_A, _E, _R>): Effect<_A, _E, _R>;
    <A, B, C, D, E, F, G, H, I, J, K, _A, _E, _R>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => Effect<_A, _E, _R>): Effect<_A, _E, _R>;
    <A, B, C, D, E, F, G, H, I, J, K, L, _A, _E, _R>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L, lm: (l: L) => Effect<_A, _E, _R>): Effect<_A, _E, _R>;
    <A, B, C, D, E, F, G, H, I, J, K, L, M, _A, _E, _R>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L, lm: (l: L) => M, mn: (m: M) => Effect<_A, _E, _R>): Effect<_A, _E, _R>;
    <A, B, C, D, E, F, G, H, I, J, K, L, M, N, _A, _E, _R>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L, lm: (l: L) => M, mn: (m: M) => N, no: (n: N) => Effect<_A, _E, _R>): Effect<_A, _E, _R>;
    <A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, _A, _E, _R>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L, lm: (l: L) => M, mn: (m: M) => N, no: (n: N) => O, op: (o: O) => Effect<_A, _E, _R>): Effect<_A, _E, _R>;
    <A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, _A, _E, _R>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L, lm: (l: L) => M, mn: (m: M) => N, no: (n: N) => O, op: (o: O) => P, pq: (p: P) => Effect<_A, _E, _R>): Effect<_A, _E, _R>;
    <A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, _A, _E, _R>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L, lm: (l: L) => M, mn: (m: M) => N, no: (n: N) => O, op: (o: O) => P, pq: (p: P) => Q, qr: (q: Q) => Effect<_A, _E, _R>): Effect<_A, _E, _R>;
    <A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, _A, _E, _R>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L, lm: (l: L) => M, mn: (m: M) => N, no: (n: N) => O, op: (o: O) => P, pq: (p: P) => Q, qr: (q: Q) => R, rs: (r: R) => Effect<_A, _E, _R>): Effect<_A, _E, _R>;
    <A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, _A, _E, _R>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L, lm: (l: L) => M, mn: (m: M) => N, no: (n: N) => O, op: (o: O) => P, pq: (p: P) => Q, qr: (q: Q) => R, rs: (r: R) => S, st: (s: S) => Effect<_A, _E, _R>): Effect<_A, _E, _R>;
    <A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, _A, _E, _R>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F, fg: (f: F) => G, gh: (g: G) => H, hi: (h: H) => I, ij: (i: I) => J, jk: (j: J) => K, kl: (k: K) => L, lm: (l: L) => M, mn: (m: M) => N, no: (n: N) => O, op: (o: O) => P, pq: (p: P) => Q, qr: (q: Q) => R, rs: (r: R) => S, st: (s: S) => T, tu: (s: T) => Effect<_A, _E, _R>): Effect<_A, _E, _R>;
}
declare const never: Effect<never>;
declare const none: <A, E, R>(self: Effect<Option<A>, E, R>) => Effect<void, E | NoSuchElementException, R>;
declare const promise: <A>(evaluate: (signal: AbortSignal) => PromiseLike<A>) => Effect<A>;
declare const succeed: <A>(value: A) => Effect<A>;
declare const succeedNone: Effect<Option<never>>;
declare const succeedSome: <A>(value: A) => Effect<Option<A>>;
declare const suspend: <A, E, R>(effect: LazyArg<Effect<A, E, R>>) => Effect<A, E, R>;
declare const sync: <A>(thunk: LazyArg<A>) => Effect<A>;
declare const _void: Effect<void>;
declare const yieldNow: (options?: {
    readonly priority?: number | undefined;
}) => Effect<void>;
declare const _catch: {
    <N extends keyof E, K extends E[N] & string, E, A1, E1, R1>(discriminator: N, options: {
        readonly failure: K;
        readonly onFailure: (error: Extract<E, {
            [n in N]: K;
        }>) => Effect<A1, E1, R1>;
    }): <A, R>(self: Effect<A, E, R>) => Effect<A1 | A, E1 | Exclude<E, {
        [n in N]: K;
    }>, R1 | R>;
    <A, E, R, N extends keyof E, K extends E[N] & string, A1, E1, R1>(self: Effect<A, E, R>, discriminator: N, options: {
        readonly failure: K;
        readonly onFailure: (error: Extract<E, {
            [n in N]: K;
        }>) => Effect<A1, E1, R1>;
    }): Effect<A | A1, E1 | Exclude<E, {
        [n in N]: K;
    }>, R | R1>;
};
declare const catchAll: {
    <E, A2, E2, R2>(f: (e: E) => Effect<A2, E2, R2>): <A, R>(self: Effect<A, E, R>) => Effect<A2 | A, E2, R2 | R>;
    <A, E, R, A2, E2, R2>(self: Effect<A, E, R>, f: (e: E) => Effect<A2, E2, R2>): Effect<A2 | A, E2, R2 | R>;
};
declare const catchAllCause: {
    <E, A2, E2, R2>(f: (cause: Cause<E>) => Effect<A2, E2, R2>): <A, R>(self: Effect<A, E, R>) => Effect<A2 | A, E2, R2 | R>;
    <A, E, R, A2, E2, R2>(self: Effect<A, E, R>, f: (cause: Cause<E>) => Effect<A2, E2, R2>): Effect<A | A2, E2, R | R2>;
};
declare const catchAllDefect: {
    <A2, E2, R2>(f: (defect: unknown) => Effect<A2, E2, R2>): <A, E, R>(self: Effect<A, E, R>) => Effect<A2 | A, E2 | E, R2 | R>;
    <A, E, R, A2, E2, R2>(self: Effect<A, E, R>, f: (defect: unknown) => Effect<A2, E2, R2>): Effect<A | A2, E | E2, R | R2>;
};
declare const catchIf: {
    <E, EB extends E, A2, E2, R2>(refinement: Refinement<NoInfer$1<E>, EB>, f: (e: EB) => Effect<A2, E2, R2>): <A, R>(self: Effect<A, E, R>) => Effect<A2 | A, E2 | Exclude<E, EB>, R2 | R>;
    <E, A2, E2, R2>(predicate: Predicate<NoInfer$1<E>>, f: (e: NoInfer$1<E>) => Effect<A2, E2, R2>): <A, R>(self: Effect<A, E, R>) => Effect<A2 | A, E | E2, R2 | R>;
    <A, E, R, EB extends E, A2, E2, R2>(self: Effect<A, E, R>, refinement: Refinement<E, EB>, f: (e: EB) => Effect<A2, E2, R2>): Effect<A | A2, E2 | Exclude<E, EB>, R | R2>;
    <A, E, R, A2, E2, R2>(self: Effect<A, E, R>, predicate: Predicate<E>, f: (e: E) => Effect<A2, E2, R2>): Effect<A | A2, E | E2, R | R2>;
};
declare const catchSome: {
    <E, A2, E2, R2>(pf: (e: NoInfer$1<E>) => Option<Effect<A2, E2, R2>>): <A, R>(self: Effect<A, E, R>) => Effect<A2 | A, E | E2, R2 | R>;
    <A, E, R, A2, E2, R2>(self: Effect<A, E, R>, pf: (e: NoInfer$1<E>) => Option<Effect<A2, E2, R2>>): Effect<A | A2, E | E2, R | R2>;
};
declare const catchSomeCause: {
    <E, A2, E2, R2>(f: (cause: Cause<NoInfer$1<E>>) => Option<Effect<A2, E2, R2>>): <A, R>(self: Effect<A, E, R>) => Effect<A2 | A, E | E2, R2 | R>;
    <A, E, R, A2, E2, R2>(self: Effect<A, E, R>, f: (cause: Cause<NoInfer$1<E>>) => Option<Effect<A2, E2, R2>>): Effect<A2 | A, E | E2, R2 | R>;
};
declare const catchSomeDefect: {
    <A2, E2, R2>(pf: (defect: unknown) => Option<Effect<A2, E2, R2>>): <A, E, R>(self: Effect<A, E, R>) => Effect<A2 | A, E2 | E, R2 | R>;
    <A, E, R, A2, E2, R2>(self: Effect<A, E, R>, pf: (defect: unknown) => Option<Effect<A2, E2, R2>>): Effect<A | A2, E | E2, R | R2>;
};
declare const catchTag: {
    <E, const K extends NonEmptyReadonlyArray<E extends {
        _tag: string;
    } ? E["_tag"] : never>, A1, E1, R1>(...args: [
        ...tags: K,
        f: (e: Extract<NoInfer$1<E>, {
            _tag: K[number];
        }>) => Effect<A1, E1, R1>
    ]): <A, R>(self: Effect<A, E, R>) => Effect<A | A1, Exclude<E, {
        _tag: K[number];
    }> | E1, R | R1>;
    <A, E, R, const K extends NonEmptyReadonlyArray<E extends {
        _tag: string;
    } ? E["_tag"] : never>, A1, E1, R1>(self: Effect<A, E, R>, ...args: [
        ...tags: K,
        f: (e: Extract<NoInfer$1<E>, {
            _tag: K[number];
        }>) => Effect<A1, E1, R1>
    ]): Effect<A | A1, Exclude<E, {
        _tag: K[number];
    }> | E1, R | R1>;
};
declare const catchTags: {
    <E, Cases extends {
        [K in Extract<E, {
            _tag: string;
        }>["_tag"]]+?: ((error: Extract<E, {
            _tag: K;
        }>) => Effect<any, any, any>);
    } & (unknown extends E ? {} : {
        [K in Exclude<keyof Cases, Extract<E, {
            _tag: string;
        }>["_tag"]>]: never;
    })>(cases: Cases): <A, R>(self: Effect<A, E, R>) => Effect<A | {
        [K in keyof Cases]: Cases[K] extends (...args: Array<any>) => Effect<infer A, any, any> ? A : never;
    }[keyof Cases], Exclude<E, {
        _tag: keyof Cases;
    }> | {
        [K in keyof Cases]: Cases[K] extends (...args: Array<any>) => Effect<any, infer E, any> ? E : never;
    }[keyof Cases], R | {
        [K in keyof Cases]: Cases[K] extends (...args: Array<any>) => Effect<any, any, infer R> ? R : never;
    }[keyof Cases]>;
    <R, E, A, Cases extends {
        [K in Extract<E, {
            _tag: string;
        }>["_tag"]]+?: ((error: Extract<E, {
            _tag: K;
        }>) => Effect<any, any, any>);
    } & (unknown extends E ? {} : {
        [K in Exclude<keyof Cases, Extract<E, {
            _tag: string;
        }>["_tag"]>]: never;
    })>(self: Effect<A, E, R>, cases: Cases): Effect<A | {
        [K in keyof Cases]: Cases[K] extends (...args: Array<any>) => Effect<infer A, any, any> ? A : never;
    }[keyof Cases], Exclude<E, {
        _tag: keyof Cases;
    }> | {
        [K in keyof Cases]: Cases[K] extends (...args: Array<any>) => Effect<any, infer E, any> ? E : never;
    }[keyof Cases], R | {
        [K in keyof Cases]: Cases[K] extends (...args: Array<any>) => Effect<any, any, infer R> ? R : never;
    }[keyof Cases]>;
};
declare const cause: <A, E, R>(self: Effect<A, E, R>) => Effect<Cause<E>, never, R>;
declare const eventually: <A, E, R>(self: Effect<A, E, R>) => Effect<A, never, R>;
declare const ignore: <A, E, R>(self: Effect<A, E, R>) => Effect<void, never, R>;
declare const ignoreLogged: <A, E, R>(self: Effect<A, E, R>) => Effect<void, never, R>;
declare const parallelErrors: <A, E, R>(self: Effect<A, E, R>) => Effect<A, Array<E>, R>;
declare const sandbox: <A, E, R>(self: Effect<A, E, R>) => Effect<A, Cause<E>, R>;
declare namespace Retry {
    type Return<R, E, A, O extends NoExcessProperties<Options<E>, O>> = Effect<A, (O extends {
        schedule: Schedule<infer _O, infer _I, infer _R>;
    } ? E : O extends {
        until: Refinement<E, infer E2>;
    } ? E2 : E) | (O extends {
        while: (...args: Array<any>) => Effect<infer _A, infer E, infer _R>;
    } ? E : never) | (O extends {
        until: (...args: Array<any>) => Effect<infer _A, infer E, infer _R>;
    } ? E : never), R | (O extends {
        schedule: Schedule<infer _O, infer _I, infer R>;
    } ? R : never) | (O extends {
        while: (...args: Array<any>) => Effect<infer _A, infer _E, infer R>;
    } ? R : never) | (O extends {
        until: (...args: Array<any>) => Effect<infer _A, infer _E, infer R>;
    } ? R : never)> extends infer Z ? Z : never;
    interface Options<E> {
        while?: ((error: E) => boolean | Effect<boolean, any, any>) | undefined;
        until?: ((error: E) => boolean | Effect<boolean, any, any>) | undefined;
        times?: number | undefined;
        schedule?: Schedule<any, E, any> | undefined;
    }
}
declare const retry: {
    <E, O extends NoExcessProperties<Retry.Options<E>, O>>(options: O): <A, R>(self: Effect<A, E, R>) => Retry.Return<R, E, A, O>;
    <B, E, R1>(policy: Schedule<B, NoInfer$1<E>, R1>): <A, R>(self: Effect<A, E, R>) => Effect<A, E, R1 | R>;
    <A, E, R, O extends NoExcessProperties<Retry.Options<E>, O>>(self: Effect<A, E, R>, options: O): Retry.Return<R, E, A, O>;
    <A, E, R, B, R1>(self: Effect<A, E, R>, policy: Schedule<B, NoInfer$1<E>, R1>): Effect<A, E, R1 | R>;
};
declare const withExecutionPlan: {
    <Input, Provides, PlanE, PlanR>(plan: ExecutionPlan<{
        provides: Provides;
        input: Input;
        error: PlanE;
        requirements: PlanR;
    }>): <A, E extends Input, R>(effect: Effect<A, E, R>) => Effect<A, E | PlanE, Exclude<R, Provides> | PlanR>;
    <A, E extends Input, R, Provides, Input, PlanE, PlanR>(effect: Effect<A, E, R>, plan: ExecutionPlan<{
        provides: Provides;
        input: Input;
        error: PlanE;
        requirements: PlanR;
    }>): Effect<A, E | PlanE, Exclude<R, Provides> | PlanR>;
};
declare const retryOrElse: {
    <A1, E, R1, A2, E2, R2>(policy: Schedule<A1, NoInfer$1<E>, R1>, orElse: (e: NoInfer$1<E>, out: A1) => Effect<A2, E2, R2>): <A, R>(self: Effect<A, E, R>) => Effect<A2 | A, E2, R1 | R2 | R>;
    <A, E, R, A1, R1, A2, E2, R2>(self: Effect<A, E, R>, policy: Schedule<A1, NoInfer$1<E>, R1>, orElse: (e: NoInfer$1<E>, out: A1) => Effect<A2, E2, R2>): Effect<A | A2, E2, R | R1 | R2>;
};
declare const try_$1: {
    <A, E>(options: {
        readonly try: LazyArg<A>;
        readonly catch: (error: unknown) => E;
    }): Effect<A, E>;
    <A>(thunk: LazyArg<A>): Effect<A, UnknownException>;
};
declare const tryMap: {
    <A, B, E1>(options: {
        readonly try: (a: A) => B;
        readonly catch: (error: unknown) => E1;
    }): <E, R>(self: Effect<A, E, R>) => Effect<B, E1 | E, R>;
    <A, E, R, B, E1>(self: Effect<A, E, R>, options: {
        readonly try: (a: A) => B;
        readonly catch: (error: unknown) => E1;
    }): Effect<B, E | E1, R>;
};
declare const tryMapPromise: {
    <A, B, E1>(options: {
        readonly try: (a: A, signal: AbortSignal) => PromiseLike<B>;
        readonly catch: (error: unknown) => E1;
    }): <E, R>(self: Effect<A, E, R>) => Effect<B, E1 | E, R>;
    <A, E, R, B, E1>(self: Effect<A, E, R>, options: {
        readonly try: (a: A, signal: AbortSignal) => PromiseLike<B>;
        readonly catch: (error: unknown) => E1;
    }): Effect<B, E | E1, R>;
};
declare const tryPromise: {
    <A, E>(options: {
        readonly try: (signal: AbortSignal) => PromiseLike<A>;
        readonly catch: (error: unknown) => E;
    }): Effect<A, E>;
    <A>(evaluate: (signal: AbortSignal) => PromiseLike<A>): Effect<A, UnknownException>;
};
declare const unsandbox: <A, E, R>(self: Effect<A, Cause<E>, R>) => Effect<A, E, R>;
declare const allowInterrupt: Effect<void>;
declare const checkInterruptible: <A, E, R>(f: (isInterruptible: boolean) => Effect<A, E, R>) => Effect<A, E, R>;
declare const disconnect: <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
declare const interrupt: Effect<never>;
declare const interruptWith: (fiberId: FiberId) => Effect<never>;
declare const interruptible: <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
declare const interruptibleMask: <A, E, R>(f: (restore: <AX, EX, RX>(effect: Effect<AX, EX, RX>) => Effect<AX, EX, RX>) => Effect<A, E, R>) => Effect<A, E, R>;
declare const onInterrupt: {
    <X, R2>(cleanup: (interruptors: HashSet<FiberId>) => Effect<X, never, R2>): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R2 | R>;
    <A, E, R, X, R2>(self: Effect<A, E, R>, cleanup: (interruptors: HashSet<FiberId>) => Effect<X, never, R2>): Effect<A, E, R | R2>;
};
declare const uninterruptible: <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
declare const uninterruptibleMask: <A, E, R>(f: (restore: <AX, EX, RX>(effect: Effect<AX, EX, RX>) => Effect<AX, EX, RX>) => Effect<A, E, R>) => Effect<A, E, R>;
declare const liftPredicate$1: {
    <T extends A, E, B extends T = T, A = T>(predicate: Refinement<T, B> | Predicate<T>, orFailWith: (a: EqualsWith<T, B, A, Exclude<A, B>>) => E): (a: A) => Effect<EqualsWith<T, B, A, B>, E>;
    <A, E, B extends A = A>(self: A, predicate: Refinement<A, B> | Predicate<A>, orFailWith: (a: EqualsWith<A, B, A, Exclude<A, B>>) => E): Effect<B, E>;
};
declare const as: {
    <B>(value: B): <A, E, R>(self: Effect<A, E, R>) => Effect<B, E, R>;
    <A, E, R, B>(self: Effect<A, E, R>, value: B): Effect<B, E, R>;
};
declare const asSome: <A, E, R>(self: Effect<A, E, R>) => Effect<Option<A>, E, R>;
declare const asSomeError: <A, E, R>(self: Effect<A, E, R>) => Effect<A, Option<E>, R>;
declare const asVoid: <A, E, R>(self: Effect<A, E, R>) => Effect<void, E, R>;
declare const flip$1: <A, E, R>(self: Effect<A, E, R>) => Effect<E, A, R>;
declare const flipWith: {
    <E, A, R, E2, A2, R2>(f: (effect: Effect<E, A, R>) => Effect<E2, A2, R2>): (self: Effect<A, E, R>) => Effect<A2, E2, R2>;
    <A, E, R, E2, A2, R2>(self: Effect<A, E, R>, f: (effect: Effect<E, A, R>) => Effect<E2, A2, R2>): Effect<A2, E2, R2>;
};
declare const map$1: {
    <A, B>(f: (a: A) => B): <E, R>(self: Effect<A, E, R>) => Effect<B, E, R>;
    <A, E, R, B>(self: Effect<A, E, R>, f: (a: A) => B): Effect<B, E, R>;
};
declare const mapAccum: {
    <S, A, B, E, R, I extends Iterable<A> = Iterable<A>>(initial: S, f: (state: S, a: ReadonlyArray$1.Infer<I>, i: number) => Effect<readonly [
        S,
        B
    ], E, R>): (elements: I) => Effect<[
        S,
        ReadonlyArray$1.With<I, B>
    ], E, R>;
    <A, S, B, E, R, I extends Iterable<A> = Iterable<A>>(elements: I, initial: S, f: (state: S, a: ReadonlyArray$1.Infer<I>, i: number) => Effect<readonly [
        S,
        B
    ], E, R>): Effect<[
        S,
        ReadonlyArray$1.With<I, B>
    ], E, R>;
};
declare const mapBoth$1: {
    <E, E2, A, A2>(options: {
        readonly onFailure: (e: E) => E2;
        readonly onSuccess: (a: A) => A2;
    }): <R>(self: Effect<A, E, R>) => Effect<A2, E2, R>;
    <A, E, R, E2, A2>(self: Effect<A, E, R>, options: {
        readonly onFailure: (e: E) => E2;
        readonly onSuccess: (a: A) => A2;
    }): Effect<A2, E2, R>;
};
declare const mapError: {
    <E, E2>(f: (e: E) => E2): <A, R>(self: Effect<A, E, R>) => Effect<A, E2, R>;
    <A, E, R, E2>(self: Effect<A, E, R>, f: (e: E) => E2): Effect<A, E2, R>;
};
declare const mapErrorCause: {
    <E, E2>(f: (cause: Cause<E>) => Cause<E2>): <A, R>(self: Effect<A, E, R>) => Effect<A, E2, R>;
    <A, E, R, E2>(self: Effect<A, E, R>, f: (cause: Cause<E>) => Cause<E2>): Effect<A, E2, R>;
};
declare const merge$1: <A, E, R>(self: Effect<A, E, R>) => Effect<E | A, never, R>;
declare const negate: <E, R>(self: Effect<boolean, E, R>) => Effect<boolean, E, R>;
declare const acquireRelease: {
    <A, X, R2>(release: (a: A, exit: Exit<unknown, unknown>) => Effect<X, never, R2>): <E, R>(acquire: Effect<A, E, R>) => Effect<A, E, Scope | R2 | R>;
    <A, E, R, X, R2>(acquire: Effect<A, E, R>, release: (a: A, exit: Exit<unknown, unknown>) => Effect<X, never, R2>): Effect<A, E, Scope | R | R2>;
};
declare const acquireReleaseInterruptible: {
    <X, R2>(release: (exit: Exit<unknown, unknown>) => Effect<X, never, R2>): <A, E, R>(acquire: Effect<A, E, R>) => Effect<A, E, Scope | R2 | R>;
    <A, E, R, X, R2>(acquire: Effect<A, E, R>, release: (exit: Exit<unknown, unknown>) => Effect<X, never, R2>): Effect<A, E, Scope | R | R2>;
};
declare const acquireUseRelease: {
    <A2, E2, R2, A, X, R3>(use: (a: A) => Effect<A2, E2, R2>, release: (a: A, exit: Exit<A2, E2>) => Effect<X, never, R3>): <E, R>(acquire: Effect<A, E, R>) => Effect<A2, E2 | E, R2 | R3 | R>;
    <A, E, R, A2, E2, R2, X, R3>(acquire: Effect<A, E, R>, use: (a: A) => Effect<A2, E2, R2>, release: (a: A, exit: Exit<A2, E2>) => Effect<X, never, R3>): Effect<A2, E | E2, R | R2 | R3>;
};
declare const addFinalizer: <X, R>(finalizer: (exit: Exit<unknown, unknown>) => Effect<X, never, R>) => Effect<void, never, Scope | R>;
declare const ensuring: {
    <X, R1>(finalizer: Effect<X, never, R1>): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R1 | R>;
    <A, E, R, X, R1>(self: Effect<A, E, R>, finalizer: Effect<X, never, R1>): Effect<A, E, R1 | R>;
};
declare const onError: {
    <E, X, R2>(cleanup: (cause: Cause<E>) => Effect<X, never, R2>): <A, R>(self: Effect<A, E, R>) => Effect<A, E, R2 | R>;
    <A, E, R, X, R2>(self: Effect<A, E, R>, cleanup: (cause: Cause<E>) => Effect<X, never, R2>): Effect<A, E, R2 | R>;
};
declare const onExit: {
    <A, E, X, R2>(cleanup: (exit: Exit<A, E>) => Effect<X, never, R2>): <R>(self: Effect<A, E, R>) => Effect<A, E, R2 | R>;
    <A, E, R, X, R2>(self: Effect<A, E, R>, cleanup: (exit: Exit<A, E>) => Effect<X, never, R2>): Effect<A, E, R | R2>;
};
declare const parallelFinalizers: <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
declare const sequentialFinalizers: <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
declare const finalizersMask: (strategy: ExecutionStrategy) => <A, E, R>(self: (restore: <A1, E1, R1>(self: Effect<A1, E1, R1>) => Effect<A1, E1, R1>) => Effect<A, E, R>) => Effect<A, E, R>;
declare const scope: Effect<Scope, never, Scope>;
declare const scopeWith: <A, E, R>(f: (scope: Scope) => Effect<A, E, R>) => Effect<A, E, R | Scope>;
declare const scopedWith: <A, E, R>(f: (scope: Scope) => Effect<A, E, R>) => Effect<A, E, R>;
declare const scoped: <A, E, R>(effect: Effect<A, E, R>) => Effect<A, E, Exclude<R, Scope>>;
declare const using: {
    <A, A2, E2, R2>(use: (a: A) => Effect<A2, E2, R2>): <E, R>(self: Effect<A, E, R>) => Effect<A2, E2 | E, R2 | Exclude<R, Scope>>;
    <A, E, R, A2, E2, R2>(self: Effect<A, E, R>, use: (a: A) => Effect<A2, E2, R2>): Effect<A2, E | E2, R2 | Exclude<R, Scope>>;
};
declare const withEarlyRelease: <A, E, R>(self: Effect<A, E, R>) => Effect<[
    finalizer: Effect<void>,
    result: A
], E, R | Scope>;
declare const awaitAllChildren: <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
declare const daemonChildren: <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
declare const descriptor: Effect<Fiber.Descriptor>;
declare const descriptorWith: <A, E, R>(f: (descriptor: Fiber.Descriptor) => Effect<A, E, R>) => Effect<A, E, R>;
declare const diffFiberRefs: <A, E, R>(self: Effect<A, E, R>) => Effect<[
    FiberRefsPatch,
    A
], E, R>;
declare const ensuringChild: {
    <X, R2>(f: (fiber: Fiber<ReadonlyArray<unknown>, any>) => Effect<X, never, R2>): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R2 | R>;
    <A, E, R, X, R2>(self: Effect<A, E, R>, f: (fiber: Fiber<ReadonlyArray<unknown>, any>) => Effect<X, never, R2>): Effect<A, E, R | R2>;
};
declare const ensuringChildren: {
    <X, R2>(children: (fibers: ReadonlyArray<RuntimeFiber<any, any>>) => Effect<X, never, R2>): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R2 | R>;
    <A, E, R, X, R2>(self: Effect<A, E, R>, children: (fibers: ReadonlyArray<RuntimeFiber<any, any>>) => Effect<X, never, R2>): Effect<A, E, R | R2>;
};
declare const fiberId: Effect<FiberId>;
declare const fiberIdWith: <A, E, R>(f: (descriptor: Runtime$1) => Effect<A, E, R>) => Effect<A, E, R>;
declare const fork: <A, E, R>(self: Effect<A, E, R>) => Effect<RuntimeFiber<A, E>, never, R>;
declare const forkDaemon: <A, E, R>(self: Effect<A, E, R>) => Effect<RuntimeFiber<A, E>, never, R>;
declare const forkAll: {
    (options?: {
        readonly discard?: false | undefined;
    } | undefined): <Eff extends Effect<any, any, any>>(effects: Iterable<Eff>) => Effect<Fiber<Array<Effect.Success<Eff>>, Effect.Error<Eff>>, never, Effect.Context<Eff>>;
    (options: {
        readonly discard: true;
    }): <Eff extends Effect<any, any, any>>(effects: Iterable<Eff>) => Effect<void, never, Effect.Context<Eff>>;
    <Eff extends Effect<any, any, any>>(effects: Iterable<Eff>, options?: {
        readonly discard?: false | undefined;
    } | undefined): Effect<Fiber<Array<Effect.Success<Eff>>, Effect.Error<Eff>>, never, Effect.Context<Eff>>;
    <Eff extends Effect<any, any, any>>(effects: Iterable<Eff>, options: {
        readonly discard: true;
    }): Effect<void, never, Effect.Context<Eff>>;
};
declare const forkIn: {
    (scope: Scope): <A, E, R>(self: Effect<A, E, R>) => Effect<RuntimeFiber<A, E>, never, R>;
    <A, E, R>(self: Effect<A, E, R>, scope: Scope): Effect<RuntimeFiber<A, E>, never, R>;
};
declare const forkScoped: <A, E, R>(self: Effect<A, E, R>) => Effect<RuntimeFiber<A, E>, never, Scope | R>;
declare const forkWithErrorHandler: {
    <E, X>(handler: (e: E) => Effect<X>): <A, R>(self: Effect<A, E, R>) => Effect<RuntimeFiber<A, E>, never, R>;
    <A, E, R, X>(self: Effect<A, E, R>, handler: (e: E) => Effect<X>): Effect<RuntimeFiber<A, E>, never, R>;
};
declare const fromFiber: <A, E>(fiber: Fiber<A, E>) => Effect<A, E>;
declare const fromFiberEffect: <A, E, R>(fiber: Effect<Fiber<A, E>, E, R>) => Effect<A, E, R>;
declare const supervised: {
    <X>(supervisor: Supervisor<X>): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
    <A, E, R, X>(self: Effect<A, E, R>, supervisor: Supervisor<X>): Effect<A, E, R>;
};
declare const transplant: <A, E, R>(f: (grafter: <A2, E2, R2>(effect: Effect<A2, E2, R2>) => Effect<A2, E2, R2>) => Effect<A, E, R>) => Effect<A, E, R>;
declare const withConcurrency: {
    (concurrency: number | "unbounded"): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
    <A, E, R>(self: Effect<A, E, R>, concurrency: number | "unbounded"): Effect<A, E, R>;
};
declare const withScheduler: {
    (scheduler: Scheduler): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
    <A, E, R>(self: Effect<A, E, R>, scheduler: Scheduler): Effect<A, E, R>;
};
declare const withSchedulingPriority: {
    (priority: number): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
    <A, E, R>(self: Effect<A, E, R>, priority: number): Effect<A, E, R>;
};
declare const withMaxOpsBeforeYield: {
    (priority: number): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
    <A, E, R>(self: Effect<A, E, R>, priority: number): Effect<A, E, R>;
};
declare const clock: Effect<Clock>;
declare const clockWith: <A, E, R>(f: (clock: Clock) => Effect<A, E, R>) => Effect<A, E, R>;
declare const withClockScoped: <C extends Clock>(clock: C) => Effect<void, never, Scope>;
declare const withClock: {
    <C extends Clock>(clock: C): <A, E, R>(effect: Effect<A, E, R>) => Effect<A, E, R>;
    <C extends Clock, A, E, R>(effect: Effect<A, E, R>, clock: C): Effect<A, E, R>;
};
declare const console: Effect<Console>;
declare const consoleWith: <A, E, R>(f: (console: Console) => Effect<A, E, R>) => Effect<A, E, R>;
declare const withConsoleScoped: <A extends Console>(console: A) => Effect<void, never, Scope>;
declare const withConsole: {
    <C extends Console>(console: C): <A, E, R>(effect: Effect<A, E, R>) => Effect<A, E, R>;
    <A, E, R, C extends Console>(effect: Effect<A, E, R>, console: C): Effect<A, E, R>;
};
declare const delay: {
    (duration: DurationInput): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
    <A, E, R>(self: Effect<A, E, R>, duration: DurationInput): Effect<A, E, R>;
};
declare const sleep: (duration: DurationInput) => Effect<void>;
declare const timed: <A, E, R>(self: Effect<A, E, R>) => Effect<[
    duration: Duration,
    result: A
], E, R>;
declare const timedWith: {
    <E1, R1>(nanoseconds: Effect<bigint, E1, R1>): <A, E, R>(self: Effect<A, E, R>) => Effect<[
        Duration,
        A
    ], E1 | E, R1 | R>;
    <A, E, R, E1, R1>(self: Effect<A, E, R>, nanoseconds: Effect<bigint, E1, R1>): Effect<[
        Duration,
        A
    ], E | E1, R | R1>;
};
declare const timeout: {
    (duration: DurationInput): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E | TimeoutException, R>;
    <A, E, R>(self: Effect<A, E, R>, duration: DurationInput): Effect<A, TimeoutException | E, R>;
};
declare const timeoutOption: {
    (duration: DurationInput): <A, E, R>(self: Effect<A, E, R>) => Effect<Option<A>, E, R>;
    <A, E, R>(self: Effect<A, E, R>, duration: DurationInput): Effect<Option<A>, E, R>;
};
declare const timeoutFail: {
    <E1>(options: {
        readonly onTimeout: LazyArg<E1>;
        readonly duration: DurationInput;
    }): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E1 | E, R>;
    <A, E, R, E1>(self: Effect<A, E, R>, options: {
        readonly onTimeout: LazyArg<E1>;
        readonly duration: DurationInput;
    }): Effect<A, E | E1, R>;
};
declare const timeoutFailCause: {
    <E1>(options: {
        readonly onTimeout: LazyArg<Cause<E1>>;
        readonly duration: DurationInput;
    }): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E1 | E, R>;
    <A, E, R, E1>(self: Effect<A, E, R>, options: {
        readonly onTimeout: LazyArg<Cause<E1>>;
        readonly duration: DurationInput;
    }): Effect<A, E | E1, R>;
};
declare const timeoutTo: {
    <A, B, B1>(options: {
        readonly onTimeout: LazyArg<B1>;
        readonly onSuccess: (a: A) => B;
        readonly duration: DurationInput;
    }): <E, R>(self: Effect<A, E, R>) => Effect<B | B1, E, R>;
    <A, E, R, B1, B>(self: Effect<A, E, R>, options: {
        readonly onTimeout: LazyArg<B1>;
        readonly onSuccess: (a: A) => B;
        readonly duration: DurationInput;
    }): Effect<B1 | B, E, R>;
};
declare const configProviderWith: <A, E, R>(f: (provider: ConfigProvider) => Effect<A, E, R>) => Effect<A, E, R>;
declare const withConfigProvider: {
    (provider: ConfigProvider): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
    <A, E, R>(self: Effect<A, E, R>, provider: ConfigProvider): Effect<A, E, R>;
};
declare const withConfigProviderScoped: (provider: ConfigProvider) => Effect<void, never, Scope>;
declare const context: <R>() => Effect<Context<R>, never, R>;
declare const contextWith: <R, A>(f: (context: Context<R>) => A) => Effect<A, never, R>;
declare const contextWithEffect: <R2, A, E, R>(f: (context: Context<R2>) => Effect<A, E, R>) => Effect<A, E, R | R2>;
declare const mapInputContext: {
    <R2, R>(f: (context: Context<R2>) => Context<R>): <A, E>(self: Effect<A, E, R>) => Effect<A, E, R2>;
    <A, E, R, R2>(self: Effect<A, E, R>, f: (context: Context<R2>) => Context<R>): Effect<A, E, R2>;
};
declare const provide: {
    <const Layers extends readonly [
        Layer.Any,
        ...Array<Layer.Any>
    ]>(layers: Layers): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E | {
        [k in keyof Layers]: Layer.Error<Layers[k]>;
    }[number], {
        [k in keyof Layers]: Layer.Context<Layers[k]>;
    }[number] | Exclude<R, {
        [k in keyof Layers]: Layer.Success<Layers[k]>;
    }[number]>>;
    <ROut, E2, RIn>(layer: Layer<ROut, E2, RIn>): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E | E2, RIn | Exclude<R, ROut>>;
    <R2>(context: Context<R2>): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, Exclude<R, R2>>;
    <R2>(runtime: Runtime<R2>): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, Exclude<R, R2>>;
    <E2, R2>(managedRuntime: ManagedRuntime<R2, E2>): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E | E2, Exclude<R, R2>>;
    <A, E, R, const Layers extends readonly [
        Layer.Any,
        ...Array<Layer.Any>
    ]>(self: Effect<A, E, R>, layers: Layers): Effect<A, E | {
        [k in keyof Layers]: Layer.Error<Layers[k]>;
    }[number], {
        [k in keyof Layers]: Layer.Context<Layers[k]>;
    }[number] | Exclude<R, {
        [k in keyof Layers]: Layer.Success<Layers[k]>;
    }[number]>>;
    <A, E, R, ROut, E2, RIn>(self: Effect<A, E, R>, layer: Layer<ROut, E2, RIn>): Effect<A, E | E2, RIn | Exclude<R, ROut>>;
    <A, E, R, R2>(self: Effect<A, E, R>, context: Context<R2>): Effect<A, E, Exclude<R, R2>>;
    <A, E, R, R2>(self: Effect<A, E, R>, runtime: Runtime<R2>): Effect<A, E, Exclude<R, R2>>;
    <A, E, E2, R, R2>(self: Effect<A, E, R>, runtime: ManagedRuntime<R2, E2>): Effect<A, E | E2, Exclude<R, R2>>;
};
declare const provideService: {
    <I, S>(tag: Tag$1<I, S>, service: NoInfer$1<S>): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, Exclude<R, I>>;
    <A, E, R, I, S>(self: Effect<A, E, R>, tag: Tag$1<I, S>, service: NoInfer$1<S>): Effect<A, E, Exclude<R, I>>;
};
declare const provideServiceEffect: {
    <I, S, E1, R1>(tag: Tag$1<I, S>, effect: Effect<NoInfer$1<S>, E1, R1>): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E | E1, R1 | Exclude<R, I>>;
    <A, E, R, I, S, E1, R1>(self: Effect<A, E, R>, tag: Tag$1<I, S>, effect: Effect<NoInfer$1<S>, E1, R1>): Effect<A, E | E1, R1 | Exclude<R, I>>;
};
declare const serviceFunction: <T extends Effect<any, any, any>, Args extends Array<any>, A>(getService: T, f: (_: Effect.Success<T>) => (...args: Args) => A) => (...args: Args) => Effect<A, Effect.Error<T>, Effect.Context<T>>;
declare const serviceFunctionEffect: <T extends Effect<any, any, any>, Args extends Array<any>, A, E, R>(getService: T, f: (_: Effect.Success<T>) => (...args: Args) => Effect<A, E, R>) => (...args: Args) => Effect<A, E | Effect.Error<T>, R | Effect.Context<T>>;
declare const serviceFunctions: <S, SE, SR>(getService: Effect<S, SE, SR>) => {
    [k in keyof S as S[k] extends (...args: Array<any>) => Effect<any, any, any> ? k : never]: S[k] extends (...args: infer Args) => Effect<infer A, infer E, infer R> ? (...args: Args) => Effect<A, SE | E, SR | R> : never;
};
declare const serviceConstants: <S, SE, SR>(getService: Effect<S, SE, SR>) => {
    [k in {
        [k in keyof S]: k;
    }[keyof S]]: S[k] extends Effect<infer A, infer E, infer R> ? Effect<A, SE | E, SR | R> : Effect<S[k], SE, SR>;
};
declare const serviceMembers: <S, SE, SR>(getService: Effect<S, SE, SR>) => {
    functions: {
        [k in keyof S as S[k] extends (...args: Array<any>) => Effect<any, any, any> ? k : never]: S[k] extends (...args: infer Args) => Effect<infer A, infer E, infer R> ? (...args: Args) => Effect<A, SE | E, SR | R> : never;
    };
    constants: {
        [k in {
            [k in keyof S]: k;
        }[keyof S]]: S[k] extends Effect<infer A, infer E, infer R> ? Effect<A, SE | E, SR | R> : Effect<S[k], SE, SR>;
    };
};
declare const serviceOption: <I, S>(tag: Tag$1<I, S>) => Effect<Option<S>>;
declare const serviceOptional: <I, S>(tag: Tag$1<I, S>) => Effect<S, NoSuchElementException>;
declare const updateService: {
    <I, S>(tag: Tag$1<I, S>, f: (service: NoInfer$1<S>) => NoInfer$1<S>): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R | I>;
    <A, E, R, I, S>(self: Effect<A, E, R>, tag: Tag$1<I, S>, f: (service: NoInfer$1<S>) => NoInfer$1<S>): Effect<A, E, R | I>;
};
declare const Do$1: Effect<{}>;
declare const bind$1: {
    <N extends string, A extends object, B, E2, R2>(name: Exclude<N, keyof A>, f: (a: NoInfer$1<A>) => Effect<B, E2, R2>): <E1, R1>(self: Effect<A, E1, R1>) => Effect<{
        [K in N | keyof A]: K extends keyof A ? A[K] : B;
    }, E2 | E1, R2 | R1>;
    <A extends object, N extends string, E1, R1, B, E2, R2>(self: Effect<A, E1, R1>, name: Exclude<N, keyof A>, f: (a: NoInfer$1<A>) => Effect<B, E2, R2>): Effect<{
        [K in N | keyof A]: K extends keyof A ? A[K] : B;
    }, E1 | E2, R1 | R2>;
};
declare const bindAll: {
    <A extends object, X extends Record<string, Effect<any, any, any>>, O extends NoExcessProperties<{
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly mode?: "default" | "validate" | "either" | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    }, O>>(f: (a: NoInfer$1<A>) => [
        Extract<keyof X, keyof A>
    ] extends [
        never
    ] ? X : `Duplicate keys`, options?: undefined | O): <E1, R1>(self: Effect<A, E1, R1>) => [
        All.ReturnObject<X, false, All.ExtractMode<O>>
    ] extends [
        Effect<infer Success, infer Error, infer Context>
    ] ? Effect<{
        [K in keyof A | keyof Success]: K extends keyof A ? A[K] : K extends keyof Success ? Success[K] : never;
    }, E1 | Error, R1 | Context> : never;
    <A extends object, X extends Record<string, Effect<any, any, any>>, O extends NoExcessProperties<{
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly mode?: "default" | "validate" | "either" | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    }, O>, E1, R1>(self: Effect<A, E1, R1>, f: (a: NoInfer$1<A>) => [
        Extract<keyof X, keyof A>
    ] extends [
        never
    ] ? X : `Duplicate keys`, options?: undefined | {
        readonly concurrency?: Concurrency | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly mode?: "default" | "validate" | "either" | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    }): [
        All.ReturnObject<X, false, All.ExtractMode<O>>
    ] extends [
        Effect<infer Success, infer Error, infer Context>
    ] ? Effect<{
        [K in keyof A | keyof Success]: K extends keyof A ? A[K] : K extends keyof Success ? Success[K] : never;
    }, E1 | Error, R1 | Context> : never;
};
declare const bindTo$1: {
    <N extends string>(name: N): <A, E, R>(self: Effect<A, E, R>) => Effect<{
        [K in N]: A;
    }, E, R>;
    <A, E, R, N extends string>(self: Effect<A, E, R>, name: N): Effect<{
        [K in N]: A;
    }, E, R>;
};
declare const let_$1: {
    <N extends string, A extends object, B>(name: Exclude<N, keyof A>, f: (a: NoInfer$1<A>) => B): <E, R>(self: Effect<A, E, R>) => Effect<{
        [K in N | keyof A]: K extends keyof A ? A[K] : B;
    }, E, R>;
    <A extends object, N extends string, E, R, B>(self: Effect<A, E, R>, name: Exclude<N, keyof A>, f: (a: NoInfer$1<A>) => B): Effect<{
        [K in N | keyof A]: K extends keyof A ? A[K] : B;
    }, E, R>;
};
declare const option$1: <A, E, R>(self: Effect<A, E, R>) => Effect<Option<A>, never, R>;
declare const either$1: <A, E, R>(self: Effect<A, E, R>) => Effect<Either<A, E>, never, R>;
declare const exit: <A, E, R>(self: Effect<A, E, R>) => Effect<Exit<A, E>, never, R>;
declare const intoDeferred: {
    <A, E>(deferred: Deferred<A, E>): <R>(self: Effect<A, E, R>) => Effect<boolean, never, R>;
    <A, E, R>(self: Effect<A, E, R>, deferred: Deferred<A, E>): Effect<boolean, never, R>;
};
declare const if_: {
    <A1, E1, R1, A2, E2, R2>(options: {
        readonly onTrue: LazyArg<Effect<A1, E1, R1>>;
        readonly onFalse: LazyArg<Effect<A2, E2, R2>>;
    }): <E = never, R = never>(self: boolean | Effect<boolean, E, R>) => Effect<A1 | A2, E1 | E2 | E, R1 | R2 | R>;
    <A1, E1, R1, A2, E2, R2, E = never, R = never>(self: boolean | Effect<boolean, E, R>, options: {
        readonly onTrue: LazyArg<Effect<A1, E1, R1>>;
        readonly onFalse: LazyArg<Effect<A2, E2, R2>>;
    }): Effect<A1 | A2, E1 | E2 | E, R1 | R2 | R>;
};
declare const filterOrDie: {
    <A, B extends A>(refinement: Refinement<NoInfer$1<A>, B>, orDieWith: (a: EqualsWith<A, B, A, Exclude<A, B>>) => unknown): <E, R>(self: Effect<A, E, R>) => Effect<B, E, R>;
    <A>(predicate: Predicate<NoInfer$1<A>>, orDieWith: (a: NoInfer$1<A>) => unknown): <E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
    <A, E, R, B extends A>(self: Effect<A, E, R>, refinement: Refinement<A, B>, orDieWith: (a: EqualsWith<A, B, A, Exclude<A, B>>) => unknown): Effect<B, E, R>;
    <A, E, R>(self: Effect<A, E, R>, predicate: Predicate<A>, orDieWith: (a: A) => unknown): Effect<A, E, R>;
};
declare const filterOrDieMessage: {
    <A, B extends A>(refinement: Refinement<NoInfer$1<A>, B>, message: string): <E, R>(self: Effect<A, E, R>) => Effect<B, E, R>;
    <A>(predicate: Predicate<NoInfer$1<A>>, message: string): <E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
    <A, E, R, B extends A>(self: Effect<A, E, R>, refinement: Refinement<A, B>, message: string): Effect<B, E, R>;
    <A, E, R>(self: Effect<A, E, R>, predicate: Predicate<A>, message: string): Effect<A, E, R>;
};
declare const filterOrElse: {
    <A, C, E2, R2, B extends A>(refinement: Refinement<NoInfer$1<A>, B>, orElse: (a: EqualsWith<A, B, NoInfer$1<A>, Exclude<NoInfer$1<A>, B>>) => Effect<C, E2, R2>): <E, R>(self: Effect<A, E, R>) => Effect<B | C, E2 | E, R2 | R>;
    <A, C, E2, R2>(predicate: Predicate<NoInfer$1<A>>, orElse: (a: NoInfer$1<A>) => Effect<C, E2, R2>): <E, R>(self: Effect<A, E, R>) => Effect<A | C, E2 | E, R2 | R>;
    <A, E, R, C, E2, R2, B extends A>(self: Effect<A, E, R>, refinement: Refinement<A, B>, orElse: (a: EqualsWith<A, B, A, Exclude<A, B>>) => Effect<C, E2, R2>): Effect<B | C, E | E2, R | R2>;
    <A, E, R, C, E2, R2>(self: Effect<A, E, R>, predicate: Predicate<A>, orElse: (a: A) => Effect<C, E2, R2>): Effect<A | C, E | E2, R | R2>;
};
declare const filterOrFail: {
    <A, E2, B extends A>(refinement: Refinement<NoInfer$1<A>, B>, orFailWith: (a: EqualsWith<A, B, NoInfer$1<A>, Exclude<NoInfer$1<A>, B>>) => E2): <E, R>(self: Effect<A, E, R>) => Effect<NoInfer$1<B>, E2 | E, R>;
    <A, E2>(predicate: Predicate<NoInfer$1<A>>, orFailWith: (a: NoInfer$1<A>) => E2): <E, R>(self: Effect<A, E, R>) => Effect<A, E2 | E, R>;
    <A, E, R, E2, B extends A>(self: Effect<A, E, R>, refinement: Refinement<A, B>, orFailWith: (a: EqualsWith<A, B, A, Exclude<A, B>>) => E2): Effect<NoInfer$1<B>, E2 | E, R>;
    <A, E, R, E2>(self: Effect<A, E, R>, predicate: Predicate<A>, orFailWith: (a: A) => E2): Effect<A, E2 | E, R>;
    <A, B extends A>(refinement: Refinement<NoInfer$1<A>, B>): <E, R>(self: Effect<A, E, R>) => Effect<NoInfer$1<B>, NoSuchElementException | E, R>;
    <A>(predicate: Predicate<NoInfer$1<A>>): <E, R>(self: Effect<A, E, R>) => Effect<A, NoSuchElementException | E, R>;
    <A, E, R, B extends A>(self: Effect<A, E, R>, refinement: Refinement<A, B>): Effect<NoInfer$1<B>, E | NoSuchElementException, R>;
    <A, E, R>(self: Effect<A, E, R>, predicate: Predicate<A>): Effect<A, E | NoSuchElementException, R>;
};
declare const filterEffectOrElse: {
    <A, E2, R2, A2, E3, R3>(options: {
        readonly predicate: (a: NoInfer$1<A>) => Effect<boolean, E2, R2>;
        readonly orElse: (a: NoInfer$1<A>) => Effect<A2, E3, R3>;
    }): <E, R>(self: Effect<A, E, R>) => Effect<A | A2, E | E2 | E3, R | R2 | R3>;
    <A, E, R, E2, R2, A2, E3, R3>(self: Effect<A, E, R>, options: {
        readonly predicate: (a: A) => Effect<boolean, E2, R2>;
        readonly orElse: (a: A) => Effect<A2, E3, R3>;
    }): Effect<A | A2, E | E2 | E3, R | R2 | R3>;
};
declare const filterEffectOrFail: {
    <A, E2, R2, E3>(options: {
        readonly predicate: (a: NoInfer$1<A>) => Effect<boolean, E2, R2>;
        readonly orFailWith: (a: NoInfer$1<A>) => E3;
    }): <E, R>(self: Effect<A, E, R>) => Effect<A, E | E2 | E3, R | R2>;
    <A, E, R, E2, R2, E3>(self: Effect<A, E, R>, options: {
        readonly predicate: (a: A) => Effect<boolean, E2, R2>;
        readonly orFailWith: (a: A) => E3;
    }): Effect<A, E | E2 | E3, R | R2>;
};
declare const unless: {
    (condition: LazyArg<boolean>): <A, E, R>(self: Effect<A, E, R>) => Effect<Option<A>, E, R>;
    <A, E, R>(self: Effect<A, E, R>, condition: LazyArg<boolean>): Effect<Option<A>, E, R>;
};
declare const unlessEffect: {
    <E2, R2>(condition: Effect<boolean, E2, R2>): <A, E, R>(self: Effect<A, E, R>) => Effect<Option<A>, E2 | E, R2 | R>;
    <A, E, R, E2, R2>(self: Effect<A, E, R>, condition: Effect<boolean, E2, R2>): Effect<Option<A>, E | E2, R | R2>;
};
declare const when$1: {
    (condition: LazyArg<boolean>): <A, E, R>(self: Effect<A, E, R>) => Effect<Option<A>, E, R>;
    <A, E, R>(self: Effect<A, E, R>, condition: LazyArg<boolean>): Effect<Option<A>, E, R>;
};
declare const whenEffect: {
    <E, R>(condition: Effect<boolean, E, R>): <A, E2, R2>(effect: Effect<A, E2, R2>) => Effect<Option<A>, E | E2, R | R2>;
    <A, E2, R2, E, R>(self: Effect<A, E2, R2>, condition: Effect<boolean, E, R>): Effect<Option<A>, E2 | E, R2 | R>;
};
declare const whenFiberRef: {
    <S>(fiberRef: FiberRef<S>, predicate: Predicate<S>): <A, E, R>(self: Effect<A, E, R>) => Effect<[
        S,
        Option<A>
    ], E, R>;
    <A, E, R, S>(self: Effect<A, E, R>, fiberRef: FiberRef<S>, predicate: Predicate<S>): Effect<[
        S,
        Option<A>
    ], E, R>;
};
declare const whenRef: {
    <S>(ref: Ref<S>, predicate: Predicate<S>): <A, E, R>(self: Effect<A, E, R>) => Effect<[
        S,
        Option<A>
    ], E, R>;
    <A, E, R, S>(self: Effect<A, E, R>, ref: Ref<S>, predicate: Predicate<S>): Effect<[
        S,
        Option<A>
    ], E, R>;
};
declare const flatMap$1: {
    <A, B, E1, R1>(f: (a: A) => Effect<B, E1, R1>): <E, R>(self: Effect<A, E, R>) => Effect<B, E1 | E, R1 | R>;
    <A, E, R, B, E1, R1>(self: Effect<A, E, R>, f: (a: A) => Effect<B, E1, R1>): Effect<B, E | E1, R | R1>;
};
declare const andThen$1: {
    <A, X>(f: (a: NoInfer$1<A>) => X): <E, R>(self: Effect<A, E, R>) => [
        X
    ] extends [
        Effect<infer A1, infer E1, infer R1>
    ] ? Effect<A1, E | E1, R | R1> : [
        X
    ] extends [
        PromiseLike<infer A1>
    ] ? Effect<A1, E | UnknownException, R> : Effect<X, E, R>;
    <X>(f: NotFunction<X>): <A, E, R>(self: Effect<A, E, R>) => [
        X
    ] extends [
        Effect<infer A1, infer E1, infer R1>
    ] ? Effect<A1, E | E1, R | R1> : [
        X
    ] extends [
        PromiseLike<infer A1>
    ] ? Effect<A1, E | UnknownException, R> : Effect<X, E, R>;
    <A, E, R, X>(self: Effect<A, E, R>, f: (a: NoInfer$1<A>) => X): [
        X
    ] extends [
        Effect<infer A1, infer E1, infer R1>
    ] ? Effect<A1, E | E1, R | R1> : [
        X
    ] extends [
        PromiseLike<infer A1>
    ] ? Effect<A1, E | UnknownException, R> : Effect<X, E, R>;
    <A, E, R, X>(self: Effect<A, E, R>, f: NotFunction<X>): [
        X
    ] extends [
        Effect<infer A1, infer E1, infer R1>
    ] ? Effect<A1, E | E1, R | R1> : [
        X
    ] extends [
        PromiseLike<infer A1>
    ] ? Effect<A1, E | UnknownException, R> : Effect<X, E, R>;
};
declare const flatten: <A, E1, R1, E, R>(self: Effect<Effect<A, E1, R1>, E, R>) => Effect<A, E | E1, R | R1>;
declare const race: {
    <A2, E2, R2>(that: Effect<A2, E2, R2>): <A, E, R>(self: Effect<A, E, R>) => Effect<A2 | A, E2 | E, R2 | R>;
    <A, E, R, A2, E2, R2>(self: Effect<A, E, R>, that: Effect<A2, E2, R2>): Effect<A | A2, E | E2, R | R2>;
};
declare const raceAll: <Eff extends Effect<any, any, any>>(all: Iterable<Eff>) => Effect<Effect.Success<Eff>, Effect.Error<Eff>, Effect.Context<Eff>>;
declare const raceFirst: {
    <A2, E2, R2>(that: Effect<A2, E2, R2>): <A, E, R>(self: Effect<A, E, R>) => Effect<A2 | A, E2 | E, R2 | R>;
    <A, E, R, A2, E2, R2>(self: Effect<A, E, R>, that: Effect<A2, E2, R2>): Effect<A | A2, E | E2, R | R2>;
};
declare const raceWith: {
    <A1, E1, R1, E, A, A2, E2, R2, A3, E3, R3>(other: Effect<A1, E1, R1>, options: {
        readonly onSelfDone: (exit: Exit<A, E>, fiber: Fiber<A1, E1>) => Effect<A2, E2, R2>;
        readonly onOtherDone: (exit: Exit<A1, E1>, fiber: Fiber<A, E>) => Effect<A3, E3, R3>;
    }): <R>(self: Effect<A, E, R>) => Effect<A2 | A3, E2 | E3, R1 | R2 | R3 | R>;
    <A, E, R, A1, E1, R1, A2, E2, R2, A3, E3, R3>(self: Effect<A, E, R>, other: Effect<A1, E1, R1>, options: {
        readonly onSelfDone: (exit: Exit<A, E>, fiber: Fiber<A1, E1>) => Effect<A2, E2, R2>;
        readonly onOtherDone: (exit: Exit<A1, E1>, fiber: Fiber<A, E>) => Effect<A3, E3, R3>;
    }): Effect<A2 | A3, E2 | E3, R | R1 | R2 | R3>;
};
declare const summarized: {
    <B, E2, R2, C>(summary: Effect<B, E2, R2>, f: (start: B, end: B) => C): <A, E, R>(self: Effect<A, E, R>) => Effect<[
        C,
        A
    ], E2 | E, R2 | R>;
    <A, E, R, B, E2, R2, C>(self: Effect<A, E, R>, summary: Effect<B, E2, R2>, f: (start: B, end: B) => C): Effect<[
        C,
        A
    ], E2 | E, R2 | R>;
};
declare const tap: {
    <A, X>(f: (a: NoInfer$1<A>) => X): <E, R>(self: Effect<A, E, R>) => [
        X
    ] extends [
        Effect<infer _A1, infer E1, infer R1>
    ] ? Effect<A, E | E1, R | R1> : [
        X
    ] extends [
        PromiseLike<infer _A1>
    ] ? Effect<A, E | UnknownException, R> : Effect<A, E, R>;
    <A, X, E1, R1>(f: (a: NoInfer$1<A>) => Effect<X, E1, R1>, options: {
        onlyEffect: true;
    }): <E, R>(self: Effect<A, E, R>) => Effect<A, E | E1, R | R1>;
    <X>(f: NotFunction<X>): <A, E, R>(self: Effect<A, E, R>) => [
        X
    ] extends [
        Effect<infer _A1, infer E1, infer R1>
    ] ? Effect<A, E | E1, R | R1> : [
        X
    ] extends [
        PromiseLike<infer _A1>
    ] ? Effect<A, E | UnknownException, R> : Effect<A, E, R>;
    <X, E1, R1>(f: Effect<X, E1, R1>, options: {
        onlyEffect: true;
    }): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E | E1, R | R1>;
    <A, E, R, X>(self: Effect<A, E, R>, f: (a: NoInfer$1<A>) => X): [
        X
    ] extends [
        Effect<infer _A1, infer E1, infer R1>
    ] ? Effect<A, E | E1, R | R1> : [
        X
    ] extends [
        PromiseLike<infer _A1>
    ] ? Effect<A, E | UnknownException, R> : Effect<A, E, R>;
    <A, E, R, X, E1, R1>(self: Effect<A, E, R>, f: (a: NoInfer$1<A>) => Effect<X, E1, R1>, options: {
        onlyEffect: true;
    }): Effect<A, E | E1, R | R1>;
    <A, E, R, X>(self: Effect<A, E, R>, f: NotFunction<X>): [
        X
    ] extends [
        Effect<infer _A1, infer E1, infer R1>
    ] ? Effect<A, E | E1, R | R1> : [
        X
    ] extends [
        PromiseLike<infer _A1>
    ] ? Effect<A, E | UnknownException, R> : Effect<A, E, R>;
    <A, E, R, X, E1, R1>(self: Effect<A, E, R>, f: Effect<X, E1, R1>, options: {
        onlyEffect: true;
    }): Effect<A, E | E1, R | R1>;
};
declare const tapBoth: {
    <E, X, E2, R2, A, X1, E3, R3>(options: {
        readonly onFailure: (e: NoInfer$1<E>) => Effect<X, E2, R2>;
        readonly onSuccess: (a: NoInfer$1<A>) => Effect<X1, E3, R3>;
    }): <R>(self: Effect<A, E, R>) => Effect<A, E | E2 | E3, R2 | R3 | R>;
    <A, E, R, X, E2, R2, X1, E3, R3>(self: Effect<A, E, R>, options: {
        readonly onFailure: (e: E) => Effect<X, E2, R2>;
        readonly onSuccess: (a: A) => Effect<X1, E3, R3>;
    }): Effect<A, E | E2 | E3, R | R2 | R3>;
};
declare const tapDefect: {
    <X, E2, R2>(f: (cause: Cause<never>) => Effect<X, E2, R2>): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E2 | E, R2 | R>;
    <A, E, R, X, E2, R2>(self: Effect<A, E, R>, f: (cause: Cause<never>) => Effect<X, E2, R2>): Effect<A, E | E2, R | R2>;
};
declare const tapError: {
    <E, X, E2, R2>(f: (e: NoInfer$1<E>) => Effect<X, E2, R2>): <A, R>(self: Effect<A, E, R>) => Effect<A, E | E2, R2 | R>;
    <A, E, R, X, E2, R2>(self: Effect<A, E, R>, f: (e: E) => Effect<X, E2, R2>): Effect<A, E | E2, R | R2>;
};
declare const tapErrorTag: {
    <K extends E extends {
        _tag: string;
    } ? E["_tag"] : never, E, A1, E1, R1>(k: K, f: (e: NoInfer$1<Extract<E, {
        _tag: K;
    }>>) => Effect<A1, E1, R1>): <A, R>(self: Effect<A, E, R>) => Effect<A, E | E1, R1 | R>;
    <A, E, R, K extends E extends {
        _tag: string;
    } ? E["_tag"] : never, A1, E1, R1>(self: Effect<A, E, R>, k: K, f: (e: Extract<E, {
        _tag: K;
    }>) => Effect<A1, E1, R1>): Effect<A, E | E1, R | R1>;
};
declare const tapErrorCause: {
    <E, X, E2, R2>(f: (cause: Cause<NoInfer$1<E>>) => Effect<X, E2, R2>): <A, R>(self: Effect<A, E, R>) => Effect<A, E | E2, R2 | R>;
    <A, E, R, X, E2, R2>(self: Effect<A, E, R>, f: (cause: Cause<E>) => Effect<X, E2, R2>): Effect<A, E | E2, R | R2>;
};
declare const forever: <A, E, R>(self: Effect<A, E, R>) => Effect<never, E, R>;
declare const iterate: {
    <A, B extends A, R, E>(initial: A, options: {
        readonly while: Refinement<A, B>;
        readonly body: (b: B) => Effect<A, E, R>;
    }): Effect<A, E, R>;
    <A, R, E>(initial: A, options: {
        readonly while: Predicate<A>;
        readonly body: (a: A) => Effect<A, E, R>;
    }): Effect<A, E, R>;
};
declare const loop: {
    <A, B extends A, C, E, R>(initial: A, options: {
        readonly while: Refinement<A, B>;
        readonly step: (b: B) => A;
        readonly body: (b: B) => Effect<C, E, R>;
        readonly discard?: false | undefined;
    }): Effect<Array<C>, E, R>;
    <A, C, E, R>(initial: A, options: {
        readonly while: (a: A) => boolean;
        readonly step: (a: A) => A;
        readonly body: (a: A) => Effect<C, E, R>;
        readonly discard?: false | undefined;
    }): Effect<Array<C>, E, R>;
    <A, B extends A, C, E, R>(initial: A, options: {
        readonly while: Refinement<A, B>;
        readonly step: (b: B) => A;
        readonly body: (b: B) => Effect<C, E, R>;
        readonly discard: true;
    }): Effect<void, E, R>;
    <A, C, E, R>(initial: A, options: {
        readonly while: (a: A) => boolean;
        readonly step: (a: A) => A;
        readonly body: (a: A) => Effect<C, E, R>;
        readonly discard: true;
    }): Effect<void, E, R>;
};
declare namespace Repeat {
    type Return<R, E, A, O extends NoExcessProperties<Options<A>, O>> = Effect<(O extends {
        schedule: Schedule<infer Out, infer _I, infer _R>;
    } ? Out : O extends {
        until: Refinement<A, infer B>;
    } ? B : A), E | (O extends {
        while: (...args: Array<any>) => Effect<infer _A, infer E, infer _R>;
    } ? E : never) | (O extends {
        until: (...args: Array<any>) => Effect<infer _A, infer E, infer _R>;
    } ? E : never), R | (O extends {
        schedule: Schedule<infer _O, infer _I, infer R>;
    } ? R : never) | (O extends {
        while: (...args: Array<any>) => Effect<infer _A, infer _E, infer R>;
    } ? R : never) | (O extends {
        until: (...args: Array<any>) => Effect<infer _A, infer _E, infer R>;
    } ? R : never)> extends infer Z ? Z : never;
    interface Options<A> {
        while?: ((_: A) => boolean | Effect<boolean, any, any>) | undefined;
        until?: ((_: A) => boolean | Effect<boolean, any, any>) | undefined;
        times?: number | undefined;
        schedule?: Schedule<any, A, any> | undefined;
    }
}
declare const repeat: {
    <O extends NoExcessProperties<Repeat.Options<A>, O>, A>(options: O): <E, R>(self: Effect<A, E, R>) => Repeat.Return<R, E, A, O>;
    <B, A, R1>(schedule: Schedule<B, A, R1>): <E, R>(self: Effect<A, E, R>) => Effect<B, E, R1 | R>;
    <A, E, R, O extends NoExcessProperties<Repeat.Options<A>, O>>(self: Effect<A, E, R>, options: O): Repeat.Return<R, E, A, O>;
    <A, E, R, B, R1>(self: Effect<A, E, R>, schedule: Schedule<B, A, R1>): Effect<B, E, R | R1>;
};
declare const repeatN: {
    (n: number): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
    <A, E, R>(self: Effect<A, E, R>, n: number): Effect<A, E, R>;
};
declare const repeatOrElse: {
    <R2, A, B, E, E2, R3>(schedule: Schedule<B, A, R2>, orElse: (error: E, option: Option<B>) => Effect<B, E2, R3>): <R>(self: Effect<A, E, R>) => Effect<B, E2, R2 | R3 | R>;
    <A, E, R, R2, B, E2, R3>(self: Effect<A, E, R>, schedule: Schedule<B, A, R2>, orElse: (error: E, option: Option<B>) => Effect<B, E2, R3>): Effect<B, E2, R | R2 | R3>;
};
declare const schedule: {
    <A, R2, Out>(schedule: Schedule<Out, NoInfer$1<A> | undefined, R2>): <E, R>(self: Effect<A, E, R>) => Effect<Out, E, R2 | R>;
    <A, E, R, R2, Out>(self: Effect<A, E, R>, schedule: Schedule<Out, A | undefined, R2>): Effect<Out, E, R | R2>;
};
declare const scheduleForked: {
    <Out, R2>(schedule: Schedule<Out, unknown, R2>): <A, E, R>(self: Effect<A, E, R>) => Effect<RuntimeFiber<Out, E>, never, Scope | R2 | R>;
    <A, E, R, Out, R2>(self: Effect<A, E, R>, schedule: Schedule<Out, unknown, R2>): Effect<RuntimeFiber<Out, E>, never, Scope | R | R2>;
};
declare const scheduleFrom: {
    <R2, In, Out>(initial: In, schedule: Schedule<Out, In, R2>): <E, R>(self: Effect<In, E, R>) => Effect<Out, E, R2 | R>;
    <In, E, R, R2, Out>(self: Effect<In, E, R>, initial: In, schedule: Schedule<Out, In, R2>): Effect<Out, E, R | R2>;
};
declare const whileLoop: <A, E, R>(options: {
    readonly while: LazyArg<boolean>;
    readonly body: LazyArg<Effect<A, E, R>>;
    readonly step: (a: A) => void;
}) => Effect<void, E, R>;
declare const getFiberRefs: Effect<FiberRefs>;
declare const inheritFiberRefs: (childFiberRefs: FiberRefs) => Effect<void>;
declare const locally: {
    <A>(self: FiberRef<A>, value: A): <B, E, R>(use: Effect<B, E, R>) => Effect<B, E, R>;
    <B, E, R, A>(use: Effect<B, E, R>, self: FiberRef<A>, value: A): Effect<B, E, R>;
};
declare const locallyWith: {
    <A>(self: FiberRef<A>, f: (a: A) => A): <B, E, R>(use: Effect<B, E, R>) => Effect<B, E, R>;
    <B, E, R, A>(use: Effect<B, E, R>, self: FiberRef<A>, f: (a: A) => A): Effect<B, E, R>;
};
declare const locallyScoped: {
    <A>(value: A): (self: FiberRef<A>) => Effect<void, never, Scope>;
    <A>(self: FiberRef<A>, value: A): Effect<void, never, Scope>;
};
declare const locallyScopedWith: {
    <A>(f: (a: A) => A): (self: FiberRef<A>) => Effect<void, never, Scope>;
    <A>(self: FiberRef<A>, f: (a: A) => A): Effect<void, never, Scope>;
};
declare const patchFiberRefs: (patch: FiberRefsPatch) => Effect<void>;
declare const setFiberRefs: (fiberRefs: FiberRefs) => Effect<void>;
declare const updateFiberRefs: (f: (fiberId: Runtime$1, fiberRefs: FiberRefs) => FiberRefs) => Effect<void>;
declare const isFailure: <A, E, R>(self: Effect<A, E, R>) => Effect<boolean, never, R>;
declare const isSuccess: <A, E, R>(self: Effect<A, E, R>) => Effect<boolean, never, R>;
declare const match$1: {
    <E, A2, A, A3>(options: {
        readonly onFailure: (error: E) => A2;
        readonly onSuccess: (value: A) => A3;
    }): <R>(self: Effect<A, E, R>) => Effect<A2 | A3, never, R>;
    <A, E, R, A2, A3>(self: Effect<A, E, R>, options: {
        readonly onFailure: (error: E) => A2;
        readonly onSuccess: (value: A) => A3;
    }): Effect<A2 | A3, never, R>;
};
declare const matchCause: {
    <E, A2, A, A3>(options: {
        readonly onFailure: (cause: Cause<E>) => A2;
        readonly onSuccess: (a: A) => A3;
    }): <R>(self: Effect<A, E, R>) => Effect<A2 | A3, never, R>;
    <A, E, R, A2, A3>(self: Effect<A, E, R>, options: {
        readonly onFailure: (cause: Cause<E>) => A2;
        readonly onSuccess: (a: A) => A3;
    }): Effect<A2 | A3, never, R>;
};
declare const matchCauseEffect: {
    <E, A2, E2, R2, A, A3, E3, R3>(options: {
        readonly onFailure: (cause: Cause<E>) => Effect<A2, E2, R2>;
        readonly onSuccess: (a: A) => Effect<A3, E3, R3>;
    }): <R>(self: Effect<A, E, R>) => Effect<A2 | A3, E2 | E3, R2 | R3 | R>;
    <A, E, R, A2, E2, R2, A3, E3, R3>(self: Effect<A, E, R>, options: {
        readonly onFailure: (cause: Cause<E>) => Effect<A2, E2, R2>;
        readonly onSuccess: (a: A) => Effect<A3, E3, R3>;
    }): Effect<A2 | A3, E2 | E3, R2 | R3 | R>;
};
declare const matchEffect: {
    <E, A2, E2, R2, A, A3, E3, R3>(options: {
        readonly onFailure: (e: E) => Effect<A2, E2, R2>;
        readonly onSuccess: (a: A) => Effect<A3, E3, R3>;
    }): <R>(self: Effect<A, E, R>) => Effect<A2 | A3, E2 | E3, R2 | R3 | R>;
    <A, E, R, A2, E2, R2, A3, E3, R3>(self: Effect<A, E, R>, options: {
        readonly onFailure: (e: E) => Effect<A2, E2, R2>;
        readonly onSuccess: (a: A) => Effect<A3, E3, R3>;
    }): Effect<A2 | A3, E2 | E3, R2 | R3 | R>;
};
declare const log: (...message: ReadonlyArray<any>) => Effect<void, never, never>;
declare const logWithLevel: (level: LogLevel, ...message: ReadonlyArray<any>) => Effect<void>;
declare const logTrace: (...message: ReadonlyArray<any>) => Effect<void, never, never>;
declare const logDebug: (...message: ReadonlyArray<any>) => Effect<void, never, never>;
declare const logInfo: (...message: ReadonlyArray<any>) => Effect<void, never, never>;
declare const logWarning: (...message: ReadonlyArray<any>) => Effect<void, never, never>;
declare const logError: (...message: ReadonlyArray<any>) => Effect<void, never, never>;
declare const logFatal: (...message: ReadonlyArray<any>) => Effect<void, never, never>;
declare const withLogSpan: {
    (label: string): <A, E, R>(effect: Effect<A, E, R>) => Effect<A, E, R>;
    <A, E, R>(effect: Effect<A, E, R>, label: string): Effect<A, E, R>;
};
declare const annotateLogs: {
    (key: string, value: unknown): <A, E, R>(effect: Effect<A, E, R>) => Effect<A, E, R>;
    (values: Record<string, unknown>): <A, E, R>(effect: Effect<A, E, R>) => Effect<A, E, R>;
    <A, E, R>(effect: Effect<A, E, R>, key: string, value: unknown): Effect<A, E, R>;
    <A, E, R>(effect: Effect<A, E, R>, values: Record<string, unknown>): Effect<A, E, R>;
};
declare const annotateLogsScoped: {
    (key: string, value: unknown): Effect<void, never, Scope>;
    (values: Record<string, unknown>): Effect<void, never, Scope>;
};
declare const logAnnotations: Effect<HashMap<string, unknown>>;
declare const withUnhandledErrorLogLevel: {
    (level: Option<LogLevel>): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
    <A, E, R>(self: Effect<A, E, R>, level: Option<LogLevel>): Effect<A, E, R>;
};
declare const whenLogLevel: {
    (level: LogLevel | Literal): <A, E, R>(self: Effect<A, E, R>) => Effect<Option<A>, E, R>;
    <A, E, R>(self: Effect<A, E, R>, level: LogLevel | Literal): Effect<Option<A>, E, R>;
};
declare const orDie: <A, E, R>(self: Effect<A, E, R>) => Effect<A, never, R>;
declare const orDieWith: {
    <E>(f: (error: E) => unknown): <A, R>(self: Effect<A, E, R>) => Effect<A, never, R>;
    <A, E, R>(self: Effect<A, E, R>, f: (error: E) => unknown): Effect<A, never, R>;
};
declare const orElse$2: {
    <A2, E2, R2>(that: LazyArg<Effect<A2, E2, R2>>): <A, E, R>(self: Effect<A, E, R>) => Effect<A2 | A, E2, R2 | R>;
    <A, E, R, A2, E2, R2>(self: Effect<A, E, R>, that: LazyArg<Effect<A2, E2, R2>>): Effect<A2 | A, E2, R2 | R>;
};
declare const orElseFail: {
    <E2>(evaluate: LazyArg<E2>): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E2, R>;
    <A, E, R, E2>(self: Effect<A, E, R>, evaluate: LazyArg<E2>): Effect<A, E2, R>;
};
declare const orElseSucceed: {
    <A2>(evaluate: LazyArg<A2>): <A, E, R>(self: Effect<A, E, R>) => Effect<A2 | A, never, R>;
    <A, E, R, A2>(self: Effect<A, E, R>, evaluate: LazyArg<A2>): Effect<A | A2, never, R>;
};
declare const firstSuccessOf: <Eff extends Effect<any, any, any>>(effects: Iterable<Eff>) => Effect<Effect.Success<Eff>, Effect.Error<Eff>, Effect.Context<Eff>>;
declare const random: Effect<Random>;
declare const randomWith: <A, E, R>(f: (random: Random) => Effect<A, E, R>) => Effect<A, E, R>;
declare const withRandom: {
    <X extends Random>(value: X): <A, E, R>(effect: Effect<A, E, R>) => Effect<A, E, R>;
    <X extends Random, A, E, R>(effect: Effect<A, E, R>, value: X): Effect<A, E, R>;
};
declare const withRandomFixed: {
    <T extends NonEmptyArray<any>>(values: T): <A, E, R>(effect: Effect<A, E, R>) => Effect<A, E, R>;
    <T extends NonEmptyArray<any>, A, E, R>(effect: Effect<A, E, R>, values: T): Effect<A, E, R>;
};
declare const withRandomScoped: <A extends Random>(value: A) => Effect<void, never, Scope>;
declare const runtime: <R = never>() => Effect<Runtime<R>, never, R>;
declare const getRuntimeFlags: Effect<RuntimeFlags>;
declare const patchRuntimeFlags: (patch: RuntimeFlagsPatch) => Effect<void>;
declare const withRuntimeFlagsPatch: {
    (update: RuntimeFlagsPatch): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
    <A, E, R>(self: Effect<A, E, R>, update: RuntimeFlagsPatch): Effect<A, E, R>;
};
declare const withRuntimeFlagsPatchScoped: (update: RuntimeFlagsPatch) => Effect<void, never, Scope>;
declare const tagMetrics: {
    (key: string, value: string): <A, E, R>(effect: Effect<A, E, R>) => Effect<A, E, R>;
    (values: Record<string, string>): <A, E, R>(effect: Effect<A, E, R>) => Effect<A, E, R>;
    <A, E, R>(effect: Effect<A, E, R>, key: string, value: string): Effect<A, E, R>;
    <A, E, R>(effect: Effect<A, E, R>, values: Record<string, string>): Effect<A, E, R>;
};
declare const labelMetrics: {
    (labels: Iterable<MetricLabel>): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
    <A, E, R>(self: Effect<A, E, R>, labels: Iterable<MetricLabel>): Effect<A, E, R>;
};
declare const tagMetricsScoped: (key: string, value: string) => Effect<void, never, Scope>;
declare const labelMetricsScoped: (labels: ReadonlyArray<MetricLabel>) => Effect<void, never, Scope>;
declare const metricLabels: Effect<ReadonlyArray<MetricLabel>>;
declare const withMetric: {
    <Type, In, Out>(metric: Metric<Type, In, Out>): <A extends In, E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
    <A extends In, E, R, Type, In, Out>(self: Effect<A, E, R>, metric: Metric<Type, In, Out>): Effect<A, E, R>;
};
interface Permit {
    readonly index: number;
}
interface Semaphore {
    resize(permits: number): Effect<void>;
    withPermits(permits: number): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
    withPermitsIfAvailable(permits: number): <A, E, R>(self: Effect<A, E, R>) => Effect<Option<A>, E, R>;
    take(permits: number): Effect<number>;
    release(permits: number): Effect<number>;
    releaseAll: Effect<number>;
}
declare const unsafeMakeSemaphore: (permits: number) => Semaphore;
declare const makeSemaphore: (permits: number) => Effect<Semaphore>;
interface Latch extends Effect<void> {
    readonly open: Effect<void>;
    readonly unsafeOpen: () => void;
    readonly release: Effect<void>;
    readonly await: Effect<void>;
    readonly close: Effect<void>;
    readonly unsafeClose: () => void;
    readonly whenOpen: <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
    readonly [typeSymbol]?: unknown;
    readonly [unifySymbol]?: LatchUnify<this>;
    readonly [ignoreSymbol]?: LatchUnifyIgnore;
}
interface LatchUnify<A extends {
    [typeSymbol]?: any;
}> extends EffectUnify<A> {
    Latch?: () => Latch;
}
interface LatchUnifyIgnore extends EffectUnifyIgnore {
    Effect?: true;
}
declare const unsafeMakeLatch: (open?: boolean | undefined) => Latch;
declare const makeLatch: (open?: boolean | undefined) => Effect<Latch, never, never>;
declare const runFork: <A, E>(effect: Effect<A, E>, options?: RunForkOptions) => RuntimeFiber<A, E>;
declare const runCallback: <A, E>(effect: Effect<A, E>, options?: RunCallbackOptions<A, E> | undefined) => Cancel<A, E>;
declare const runPromise: <A, E>(effect: Effect<A, E, never>, options?: {
    readonly signal?: AbortSignal | undefined;
} | undefined) => Promise<A>;
declare const runPromiseExit: <A, E>(effect: Effect<A, E, never>, options?: {
    readonly signal?: AbortSignal;
} | undefined) => Promise<Exit<A, E>>;
declare const runSync: <A, E>(effect: Effect<A, E>) => A;
declare const runSyncExit: <A, E>(effect: Effect<A, E>) => Exit<A, E>;
declare const validate: {
    <B, E1, R1>(that: Effect<B, E1, R1>, options?: {
        readonly concurrent?: boolean | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    } | undefined): <A, E, R>(self: Effect<A, E, R>) => Effect<[
        A,
        B
    ], E1 | E, R1 | R>;
    <A, E, R, B, E1, R1>(self: Effect<A, E, R>, that: Effect<B, E1, R1>, options?: {
        readonly concurrent?: boolean | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    } | undefined): Effect<[
        A,
        B
    ], E | E1, R | R1>;
};
declare const validateWith: {
    <B, E1, R1, A, C>(that: Effect<B, E1, R1>, f: (a: A, b: B) => C, options?: {
        readonly concurrent?: boolean | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    } | undefined): <E, R>(self: Effect<A, E, R>) => Effect<C, E1 | E, R1 | R>;
    <A, E, R, B, E1, R1, C>(self: Effect<A, E, R>, that: Effect<B, E1, R1>, f: (a: A, b: B) => C, options?: {
        readonly concurrent?: boolean | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    } | undefined): Effect<C, E | E1, R | R1>;
};
declare const zip: {
    <A2, E2, R2>(that: Effect<A2, E2, R2>, options?: {
        readonly concurrent?: boolean | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    } | undefined): <A, E, R>(self: Effect<A, E, R>) => Effect<[
        A,
        A2
    ], E2 | E, R2 | R>;
    <A, E, R, A2, E2, R2>(self: Effect<A, E, R>, that: Effect<A2, E2, R2>, options?: {
        readonly concurrent?: boolean | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    } | undefined): Effect<[
        A,
        A2
    ], E | E2, R | R2>;
};
declare const zipLeft: {
    <A2, E2, R2>(that: Effect<A2, E2, R2>, options?: {
        readonly concurrent?: boolean | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    } | undefined): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E2 | E, R2 | R>;
    <A, E, R, A2, E2, R2>(self: Effect<A, E, R>, that: Effect<A2, E2, R2>, options?: {
        readonly concurrent?: boolean | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    } | undefined): Effect<A, E | E2, R | R2>;
};
declare const zipRight: {
    <A2, E2, R2>(that: Effect<A2, E2, R2>, options?: {
        readonly concurrent?: boolean | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    }): <A, E, R>(self: Effect<A, E, R>) => Effect<A2, E2 | E, R2 | R>;
    <A, E, R, A2, E2, R2>(self: Effect<A, E, R>, that: Effect<A2, E2, R2>, options?: {
        readonly concurrent?: boolean | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    }): Effect<A2, E2 | E, R2 | R>;
};
declare const zipWith$1: {
    <A2, E2, R2, A, B>(that: Effect<A2, E2, R2>, f: (a: A, b: A2) => B, options?: {
        readonly concurrent?: boolean | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    }): <E, R>(self: Effect<A, E, R>) => Effect<B, E2 | E, R2 | R>;
    <A, E, R, A2, E2, R2, B>(self: Effect<A, E, R>, that: Effect<A2, E2, R2>, f: (a: A, b: A2) => B, options?: {
        readonly concurrent?: boolean | undefined;
        readonly batching?: boolean | "inherit" | undefined;
        readonly concurrentFinalizers?: boolean | undefined;
    }): Effect<B, E2 | E, R2 | R>;
};
declare const ap$1: {
    <A, E2, R2>(that: Effect<A, E2, R2>): <B, R, E>(self: Effect<(a: A) => B, E, R>) => Effect<B, E | E2, R | R2>;
    <A, B, E, R, E2, R2>(self: Effect<(a: A) => B, E, R>, that: Effect<A, E2, R2>): Effect<B, E | E2, R | R2>;
};
declare const blocked: <A, E>(blockedRequests: RequestBlock, _continue: Effect<A, E>) => Blocked<A, E>;
declare const runRequestBlock: (blockedRequests: RequestBlock) => Effect<void>;
declare const step: <A, E, R>(self: Effect<A, E, R>) => Effect<Exit<A, E> | Blocked<A, E>, never, R>;
declare const request: {
    <A extends Request<any, any>, Ds extends RequestResolver<A> | Effect<RequestResolver<A>, any, any>>(dataSource: Ds): (self: A) => Effect<Request.Success<A>, Request.Error<A>, [
        Ds
    ] extends [
        Effect<any, any, any>
    ] ? Effect.Context<Ds> : never>;
    <Ds extends RequestResolver<A> | Effect<RequestResolver<A>, any, any>, A extends Request<any, any>>(self: A, dataSource: Ds): Effect<Request.Success<A>, Request.Error<A>, [
        Ds
    ] extends [
        Effect<any, any, any>
    ] ? Effect.Context<Ds> : never>;
};
declare const cacheRequestResult: <A extends Request<any, any>>(request: A, result: Request.Result<A>) => Effect<void>;
declare const withRequestBatching: {
    (requestBatching: boolean): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
    <A, E, R>(self: Effect<A, E, R>, requestBatching: boolean): Effect<A, E, R>;
};
declare const withRequestCaching: {
    (strategy: boolean): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
    <A, E, R>(self: Effect<A, E, R>, strategy: boolean): Effect<A, E, R>;
};
declare const withRequestCache: {
    (cache: Cache): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
    <A, E, R>(self: Effect<A, E, R>, cache: Cache): Effect<A, E, R>;
};
declare const tracer: Effect<Tracer>;
declare const tracerWith: <A, E, R>(f: (tracer: Tracer) => Effect<A, E, R>) => Effect<A, E, R>;
declare const withTracer: {
    (value: Tracer): <A, E, R>(effect: Effect<A, E, R>) => Effect<A, E, R>;
    <A, E, R>(effect: Effect<A, E, R>, value: Tracer): Effect<A, E, R>;
};
declare const withTracerScoped: (value: Tracer) => Effect<void, never, Scope>;
declare const withTracerEnabled: {
    (enabled: boolean): <A, E, R>(effect: Effect<A, E, R>) => Effect<A, E, R>;
    <A, E, R>(effect: Effect<A, E, R>, enabled: boolean): Effect<A, E, R>;
};
declare const withTracerTiming: {
    (enabled: boolean): <A, E, R>(effect: Effect<A, E, R>) => Effect<A, E, R>;
    <A, E, R>(effect: Effect<A, E, R>, enabled: boolean): Effect<A, E, R>;
};
declare const annotateSpans: {
    (key: string, value: unknown): <A, E, R>(effect: Effect<A, E, R>) => Effect<A, E, R>;
    (values: Record<string, unknown>): <A, E, R>(effect: Effect<A, E, R>) => Effect<A, E, R>;
    <A, E, R>(effect: Effect<A, E, R>, key: string, value: unknown): Effect<A, E, R>;
    <A, E, R>(effect: Effect<A, E, R>, values: Record<string, unknown>): Effect<A, E, R>;
};
declare const annotateCurrentSpan: {
    (key: string, value: unknown): Effect<void>;
    (values: Record<string, unknown>): Effect<void>;
};
declare const currentSpan: Effect<Span, NoSuchElementException>;
declare const currentPropagatedSpan: Effect<Span, NoSuchElementException>;
declare const currentParentSpan: Effect<AnySpan, NoSuchElementException>;
declare const spanAnnotations: Effect<HashMap<string, unknown>>;
declare const spanLinks: Effect<Chunk<SpanLink>>;
declare const linkSpans: {
    (span: AnySpan, attributes?: Record<string, unknown>): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, R>;
    <A, E, R>(self: Effect<A, E, R>, span: AnySpan, attributes?: Record<string, unknown>): Effect<A, E, R>;
};
declare const linkSpanCurrent: {
    (span: AnySpan, attributes?: Readonly<Record<string, unknown>> | undefined): Effect<void>;
    (links: ReadonlyArray<SpanLink>): Effect<void>;
};
declare const makeSpan: (name: string, options?: SpanOptions) => Effect<Span>;
declare const makeSpanScoped: (name: string, options?: SpanOptions | undefined) => Effect<Span, never, Scope>;
declare const useSpan: {
    <A, E, R>(name: string, evaluate: (span: Span) => Effect<A, E, R>): Effect<A, E, R>;
    <A, E, R>(name: string, options: SpanOptions, evaluate: (span: Span) => Effect<A, E, R>): Effect<A, E, R>;
};
declare const withSpan: {
    (name: string, options?: SpanOptions | undefined): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, Exclude<R, ParentSpan>>;
    <A, E, R>(self: Effect<A, E, R>, name: string, options?: SpanOptions | undefined): Effect<A, E, Exclude<R, ParentSpan>>;
};
interface FunctionWithSpanOptions {
    readonly name: string;
    readonly attributes?: Record<string, unknown> | undefined;
    readonly links?: ReadonlyArray<SpanLink> | undefined;
    readonly parent?: AnySpan | undefined;
    readonly root?: boolean | undefined;
    readonly context?: Context<never> | undefined;
    readonly kind?: SpanKind | undefined;
}
declare const functionWithSpan: <Args extends Array<any>, Ret extends Effect<any, any, any>>(options: {
    readonly body: (...args: Args) => Ret;
    readonly options: FunctionWithSpanOptions | ((...args: Args) => FunctionWithSpanOptions);
    readonly captureStackTrace?: boolean | undefined;
}) => (...args: Args) => Unify<Ret>;
declare const withSpanScoped: {
    (name: string, options?: SpanOptions): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, Exclude<R, ParentSpan> | Scope>;
    <A, E, R>(self: Effect<A, E, R>, name: string, options?: SpanOptions): Effect<A, E, Exclude<R, ParentSpan> | Scope>;
};
declare const withParentSpan: {
    (span: AnySpan): <A, E, R>(self: Effect<A, E, R>) => Effect<A, E, Exclude<R, ParentSpan>>;
    <A, E, R>(self: Effect<A, E, R>, span: AnySpan): Effect<A, E, Exclude<R, ParentSpan>>;
};
declare const fromNullable$1: <A>(value: A) => Effect<NonNullable<A>, NoSuchElementException>;
declare const optionFromOptional: <A, E, R>(self: Effect<A, E, R>) => Effect<Option<A>, Exclude<E, NoSuchElementException>, R>;
declare const transposeOption$1: <A = never, E = never, R = never>(self: Option<Effect<A, E, R>>) => Effect<Option<A>, E, R>;
declare const transposeMapOption$1: (<A, B, E = never, R = never>(f: (self: A) => Effect<B, E, R>) => (self: Option<A>) => Effect<Option<B>, E, R>) & (<A, B, E = never, R = never>(self: Option<A>, f: (self: A) => Effect<B, E, R>) => Effect<Option<B>, E, R>);
declare namespace Tag {
    interface ProhibitedType {
        Service?: `property "Service" is forbidden`;
        Identifier?: `property "Identifier" is forbidden`;
        _op?: `property "_op" is forbidden`;
        of?: `property "of" is forbidden`;
        context?: `property "context" is forbidden`;
        key?: `property "key" is forbidden`;
        stack?: `property "stack" is forbidden`;
        name?: `property "name" is forbidden`;
        pipe?: `property "pipe" is forbidden`;
        use?: `property "use" is forbidden`;
    }
    type AllowedType = (Record<PropertyKey, any> & ProhibitedType) | string | number | symbol;
    type Proxy<Self, Type> = {
        [k in keyof Type as Type[k] extends ((...args: infer Args extends ReadonlyArray<any>) => infer Ret) ? ((...args: Readonly<Args>) => Ret) extends Type[k] ? k : never : k]: Type[k] extends (...args: infer Args extends ReadonlyArray<any>) => Effect<infer A, infer E, infer R> ? (...args: Readonly<Args>) => Effect<A, E, Self | R> : Type[k] extends (...args: infer Args extends ReadonlyArray<any>) => Promise<infer A> ? (...args: Readonly<Args>) => Effect<A, UnknownException, Self> : Type[k] extends (...args: infer Args extends ReadonlyArray<any>) => infer A ? (...args: Readonly<Args>) => Effect<A, never, Self> : Type[k] extends Effect<infer A, infer E, infer R> ? Effect<A, E, Self | R> : Effect<Type[k], never, Self>;
    };
}
declare const Tag: <const Id extends string>(id: Id) => <Self, Type extends Tag.AllowedType>() => TagClass<Self, Id, Type> & (Type extends Record<PropertyKey, any> ? Tag.Proxy<Self, Type> : {}) & {
    use: <X>(body: (_: Type) => X) => [
        X
    ] extends [
        Effect<infer A, infer E, infer R>
    ] ? Effect<A, E, R | Self> : [
        X
    ] extends [
        PromiseLike<infer A>
    ] ? Effect<A, UnknownException, Self> : Effect<X, never, Self>;
};
type MissingSelfGeneric = `Missing \`Self\` generic - use \`class Self extends Effect.Service<Self>()...\``;
declare const Service: <Self = never>() => [
    Self
] extends [
    never
] ? MissingSelfGeneric : {
    <const Key extends string, const Make extends {
        readonly scoped: Effect<Service.AllowedType<Key, Make>, any, any> | ((...args: any) => Effect<Service.AllowedType<Key, Make>, any, any>);
        readonly dependencies?: ReadonlyArray<Layer.Any>;
        readonly accessors?: boolean;
        readonly ಠ_ಠ: never;
    } | {
        readonly effect: Effect<Service.AllowedType<Key, Make>, any, any> | ((...args: any) => Effect<Service.AllowedType<Key, Make>, any, any>);
        readonly dependencies?: ReadonlyArray<Layer.Any>;
        readonly accessors?: boolean;
        readonly ಠ_ಠ: never;
    } | {
        readonly sync: LazyArg<Service.AllowedType<Key, Make>>;
        readonly dependencies?: ReadonlyArray<Layer.Any>;
        readonly accessors?: boolean;
        readonly ಠ_ಠ: never;
    } | {
        readonly succeed: Service.AllowedType<Key, Make>;
        readonly dependencies?: ReadonlyArray<Layer.Any>;
        readonly accessors?: boolean;
        readonly ಠ_ಠ: never;
    }>(key: Key, make: Make): Service.Class<Self, Key, Make>;
    <const Key extends string, const Make extends NoExcessProperties<{
        readonly scoped: Effect<Service.AllowedType<Key, Make>, any, any> | ((...args: any) => Effect<Service.AllowedType<Key, Make>, any, any>);
        readonly dependencies?: ReadonlyArray<Layer.Any>;
        readonly accessors?: boolean;
    }, Make>>(key: Key, make: Make): Service.Class<Self, Key, Make>;
    <const Key extends string, const Make extends NoExcessProperties<{
        readonly effect: Effect<Service.AllowedType<Key, Make>, any, any> | ((...args: any) => Effect<Service.AllowedType<Key, Make>, any, any>);
        readonly dependencies?: ReadonlyArray<Layer.Any>;
        readonly accessors?: boolean;
    }, Make>>(key: Key, make: Make): Service.Class<Self, Key, Make>;
    <const Key extends string, const Make extends NoExcessProperties<{
        readonly sync: LazyArg<Service.AllowedType<Key, Make>>;
        readonly dependencies?: ReadonlyArray<Layer.Any>;
        readonly accessors?: boolean;
    }, Make>>(key: Key, make: Make): Service.Class<Self, Key, Make>;
    <const Key extends string, const Make extends NoExcessProperties<{
        readonly succeed: Service.AllowedType<Key, Make>;
        readonly dependencies?: ReadonlyArray<Layer.Any>;
        readonly accessors?: boolean;
    }, Make>>(key: Key, make: Make): Service.Class<Self, Key, Make>;
};
declare namespace Service {
    interface ProhibitedType {
        Service?: `property "Service" is forbidden`;
        Identifier?: `property "Identifier" is forbidden`;
        Default?: `property "Default" is forbidden`;
        DefaultWithoutDependencies?: `property "DefaultWithoutDependencies" is forbidden`;
        _op_layer?: `property "_op_layer" is forbidden`;
        _op?: `property "_op" is forbidden`;
        of?: `property "of" is forbidden`;
        make?: `property "make" is forbidden`;
        context?: `property "context" is forbidden`;
        key?: `property "key" is forbidden`;
        stack?: `property "stack" is forbidden`;
        name?: `property "name" is forbidden`;
        pipe?: `property "pipe" is forbidden`;
        use?: `property "use" is forbidden`;
        _tag?: `property "_tag" is forbidden`;
    }
    type AllowedType<Key extends string, Make> = MakeAccessors<Make> extends true ? Record<PropertyKey, any> & {
        readonly [K in Extract<keyof MakeService<Make>, keyof ProhibitedType>]: K extends "_tag" ? Key : ProhibitedType[K];
    } : Record<PropertyKey, any> & {
        readonly _tag?: Key;
    };
    type Class<Self, Key extends string, Make> = {
        new (_: MakeService<Make>): MakeService<Make> & {
            readonly _tag: Key;
        };
        readonly use: <X>(body: (_: Self) => X) => [
            X
        ] extends [
            Effect<infer A, infer E, infer R>
        ] ? Effect<A, E, R | Self> : [
            X
        ] extends [
            PromiseLike<infer A>
        ] ? Effect<A, UnknownException, Self> : Effect<X, never, Self>;
        readonly make: (_: MakeService<Make>) => Self;
    } & Tag$1<Self, Self> & {
        key: Key;
    } & (MakeAccessors<Make> extends true ? Tag.Proxy<Self, MakeService<Make>> : {}) & (MakeDeps<Make> extends never ? {
        readonly Default: HasArguments<Make> extends true ? (...args: MakeArguments<Make>) => Layer<Self, MakeError<Make>, MakeContext<Make>> : Layer<Self, MakeError<Make>, MakeContext<Make>>;
    } : {
        readonly DefaultWithoutDependencies: HasArguments<Make> extends true ? (...args: MakeArguments<Make>) => Layer<Self, MakeError<Make>, MakeContext<Make>> : Layer<Self, MakeError<Make>, MakeContext<Make>>;
        readonly Default: HasArguments<Make> extends true ? (...args: MakeArguments<Make>) => Layer<Self, MakeError<Make> | MakeDepsE<Make>, Exclude<MakeContext<Make>, MakeDepsOut<Make>> | MakeDepsIn<Make>> : Layer<Self, MakeError<Make> | MakeDepsE<Make>, Exclude<MakeContext<Make>, MakeDepsOut<Make>> | MakeDepsIn<Make>>;
    });
    type MakeService<Make> = Make extends {
        readonly effect: Effect<infer _A, infer _E, infer _R>;
    } ? _A : Make extends {
        readonly scoped: Effect<infer _A, infer _E, infer _R>;
    } ? _A : Make extends {
        readonly effect: (...args: infer _Args) => Effect<infer _A, infer _E, infer _R>;
    } ? _A : Make extends {
        readonly scoped: (...args: infer _Args) => Effect<infer _A, infer _E, infer _R>;
    } ? _A : Make extends {
        readonly sync: LazyArg<infer A>;
    } ? A : Make extends {
        readonly succeed: infer A;
    } ? A : never;
    type MakeError<Make> = Make extends {
        readonly effect: Effect<infer _A, infer _E, infer _R>;
    } ? _E : Make extends {
        readonly scoped: Effect<infer _A, infer _E, infer _R>;
    } ? _E : Make extends {
        readonly effect: (...args: infer _Args) => Effect<infer _A, infer _E, infer _R>;
    } ? _E : Make extends {
        readonly scoped: (...args: infer _Args) => Effect<infer _A, infer _E, infer _R>;
    } ? _E : never;
    type MakeContext<Make> = Make extends {
        readonly effect: Effect<infer _A, infer _E, infer _R>;
    } ? _R : Make extends {
        readonly scoped: Effect<infer _A, infer _E, infer _R>;
    } ? Exclude<_R, Scope> : Make extends {
        readonly effect: (...args: infer _Args) => Effect<infer _A, infer _E, infer _R>;
    } ? _R : Make extends {
        readonly scoped: (...args: infer _Args) => Effect<infer _A, infer _E, infer _R>;
    } ? Exclude<_R, Scope> : never;
    type MakeDeps<Make> = Make extends {
        readonly dependencies: ReadonlyArray<Layer.Any>;
    } ? Make["dependencies"][number] : never;
    type MakeDepsOut<Make> = Contravariant.Type<MakeDeps<Make>[LayerTypeId]["_ROut"]>;
    type MakeDepsE<Make> = Covariant.Type<MakeDeps<Make>[LayerTypeId]["_E"]>;
    type MakeDepsIn<Make> = Covariant.Type<MakeDeps<Make>[LayerTypeId]["_RIn"]>;
    type MakeAccessors<Make> = Make extends {
        readonly accessors: true;
    } ? true : false;
    type MakeArguments<Make> = Make extends {
        readonly effect: (...args: infer Args) => Effect<infer _A, infer _E, infer _R>;
    } ? Args : Make extends {
        readonly scoped: (...args: infer Args) => Effect<infer _A, infer _E, infer _R>;
    } ? Args : never;
    type HasArguments<Make> = Make extends {
        readonly scoped: (...args: ReadonlyArray<any>) => Effect<infer _A, infer _E, infer _R>;
    } ? true : Make extends {
        readonly effect: (...args: ReadonlyArray<any>) => Effect<infer _A, infer _E, infer _R>;
    } ? true : false;
}
declare namespace fn {
    type Return<A, E = never, R = never> = Generator<YieldWrap<Effect<any, E, R>>, A, any>;
    type Gen = {
        <Eff extends YieldWrap<Effect<any, any, any>>, AEff, Args extends Array<any>>(body: (...args: Args) => Generator<Eff, AEff, never>): (...args: Args) => Effect<AEff, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer E, infer _R>>
        ] ? E : never, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer _E, infer R>>
        ] ? R : never>;
        <Eff extends YieldWrap<Effect<any, any, any>>, AEff, Args extends Array<any>, A extends Effect<any, any, any>>(body: (...args: Args) => Generator<Eff, AEff, never>, a: (_: Effect<AEff, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer E, infer _R>>
        ] ? E : never, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer _E, infer R>>
        ] ? R : never>, ...args: NoInfer$1<Args>) => A): (...args: Args) => Effect.AsEffect<A>;
        <Eff extends YieldWrap<Effect<any, any, any>>, AEff, Args extends Array<any>, A, B extends Effect<any, any, any>>(body: (...args: Args) => Generator<Eff, AEff, never>, a: (_: Effect<AEff, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer E, infer _R>>
        ] ? E : never, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer _E, infer R>>
        ] ? R : never>, ...args: NoInfer$1<Args>) => A, b: (_: A, ...args: NoInfer$1<Args>) => B): (...args: Args) => Effect.AsEffect<B>;
        <Eff extends YieldWrap<Effect<any, any, any>>, AEff, Args extends Array<any>, A, B, C extends Effect<any, any, any>>(body: (...args: Args) => Generator<Eff, AEff, never>, a: (_: Effect<AEff, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer E, infer _R>>
        ] ? E : never, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer _E, infer R>>
        ] ? R : never>, ...args: NoInfer$1<Args>) => A, b: (_: A, ...args: NoInfer$1<Args>) => B, c: (_: B, ...args: NoInfer$1<Args>) => C): (...args: Args) => Effect.AsEffect<C>;
        <Eff extends YieldWrap<Effect<any, any, any>>, AEff, Args extends Array<any>, A, B, C, D extends Effect<any, any, any>>(body: (...args: Args) => Generator<Eff, AEff, never>, a: (_: Effect<AEff, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer E, infer _R>>
        ] ? E : never, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer _E, infer R>>
        ] ? R : never>, ...args: NoInfer$1<Args>) => A, b: (_: A, ...args: NoInfer$1<Args>) => B, c: (_: B, ...args: NoInfer$1<Args>) => C, d: (_: C, ...args: NoInfer$1<Args>) => D): (...args: Args) => Effect.AsEffect<D>;
        <Eff extends YieldWrap<Effect<any, any, any>>, AEff, Args extends Array<any>, A, B, C, D, E extends Effect<any, any, any>>(body: (...args: Args) => Generator<Eff, AEff, never>, a: (_: Effect<AEff, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer E, infer _R>>
        ] ? E : never, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer _E, infer R>>
        ] ? R : never>, ...args: NoInfer$1<Args>) => A, b: (_: A, ...args: NoInfer$1<Args>) => B, c: (_: B, ...args: NoInfer$1<Args>) => C, d: (_: C, ...args: NoInfer$1<Args>) => D, e: (_: D, ...args: NoInfer$1<Args>) => E): (...args: Args) => Effect.AsEffect<E>;
        <Eff extends YieldWrap<Effect<any, any, any>>, AEff, Args extends Array<any>, A, B, C, D, E, F extends Effect<any, any, any>>(body: (...args: Args) => Generator<Eff, AEff, never>, a: (_: Effect<AEff, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer E, infer _R>>
        ] ? E : never, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer _E, infer R>>
        ] ? R : never>, ...args: NoInfer$1<Args>) => A, b: (_: A, ...args: NoInfer$1<Args>) => B, c: (_: B, ...args: NoInfer$1<Args>) => C, d: (_: C, ...args: NoInfer$1<Args>) => D, e: (_: D, ...args: NoInfer$1<Args>) => E, f: (_: E, ...args: NoInfer$1<Args>) => F): (...args: Args) => Effect.AsEffect<F>;
        <Eff extends YieldWrap<Effect<any, any, any>>, AEff, Args extends Array<any>, A, B, C, D, E, F, G extends Effect<any, any, any>>(body: (...args: Args) => Generator<Eff, AEff, never>, a: (_: Effect<AEff, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer E, infer _R>>
        ] ? E : never, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer _E, infer R>>
        ] ? R : never>, ...args: NoInfer$1<Args>) => A, b: (_: A, ...args: NoInfer$1<Args>) => B, c: (_: B, ...args: NoInfer$1<Args>) => C, d: (_: C, ...args: NoInfer$1<Args>) => D, e: (_: D, ...args: NoInfer$1<Args>) => E, f: (_: E, ...args: NoInfer$1<Args>) => F, g: (_: F, ...args: NoInfer$1<Args>) => G): (...args: Args) => Effect.AsEffect<G>;
        <Eff extends YieldWrap<Effect<any, any, any>>, AEff, Args extends Array<any>, A, B, C, D, E, F, G, H extends Effect<any, any, any>>(body: (...args: Args) => Generator<Eff, AEff, never>, a: (_: Effect<AEff, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer E, infer _R>>
        ] ? E : never, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer _E, infer R>>
        ] ? R : never>, ...args: NoInfer$1<Args>) => A, b: (_: A, ...args: NoInfer$1<Args>) => B, c: (_: B, ...args: NoInfer$1<Args>) => C, d: (_: C, ...args: NoInfer$1<Args>) => D, e: (_: D, ...args: NoInfer$1<Args>) => E, f: (_: E, ...args: NoInfer$1<Args>) => F, g: (_: F, ...args: NoInfer$1<Args>) => G, h: (_: G, ...args: NoInfer$1<Args>) => H): (...args: Args) => Effect.AsEffect<H>;
        <Eff extends YieldWrap<Effect<any, any, any>>, AEff, Args extends Array<any>, A, B, C, D, E, F, G, H, I extends Effect<any, any, any>>(body: (...args: Args) => Generator<Eff, AEff, never>, a: (_: Effect<AEff, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer E, infer _R>>
        ] ? E : never, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer _E, infer R>>
        ] ? R : never>, ...args: NoInfer$1<Args>) => A, b: (_: A, ...args: NoInfer$1<Args>) => B, c: (_: B, ...args: NoInfer$1<Args>) => C, d: (_: C, ...args: NoInfer$1<Args>) => D, e: (_: D, ...args: NoInfer$1<Args>) => E, f: (_: E, ...args: NoInfer$1<Args>) => F, g: (_: F, ...args: NoInfer$1<Args>) => G, h: (_: G, ...args: NoInfer$1<Args>) => H, i: (_: H, ...args: NoInfer$1<Args>) => I): (...args: Args) => Effect.AsEffect<I>;
    };
    type NonGen = {
        <Eff extends Effect<any, any, any>, Args extends Array<any>>(body: (...args: Args) => Eff): (...args: Args) => Effect.AsEffect<Eff>;
        <Eff extends Effect<any, any, any>, A, Args extends Array<any>>(body: (...args: Args) => A, a: (_: A, ...args: NoInfer$1<Args>) => Eff): (...args: Args) => Effect.AsEffect<Eff>;
        <Eff extends Effect<any, any, any>, A, B, Args extends Array<any>>(body: (...args: Args) => A, a: (_: A, ...args: NoInfer$1<Args>) => B, b: (_: B, ...args: NoInfer$1<Args>) => Eff): (...args: Args) => Effect.AsEffect<Eff>;
        <Eff extends Effect<any, any, any>, A, B, C, Args extends Array<any>>(body: (...args: Args) => A, a: (_: A, ...args: NoInfer$1<Args>) => B, b: (_: B, ...args: NoInfer$1<Args>) => C, c: (_: C, ...args: NoInfer$1<Args>) => Eff): (...args: Args) => Effect.AsEffect<Eff>;
        <Eff extends Effect<any, any, any>, A, B, C, D, Args extends Array<any>>(body: (...args: Args) => A, a: (_: A, ...args: NoInfer$1<Args>) => B, b: (_: B, ...args: NoInfer$1<Args>) => C, c: (_: C, ...args: NoInfer$1<Args>) => D, d: (_: D, ...args: NoInfer$1<Args>) => Eff): (...args: Args) => Effect.AsEffect<Eff>;
        <Eff extends Effect<any, any, any>, A, B, C, D, E, Args extends Array<any>>(body: (...args: Args) => A, a: (_: A, ...args: NoInfer$1<Args>) => B, b: (_: B, ...args: NoInfer$1<Args>) => C, c: (_: C, ...args: NoInfer$1<Args>) => D, d: (_: D, ...args: NoInfer$1<Args>) => E, e: (_: E, ...args: NoInfer$1<Args>) => Eff): (...args: Args) => Effect.AsEffect<Eff>;
        <Eff extends Effect<any, any, any>, A, B, C, D, E, F, Args extends Array<any>>(body: (...args: Args) => A, a: (_: A, ...args: NoInfer$1<Args>) => B, b: (_: B, ...args: NoInfer$1<Args>) => C, c: (_: C, ...args: NoInfer$1<Args>) => D, d: (_: D, ...args: NoInfer$1<Args>) => E, e: (_: E, ...args: NoInfer$1<Args>) => F, f: (_: F, ...args: NoInfer$1<Args>) => Eff): (...args: Args) => Effect.AsEffect<Eff>;
        <Eff extends Effect<any, any, any>, A, B, C, D, E, F, G, Args extends Array<any>>(body: (...args: Args) => A, a: (_: A, ...args: NoInfer$1<Args>) => B, b: (_: B, ...args: NoInfer$1<Args>) => C, c: (_: C, ...args: NoInfer$1<Args>) => D, d: (_: D, ...args: NoInfer$1<Args>) => E, e: (_: E, ...args: NoInfer$1<Args>) => F, f: (_: F, ...args: NoInfer$1<Args>) => G, g: (_: G, ...args: NoInfer$1<Args>) => Eff): (...args: Args) => Effect.AsEffect<Eff>;
        <Eff extends Effect<any, any, any>, A, B, C, D, E, F, G, H, Args extends Array<any>>(body: (...args: Args) => A, a: (_: A, ...args: NoInfer$1<Args>) => B, b: (_: B, ...args: NoInfer$1<Args>) => C, c: (_: C, ...args: NoInfer$1<Args>) => D, d: (_: D, ...args: NoInfer$1<Args>) => E, e: (_: E, ...args: NoInfer$1<Args>) => F, f: (_: F, ...args: NoInfer$1<Args>) => G, g: (_: G, ...args: NoInfer$1<Args>) => H, h: (_: H, ...args: NoInfer$1<Args>) => Eff): (...args: Args) => Effect.AsEffect<Eff>;
        <Eff extends Effect<any, any, any>, A, B, C, D, E, F, G, H, I, Args extends Array<any>>(body: (...args: Args) => A, a: (_: A, ...args: NoInfer$1<Args>) => B, b: (_: B, ...args: NoInfer$1<Args>) => C, c: (_: C, ...args: NoInfer$1<Args>) => D, d: (_: D, ...args: NoInfer$1<Args>) => E, e: (_: E, ...args: NoInfer$1<Args>) => F, f: (_: F, ...args: NoInfer$1<Args>) => G, g: (_: G, ...args: NoInfer$1<Args>) => H, h: (_: H, ...args: NoInfer$1<Args>) => I, i: (_: H, ...args: NoInfer$1<Args>) => Eff): (...args: Args) => Effect.AsEffect<Eff>;
    };
    type Untraced = {
        <Eff extends YieldWrap<Effect<any, any, any>>, AEff, Args extends Array<any>>(body: (...args: Args) => Generator<Eff, AEff, never>): (...args: Args) => Effect<AEff, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer E, infer _R>>
        ] ? E : never, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer _E, infer R>>
        ] ? R : never>;
        <Eff extends YieldWrap<Effect<any, any, any>>, AEff, Args extends Array<any>, A>(body: (...args: Args) => Generator<Eff, AEff, never>, a: (_: Effect<AEff, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer E, infer _R>>
        ] ? E : never, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer _E, infer R>>
        ] ? R : never>, ...args: NoInfer$1<Args>) => A): (...args: Args) => A;
        <Eff extends YieldWrap<Effect<any, any, any>>, AEff, Args extends Array<any>, A, B>(body: (...args: Args) => Generator<Eff, AEff, never>, a: (_: Effect<AEff, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer E, infer _R>>
        ] ? E : never, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer _E, infer R>>
        ] ? R : never>, ...args: NoInfer$1<Args>) => A, b: (_: A, ...args: NoInfer$1<Args>) => B): (...args: Args) => B;
        <Eff extends YieldWrap<Effect<any, any, any>>, AEff, Args extends Array<any>, A, B, C>(body: (...args: Args) => Generator<Eff, AEff, never>, a: (_: Effect<AEff, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer E, infer _R>>
        ] ? E : never, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer _E, infer R>>
        ] ? R : never>, ...args: NoInfer$1<Args>) => A, b: (_: A, ...args: NoInfer$1<Args>) => B, c: (_: B, ...args: NoInfer$1<Args>) => C): (...args: Args) => C;
        <Eff extends YieldWrap<Effect<any, any, any>>, AEff, Args extends Array<any>, A, B, C, D>(body: (...args: Args) => Generator<Eff, AEff, never>, a: (_: Effect<AEff, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer E, infer _R>>
        ] ? E : never, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer _E, infer R>>
        ] ? R : never>, ...args: NoInfer$1<Args>) => A, b: (_: A, ...args: NoInfer$1<Args>) => B, c: (_: B, ...args: NoInfer$1<Args>) => C, d: (_: C, ...args: NoInfer$1<Args>) => D): (...args: Args) => D;
        <Eff extends YieldWrap<Effect<any, any, any>>, AEff, Args extends Array<any>, A, B, C, D, E>(body: (...args: Args) => Generator<Eff, AEff, never>, a: (_: Effect<AEff, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer E, infer _R>>
        ] ? E : never, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer _E, infer R>>
        ] ? R : never>, ...args: NoInfer$1<Args>) => A, b: (_: A, ...args: NoInfer$1<Args>) => B, c: (_: B, ...args: NoInfer$1<Args>) => C, d: (_: C, ...args: NoInfer$1<Args>) => D, e: (_: D, ...args: NoInfer$1<Args>) => E): (...args: Args) => E;
        <Eff extends YieldWrap<Effect<any, any, any>>, AEff, Args extends Array<any>, A, B, C, D, E, F>(body: (...args: Args) => Generator<Eff, AEff, never>, a: (_: Effect<AEff, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer E, infer _R>>
        ] ? E : never, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer _E, infer R>>
        ] ? R : never>, ...args: NoInfer$1<Args>) => A, b: (_: A, ...args: NoInfer$1<Args>) => B, c: (_: B, ...args: NoInfer$1<Args>) => C, d: (_: C, ...args: NoInfer$1<Args>) => D, e: (_: D, ...args: NoInfer$1<Args>) => E, f: (_: E, ...args: NoInfer$1<Args>) => F): (...args: Args) => F;
        <Eff extends YieldWrap<Effect<any, any, any>>, AEff, Args extends Array<any>, A, B, C, D, E, F, G>(body: (...args: Args) => Generator<Eff, AEff, never>, a: (_: Effect<AEff, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer E, infer _R>>
        ] ? E : never, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer _E, infer R>>
        ] ? R : never>, ...args: NoInfer$1<Args>) => A, b: (_: A, ...args: NoInfer$1<Args>) => B, c: (_: B, ...args: NoInfer$1<Args>) => C, d: (_: C, ...args: NoInfer$1<Args>) => D, e: (_: D, ...args: NoInfer$1<Args>) => E, f: (_: E, ...args: NoInfer$1<Args>) => F, g: (_: F, ...args: NoInfer$1<Args>) => G): (...args: Args) => G;
        <Eff extends YieldWrap<Effect<any, any, any>>, AEff, Args extends Array<any>, A, B, C, D, E, F, G, H>(body: (...args: Args) => Generator<Eff, AEff, never>, a: (_: Effect<AEff, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer E, infer _R>>
        ] ? E : never, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer _E, infer R>>
        ] ? R : never>, ...args: NoInfer$1<Args>) => A, b: (_: A, ...args: NoInfer$1<Args>) => B, c: (_: B, ...args: NoInfer$1<Args>) => C, d: (_: C, ...args: NoInfer$1<Args>) => D, e: (_: D, ...args: NoInfer$1<Args>) => E, f: (_: E, ...args: NoInfer$1<Args>) => F, g: (_: F, ...args: NoInfer$1<Args>) => G, h: (_: G, ...args: NoInfer$1<Args>) => H): (...args: Args) => H;
        <Eff extends YieldWrap<Effect<any, any, any>>, AEff, Args extends Array<any>, A, B, C, D, E, F, G, H, I>(body: (...args: Args) => Generator<Eff, AEff, never>, a: (_: Effect<AEff, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer E, infer _R>>
        ] ? E : never, [
            Eff
        ] extends [
            never
        ] ? never : [
            Eff
        ] extends [
            YieldWrap<Effect<infer _A, infer _E, infer R>>
        ] ? R : never>, ...args: NoInfer$1<Args>) => A, b: (_: A, ...args: NoInfer$1<Args>) => B, c: (_: B, ...args: NoInfer$1<Args>) => C, d: (_: C, ...args: NoInfer$1<Args>) => D, e: (_: D, ...args: NoInfer$1<Args>) => E, f: (_: E, ...args: NoInfer$1<Args>) => F, g: (_: F, ...args: NoInfer$1<Args>) => G, h: (_: G, ...args: NoInfer$1<Args>) => H, i: (_: H, ...args: NoInfer$1<Args>) => I): (...args: Args) => I;
    };
}
declare const fn: fn.Gen & fn.NonGen & ((name: string, options?: SpanOptions) => fn.Gen & fn.NonGen);
declare const fnUntraced: fn.Untraced;
declare const ensureSuccessType: <A>() => <A2 extends A, E, R>(effect: Effect<A2, E, R>) => Effect<A2, E, R>;
declare const ensureErrorType: <E>() => <A, E2 extends E, R>(effect: Effect<A, E2, R>) => Effect<A, E2, R>;
declare const ensureRequirementsType: <R>() => <A, E, R2 extends R>(effect: Effect<A, E, R2>) => Effect<A, E, R2>;
type Effect_d_Adapter = Adapter;
import Effect_d_All = All;
type Effect_d_Blocked<out A, out E> = Blocked<A, E>;
import Effect_d_Effect = Effect;
type Effect_d_EffectGenerator<T extends Effect<any, any, any>> = EffectGenerator<T>;
type Effect_d_EffectTypeId = EffectTypeId;
type Effect_d_EffectTypeLambda = EffectTypeLambda;
type Effect_d_EffectUnify<A extends {
    [typeSymbol]?: any;
}> = EffectUnify<A>;
type Effect_d_EffectUnifyIgnore = EffectUnifyIgnore;
type Effect_d_FunctionWithSpanOptions = FunctionWithSpanOptions;
type Effect_d_Latch = Latch;
type Effect_d_LatchUnify<A extends {
    [typeSymbol]?: any;
}> = LatchUnify<A>;
type Effect_d_LatchUnifyIgnore = LatchUnifyIgnore;
type Effect_d_Permit = Permit;
import Effect_d_Repeat = Repeat;
import Effect_d_Retry = Retry;
type Effect_d_Semaphore = Semaphore;
import Effect_d_Service = Service;
declare const Effect_d_Tag: typeof Tag;
declare const Effect_d_acquireRelease: typeof acquireRelease;
declare const Effect_d_acquireReleaseInterruptible: typeof acquireReleaseInterruptible;
declare const Effect_d_acquireUseRelease: typeof acquireUseRelease;
declare const Effect_d_addFinalizer: typeof addFinalizer;
declare const Effect_d_allSuccesses: typeof allSuccesses;
declare const Effect_d_allWith: typeof allWith;
declare const Effect_d_allowInterrupt: typeof allowInterrupt;
declare const Effect_d_annotateCurrentSpan: typeof annotateCurrentSpan;
declare const Effect_d_annotateLogs: typeof annotateLogs;
declare const Effect_d_annotateLogsScoped: typeof annotateLogsScoped;
declare const Effect_d_annotateSpans: typeof annotateSpans;
declare const Effect_d_as: typeof as;
declare const Effect_d_asSome: typeof asSome;
declare const Effect_d_asSomeError: typeof asSomeError;
declare const Effect_d_asVoid: typeof asVoid;
declare const Effect_d_async: typeof async;
declare const Effect_d_asyncEffect: typeof asyncEffect;
declare const Effect_d_awaitAllChildren: typeof awaitAllChildren;
declare const Effect_d_bindAll: typeof bindAll;
declare const Effect_d_blocked: typeof blocked;
declare const Effect_d_cacheRequestResult: typeof cacheRequestResult;
declare const Effect_d_cached: typeof cached;
declare const Effect_d_cachedFunction: typeof cachedFunction;
declare const Effect_d_cachedInvalidateWithTTL: typeof cachedInvalidateWithTTL;
declare const Effect_d_cachedWithTTL: typeof cachedWithTTL;
declare const Effect_d_catchAll: typeof catchAll;
declare const Effect_d_catchAllCause: typeof catchAllCause;
declare const Effect_d_catchAllDefect: typeof catchAllDefect;
declare const Effect_d_catchIf: typeof catchIf;
declare const Effect_d_catchSome: typeof catchSome;
declare const Effect_d_catchSomeCause: typeof catchSomeCause;
declare const Effect_d_catchSomeDefect: typeof catchSomeDefect;
declare const Effect_d_catchTag: typeof catchTag;
declare const Effect_d_catchTags: typeof catchTags;
declare const Effect_d_cause: typeof cause;
declare const Effect_d_checkInterruptible: typeof checkInterruptible;
declare const Effect_d_clock: typeof clock;
declare const Effect_d_clockWith: typeof clockWith;
declare const Effect_d_configProviderWith: typeof configProviderWith;
declare const Effect_d_console: typeof console;
declare const Effect_d_consoleWith: typeof consoleWith;
declare const Effect_d_context: typeof context;
declare const Effect_d_contextWith: typeof contextWith;
declare const Effect_d_contextWithEffect: typeof contextWithEffect;
declare const Effect_d_currentParentSpan: typeof currentParentSpan;
declare const Effect_d_currentPropagatedSpan: typeof currentPropagatedSpan;
declare const Effect_d_currentSpan: typeof currentSpan;
declare const Effect_d_custom: typeof custom;
declare const Effect_d_daemonChildren: typeof daemonChildren;
declare const Effect_d_delay: typeof delay;
declare const Effect_d_descriptor: typeof descriptor;
declare const Effect_d_descriptorWith: typeof descriptorWith;
declare const Effect_d_die: typeof die;
declare const Effect_d_dieMessage: typeof dieMessage;
declare const Effect_d_dieSync: typeof dieSync;
declare const Effect_d_diffFiberRefs: typeof diffFiberRefs;
declare const Effect_d_disconnect: typeof disconnect;
declare const Effect_d_dropUntil: typeof dropUntil;
declare const Effect_d_dropWhile: typeof dropWhile;
declare const Effect_d_ensureErrorType: typeof ensureErrorType;
declare const Effect_d_ensureRequirementsType: typeof ensureRequirementsType;
declare const Effect_d_ensureSuccessType: typeof ensureSuccessType;
declare const Effect_d_ensuring: typeof ensuring;
declare const Effect_d_ensuringChild: typeof ensuringChild;
declare const Effect_d_ensuringChildren: typeof ensuringChildren;
declare const Effect_d_eventually: typeof eventually;
declare const Effect_d_every: typeof every;
declare const Effect_d_exists: typeof exists;
declare const Effect_d_exit: typeof exit;
declare const Effect_d_fail: typeof fail;
declare const Effect_d_failCause: typeof failCause;
declare const Effect_d_failCauseSync: typeof failCauseSync;
declare const Effect_d_failSync: typeof failSync;
declare const Effect_d_fiberId: typeof fiberId;
declare const Effect_d_fiberIdWith: typeof fiberIdWith;
declare const Effect_d_filter: typeof filter;
declare const Effect_d_filterEffectOrElse: typeof filterEffectOrElse;
declare const Effect_d_filterEffectOrFail: typeof filterEffectOrFail;
declare const Effect_d_filterMap: typeof filterMap;
declare const Effect_d_filterOrDie: typeof filterOrDie;
declare const Effect_d_filterOrDieMessage: typeof filterOrDieMessage;
declare const Effect_d_filterOrElse: typeof filterOrElse;
declare const Effect_d_filterOrFail: typeof filterOrFail;
declare const Effect_d_finalizersMask: typeof finalizersMask;
declare const Effect_d_findFirst: typeof findFirst;
declare const Effect_d_firstSuccessOf: typeof firstSuccessOf;
declare const Effect_d_flatten: typeof flatten;
declare const Effect_d_flipWith: typeof flipWith;
declare const Effect_d_fn: typeof fn;
declare const Effect_d_fnUntraced: typeof fnUntraced;
declare const Effect_d_forEach: typeof forEach;
declare const Effect_d_forever: typeof forever;
declare const Effect_d_fork: typeof fork;
declare const Effect_d_forkAll: typeof forkAll;
declare const Effect_d_forkDaemon: typeof forkDaemon;
declare const Effect_d_forkIn: typeof forkIn;
declare const Effect_d_forkScoped: typeof forkScoped;
declare const Effect_d_forkWithErrorHandler: typeof forkWithErrorHandler;
declare const Effect_d_fromFiber: typeof fromFiber;
declare const Effect_d_fromFiberEffect: typeof fromFiberEffect;
declare const Effect_d_functionWithSpan: typeof functionWithSpan;
declare const Effect_d_getFiberRefs: typeof getFiberRefs;
declare const Effect_d_getRuntimeFlags: typeof getRuntimeFlags;
declare const Effect_d_head: typeof head;
declare const Effect_d_ignore: typeof ignore;
declare const Effect_d_ignoreLogged: typeof ignoreLogged;
declare const Effect_d_inheritFiberRefs: typeof inheritFiberRefs;
declare const Effect_d_interrupt: typeof interrupt;
declare const Effect_d_interruptWith: typeof interruptWith;
declare const Effect_d_interruptible: typeof interruptible;
declare const Effect_d_interruptibleMask: typeof interruptibleMask;
declare const Effect_d_intoDeferred: typeof intoDeferred;
declare const Effect_d_isEffect: typeof isEffect;
declare const Effect_d_isFailure: typeof isFailure;
declare const Effect_d_isSuccess: typeof isSuccess;
declare const Effect_d_iterate: typeof iterate;
declare const Effect_d_labelMetrics: typeof labelMetrics;
declare const Effect_d_labelMetricsScoped: typeof labelMetricsScoped;
declare const Effect_d_linkSpanCurrent: typeof linkSpanCurrent;
declare const Effect_d_linkSpans: typeof linkSpans;
declare const Effect_d_locally: typeof locally;
declare const Effect_d_locallyScoped: typeof locallyScoped;
declare const Effect_d_locallyScopedWith: typeof locallyScopedWith;
declare const Effect_d_locallyWith: typeof locallyWith;
declare const Effect_d_log: typeof log;
declare const Effect_d_logAnnotations: typeof logAnnotations;
declare const Effect_d_logDebug: typeof logDebug;
declare const Effect_d_logError: typeof logError;
declare const Effect_d_logFatal: typeof logFatal;
declare const Effect_d_logInfo: typeof logInfo;
declare const Effect_d_logTrace: typeof logTrace;
declare const Effect_d_logWarning: typeof logWarning;
declare const Effect_d_logWithLevel: typeof logWithLevel;
declare const Effect_d_loop: typeof loop;
declare const Effect_d_makeLatch: typeof makeLatch;
declare const Effect_d_makeSemaphore: typeof makeSemaphore;
declare const Effect_d_makeSpan: typeof makeSpan;
declare const Effect_d_makeSpanScoped: typeof makeSpanScoped;
declare const Effect_d_mapAccum: typeof mapAccum;
declare const Effect_d_mapError: typeof mapError;
declare const Effect_d_mapErrorCause: typeof mapErrorCause;
declare const Effect_d_mapInputContext: typeof mapInputContext;
declare const Effect_d_matchCause: typeof matchCause;
declare const Effect_d_matchCauseEffect: typeof matchCauseEffect;
declare const Effect_d_matchEffect: typeof matchEffect;
declare const Effect_d_mergeAll: typeof mergeAll;
declare const Effect_d_metricLabels: typeof metricLabels;
declare const Effect_d_negate: typeof negate;
declare const Effect_d_never: typeof never;
declare const Effect_d_none: typeof none;
declare const Effect_d_onError: typeof onError;
declare const Effect_d_onExit: typeof onExit;
declare const Effect_d_onInterrupt: typeof onInterrupt;
declare const Effect_d_once: typeof once;
declare const Effect_d_optionFromOptional: typeof optionFromOptional;
declare const Effect_d_orDie: typeof orDie;
declare const Effect_d_orDieWith: typeof orDieWith;
declare const Effect_d_orElseFail: typeof orElseFail;
declare const Effect_d_orElseSucceed: typeof orElseSucceed;
declare const Effect_d_parallelErrors: typeof parallelErrors;
declare const Effect_d_parallelFinalizers: typeof parallelFinalizers;
declare const Effect_d_partition: typeof partition;
declare const Effect_d_patchFiberRefs: typeof patchFiberRefs;
declare const Effect_d_patchRuntimeFlags: typeof patchRuntimeFlags;
declare const Effect_d_promise: typeof promise;
declare const Effect_d_provide: typeof provide;
declare const Effect_d_provideService: typeof provideService;
declare const Effect_d_provideServiceEffect: typeof provideServiceEffect;
declare const Effect_d_race: typeof race;
declare const Effect_d_raceAll: typeof raceAll;
declare const Effect_d_raceFirst: typeof raceFirst;
declare const Effect_d_raceWith: typeof raceWith;
declare const Effect_d_random: typeof random;
declare const Effect_d_randomWith: typeof randomWith;
declare const Effect_d_reduce: typeof reduce;
declare const Effect_d_reduceEffect: typeof reduceEffect;
declare const Effect_d_reduceRight: typeof reduceRight;
declare const Effect_d_reduceWhile: typeof reduceWhile;
declare const Effect_d_repeat: typeof repeat;
declare const Effect_d_repeatN: typeof repeatN;
declare const Effect_d_repeatOrElse: typeof repeatOrElse;
declare const Effect_d_replicate: typeof replicate;
declare const Effect_d_replicateEffect: typeof replicateEffect;
declare const Effect_d_request: typeof request;
declare const Effect_d_retry: typeof retry;
declare const Effect_d_retryOrElse: typeof retryOrElse;
declare const Effect_d_runCallback: typeof runCallback;
declare const Effect_d_runFork: typeof runFork;
declare const Effect_d_runPromise: typeof runPromise;
declare const Effect_d_runPromiseExit: typeof runPromiseExit;
declare const Effect_d_runRequestBlock: typeof runRequestBlock;
declare const Effect_d_runSync: typeof runSync;
declare const Effect_d_runSyncExit: typeof runSyncExit;
declare const Effect_d_runtime: typeof runtime;
declare const Effect_d_sandbox: typeof sandbox;
declare const Effect_d_schedule: typeof schedule;
declare const Effect_d_scheduleForked: typeof scheduleForked;
declare const Effect_d_scheduleFrom: typeof scheduleFrom;
declare const Effect_d_scope: typeof scope;
declare const Effect_d_scopeWith: typeof scopeWith;
declare const Effect_d_scoped: typeof scoped;
declare const Effect_d_scopedWith: typeof scopedWith;
declare const Effect_d_sequentialFinalizers: typeof sequentialFinalizers;
declare const Effect_d_serviceConstants: typeof serviceConstants;
declare const Effect_d_serviceFunction: typeof serviceFunction;
declare const Effect_d_serviceFunctionEffect: typeof serviceFunctionEffect;
declare const Effect_d_serviceFunctions: typeof serviceFunctions;
declare const Effect_d_serviceMembers: typeof serviceMembers;
declare const Effect_d_serviceOption: typeof serviceOption;
declare const Effect_d_serviceOptional: typeof serviceOptional;
declare const Effect_d_setFiberRefs: typeof setFiberRefs;
declare const Effect_d_sleep: typeof sleep;
declare const Effect_d_spanAnnotations: typeof spanAnnotations;
declare const Effect_d_spanLinks: typeof spanLinks;
declare const Effect_d_step: typeof step;
declare const Effect_d_succeed: typeof succeed;
declare const Effect_d_succeedNone: typeof succeedNone;
declare const Effect_d_succeedSome: typeof succeedSome;
declare const Effect_d_summarized: typeof summarized;
declare const Effect_d_supervised: typeof supervised;
declare const Effect_d_suspend: typeof suspend;
declare const Effect_d_sync: typeof sync;
declare const Effect_d_tagMetrics: typeof tagMetrics;
declare const Effect_d_tagMetricsScoped: typeof tagMetricsScoped;
declare const Effect_d_takeUntil: typeof takeUntil;
declare const Effect_d_takeWhile: typeof takeWhile;
declare const Effect_d_tap: typeof tap;
declare const Effect_d_tapBoth: typeof tapBoth;
declare const Effect_d_tapDefect: typeof tapDefect;
declare const Effect_d_tapError: typeof tapError;
declare const Effect_d_tapErrorCause: typeof tapErrorCause;
declare const Effect_d_tapErrorTag: typeof tapErrorTag;
declare const Effect_d_timed: typeof timed;
declare const Effect_d_timedWith: typeof timedWith;
declare const Effect_d_timeout: typeof timeout;
declare const Effect_d_timeoutFail: typeof timeoutFail;
declare const Effect_d_timeoutFailCause: typeof timeoutFailCause;
declare const Effect_d_timeoutOption: typeof timeoutOption;
declare const Effect_d_timeoutTo: typeof timeoutTo;
declare const Effect_d_tracer: typeof tracer;
declare const Effect_d_tracerWith: typeof tracerWith;
declare const Effect_d_transplant: typeof transplant;
declare const Effect_d_tryMap: typeof tryMap;
declare const Effect_d_tryMapPromise: typeof tryMapPromise;
declare const Effect_d_tryPromise: typeof tryPromise;
declare const Effect_d_uninterruptible: typeof uninterruptible;
declare const Effect_d_uninterruptibleMask: typeof uninterruptibleMask;
declare const Effect_d_unless: typeof unless;
declare const Effect_d_unlessEffect: typeof unlessEffect;
declare const Effect_d_unsafeMakeLatch: typeof unsafeMakeLatch;
declare const Effect_d_unsafeMakeSemaphore: typeof unsafeMakeSemaphore;
declare const Effect_d_unsandbox: typeof unsandbox;
declare const Effect_d_updateFiberRefs: typeof updateFiberRefs;
declare const Effect_d_updateService: typeof updateService;
declare const Effect_d_useSpan: typeof useSpan;
declare const Effect_d_using: typeof using;
declare const Effect_d_validate: typeof validate;
declare const Effect_d_validateAll: typeof validateAll;
declare const Effect_d_validateFirst: typeof validateFirst;
declare const Effect_d_validateWith: typeof validateWith;
declare const Effect_d_whenEffect: typeof whenEffect;
declare const Effect_d_whenFiberRef: typeof whenFiberRef;
declare const Effect_d_whenLogLevel: typeof whenLogLevel;
declare const Effect_d_whenRef: typeof whenRef;
declare const Effect_d_whileLoop: typeof whileLoop;
declare const Effect_d_withClock: typeof withClock;
declare const Effect_d_withClockScoped: typeof withClockScoped;
declare const Effect_d_withConcurrency: typeof withConcurrency;
declare const Effect_d_withConfigProvider: typeof withConfigProvider;
declare const Effect_d_withConfigProviderScoped: typeof withConfigProviderScoped;
declare const Effect_d_withConsole: typeof withConsole;
declare const Effect_d_withConsoleScoped: typeof withConsoleScoped;
declare const Effect_d_withEarlyRelease: typeof withEarlyRelease;
declare const Effect_d_withExecutionPlan: typeof withExecutionPlan;
declare const Effect_d_withFiberRuntime: typeof withFiberRuntime;
declare const Effect_d_withLogSpan: typeof withLogSpan;
declare const Effect_d_withMaxOpsBeforeYield: typeof withMaxOpsBeforeYield;
declare const Effect_d_withMetric: typeof withMetric;
declare const Effect_d_withParentSpan: typeof withParentSpan;
declare const Effect_d_withRandom: typeof withRandom;
declare const Effect_d_withRandomFixed: typeof withRandomFixed;
declare const Effect_d_withRandomScoped: typeof withRandomScoped;
declare const Effect_d_withRequestBatching: typeof withRequestBatching;
declare const Effect_d_withRequestCache: typeof withRequestCache;
declare const Effect_d_withRequestCaching: typeof withRequestCaching;
declare const Effect_d_withRuntimeFlagsPatch: typeof withRuntimeFlagsPatch;
declare const Effect_d_withRuntimeFlagsPatchScoped: typeof withRuntimeFlagsPatchScoped;
declare const Effect_d_withScheduler: typeof withScheduler;
declare const Effect_d_withSchedulingPriority: typeof withSchedulingPriority;
declare const Effect_d_withSpan: typeof withSpan;
declare const Effect_d_withSpanScoped: typeof withSpanScoped;
declare const Effect_d_withTracer: typeof withTracer;
declare const Effect_d_withTracerEnabled: typeof withTracerEnabled;
declare const Effect_d_withTracerScoped: typeof withTracerScoped;
declare const Effect_d_withTracerTiming: typeof withTracerTiming;
declare const Effect_d_withUnhandledErrorLogLevel: typeof withUnhandledErrorLogLevel;
declare const Effect_d_yieldNow: typeof yieldNow;
declare const Effect_d_zip: typeof zip;
declare const Effect_d_zipLeft: typeof zipLeft;
declare const Effect_d_zipRight: typeof zipRight;
declare namespace Effect_d {
    export { Effect_d_All as All, Do$1 as Do, Effect_d_Effect as Effect, Effect_d_Repeat as Repeat, Effect_d_Retry as Retry, Effect_d_Service as Service, Effect_d_Tag as Tag, Effect_d_acquireRelease as acquireRelease, Effect_d_acquireReleaseInterruptible as acquireReleaseInterruptible, Effect_d_acquireUseRelease as acquireUseRelease, Effect_d_addFinalizer as addFinalizer, all$1 as all, Effect_d_allSuccesses as allSuccesses, Effect_d_allWith as allWith, Effect_d_allowInterrupt as allowInterrupt, andThen$1 as andThen, Effect_d_annotateCurrentSpan as annotateCurrentSpan, Effect_d_annotateLogs as annotateLogs, Effect_d_annotateLogsScoped as annotateLogsScoped, Effect_d_annotateSpans as annotateSpans, ap$1 as ap, Effect_d_as as as, Effect_d_asSome as asSome, Effect_d_asSomeError as asSomeError, Effect_d_asVoid as asVoid, Effect_d_async as async, Effect_d_asyncEffect as asyncEffect, Effect_d_awaitAllChildren as awaitAllChildren, bind$1 as bind, Effect_d_bindAll as bindAll, bindTo$1 as bindTo, Effect_d_blocked as blocked, Effect_d_cacheRequestResult as cacheRequestResult, Effect_d_cached as cached, Effect_d_cachedFunction as cachedFunction, Effect_d_cachedInvalidateWithTTL as cachedInvalidateWithTTL, Effect_d_cachedWithTTL as cachedWithTTL, _catch as catch, Effect_d_catchAll as catchAll, Effect_d_catchAllCause as catchAllCause, Effect_d_catchAllDefect as catchAllDefect, Effect_d_catchIf as catchIf, Effect_d_catchSome as catchSome, Effect_d_catchSomeCause as catchSomeCause, Effect_d_catchSomeDefect as catchSomeDefect, Effect_d_catchTag as catchTag, Effect_d_catchTags as catchTags, Effect_d_cause as cause, Effect_d_checkInterruptible as checkInterruptible, Effect_d_clock as clock, Effect_d_clockWith as clockWith, Effect_d_configProviderWith as configProviderWith, Effect_d_console as console, Effect_d_consoleWith as consoleWith, Effect_d_context as context, Effect_d_contextWith as contextWith, Effect_d_contextWithEffect as contextWithEffect, Effect_d_currentParentSpan as currentParentSpan, Effect_d_currentPropagatedSpan as currentPropagatedSpan, Effect_d_currentSpan as currentSpan, Effect_d_custom as custom, Effect_d_daemonChildren as daemonChildren, Effect_d_delay as delay, Effect_d_descriptor as descriptor, Effect_d_descriptorWith as descriptorWith, Effect_d_die as die, Effect_d_dieMessage as dieMessage, Effect_d_dieSync as dieSync, Effect_d_diffFiberRefs as diffFiberRefs, Effect_d_disconnect as disconnect, Effect_d_dropUntil as dropUntil, Effect_d_dropWhile as dropWhile, either$1 as either, Effect_d_ensureErrorType as ensureErrorType, Effect_d_ensureRequirementsType as ensureRequirementsType, Effect_d_ensureSuccessType as ensureSuccessType, Effect_d_ensuring as ensuring, Effect_d_ensuringChild as ensuringChild, Effect_d_ensuringChildren as ensuringChildren, Effect_d_eventually as eventually, Effect_d_every as every, Effect_d_exists as exists, Effect_d_exit as exit, Effect_d_fail as fail, Effect_d_failCause as failCause, Effect_d_failCauseSync as failCauseSync, Effect_d_failSync as failSync, Effect_d_fiberId as fiberId, Effect_d_fiberIdWith as fiberIdWith, Effect_d_filter as filter, Effect_d_filterEffectOrElse as filterEffectOrElse, Effect_d_filterEffectOrFail as filterEffectOrFail, Effect_d_filterMap as filterMap, Effect_d_filterOrDie as filterOrDie, Effect_d_filterOrDieMessage as filterOrDieMessage, Effect_d_filterOrElse as filterOrElse, Effect_d_filterOrFail as filterOrFail, Effect_d_finalizersMask as finalizersMask, Effect_d_findFirst as findFirst, Effect_d_firstSuccessOf as firstSuccessOf, flatMap$1 as flatMap, Effect_d_flatten as flatten, flip$1 as flip, Effect_d_flipWith as flipWith, Effect_d_fn as fn, Effect_d_fnUntraced as fnUntraced, Effect_d_forEach as forEach, Effect_d_forever as forever, Effect_d_fork as fork, Effect_d_forkAll as forkAll, Effect_d_forkDaemon as forkDaemon, Effect_d_forkIn as forkIn, Effect_d_forkScoped as forkScoped, Effect_d_forkWithErrorHandler as forkWithErrorHandler, Effect_d_fromFiber as fromFiber, Effect_d_fromFiberEffect as fromFiberEffect, fromNullable$1 as fromNullable, Effect_d_functionWithSpan as functionWithSpan, gen$1 as gen, Effect_d_getFiberRefs as getFiberRefs, Effect_d_getRuntimeFlags as getRuntimeFlags, Effect_d_head as head, if_ as if, Effect_d_ignore as ignore, Effect_d_ignoreLogged as ignoreLogged, Effect_d_inheritFiberRefs as inheritFiberRefs, Effect_d_interrupt as interrupt, Effect_d_interruptWith as interruptWith, Effect_d_interruptible as interruptible, Effect_d_interruptibleMask as interruptibleMask, Effect_d_intoDeferred as intoDeferred, Effect_d_isEffect as isEffect, Effect_d_isFailure as isFailure, Effect_d_isSuccess as isSuccess, Effect_d_iterate as iterate, Effect_d_labelMetrics as labelMetrics, Effect_d_labelMetricsScoped as labelMetricsScoped, let_$1 as let, liftPredicate$1 as liftPredicate, Effect_d_linkSpanCurrent as linkSpanCurrent, Effect_d_linkSpans as linkSpans, Effect_d_locally as locally, Effect_d_locallyScoped as locallyScoped, Effect_d_locallyScopedWith as locallyScopedWith, Effect_d_locallyWith as locallyWith, Effect_d_log as log, Effect_d_logAnnotations as logAnnotations, Effect_d_logDebug as logDebug, Effect_d_logError as logError, Effect_d_logFatal as logFatal, Effect_d_logInfo as logInfo, Effect_d_logTrace as logTrace, Effect_d_logWarning as logWarning, Effect_d_logWithLevel as logWithLevel, Effect_d_loop as loop, Effect_d_makeLatch as makeLatch, Effect_d_makeSemaphore as makeSemaphore, Effect_d_makeSpan as makeSpan, Effect_d_makeSpanScoped as makeSpanScoped, map$1 as map, Effect_d_mapAccum as mapAccum, mapBoth$1 as mapBoth, Effect_d_mapError as mapError, Effect_d_mapErrorCause as mapErrorCause, Effect_d_mapInputContext as mapInputContext, match$1 as match, Effect_d_matchCause as matchCause, Effect_d_matchCauseEffect as matchCauseEffect, Effect_d_matchEffect as matchEffect, merge$1 as merge, Effect_d_mergeAll as mergeAll, Effect_d_metricLabels as metricLabels, Effect_d_negate as negate, Effect_d_never as never, Effect_d_none as none, Effect_d_onError as onError, Effect_d_onExit as onExit, Effect_d_onInterrupt as onInterrupt, Effect_d_once as once, option$1 as option, Effect_d_optionFromOptional as optionFromOptional, Effect_d_orDie as orDie, Effect_d_orDieWith as orDieWith, orElse$2 as orElse, Effect_d_orElseFail as orElseFail, Effect_d_orElseSucceed as orElseSucceed, Effect_d_parallelErrors as parallelErrors, Effect_d_parallelFinalizers as parallelFinalizers, Effect_d_partition as partition, Effect_d_patchFiberRefs as patchFiberRefs, Effect_d_patchRuntimeFlags as patchRuntimeFlags, Effect_d_promise as promise, Effect_d_provide as provide, Effect_d_provideService as provideService, Effect_d_provideServiceEffect as provideServiceEffect, Effect_d_race as race, Effect_d_raceAll as raceAll, Effect_d_raceFirst as raceFirst, Effect_d_raceWith as raceWith, Effect_d_random as random, Effect_d_randomWith as randomWith, Effect_d_reduce as reduce, Effect_d_reduceEffect as reduceEffect, Effect_d_reduceRight as reduceRight, Effect_d_reduceWhile as reduceWhile, Effect_d_repeat as repeat, Effect_d_repeatN as repeatN, Effect_d_repeatOrElse as repeatOrElse, Effect_d_replicate as replicate, Effect_d_replicateEffect as replicateEffect, Effect_d_request as request, Effect_d_retry as retry, Effect_d_retryOrElse as retryOrElse, Effect_d_runCallback as runCallback, Effect_d_runFork as runFork, Effect_d_runPromise as runPromise, Effect_d_runPromiseExit as runPromiseExit, Effect_d_runRequestBlock as runRequestBlock, Effect_d_runSync as runSync, Effect_d_runSyncExit as runSyncExit, Effect_d_runtime as runtime, Effect_d_sandbox as sandbox, Effect_d_schedule as schedule, Effect_d_scheduleForked as scheduleForked, Effect_d_scheduleFrom as scheduleFrom, Effect_d_scope as scope, Effect_d_scopeWith as scopeWith, Effect_d_scoped as scoped, Effect_d_scopedWith as scopedWith, Effect_d_sequentialFinalizers as sequentialFinalizers, Effect_d_serviceConstants as serviceConstants, Effect_d_serviceFunction as serviceFunction, Effect_d_serviceFunctionEffect as serviceFunctionEffect, Effect_d_serviceFunctions as serviceFunctions, Effect_d_serviceMembers as serviceMembers, Effect_d_serviceOption as serviceOption, Effect_d_serviceOptional as serviceOptional, Effect_d_setFiberRefs as setFiberRefs, Effect_d_sleep as sleep, Effect_d_spanAnnotations as spanAnnotations, Effect_d_spanLinks as spanLinks, Effect_d_step as step, Effect_d_succeed as succeed, Effect_d_succeedNone as succeedNone, Effect_d_succeedSome as succeedSome, Effect_d_summarized as summarized, Effect_d_supervised as supervised, Effect_d_suspend as suspend, Effect_d_sync as sync, Effect_d_tagMetrics as tagMetrics, Effect_d_tagMetricsScoped as tagMetricsScoped, Effect_d_takeUntil as takeUntil, Effect_d_takeWhile as takeWhile, Effect_d_tap as tap, Effect_d_tapBoth as tapBoth, Effect_d_tapDefect as tapDefect, Effect_d_tapError as tapError, Effect_d_tapErrorCause as tapErrorCause, Effect_d_tapErrorTag as tapErrorTag, Effect_d_timed as timed, Effect_d_timedWith as timedWith, Effect_d_timeout as timeout, Effect_d_timeoutFail as timeoutFail, Effect_d_timeoutFailCause as timeoutFailCause, Effect_d_timeoutOption as timeoutOption, Effect_d_timeoutTo as timeoutTo, Effect_d_tracer as tracer, Effect_d_tracerWith as tracerWith, Effect_d_transplant as transplant, transposeMapOption$1 as transposeMapOption, transposeOption$1 as transposeOption, try_$1 as try, Effect_d_tryMap as tryMap, Effect_d_tryMapPromise as tryMapPromise, Effect_d_tryPromise as tryPromise, Effect_d_uninterruptible as uninterruptible, Effect_d_uninterruptibleMask as uninterruptibleMask, Effect_d_unless as unless, Effect_d_unlessEffect as unlessEffect, Effect_d_unsafeMakeLatch as unsafeMakeLatch, Effect_d_unsafeMakeSemaphore as unsafeMakeSemaphore, Effect_d_unsandbox as unsandbox, Effect_d_updateFiberRefs as updateFiberRefs, Effect_d_updateService as updateService, Effect_d_useSpan as useSpan, Effect_d_using as using, Effect_d_validate as validate, Effect_d_validateAll as validateAll, Effect_d_validateFirst as validateFirst, Effect_d_validateWith as validateWith, _void as void, when$1 as when, Effect_d_whenEffect as whenEffect, Effect_d_whenFiberRef as whenFiberRef, Effect_d_whenLogLevel as whenLogLevel, Effect_d_whenRef as whenRef, Effect_d_whileLoop as whileLoop, Effect_d_withClock as withClock, Effect_d_withClockScoped as withClockScoped, Effect_d_withConcurrency as withConcurrency, Effect_d_withConfigProvider as withConfigProvider, Effect_d_withConfigProviderScoped as withConfigProviderScoped, Effect_d_withConsole as withConsole, Effect_d_withConsoleScoped as withConsoleScoped, Effect_d_withEarlyRelease as withEarlyRelease, Effect_d_withExecutionPlan as withExecutionPlan, Effect_d_withFiberRuntime as withFiberRuntime, Effect_d_withLogSpan as withLogSpan, Effect_d_withMaxOpsBeforeYield as withMaxOpsBeforeYield, Effect_d_withMetric as withMetric, Effect_d_withParentSpan as withParentSpan, Effect_d_withRandom as withRandom, Effect_d_withRandomFixed as withRandomFixed, Effect_d_withRandomScoped as withRandomScoped, Effect_d_withRequestBatching as withRequestBatching, Effect_d_withRequestCache as withRequestCache, Effect_d_withRequestCaching as withRequestCaching, Effect_d_withRuntimeFlagsPatch as withRuntimeFlagsPatch, Effect_d_withRuntimeFlagsPatchScoped as withRuntimeFlagsPatchScoped, Effect_d_withScheduler as withScheduler, Effect_d_withSchedulingPriority as withSchedulingPriority, Effect_d_withSpan as withSpan, Effect_d_withSpanScoped as withSpanScoped, Effect_d_withTracer as withTracer, Effect_d_withTracerEnabled as withTracerEnabled, Effect_d_withTracerScoped as withTracerScoped, Effect_d_withTracerTiming as withTracerTiming, Effect_d_withUnhandledErrorLogLevel as withUnhandledErrorLogLevel, Effect_d_yieldNow as yieldNow, Effect_d_zip as zip, Effect_d_zipLeft as zipLeft, Effect_d_zipRight as zipRight, zipWith$1 as zipWith };
    export type { Effect_d_Adapter as Adapter, Effect_d_Blocked as Blocked, Effect_d_EffectGenerator as EffectGenerator, Effect_d_EffectTypeId as EffectTypeId, Effect_d_EffectTypeLambda as EffectTypeLambda, Effect_d_EffectUnify as EffectUnify, Effect_d_EffectUnifyIgnore as EffectUnifyIgnore, Effect_d_FunctionWithSpanOptions as FunctionWithSpanOptions, Effect_d_Latch as Latch, Effect_d_LatchUnify as LatchUnify, Effect_d_LatchUnifyIgnore as LatchUnifyIgnore, Effect_d_Permit as Permit, Effect_d_Semaphore as Semaphore };
}
declare const FiberRefsSym: unique symbol;
type FiberRefsSym = typeof FiberRefsSym;
interface FiberRefs extends Pipeable {
    readonly [FiberRefsSym]: FiberRefsSym;
    readonly locals: Map<FiberRef<any>, NonEmptyReadonlyArray<readonly [
        Single$1,
        any
    ]>>;
}
declare const NodeInspectSymbol: unique symbol;
type NodeInspectSymbol = typeof NodeInspectSymbol;
interface Inspectable {
    toString(): string;
    toJSON(): unknown;
    [NodeInspectSymbol](): unknown;
}
declare const TypeId: unique symbol;
type TypeId = typeof TypeId;
interface Left<out E, out A> extends Pipeable, Inspectable {
    readonly _tag: "Left";
    readonly _op: "Left";
    readonly left: E;
    readonly [TypeId]: {
        readonly _R: Covariant<A>;
        readonly _L: Covariant<E>;
    };
    [typeSymbol]?: unknown;
    [unifySymbol]?: EitherUnify<this>;
    [ignoreSymbol]?: EitherUnifyIgnore;
}
interface Right<out E, out A> extends Pipeable, Inspectable {
    readonly _tag: "Right";
    readonly _op: "Right";
    readonly right: A;
    readonly [TypeId]: {
        readonly _R: Covariant<A>;
        readonly _L: Covariant<E>;
    };
    [typeSymbol]?: unknown;
    [unifySymbol]?: EitherUnify<this>;
    [ignoreSymbol]?: EitherUnifyIgnore;
}
interface EitherUnify<A extends {
    [typeSymbol]?: any;
}> {
    Either?: () => A[typeSymbol] extends Either<infer R0, infer L0> | infer _ ? Either<R0, L0> : never;
}
interface EitherUnifyIgnore {
}
interface EitherTypeLambda extends TypeLambda {
    readonly type: Either<this["Target"], this["Out1"]>;
}
type Either<A, E = never> = Left<E, A> | Right<E, A>;
declare namespace Either {
    type Left<T extends Either<any, any>> = [
        T
    ] extends [
        Either<infer _A, infer _E>
    ] ? _E : never;
    type Right<T extends Either<any, any>> = [
        T
    ] extends [
        Either<infer _A, infer _E>
    ] ? _A : never;
}
declare const right: <A>(a: A) => Either<A>;
declare const void_: Either<void>;
declare const left: <E>(e: E) => Either<never, E>;
declare const fromNullable: {
    <A, E>(onNullable: (right: A) => E): (self: A) => Either<NonNullable<A>, E>;
    <A, E>(self: A, onNullable: (right: A) => E): Either<NonNullable<A>, E>;
};
declare const fromOption: {
    <E>(onNone: () => E): <A>(self: Option<A>) => Either<A, E>;
    <A, E>(self: Option<A>, onNone: () => E): Either<A, E>;
};
declare const try_: {
    <A, E>(options: {
        readonly try: LazyArg<A>;
        readonly catch: (error: unknown) => E;
    }): Either<A, E>;
    <A>(evaluate: LazyArg<A>): Either<A, unknown>;
};
declare const isEither: (input: unknown) => input is Either<unknown, unknown>;
declare const isLeft: <A, E>(self: Either<A, E>) => self is Left<E, A>;
declare const isRight: <A, E>(self: Either<A, E>) => self is Right<E, A>;
declare const getRight: <A, E>(self: Either<A, E>) => Option<A>;
declare const getLeft: <A, E>(self: Either<A, E>) => Option<E>;
declare const getEquivalence: <A, E>({ left, right }: {
    right: Equivalence$1<A>;
    left: Equivalence$1<E>;
}) => Equivalence$1<Either<A, E>>;
declare const mapBoth: {
    <E, E2, A, A2>(options: {
        readonly onLeft: (left: E) => E2;
        readonly onRight: (right: A) => A2;
    }): (self: Either<A, E>) => Either<A2, E2>;
    <A, E, E2, A2>(self: Either<A, E>, options: {
        readonly onLeft: (left: E) => E2;
        readonly onRight: (right: A) => A2;
    }): Either<A2, E2>;
};
declare const mapLeft: {
    <E, E2>(f: (left: E) => E2): <A>(self: Either<A, E>) => Either<A, E2>;
    <A, E, E2>(self: Either<A, E>, f: (left: E) => E2): Either<A, E2>;
};
declare const map: {
    <A, A2>(f: (right: A) => A2): <E>(self: Either<A, E>) => Either<A2, E>;
    <A, E, A2>(self: Either<A, E>, f: (right: A) => A2): Either<A2, E>;
};
declare const match: {
    <E, B, A, C = B>(options: {
        readonly onLeft: (left: E) => B;
        readonly onRight: (right: A) => C;
    }): (self: Either<A, E>) => B | C;
    <A, E, B, C = B>(self: Either<A, E>, options: {
        readonly onLeft: (left: E) => B;
        readonly onRight: (right: A) => C;
    }): B | C;
};
declare const liftPredicate: {
    <A, B extends A, E>(refinement: Refinement<A, B>, orLeftWith: (a: A) => E): (a: A) => Either<B, E>;
    <B extends A, E, A = B>(predicate: Predicate<A>, orLeftWith: (a: A) => E): (a: B) => Either<B, E>;
    <A, E, B extends A>(self: A, refinement: Refinement<A, B>, orLeftWith: (a: A) => E): Either<B, E>;
    <B extends A, E, A = B>(self: B, predicate: Predicate<A>, orLeftWith: (a: A) => E): Either<B, E>;
};
declare const filterOrLeft: {
    <A, B extends A, E2>(refinement: Refinement<NoInfer$1<A>, B>, orLeftWith: (right: NoInfer$1<A>) => E2): <E>(self: Either<A, E>) => Either<B, E2 | E>;
    <A, E2>(predicate: Predicate<NoInfer$1<A>>, orLeftWith: (right: NoInfer$1<A>) => E2): <E>(self: Either<A, E>) => Either<A, E2 | E>;
    <A, E, B extends A, E2>(self: Either<A, E>, refinement: Refinement<A, B>, orLeftWith: (right: A) => E2): Either<B, E | E2>;
    <A, E, E2>(self: Either<A, E>, predicate: Predicate<A>, orLeftWith: (right: A) => E2): Either<A, E | E2>;
};
declare const merge: <A, E>(self: Either<A, E>) => E | A;
declare const getOrElse: {
    <E, A2>(onLeft: (left: E) => A2): <A>(self: Either<A, E>) => A2 | A;
    <A, E, A2>(self: Either<A, E>, onLeft: (left: E) => A2): A | A2;
};
declare const getOrNull: <A, E>(self: Either<A, E>) => A | null;
declare const getOrUndefined: <A, E>(self: Either<A, E>) => A | undefined;
declare const getOrThrowWith: {
    <E>(onLeft: (left: E) => unknown): <A>(self: Either<A, E>) => A;
    <A, E>(self: Either<A, E>, onLeft: (left: E) => unknown): A;
};
declare const getOrThrow: <A, E>(self: Either<A, E>) => A;
declare const orElse$1: {
    <E, A2, E2>(that: (left: E) => Either<A2, E2>): <A>(self: Either<A, E>) => Either<A | A2, E2>;
    <A, E, A2, E2>(self: Either<A, E>, that: (left: E) => Either<A2, E2>): Either<A | A2, E2>;
};
declare const flatMap: {
    <A, A2, E2>(f: (right: A) => Either<A2, E2>): <E>(self: Either<A, E>) => Either<A2, E | E2>;
    <A, E, A2, E2>(self: Either<A, E>, f: (right: A) => Either<A2, E2>): Either<A2, E | E2>;
};
declare const andThen: {
    <A, A2, E2>(f: (right: A) => Either<A2, E2>): <E>(self: Either<A, E>) => Either<A2, E | E2>;
    <A2, E2>(f: Either<A2, E2>): <E, A>(self: Either<A, E>) => Either<A2, E | E2>;
    <A, A2>(f: (right: A) => A2): <E>(self: Either<A, E>) => Either<A2, E>;
    <A2>(right: NotFunction<A2>): <A, E>(self: Either<A, E>) => Either<A2, E>;
    <A, E, A2, E2>(self: Either<A, E>, f: (right: A) => Either<A2, E2>): Either<A2, E | E2>;
    <A, E, A2, E2>(self: Either<A, E>, f: Either<A2, E2>): Either<A2, E | E2>;
    <A, E, A2>(self: Either<A, E>, f: (right: A) => A2): Either<A2, E>;
    <A, E, A2>(self: Either<A, E>, f: NotFunction<A2>): Either<A2, E>;
};
declare const zipWith: {
    <A2, E2, A, B>(that: Either<A2, E2>, f: (right: A, right2: A2) => B): <E>(self: Either<A, E>) => Either<B, E2 | E>;
    <A, E, A2, E2, B>(self: Either<A, E>, that: Either<A2, E2>, f: (right: A, right2: A2) => B): Either<B, E | E2>;
};
declare const ap: {
    <A, E2>(that: Either<A, E2>): <A2, E>(self: Either<(right: A) => A2, E>) => Either<A2, E | E2>;
    <A, A2, E, E2>(self: Either<(right: A) => A2, E>, that: Either<A, E2>): Either<A2, E | E2>;
};
declare const all: <const I extends Iterable<Either<any, any>> | Record<string, Either<any, any>>>(input: I) => [
    I
] extends [
    ReadonlyArray<Either<any, any>>
] ? Either<{
    -readonly [K in keyof I]: [
        I[K]
    ] extends [
        Either<infer A, any>
    ] ? A : never;
}, I[number] extends never ? never : [
    I[number]
] extends [
    Either<any, infer E>
] ? E : never> : [
    I
] extends [
    Iterable<Either<infer A, infer E>>
] ? Either<Array<A>, E> : Either<{
    -readonly [K in keyof I]: [
        I[K]
    ] extends [
        Either<infer A, any>
    ] ? A : never;
}, I[keyof I] extends never ? never : [
    I[keyof I]
] extends [
    Either<any, infer E>
] ? E : never>;
declare const flip: <A, E>(self: Either<A, E>) => Either<E, A>;
declare const gen: Gen<EitherTypeLambda, Adapter$1<EitherTypeLambda>>;
declare const Do: Either<{}>;
declare const bind: {
    <N extends string, A extends object, B, E2>(name: Exclude<N, keyof A>, f: (a: NoInfer$1<A>) => Either<B, E2>): <E>(self: Either<A, E>) => Either<{
        [K in N | keyof A]: K extends keyof A ? A[K] : B;
    }, E | E2>;
    <A extends object, E, N extends string, B, E2>(self: Either<A, E>, name: Exclude<N, keyof A>, f: (a: NoInfer$1<A>) => Either<B, E2>): Either<{
        [K in N | keyof A]: K extends keyof A ? A[K] : B;
    }, E | E2>;
};
declare const bindTo: {
    <N extends string>(name: N): <A, E>(self: Either<A, E>) => Either<{
        [K in N]: A;
    }, E>;
    <A, E, N extends string>(self: Either<A, E>, name: N): Either<{
        [K in N]: A;
    }, E>;
};
declare const let_: {
    <N extends string, A extends object, B>(name: Exclude<N, keyof A>, f: (r: NoInfer$1<A>) => B): <E>(self: Either<A, E>) => Either<{
        [K in N | keyof A]: K extends keyof A ? A[K] : B;
    }, E>;
    <A extends object, E, N extends string, B>(self: Either<A, E>, name: Exclude<N, keyof A>, f: (r: NoInfer$1<A>) => B): Either<{
        [K in N | keyof A]: K extends keyof A ? A[K] : B;
    }, E>;
};
declare const transposeOption: <A = never, E = never>(self: Option<Either<A, E>>) => Either<Option<A>, E>;
declare const transposeMapOption: (<A, B, E = never>(f: (self: A) => Either<B, E>) => (self: Option<A>) => Either<Option<B>, E>) & (<A, B, E = never>(self: Option<A>, f: (self: A) => Either<B, E>) => Either<Option<B>, E>);
declare const Either_d_Do: typeof Do;
import Either_d_Either = Either;
type Either_d_EitherTypeLambda = EitherTypeLambda;
type Either_d_EitherUnify<A extends {
    [typeSymbol]?: any;
}> = EitherUnify<A>;
type Either_d_EitherUnifyIgnore = EitherUnifyIgnore;
type Either_d_Left<out E, out A> = Left<E, A>;
type Either_d_Right<out E, out A> = Right<E, A>;
type Either_d_TypeId = TypeId;
declare const Either_d_all: typeof all;
declare const Either_d_andThen: typeof andThen;
declare const Either_d_ap: typeof ap;
declare const Either_d_bind: typeof bind;
declare const Either_d_bindTo: typeof bindTo;
declare const Either_d_filterOrLeft: typeof filterOrLeft;
declare const Either_d_flatMap: typeof flatMap;
declare const Either_d_flip: typeof flip;
declare const Either_d_fromNullable: typeof fromNullable;
declare const Either_d_fromOption: typeof fromOption;
declare const Either_d_gen: typeof gen;
declare const Either_d_getEquivalence: typeof getEquivalence;
declare const Either_d_getLeft: typeof getLeft;
declare const Either_d_getOrElse: typeof getOrElse;
declare const Either_d_getOrNull: typeof getOrNull;
declare const Either_d_getOrThrow: typeof getOrThrow;
declare const Either_d_getOrThrowWith: typeof getOrThrowWith;
declare const Either_d_getOrUndefined: typeof getOrUndefined;
declare const Either_d_getRight: typeof getRight;
declare const Either_d_isEither: typeof isEither;
declare const Either_d_isLeft: typeof isLeft;
declare const Either_d_isRight: typeof isRight;
declare const Either_d_left: typeof left;
declare const Either_d_liftPredicate: typeof liftPredicate;
declare const Either_d_map: typeof map;
declare const Either_d_mapBoth: typeof mapBoth;
declare const Either_d_mapLeft: typeof mapLeft;
declare const Either_d_match: typeof match;
declare const Either_d_merge: typeof merge;
declare const Either_d_right: typeof right;
declare const Either_d_transposeMapOption: typeof transposeMapOption;
declare const Either_d_transposeOption: typeof transposeOption;
declare const Either_d_zipWith: typeof zipWith;
declare namespace Either_d {
    export { Either_d_Do as Do, Either_d_Either as Either, Either_d_all as all, Either_d_andThen as andThen, Either_d_ap as ap, Either_d_bind as bind, Either_d_bindTo as bindTo, Either_d_filterOrLeft as filterOrLeft, Either_d_flatMap as flatMap, Either_d_flip as flip, Either_d_fromNullable as fromNullable, Either_d_fromOption as fromOption, Either_d_gen as gen, Either_d_getEquivalence as getEquivalence, Either_d_getLeft as getLeft, Either_d_getOrElse as getOrElse, Either_d_getOrNull as getOrNull, Either_d_getOrThrow as getOrThrow, Either_d_getOrThrowWith as getOrThrowWith, Either_d_getOrUndefined as getOrUndefined, Either_d_getRight as getRight, Either_d_isEither as isEither, Either_d_isLeft as isLeft, Either_d_isRight as isRight, Either_d_left as left, let_ as let, Either_d_liftPredicate as liftPredicate, Either_d_map as map, Either_d_mapBoth as mapBoth, Either_d_mapLeft as mapLeft, Either_d_match as match, Either_d_merge as merge, orElse$1 as orElse, Either_d_right as right, Either_d_transposeMapOption as transposeMapOption, Either_d_transposeOption as transposeOption, try_ as try, void_ as void, Either_d_zipWith as zipWith };
    export type { Either_d_EitherTypeLambda as EitherTypeLambda, Either_d_EitherUnify as EitherUnify, Either_d_EitherUnifyIgnore as EitherUnifyIgnore, Either_d_Left as Left, Either_d_Right as Right, Either_d_TypeId as TypeId };
}
type NonEmptyReadonlyArray<A> = readonly [
    A,
    ...Array<A>
];
type NonEmptyArray<A> = [
    A,
    ...Array<A>
];
declare namespace ReadonlyArray$1 {
    type Infer<S extends Iterable<any>> = S extends ReadonlyArray<infer A> ? A : S extends Iterable<infer A> ? A : never;
    type With<S extends Iterable<any>, A> = S extends NonEmptyReadonlyArray<any> ? NonEmptyArray<A> : Array<A>;
    type OrNonEmpty<S extends Iterable<any>, T extends Iterable<any>, A> = S extends NonEmptyReadonlyArray<any> ? NonEmptyArray<A> : T extends NonEmptyReadonlyArray<any> ? NonEmptyArray<A> : Array<A>;
    type AndNonEmpty<S extends Iterable<any>, T extends Iterable<any>, A> = S extends NonEmptyReadonlyArray<any> ? T extends NonEmptyReadonlyArray<any> ? NonEmptyArray<A> : Array<A> : Array<A>;
    type Flatten<T extends ReadonlyArray<ReadonlyArray<any>>> = T extends NonEmptyReadonlyArray<NonEmptyReadonlyArray<any>> ? NonEmptyArray<T[number][number]> : Array<T[number][number]>;
}
declare namespace Case$1 {
    interface Constructor<A, Tag extends keyof A = never> {
        (args: VoidIfEmpty<{
            readonly [P in keyof A as P extends Tag ? never : P]: A[P];
        }>): A;
    }
}
declare const struct: <A extends Record<string, any>>(a: A) => {
    readonly [P in keyof A]: A[P];
};
declare const unsafeStruct: <A extends Record<string, any>>(as: A) => {
    readonly [P in keyof A]: A[P];
};
declare const tuple: <As extends ReadonlyArray<any>>(...as: As) => Readonly<As>;
declare const array: <As extends ReadonlyArray<any>>(as: As) => Readonly<As>;
declare const unsafeArray: <As extends ReadonlyArray<any>>(as: As) => Readonly<As>;
declare const _case: <A>() => Case$1.Constructor<A>;
declare const tagged: <A extends {
    readonly _tag: string;
}>(tag: A["_tag"]) => Case$1.Constructor<A, "_tag">;
declare const Class: new <A extends Record<string, any> = {}>(args: VoidIfEmpty<{
    readonly [P in keyof A]: A[P];
}>) => Readonly<A>;
declare const TaggedClass: <Tag extends string>(tag: Tag) => new <A extends Record<string, any> = {}>(args: VoidIfEmpty<{
    readonly [P in keyof A as P extends "_tag" ? never : P]: A[P];
}>) => Readonly<A> & {
    readonly _tag: Tag;
};
declare const Structural: new <A>(args: VoidIfEmpty<{
    readonly [P in keyof A]: A[P];
}>) => {};
type ChildrenAreTagged<A> = keyof A extends infer K ? K extends keyof A ? "_tag" extends keyof A[K] ? true : false : never : never;
type UntaggedChildren<A> = true extends ChildrenAreTagged<A> ? "It looks like you're trying to create a tagged enum, but one or more of its members already has a `_tag` property." : unknown;
type TaggedEnum<A extends Record<string, Record<string, any>> & UntaggedChildren<A>> = keyof A extends infer Tag ? Tag extends keyof A ? Simplify<{
    readonly _tag: Tag;
} & {
    readonly [K in keyof A[Tag]]: A[Tag][K];
}> : never : never;
declare namespace TaggedEnum {
    interface WithGenerics<Count extends number> {
        readonly taggedEnum: {
            readonly _tag: string;
        };
        readonly numberOfGenerics: Count;
        readonly A: unknown;
        readonly B: unknown;
        readonly C: unknown;
        readonly D: unknown;
    }
    type Kind<Z extends WithGenerics<number>, A = unknown, B = unknown, C = unknown, D = unknown> = (Z & {
        readonly A: A;
        readonly B: B;
        readonly C: C;
        readonly D: D;
    })["taggedEnum"];
    type Args<A extends {
        readonly _tag: string;
    }, K extends A["_tag"], E = Extract<A, {
        readonly _tag: K;
    }>> = {
        readonly [K in keyof E as K extends "_tag" ? never : K]: E[K];
    } extends infer T ? VoidIfEmpty<T> : never;
    type Value<A extends {
        readonly _tag: string;
    }, K extends A["_tag"]> = Extract<A, {
        readonly _tag: K;
    }>;
    type Constructor<A extends {
        readonly _tag: string;
    }> = Simplify<{
        readonly [Tag in A["_tag"]]: Case$1.Constructor<Extract<A, {
            readonly _tag: Tag;
        }>, "_tag">;
    } & {
        readonly $is: <Tag extends A["_tag"]>(tag: Tag) => (u: unknown) => u is Extract<A, {
            readonly _tag: Tag;
        }>;
        readonly $match: {
            <const Cases extends {
                readonly [Tag in A["_tag"]]: (args: Extract<A, {
                    readonly _tag: Tag;
                }>) => any;
            }>(cases: Cases & {
                [K in Exclude<keyof Cases, A["_tag"]>]: never;
            }): (value: A) => Unify<ReturnType<Cases[A["_tag"]]>>;
            <const Cases extends {
                readonly [Tag in A["_tag"]]: (args: Extract<A, {
                    readonly _tag: Tag;
                }>) => any;
            }>(value: A, cases: Cases & {
                [K in Exclude<keyof Cases, A["_tag"]>]: never;
            }): Unify<ReturnType<Cases[A["_tag"]]>>;
        };
    }>;
    interface GenericMatchers<Z extends WithGenerics<number>> {
        readonly $is: <Tag extends Z["taggedEnum"]["_tag"]>(tag: Tag) => {
            <T extends TaggedEnum.Kind<Z, any, any, any, any>>(u: T): u is T & {
                readonly _tag: Tag;
            };
            (u: unknown): u is Extract<TaggedEnum.Kind<Z>, {
                readonly _tag: Tag;
            }>;
        };
        readonly $match: {
            <const Self extends TaggedEnum.Kind<Z, any, any, any, any>, const Cases extends {
                readonly [Tag in Self["_tag"]]: (args: Extract<Self, {
                    readonly _tag: Tag;
                }>) => any;
            }>(self: Self, cases: Cases & {
                [K in Exclude<keyof Cases, Self["_tag"]>]: never;
            }): Unify<ReturnType<Cases[Self["_tag"]]>>;
            <A, B, C, D, Cases extends {
                readonly [Tag in Z["taggedEnum"]["_tag"]]: (args: Extract<TaggedEnum.Kind<Z, A, B, C, D>, {
                    readonly _tag: Tag;
                }>) => any;
            }>(cases: Cases & {
                [K in Exclude<keyof Cases, Z["taggedEnum"]["_tag"]>]: never;
            }): (self: TaggedEnum.Kind<Z, A, B, C, D>) => Unify<ReturnType<Cases[Z["taggedEnum"]["_tag"]]>>;
        };
    }
}
declare const taggedEnum: {
    <Z extends TaggedEnum.WithGenerics<1>>(): Simplify<{
        readonly [Tag in Z["taggedEnum"]["_tag"]]: <A>(args: TaggedEnum.Args<TaggedEnum.Kind<Z, A>, Tag, Extract<TaggedEnum.Kind<Z, A>, {
            readonly _tag: Tag;
        }>>) => TaggedEnum.Value<TaggedEnum.Kind<Z, A>, Tag>;
    } & TaggedEnum.GenericMatchers<Z>>;
    <Z extends TaggedEnum.WithGenerics<2>>(): Simplify<{
        readonly [Tag in Z["taggedEnum"]["_tag"]]: <A, B>(args: TaggedEnum.Args<TaggedEnum.Kind<Z, A, B>, Tag, Extract<TaggedEnum.Kind<Z, A, B>, {
            readonly _tag: Tag;
        }>>) => TaggedEnum.Value<TaggedEnum.Kind<Z, A, B>, Tag>;
    } & TaggedEnum.GenericMatchers<Z>>;
    <Z extends TaggedEnum.WithGenerics<3>>(): Simplify<{
        readonly [Tag in Z["taggedEnum"]["_tag"]]: <A, B, C>(args: TaggedEnum.Args<TaggedEnum.Kind<Z, A, B, C>, Tag, Extract<TaggedEnum.Kind<Z, A, B, C>, {
            readonly _tag: Tag;
        }>>) => TaggedEnum.Value<TaggedEnum.Kind<Z, A, B, C>, Tag>;
    } & TaggedEnum.GenericMatchers<Z>>;
    <Z extends TaggedEnum.WithGenerics<4>>(): Simplify<{
        readonly [Tag in Z["taggedEnum"]["_tag"]]: <A, B, C, D>(args: TaggedEnum.Args<TaggedEnum.Kind<Z, A, B, C, D>, Tag, Extract<TaggedEnum.Kind<Z, A, B, C, D>, {
            readonly _tag: Tag;
        }>>) => TaggedEnum.Value<TaggedEnum.Kind<Z, A, B, C, D>, Tag>;
    } & TaggedEnum.GenericMatchers<Z>>;
    <A extends {
        readonly _tag: string;
    }>(): TaggedEnum.Constructor<A>;
};
declare const Error$1: new <A extends Record<string, any> = {}>(args: VoidIfEmpty<{
    readonly [P in keyof A]: A[P];
}>) => YieldableError & Readonly<A>;
declare const TaggedError: <Tag extends string>(tag: Tag) => new <A extends Record<string, any> = {}>(args: VoidIfEmpty<{
    readonly [P in keyof A as P extends "_tag" ? never : P]: A[P];
}>) => YieldableError & {
    readonly _tag: Tag;
} & Readonly<A>;
declare const Data_d_Class: typeof Class;
declare const Data_d_Structural: typeof Structural;
declare const Data_d_TaggedClass: typeof TaggedClass;
import Data_d_TaggedEnum = TaggedEnum;
declare const Data_d_TaggedError: typeof TaggedError;
declare const Data_d_array: typeof array;
declare const Data_d_struct: typeof struct;
declare const Data_d_tagged: typeof tagged;
declare const Data_d_taggedEnum: typeof taggedEnum;
declare const Data_d_tuple: typeof tuple;
declare const Data_d_unsafeArray: typeof unsafeArray;
declare const Data_d_unsafeStruct: typeof unsafeStruct;
declare namespace Data_d {
    export { Case$1 as Case, Data_d_Class as Class, Error$1 as Error, Data_d_Structural as Structural, Data_d_TaggedClass as TaggedClass, Data_d_TaggedEnum as TaggedEnum, Data_d_TaggedError as TaggedError, Data_d_array as array, _case as case, Data_d_struct as struct, Data_d_tagged as tagged, Data_d_taggedEnum as taggedEnum, Data_d_tuple as tuple, Data_d_unsafeArray as unsafeArray, Data_d_unsafeStruct as unsafeStruct, };
}
declare const MatcherTypeId: unique symbol;
type MatcherTypeId = typeof MatcherTypeId;
type Matcher<Input, Filters, RemainingApplied, Result, Provided, Return = any> = TypeMatcher<Input, Filters, RemainingApplied, Result, Return> | ValueMatcher<Input, Filters, RemainingApplied, Result, Provided, Return>;
interface TypeMatcher<in Input, out Filters, out Remaining, out Result, out Return = any> extends Pipeable {
    readonly _tag: "TypeMatcher";
    readonly [MatcherTypeId]: {
        readonly _input: Contravariant<Input>;
        readonly _filters: Covariant<Filters>;
        readonly _remaining: Covariant<Remaining>;
        readonly _result: Covariant<Result>;
        readonly _return: Covariant<Return>;
    };
    readonly cases: ReadonlyArray<Case>;
    add<I, R, RA, A>(_case: Case): TypeMatcher<I, R, RA, A>;
}
interface ValueMatcher<in Input, out Filters, out Remaining, out Result, out Provided, out Return = any> extends Pipeable {
    readonly _tag: "ValueMatcher";
    readonly [MatcherTypeId]: {
        readonly _input: Contravariant<Input>;
        readonly _filters: Covariant<Filters>;
        readonly _remaining: Covariant<Remaining>;
        readonly _result: Covariant<Result>;
        readonly _provided: Covariant<Result>;
        readonly _return: Covariant<Return>;
    };
    readonly provided: Provided;
    readonly value: Either<Provided, Remaining>;
    add<I, R, RA, A, Pr>(_case: Case): ValueMatcher<I, R, RA, A, Pr>;
}
type Case = When | Not;
interface When {
    readonly _tag: "When";
    guard(u: unknown): boolean;
    evaluate(input: unknown): any;
}
interface Not {
    readonly _tag: "Not";
    guard(u: unknown): boolean;
    evaluate(input: unknown): any;
}
declare const type: <I>() => Matcher<I, Types.Without<never>, I, never, never>;
declare const value: <const I>(i: I) => Matcher<I, Types.Without<never>, I, never, I>;
declare const valueTags: {
    <const I, P extends {
        readonly [Tag in Types.Tags<"_tag", I> & string]: (_: Extract<I, {
            readonly _tag: Tag;
        }>) => any;
    } & {
        readonly [Tag in Exclude<keyof P, Types.Tags<"_tag", I>>]: never;
    }>(fields: P): (input: I) => Unify<ReturnType<P[keyof P]>>;
    <const I, P extends {
        readonly [Tag in Types.Tags<"_tag", I> & string]: (_: Extract<I, {
            readonly _tag: Tag;
        }>) => any;
    } & {
        readonly [Tag in Exclude<keyof P, Types.Tags<"_tag", I>>]: never;
    }>(input: I, fields: P): Unify<ReturnType<P[keyof P]>>;
};
declare const typeTags: {
    <I, Ret>(): <P extends {
        readonly [Tag in Types.Tags<"_tag", I> & string]: (_: Extract<I, {
            readonly _tag: Tag;
        }>) => Ret;
    } & {
        readonly [Tag in Exclude<keyof P, Types.Tags<"_tag", I>>]: never;
    }>(fields: P) => (input: I) => Ret;
    <I>(): <P extends {
        readonly [Tag in Types.Tags<"_tag", I> & string]: (_: Extract<I, {
            readonly _tag: Tag;
        }>) => any;
    } & {
        readonly [Tag in Exclude<keyof P, Types.Tags<"_tag", I>>]: never;
    }>(fields: P) => (input: I) => Unify<ReturnType<P[keyof P]>>;
};
declare const withReturnType: <Ret>() => <I, F, R, A, Pr, _>(self: Matcher<I, F, R, A, Pr, _>) => [
    Ret
] extends [
    [
        A
    ] extends [
        never
    ] ? any : A
] ? Matcher<I, F, R, A, Pr, Ret> : "withReturnType constraint does not extend Result type";
declare const when: <R, const P extends Types.PatternPrimitive<R> | Types.PatternBase<R>, Ret, Fn extends (_: Types.WhenMatch<R, P>) => Ret>(pattern: P, f: Fn) => <I, F, A, Pr>(self: Matcher<I, F, R, A, Pr, Ret>) => Matcher<I, Types.AddWithout<F, Types.PForExclude<P>>, Types.ApplyFilters<I, Types.AddWithout<F, Types.PForExclude<P>>>, A | ReturnType<Fn>, Pr, Ret>;
declare const whenOr: <R, const P extends ReadonlyArray<Types.PatternPrimitive<R> | Types.PatternBase<R>>, Ret, Fn extends (_: Types.WhenMatch<R, P[number]>) => Ret>(...args: [
    ...patterns: P,
    f: Fn
]) => <I, F, A, Pr>(self: Matcher<I, F, R, A, Pr, Ret>) => Matcher<I, Types.AddWithout<F, Types.PForExclude<P[number]>>, Types.ApplyFilters<I, Types.AddWithout<F, Types.PForExclude<P[number]>>>, A | ReturnType<Fn>, Pr, Ret>;
declare const whenAnd: <R, const P extends ReadonlyArray<Types.PatternPrimitive<R> | Types.PatternBase<R>>, Ret, Fn extends (_: Types.WhenMatch<R, UnionToIntersection<P[number]>>) => Ret>(...args: [
    ...patterns: P,
    f: Fn
]) => <I, F, A, Pr>(self: Matcher<I, F, R, A, Pr, Ret>) => Matcher<I, Types.AddWithout<F, Types.PForExclude<UnionToIntersection<P[number]>>>, Types.ApplyFilters<I, Types.AddWithout<F, Types.PForExclude<UnionToIntersection<P[number]>>>>, A | ReturnType<Fn>, Pr>;
declare const discriminator: <D extends string>(field: D) => <R, P extends Types.Tags<D, R> & string, Ret, Fn extends (_: Extract<R, Record<D, P>>) => Ret>(...pattern: [
    first: P,
    ...values: Array<P>,
    f: Fn
]) => <I, F, A, Pr>(self: Matcher<I, F, R, A, Pr, Ret>) => Matcher<I, Types.AddWithout<F, Extract<R, Record<D, P>>>, Types.ApplyFilters<I, Types.AddWithout<F, Extract<R, Record<D, P>>>>, A | ReturnType<Fn>, Pr, Ret>;
declare const discriminatorStartsWith: <D extends string>(field: D) => <R, P extends string, Ret, Fn extends (_: Extract<R, Record<D, `${P}${string}`>>) => Ret>(pattern: P, f: Fn) => <I, F, A, Pr>(self: Matcher<I, F, R, A, Pr, Ret>) => Matcher<I, Types.AddWithout<F, Extract<R, Record<D, `${P}${string}`>>>, Types.ApplyFilters<I, Types.AddWithout<F, Extract<R, Record<D, `${P}${string}`>>>>, A | ReturnType<Fn>, Pr, Ret>;
declare const discriminators: <D extends string>(field: D) => <R, Ret, P extends {
    readonly [Tag in Types.Tags<D, R> & string]?: ((_: Extract<R, Record<D, Tag>>) => Ret) | undefined;
} & {
    readonly [Tag in Exclude<keyof P, Types.Tags<D, R>>]: never;
}>(fields: P) => <I, F, A, Pr>(self: Matcher<I, F, R, A, Pr, Ret>) => Matcher<I, Types.AddWithout<F, Extract<R, Record<D, keyof P>>>, Types.ApplyFilters<I, Types.AddWithout<F, Extract<R, Record<D, keyof P>>>>, A | ReturnType<P[keyof P] & {}>, Pr, Ret>;
declare const discriminatorsExhaustive: <D extends string>(field: D) => <R, Ret, P extends {
    readonly [Tag in Types.Tags<D, R> & string]: (_: Extract<R, Record<D, Tag>>) => Ret;
} & {
    readonly [Tag in Exclude<keyof P, Types.Tags<D, R>>]: never;
}>(fields: P) => <I, F, A, Pr>(self: Matcher<I, F, R, A, Pr, Ret>) => [
    Pr
] extends [
    never
] ? (u: I) => Unify<A | ReturnType<P[keyof P]>> : Unify<A | ReturnType<P[keyof P]>>;
declare const tag: <R, P extends Types.Tags<"_tag", R> & string, Ret, Fn extends (_: Extract<R, Record<"_tag", P>>) => Ret>(...pattern: [
    first: P,
    ...values: Array<P>,
    f: Fn
]) => <I, F, A, Pr>(self: Matcher<I, F, R, A, Pr, Ret>) => Matcher<I, Types.AddWithout<F, Extract<R, Record<"_tag", P>>>, Types.ApplyFilters<I, Types.AddWithout<F, Extract<R, Record<"_tag", P>>>>, ReturnType<Fn> | A, Pr, Ret>;
declare const tagStartsWith: <R, P extends string, Ret, Fn extends (_: Extract<R, Record<"_tag", `${P}${string}`>>) => Ret>(pattern: P, f: Fn) => <I, F, A, Pr>(self: Matcher<I, F, R, A, Pr, Ret>) => Matcher<I, Types.AddWithout<F, Extract<R, Record<"_tag", `${P}${string}`>>>, Types.ApplyFilters<I, Types.AddWithout<F, Extract<R, Record<"_tag", `${P}${string}`>>>>, ReturnType<Fn> | A, Pr, Ret>;
declare const tags: <R, Ret, P extends {
    readonly [Tag in Types.Tags<"_tag", R> & string]?: ((_: Extract<R, Record<"_tag", Tag>>) => Ret) | undefined;
} & {
    readonly [Tag in Exclude<keyof P, Types.Tags<"_tag", R>>]: never;
}>(fields: P) => <I, F, A, Pr>(self: Matcher<I, F, R, A, Pr, Ret>) => Matcher<I, Types.AddWithout<F, Extract<R, Record<"_tag", keyof P>>>, Types.ApplyFilters<I, Types.AddWithout<F, Extract<R, Record<"_tag", keyof P>>>>, A | ReturnType<P[keyof P] & {}>, Pr, Ret>;
declare const tagsExhaustive: <R, Ret, P extends {
    readonly [Tag in Types.Tags<"_tag", R> & string]: (_: Extract<R, Record<"_tag", Tag>>) => Ret;
} & {
    readonly [Tag in Exclude<keyof P, Types.Tags<"_tag", R>>]: never;
}>(fields: P) => <I, F, A, Pr>(self: Matcher<I, F, R, A, Pr, Ret>) => [
    Pr
] extends [
    never
] ? (u: I) => Unify<A | ReturnType<P[keyof P]>> : Unify<A | ReturnType<P[keyof P]>>;
declare const not: <R, const P extends Types.PatternPrimitive<R> | Types.PatternBase<R>, Ret, Fn extends (_: Types.NotMatch<R, P>) => Ret>(pattern: P, f: Fn) => <I, F, A, Pr>(self: Matcher<I, F, R, A, Pr, Ret>) => Matcher<I, Types.AddOnly<F, Types.WhenMatch<R, P>>, Types.ApplyFilters<I, Types.AddOnly<F, Types.WhenMatch<R, P>>>, A | ReturnType<Fn>, Pr, Ret>;
declare const nonEmptyString: SafeRefinement<string, never>;
declare const is: <Literals extends ReadonlyArray<string | number | bigint | boolean | null>>(...literals: Literals) => SafeRefinement<Literals[number]>;
declare const string: Refinement<unknown, string>;
declare const number: Refinement<unknown, number>;
declare const any: SafeRefinement<unknown, any>;
declare const defined: <A>(u: A) => u is A & {};
declare const boolean: Refinement<unknown, boolean>;
declare const _undefined: Refinement<unknown, undefined>;
declare const _null: Refinement<unknown, null>;
declare const bigint: Refinement<unknown, bigint>;
declare const symbol: Refinement<unknown, symbol>;
declare const date: Refinement<unknown, Date>;
declare const record: Refinement<unknown, {
    [x: string | symbol]: unknown;
}>;
declare const instanceOf: <A extends abstract new (...args: any) => any>(constructor: A) => SafeRefinement<InstanceType<A>, never>;
declare const instanceOfUnsafe: <A extends abstract new (...args: any) => any>(constructor: A) => SafeRefinement<InstanceType<A>, InstanceType<A>>;
declare const orElse: <RA, Ret, F extends (_: RA) => Ret>(f: F) => <I, R, A, Pr>(self: Matcher<I, R, RA, A, Pr, Ret>) => [
    Pr
] extends [
    never
] ? (input: I) => Unify<ReturnType<F> | A> : Unify<ReturnType<F> | A>;
declare const orElseAbsurd: <I, R, RA, A, Pr, Ret>(self: Matcher<I, R, RA, A, Pr, Ret>) => [
    Pr
] extends [
    never
] ? (input: I) => Unify<A> : Unify<A>;
declare const either: <I, F, R, A, Pr, Ret>(self: Matcher<I, F, R, A, Pr, Ret>) => [
    Pr
] extends [
    never
] ? (input: I) => Either<Unify<A>, R> : Either<Unify<A>, R>;
declare const option: <I, F, R, A, Pr, Ret>(self: Matcher<I, F, R, A, Pr, Ret>) => [
    Pr
] extends [
    never
] ? (input: I) => Option<Unify<A>> : Option<Unify<A>>;
declare const exhaustive: <I, F, A, Pr, Ret>(self: Matcher<I, F, never, A, Pr, Ret>) => [
    Pr
] extends [
    never
] ? (u: I) => Unify<A> : Unify<A>;
declare const SafeRefinementId: unique symbol;
type SafeRefinementId = typeof SafeRefinementId;
interface SafeRefinement<in A, out R = A> {
    readonly [SafeRefinementId]: (a: A) => R;
}
declare const Fail: unique symbol;
type Fail = typeof Fail;
declare namespace Types {
    type WhenMatch<R, P> = [
        0
    ] extends [
        1 & R
    ] ? ResolvePred<P> : P extends SafeRefinement<infer SP, never> ? SP : P extends Refinement<infer _R, infer RP> ? [
        Extract<R, RP>
    ] extends [
        infer X
    ] ? [
        X
    ] extends [
        never
    ] ? RP : X : never : P extends PredicateA<infer PP> ? PP : ExtractMatch<R, P>;
    type NotMatch<R, P> = Exclude<R, ExtractMatch<R, PForNotMatch<P>>>;
    type PForNotMatch<P> = [
        ToInvertedRefinement<P>
    ] extends [
        infer X
    ] ? X : never;
    type PForMatch<P> = [
        ResolvePred<P>
    ] extends [
        infer X
    ] ? X : never;
    type PForExclude<P> = [
        SafeRefinementR<ToSafeRefinement<P>>
    ] extends [
        infer X
    ] ? X : never;
    type PredicateA<A> = Predicate<A> | Refinement<A, A>;
    type SafeRefinementR<A> = A extends never ? never : A extends SafeRefinement<infer _, infer R> ? R : A extends Function ? A : A extends Record<string, any> ? {
        [K in keyof A]: SafeRefinementR<A[K]>;
    } : A;
    type ResolvePred<A, Input = any> = A extends never ? never : A extends SafeRefinement<infer _A, infer _R> ? _A : A extends Refinement<Input, infer P> ? P : A extends Predicate<infer P> ? P : A extends Record<string, any> ? {
        [K in keyof A]: ResolvePred<A[K]>;
    } : A;
    type ToSafeRefinement<A> = A extends never ? never : A extends Refinement<any, infer P> ? SafeRefinement<P, P> : A extends Predicate<infer P> ? SafeRefinement<P, never> : A extends SafeRefinement<any> ? A : A extends Record<string, any> ? {
        [K in keyof A]: ToSafeRefinement<A[K]>;
    } : NonLiteralsTo<A, never>;
    type ToInvertedRefinement<A> = A extends never ? never : A extends Refinement<any, infer P> ? SafeRefinement<P> : A extends Predicate<infer _P> ? SafeRefinement<never> : A extends SafeRefinement<infer _A, infer _R> ? SafeRefinement<_R> : A extends Record<string, any> ? {
        [K in keyof A]: ToInvertedRefinement<A[K]>;
    } : NonLiteralsTo<A, never>;
    type NonLiteralsTo<A, T> = [
        A
    ] extends [
        string | number | boolean | bigint
    ] ? [
        string
    ] extends [
        A
    ] ? T : [
        number
    ] extends [
        A
    ] ? T : [
        boolean
    ] extends [
        A
    ] ? T : [
        bigint
    ] extends [
        A
    ] ? T : A : A;
    type PatternBase<A> = A extends ReadonlyArray<infer _T> ? ReadonlyArray<any> | PatternPrimitive<A> : A extends Record<string, any> ? Partial<{
        [K in keyof A]: PatternPrimitive<A[K] & {}> | PatternBase<A[K] & {}>;
    }> : never;
    type PatternPrimitive<A> = PredicateA<A> | A | SafeRefinement<any>;
    interface Without<out X> {
        readonly _tag: "Without";
        readonly _X: X;
    }
    interface Only<out X> {
        readonly _tag: "Only";
        readonly _X: X;
    }
    type AddWithout<A, X> = [
        A
    ] extends [
        Without<infer WX>
    ] ? Without<X | WX> : [
        A
    ] extends [
        Only<infer OX>
    ] ? Only<Exclude<OX, X>> : never;
    type AddOnly<A, X> = [
        A
    ] extends [
        Without<infer WX>
    ] ? [
        X
    ] extends [
        WX
    ] ? never : Only<X> : [
        A
    ] extends [
        Only<infer OX>
    ] ? [
        X
    ] extends [
        OX
    ] ? Only<X> : never : never;
    type ApplyFilters<I, A> = A extends Only<infer X> ? X : A extends Without<infer X> ? Exclude<I, X> : never;
    type Tags<D extends string, P> = P extends Record<D, infer X> ? X : never;
    type ArrayToIntersection<A extends ReadonlyArray<any>> = UnionToIntersection<A[number]>;
    type ExtractMatch<I, P> = [
        ExtractAndNarrow<I, P>
    ] extends [
        infer EI
    ] ? EI : never;
    type Replace<A, B> = A extends Function ? A : A extends Record<string | number, any> ? {
        [K in keyof A]: K extends keyof B ? Replace<A[K], B[K]> : A[K];
    } : [
        B
    ] extends [
        A
    ] ? B : A;
    type MaybeReplace<I, P> = [
        P
    ] extends [
        I
    ] ? P : [
        I
    ] extends [
        P
    ] ? Replace<I, P> : Fail;
    type BuiltInObjects = Function | Date | RegExp | Generator | {
        readonly [Symbol.toStringTag]: string;
    };
    type IsPlainObject<T> = T extends BuiltInObjects ? false : T extends Record<string, any> ? true : false;
    type Simplify<A> = {
        [K in keyof A]: A[K];
    } & {};
    type ExtractAndNarrow<Input, P> = P extends Refinement<infer _In, infer _Out> ? _Out extends Input ? Extract<_Out, Input> : Extract<Input, _Out> : P extends SafeRefinement<infer _In, infer _R> ? [
        0
    ] extends [
        1 & _R
    ] ? Input : _In extends Input ? Extract<_In, Input> : Extract<Input, _In> : P extends Predicate<infer _In> ? Extract<Input, _In> : Input extends infer I ? Exclude<I extends ReadonlyArray<any> ? P extends ReadonlyArray<any> ? {
        readonly [K in keyof I]: K extends keyof P ? ExtractAndNarrow<I[K], P[K]> : I[K];
    } extends infer R ? Fail extends R[keyof R] ? never : R : never : never : IsPlainObject<I> extends true ? string extends keyof I ? I extends P ? I : never : symbol extends keyof I ? I extends P ? I : never : Simplify<{
        [RK in Extract<keyof I, keyof P>]-?: ExtractAndNarrow<I[RK], P[RK]>;
    } & Omit<I, keyof P>> extends infer R ? keyof P extends NonFailKeys<R> ? R : never : never : MaybeReplace<I, P> extends infer R ? [
        I
    ] extends [
        R
    ] ? I : R : never, Fail> : never;
    type NonFailKeys<A> = keyof A & {} extends infer K ? K extends keyof A ? A[K] extends Fail ? never : K : never : never;
}
type Match_d_Case = Case;
type Match_d_Matcher<Input, Filters, RemainingApplied, Result, Provided, Return = any> = Matcher<Input, Filters, RemainingApplied, Result, Provided, Return>;
type Match_d_MatcherTypeId = MatcherTypeId;
type Match_d_Not = Not;
type Match_d_SafeRefinement<in A, out R = A> = SafeRefinement<A, R>;
type Match_d_SafeRefinementId = SafeRefinementId;
type Match_d_TypeMatcher<in Input, out Filters, out Remaining, out Result, out Return = any> = TypeMatcher<Input, Filters, Remaining, Result, Return>;
import Match_d_Types = Types;
type Match_d_ValueMatcher<in Input, out Filters, out Remaining, out Result, out Provided, out Return = any> = ValueMatcher<Input, Filters, Remaining, Result, Provided, Return>;
type Match_d_When = When;
declare const Match_d_any: typeof any;
declare const Match_d_bigint: typeof bigint;
declare const Match_d_boolean: typeof boolean;
declare const Match_d_date: typeof date;
declare const Match_d_defined: typeof defined;
declare const Match_d_discriminator: typeof discriminator;
declare const Match_d_discriminatorStartsWith: typeof discriminatorStartsWith;
declare const Match_d_discriminators: typeof discriminators;
declare const Match_d_discriminatorsExhaustive: typeof discriminatorsExhaustive;
declare const Match_d_either: typeof either;
declare const Match_d_exhaustive: typeof exhaustive;
declare const Match_d_instanceOf: typeof instanceOf;
declare const Match_d_instanceOfUnsafe: typeof instanceOfUnsafe;
declare const Match_d_is: typeof is;
declare const Match_d_nonEmptyString: typeof nonEmptyString;
declare const Match_d_not: typeof not;
declare const Match_d_number: typeof number;
declare const Match_d_option: typeof option;
declare const Match_d_orElse: typeof orElse;
declare const Match_d_orElseAbsurd: typeof orElseAbsurd;
declare const Match_d_record: typeof record;
declare const Match_d_string: typeof string;
declare const Match_d_symbol: typeof symbol;
declare const Match_d_tag: typeof tag;
declare const Match_d_tagStartsWith: typeof tagStartsWith;
declare const Match_d_tags: typeof tags;
declare const Match_d_tagsExhaustive: typeof tagsExhaustive;
declare const Match_d_type: typeof type;
declare const Match_d_typeTags: typeof typeTags;
declare const Match_d_value: typeof value;
declare const Match_d_valueTags: typeof valueTags;
declare const Match_d_when: typeof when;
declare const Match_d_whenAnd: typeof whenAnd;
declare const Match_d_whenOr: typeof whenOr;
declare const Match_d_withReturnType: typeof withReturnType;
declare namespace Match_d {
    export { Match_d_Types as Types, Match_d_any as any, Match_d_bigint as bigint, Match_d_boolean as boolean, Match_d_date as date, Match_d_defined as defined, Match_d_discriminator as discriminator, Match_d_discriminatorStartsWith as discriminatorStartsWith, Match_d_discriminators as discriminators, Match_d_discriminatorsExhaustive as discriminatorsExhaustive, Match_d_either as either, Match_d_exhaustive as exhaustive, Match_d_instanceOf as instanceOf, Match_d_instanceOfUnsafe as instanceOfUnsafe, Match_d_is as is, Match_d_nonEmptyString as nonEmptyString, Match_d_not as not, _null as null, Match_d_number as number, Match_d_option as option, Match_d_orElse as orElse, Match_d_orElseAbsurd as orElseAbsurd, Match_d_record as record, Match_d_string as string, Match_d_symbol as symbol, Match_d_tag as tag, Match_d_tagStartsWith as tagStartsWith, Match_d_tags as tags, Match_d_tagsExhaustive as tagsExhaustive, Match_d_type as type, Match_d_typeTags as typeTags, _undefined as undefined, Match_d_value as value, Match_d_valueTags as valueTags, Match_d_when as when, Match_d_whenAnd as whenAnd, Match_d_whenOr as whenOr, Match_d_withReturnType as withReturnType };
    export type { Match_d_Case as Case, Match_d_Matcher as Matcher, Match_d_MatcherTypeId as MatcherTypeId, Match_d_Not as Not, Match_d_SafeRefinement as SafeRefinement, Match_d_SafeRefinementId as SafeRefinementId, Match_d_TypeMatcher as TypeMatcher, Match_d_ValueMatcher as ValueMatcher, Match_d_When as When };
}
export { Cause_d as Cause, Context_d as Context, Data_d as Data, Duration_d as Duration, Effect_d as Effect, Either_d as Either, Exit_d as Exit, Layer_d as Layer, Match_d as Match, Option_d as Option, Ref_d as Ref, Schedule_d as Schedule, pipe };
