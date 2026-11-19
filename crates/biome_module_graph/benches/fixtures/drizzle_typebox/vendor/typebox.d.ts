export declare const TransformKind: unique symbol;
export declare const ReadonlyKind: unique symbol;
export declare const OptionalKind: unique symbol;
export declare const Hint: unique symbol;
export declare const Kind: unique symbol;
export interface TAny extends TSchema {
    [Kind]: "Any";
    static: any;
}
export declare function Any(options?: SchemaOptions): TAny;
export interface TMappedKey<T extends PropertyKey[] = PropertyKey[]> extends TSchema {
    [Kind]: "MappedKey";
    static: T[number];
    keys: T;
}
export declare function MappedKey<T extends PropertyKey[]>(T: [
    ...T
]): TMappedKey<T>;
export interface TMappedResult<T extends TProperties = TProperties> extends TSchema {
    [Kind]: "MappedResult";
    properties: T;
    static: unknown;
}
export declare function MappedResult<T extends TProperties>(properties: T): TMappedResult<T>;
export interface TAsyncIterator<T extends TSchema = TSchema> extends TSchema {
    [Kind]: "AsyncIterator";
    static: AsyncIterableIterator<Static<T, this["params"]>>;
    type: "AsyncIterator";
    items: T;
}
declare function AsyncIterator$1<T extends TSchema>(items: T, options?: SchemaOptions): TAsyncIterator<T>;
type TRemoveReadonly<T extends TSchema> = T extends TReadonly<infer S> ? S : T;
type TAddReadonly<T extends TSchema> = T extends TReadonly<infer S> ? TReadonly<S> : Ensure<TReadonly<T>>;
export type TReadonlyWithFlag<T extends TSchema, F extends boolean> = F extends false ? TRemoveReadonly<T> : TAddReadonly<T>;
export type TReadonly<T extends TSchema> = T & {
    [ReadonlyKind]: "Readonly";
};
declare function Readonly$1<T extends TMappedResult, F extends boolean>(schema: T, enable: F): TReadonlyFromMappedResult<T, F>;
declare function Readonly$1<T extends TSchema, F extends boolean>(schema: T, enable: F): TReadonlyWithFlag<T, F>;
declare function Readonly$1<T extends TMappedResult>(schema: T): TReadonlyFromMappedResult<T, true>;
declare function Readonly$1<T extends TSchema>(schema: T): TReadonlyWithFlag<T, true>;
type TFromProperties<P extends TProperties, F extends boolean> = ({
    [K2 in keyof P]: TReadonlyWithFlag<P[K2], F>;
});
type TFromMappedResult<R extends TMappedResult, F extends boolean> = (TFromProperties<R["properties"], F>);
export type TReadonlyFromMappedResult<R extends TMappedResult, F extends boolean, P extends TProperties = TFromMappedResult<R, F>> = (TMappedResult<P>);
export declare function ReadonlyFromMappedResult<R extends TMappedResult, F extends boolean, P extends TProperties = TFromMappedResult<R, F>>(R: R, F: F): TMappedResult<P>;
export type TReadonlyOptional<T extends TSchema> = TOptional<T> & TReadonly<T>;
export declare function ReadonlyOptional<T extends TSchema>(schema: T): TReadonly<TOptional<T>>;
type StaticReturnType<U extends TSchema, P extends unknown[]> = Static<U, P>;
type StaticParameter<T extends TSchema, P extends unknown[]> = T extends TReadonlyOptional<T> ? [
    Readonly<Static<T, P>>?
] : T extends TReadonly<T> ? [
    Readonly<Static<T, P>>
] : T extends TOptional<T> ? [
    Static<T, P>?
] : [
    Static<T, P>
];
type StaticParameters<T extends TSchema[], P extends unknown[], Acc extends unknown[] = [
]> = (T extends [
    infer L extends TSchema,
    ...infer R extends TSchema[]
] ? StaticParameters<R, P, [
    ...Acc,
    ...StaticParameter<L, P>
]> : Acc);
type StaticConstructor<T extends TSchema[], U extends TSchema, P extends unknown[]> = Ensure<new (...param: StaticParameters<T, P>) => StaticReturnType<U, P>>;
export interface TConstructor<T extends TSchema[] = TSchema[], U extends TSchema = TSchema> extends TSchema {
    [Kind]: "Constructor";
    static: StaticConstructor<T, U, this["params"]>;
    type: "Constructor";
    parameters: T;
    returns: U;
}
export declare function Constructor<T extends TSchema[], U extends TSchema>(parameters: [
    ...T
], returns: U, options?: SchemaOptions): TConstructor<T, U>;
export type TLiteralValue = boolean | number | string;
export interface TLiteral<T extends TLiteralValue = TLiteralValue> extends TSchema {
    [Kind]: "Literal";
    static: T;
    const: T;
}
export declare function Literal<T extends TLiteralValue>(value: T, options?: SchemaOptions): TLiteral<T>;
export type TEnumRecord = Record<TEnumKey, TEnumValue>;
export type TEnumValue = string | number;
export type TEnumKey = string;
export interface TEnum<T extends Record<string, string | number> = Record<string, string | number>> extends TSchema {
    [Kind]: "Union";
    [Hint]: "Enum";
    static: T[keyof T];
    anyOf: TLiteral<T[keyof T]>[];
}
export declare function Enum<V extends TEnumValue, T extends Record<TEnumKey, V>>(item: T, options?: SchemaOptions): TEnum<T>;
type StaticReturnType$1<U extends TSchema, P extends unknown[]> = Static<U, P>;
type StaticParameter$1<T extends TSchema, P extends unknown[]> = T extends TReadonlyOptional<T> ? [
    Readonly<Static<T, P>>?
] : T extends TReadonly<T> ? [
    Readonly<Static<T, P>>
] : T extends TOptional<T> ? [
    Static<T, P>?
] : [
    Static<T, P>
];
type StaticParameters$1<T extends TSchema[], P extends unknown[], Acc extends unknown[] = [
]> = (T extends [
    infer L extends TSchema,
    ...infer R extends TSchema[]
] ? StaticParameters$1<R, P, [
    ...Acc,
    ...StaticParameter$1<L, P>
]> : Acc);
type StaticFunction<T extends TSchema[], U extends TSchema, P extends unknown[]> = Ensure<(...param: StaticParameters$1<T, P>) => StaticReturnType$1<U, P>>;
export interface TFunction<T extends TSchema[] = TSchema[], U extends TSchema = TSchema> extends TSchema {
    [Kind]: "Function";
    static: StaticFunction<T, U, this["params"]>;
    type: "Function";
    parameters: T;
    returns: U;
}
declare function Function$1<T extends TSchema[], U extends TSchema>(parameters: [
    ...T
], returns: U, options?: SchemaOptions): TFunction<T, U>;
interface TComputed<Target extends string = string, Parameters extends TSchema[] = [
]> extends TSchema {
    [Kind]: "Computed";
    target: Target;
    parameters: Parameters;
}
export interface TNever extends TSchema {
    [Kind]: "Never";
    static: never;
    not: {};
}
export declare function Never(options?: SchemaOptions): TNever;
type TIntersectStatic<T extends TSchema[], P extends unknown[], Acc extends unknown = unknown> = T extends [
    infer L extends TSchema,
    ...infer R extends TSchema[]
] ? TIntersectStatic<R, P, Acc & Static<L, P>> : Acc;
export type TUnevaluatedProperties = undefined | TSchema | boolean;
export interface IntersectOptions extends SchemaOptions {
    unevaluatedProperties?: TUnevaluatedProperties;
}
export interface TIntersect<T extends TSchema[] = TSchema[]> extends TSchema, IntersectOptions {
    [Kind]: "Intersect";
    static: TIntersectStatic<T, this["params"]>;
    type?: "object";
    allOf: [
        ...T
    ];
}
type TIsIntersectOptional<Types extends TSchema[]> = (Types extends [
    infer Left extends TSchema,
    ...infer Right extends TSchema[]
] ? Left extends TOptional<TSchema> ? TIsIntersectOptional<Right> : false : true);
type TRemoveOptionalFromType<Type extends TSchema> = (Type extends TReadonly<infer Type extends TSchema> ? TReadonly<TRemoveOptionalFromType<Type>> : Type extends TOptional<infer Type extends TSchema> ? TRemoveOptionalFromType<Type> : Type);
type TRemoveOptionalFromRest<Types extends TSchema[], Result extends TSchema[] = [
]> = (Types extends [
    infer Left extends TSchema,
    ...infer Right extends TSchema[]
] ? Left extends TOptional<infer Type extends TSchema> ? TRemoveOptionalFromRest<Right, [
    ...Result,
    TRemoveOptionalFromType<Type>
]> : TRemoveOptionalFromRest<Right, [
    ...Result,
    Left
]> : Result);
type TResolveIntersect<Types extends TSchema[]> = (TIsIntersectOptional<Types> extends true ? TOptional<TIntersect<TRemoveOptionalFromRest<Types>>> : TIntersect<TRemoveOptionalFromRest<Types>>);
export type TIntersectEvaluated<Types extends TSchema[]> = (Types extends [
    TSchema
] ? Types[0] : Types extends [
] ? TNever : TResolveIntersect<Types>);
export declare function IntersectEvaluated<Types extends TSchema[], Result extends TSchema = TIntersectEvaluated<Types>>(types: [
    ...Types
], options?: IntersectOptions): Result;
export type Intersect<Types extends TSchema[]> = (Types extends [
    TSchema
] ? Types[0] : Types extends [
] ? TNever : TIntersect<Types>);
export declare function Intersect<Types extends TSchema[]>(types: [
    ...Types
], options?: IntersectOptions): Intersect<Types>;
type UnionStatic<T extends TSchema[], P extends unknown[]> = {
    [K in keyof T]: T[K] extends TSchema ? Static<T[K], P> : never;
}[number];
export interface TUnion<T extends TSchema[] = TSchema[]> extends TSchema {
    [Kind]: "Union";
    static: UnionStatic<T, this["params"]>;
    anyOf: T;
}
type TIsUnionOptional<Types extends TSchema[]> = (Types extends [
    infer Left extends TSchema,
    ...infer Right extends TSchema[]
] ? Left extends TOptional<TSchema> ? true : TIsUnionOptional<Right> : false);
type TRemoveOptionalFromRest$1<Types extends TSchema[], Result extends TSchema[] = [
]> = (Types extends [
    infer Left extends TSchema,
    ...infer Right extends TSchema[]
] ? Left extends TOptional<infer S extends TSchema> ? TRemoveOptionalFromRest$1<Right, [
    ...Result,
    TRemoveOptionalFromType$1<S>
]> : TRemoveOptionalFromRest$1<Right, [
    ...Result,
    Left
]> : Result);
type TRemoveOptionalFromType$1<Type extends TSchema> = (Type extends TReadonly<infer Type extends TSchema> ? TReadonly<TRemoveOptionalFromType$1<Type>> : Type extends TOptional<infer Type extends TSchema> ? TRemoveOptionalFromType$1<Type> : Type);
type TResolveUnion<Types extends TSchema[], Result extends TSchema[] = TRemoveOptionalFromRest$1<Types>, IsOptional extends boolean = TIsUnionOptional<Types>> = (IsOptional extends true ? TOptional<TUnion<Result>> : TUnion<Result>);
export type TUnionEvaluated<Types extends TSchema[]> = (Types extends [
    TSchema
] ? Types[0] : Types extends [
] ? TNever : TResolveUnion<Types>);
export declare function UnionEvaluated<Types extends TSchema[], Result = TUnionEvaluated<Types>>(T: [
    ...Types
], options?: SchemaOptions): Result;
export type Union<T extends TSchema[]> = (T extends [
] ? TNever : T extends [
    TSchema
] ? T[0] : TUnion<T>);
export declare function Union<Types extends TSchema[]>(types: [
    ...Types
], options?: SchemaOptions): Union<Types>;
export interface TThis extends TSchema {
    [Kind]: "This";
    static: this["params"][0];
    $ref: string;
}
type RecursiveStatic<T extends TSchema> = Static<T, [
    RecursiveStatic<T>
]>;
export interface TRecursive<T extends TSchema> extends TSchema {
    [Hint]: "Recursive";
    static: RecursiveStatic<T>;
}
export declare function Recursive<T extends TSchema>(callback: (thisType: TThis) => T, options?: SchemaOptions): TRecursive<T>;
export interface UnsafeOptions extends SchemaOptions {
    [Kind]?: string;
}
export interface TUnsafe<T> extends TSchema {
    [Kind]: string;
    static: T;
}
export declare function Unsafe<T>(options?: UnsafeOptions): TUnsafe<T>;
export interface TRef<Ref extends string = string> extends TSchema {
    [Kind]: "Ref";
    static: unknown;
    $ref: Ref;
}
export type TRefUnsafe<Type extends TSchema> = TUnsafe<Static<Type>>;
export declare function Ref<Ref extends string>($ref: Ref, options?: SchemaOptions): TRef<Ref>;
export declare function Ref<Type extends TSchema>(type: Type, options?: SchemaOptions): TRefUnsafe<Type>;
type TupleStatic<T extends TSchema[], P extends unknown[], Acc extends unknown[] = [
]> = T extends [
    infer L extends TSchema,
    ...infer R extends TSchema[]
] ? TupleStatic<R, P, [
    ...Acc,
    Static<L, P>
]> : Acc;
export interface TTuple<T extends TSchema[] = TSchema[]> extends TSchema {
    [Kind]: "Tuple";
    static: TupleStatic<T, this["params"]>;
    type: "array";
    items: T;
    additionalItems?: false;
    minItems: T["length"];
    maxItems: T["length"];
}
export declare function Tuple<Types extends TSchema[]>(types: [
    ...Types
], options?: SchemaOptions): TTuple<Types>;
export declare class TypeBoxError extends Error {
    constructor(message: string);
}
export type StringFormatOption = "date-time" | "time" | "date" | "email" | "idn-email" | "hostname" | "idn-hostname" | "ipv4" | "ipv6" | "uri" | "uri-reference" | "iri" | "uuid" | "iri-reference" | "uri-template" | "json-pointer" | "relative-json-pointer" | "regex" | ({} & string);
export type StringContentEncodingOption = "7bit" | "8bit" | "binary" | "quoted-printable" | "base64" | ({} & string);
export interface StringOptions extends SchemaOptions {
    maxLength?: number;
    minLength?: number;
    pattern?: string;
    format?: StringFormatOption;
    contentEncoding?: StringContentEncodingOption;
    contentMediaType?: string;
}
export interface TString extends TSchema, StringOptions {
    [Kind]: "String";
    static: string;
    type: "string";
}
declare function String$1(options?: StringOptions): TString;
export interface TBoolean extends TSchema {
    [Kind]: "Boolean";
    static: boolean;
    type: "boolean";
}
declare function Boolean$1(options?: SchemaOptions): TBoolean;
export interface NumberOptions extends SchemaOptions {
    exclusiveMaximum?: number;
    exclusiveMinimum?: number;
    maximum?: number;
    minimum?: number;
    multipleOf?: number;
}
export interface TNumber extends TSchema, NumberOptions {
    [Kind]: "Number";
    static: number;
    type: "number";
}
declare function Number$1(options?: NumberOptions): TNumber;
export interface IntegerOptions extends SchemaOptions {
    exclusiveMaximum?: number;
    exclusiveMinimum?: number;
    maximum?: number;
    minimum?: number;
    multipleOf?: number;
}
export interface TInteger extends TSchema, IntegerOptions {
    [Kind]: "Integer";
    static: number;
    type: "integer";
}
export declare function Integer(options?: IntegerOptions): TInteger;
export interface BigIntOptions extends SchemaOptions {
    exclusiveMaximum?: bigint;
    exclusiveMinimum?: bigint;
    maximum?: bigint;
    minimum?: bigint;
    multipleOf?: bigint;
}
export interface TBigInt extends TSchema, BigIntOptions {
    [Kind]: "BigInt";
    static: bigint;
    type: "bigint";
}
declare function BigInt$1(options?: BigIntOptions): TBigInt;
export declare class TemplateLiteralParserError extends TypeBoxError {
}
export type Expression = ExpressionAnd | ExpressionOr | ExpressionConst;
export type ExpressionConst = {
    type: "const";
    const: string;
};
export type ExpressionAnd = {
    type: "and";
    expr: Expression[];
};
export type ExpressionOr = {
    type: "or";
    expr: Expression[];
};
export declare function TemplateLiteralParse(pattern: string): Expression;
export declare function TemplateLiteralParseExact(pattern: string): Expression;
export declare class TemplateLiteralFiniteError extends TypeBoxError {
}
type TFromTemplateLiteralKind<T> = T extends TTemplateLiteral<infer U extends TTemplateLiteralKind[]> ? TFromTemplateLiteralKinds<U> : T extends TUnion<infer U extends TTemplateLiteralKind[]> ? TFromTemplateLiteralKinds<U> : T extends TString ? false : T extends TNumber ? false : T extends TInteger ? false : T extends TBigInt ? false : T extends TBoolean ? true : T extends TLiteral ? true : false;
type TFromTemplateLiteralKinds<T extends TTemplateLiteralKind[]> = T extends [
    infer L extends TTemplateLiteralKind,
    ...infer R extends TTemplateLiteralKind[]
] ? TFromTemplateLiteralKind<L> extends false ? false : TFromTemplateLiteralKinds<R> : true;
export declare function IsTemplateLiteralExpressionFinite(expression: Expression): boolean;
export type TIsTemplateLiteralFinite<T> = T extends TTemplateLiteral<infer U> ? TFromTemplateLiteralKinds<U> : false;
export declare function IsTemplateLiteralFinite<T extends TTemplateLiteral>(schema: T): boolean;
export declare class TemplateLiteralGenerateError extends TypeBoxError {
}
type TStringReduceUnary<L extends string, R extends string[], Acc extends string[] = [
]> = R extends [
    infer A extends string,
    ...infer B extends string[]
] ? TStringReduceUnary<L, B, [
    ...Acc,
    `${L}${A}`
]> : Acc;
type TStringReduceBinary<L extends string[], R extends string[], Acc extends string[] = [
]> = L extends [
    infer A extends string,
    ...infer B extends string[]
] ? TStringReduceBinary<B, R, [
    ...Acc,
    ...TStringReduceUnary<A, R>
]> : Acc;
type TStringReduceMany<T extends string[][]> = T extends [
    infer L extends string[],
    infer R extends string[],
    ...infer Rest extends string[][]
] ? TStringReduceMany<[
    TStringReduceBinary<L, R>,
    ...Rest
]> : T;
type TStringReduce<T extends string[][], O = TStringReduceMany<T>> = 0 extends keyof O ? Assert<O[0], string[]> : [
];
type TFromTemplateLiteralUnionKinds<T extends TTemplateLiteralKind[]> = T extends [
    infer L extends TLiteral,
    ...infer R extends TLiteral[]
] ? [
    `${L["const"]}`,
    ...TFromTemplateLiteralUnionKinds<R>
] : [
];
type TFromTemplateLiteralKinds$1<T extends TTemplateLiteralKind[], Acc extends TLiteralValue[][] = [
]> = T extends [
    infer L extends TTemplateLiteralKind,
    ...infer R extends TTemplateLiteralKind[]
] ? (L extends TTemplateLiteral<infer S extends TTemplateLiteralKind[]> ? TFromTemplateLiteralKinds$1<[
    ...S,
    ...R
], Acc> : L extends TLiteral<infer S extends TLiteralValue> ? TFromTemplateLiteralKinds$1<R, [
    ...Acc,
    [
        S
    ]
]> : L extends TUnion<infer S extends TTemplateLiteralKind[]> ? TFromTemplateLiteralKinds$1<R, [
    ...Acc,
    TFromTemplateLiteralUnionKinds<S>
]> : L extends TBoolean ? TFromTemplateLiteralKinds$1<R, [
    ...Acc,
    [
        "true",
        "false"
    ]
]> : Acc) : Acc;
export declare function TemplateLiteralExpressionGenerate(expression: Expression): IterableIterator<string>;
export type TTemplateLiteralGenerate<T extends TTemplateLiteral, F = TIsTemplateLiteralFinite<T>> = F extends true ? (T extends TTemplateLiteral<infer S extends TTemplateLiteralKind[]> ? TFromTemplateLiteralKinds$1<S> extends infer R extends string[][] ? TStringReduce<R> : [
] : [
]) : [
];
export declare function TemplateLiteralGenerate<T extends TTemplateLiteral>(schema: T): TTemplateLiteralGenerate<T>;
declare function FromUnion(syntax: string): IterableIterator<TTemplateLiteralKind>;
declare function FromTerminal(syntax: string): IterableIterator<TTemplateLiteralKind>;
type FromUnionLiteral<T extends string> = T extends `${infer L}|${infer R}` ? [
    TLiteral<Trim<L>>,
    ...FromUnionLiteral<R>
] : T extends `${infer L}` ? [
    TLiteral<Trim<L>>
] : [
];
type FromUnion<T extends string> = TUnionEvaluated<FromUnionLiteral<T>>;
type FromTerminal<T extends string> = T extends "boolean" ? TBoolean : T extends "bigint" ? TBigInt : T extends "number" ? TNumber : T extends "string" ? TString : FromUnion<T>;
type FromString<T extends string> = T extends `{${infer L}}${infer R}` ? [
    FromTerminal<L>,
    ...FromString<R>
] : T extends `${infer L}$\{${infer R1}\}${infer R2}` ? [
    TLiteral<L>,
    ...FromString<`{${R1}}`>,
    ...FromString<R2>
] : T extends `${infer L}$\{${infer R1}\}` ? [
    TLiteral<L>,
    ...FromString<`{${R1}}`>
] : T extends `${infer L}` ? [
    TLiteral<L>
] : [
];
export type TTemplateLiteralSyntax<T extends string> = (TTemplateLiteral<Assert<FromString<T>, TTemplateLiteralKind[]>>);
export declare function TemplateLiteralSyntax(syntax: string): TTemplateLiteralKind[];
export declare class TemplateLiteralPatternError extends TypeBoxError {
}
export declare function TemplateLiteralPattern(kinds: TTemplateLiteralKind[]): string;
type TemplateLiteralStaticKind<T, Acc extends string> = T extends TUnion<infer U> ? {
    [K in keyof U]: TemplateLiteralStatic<Assert<[
        U[K]
    ], TTemplateLiteralKind[]>, Acc>;
}[number] : T extends TTemplateLiteral ? `${Static<T>}` : T extends TLiteral<infer U> ? `${U}` : T extends TString ? `${string}` : T extends TNumber ? `${number}` : T extends TBigInt ? `${bigint}` : T extends TBoolean ? `${boolean}` : never;
type TemplateLiteralStatic<T extends TTemplateLiteralKind[], Acc extends string> = T extends [
    infer L,
    ...infer R
] ? `${TemplateLiteralStaticKind<L, Acc>}${TemplateLiteralStatic<Assert<R, TTemplateLiteralKind[]>, Acc>}` : Acc;
export type TTemplateLiteralKind = TTemplateLiteral | TUnion | TLiteral | TInteger | TNumber | TBigInt | TString | TBoolean | TNever;
export interface TTemplateLiteral<T extends TTemplateLiteralKind[] = TTemplateLiteralKind[]> extends TSchema {
    [Kind]: "TemplateLiteral";
    static: TemplateLiteralStatic<T, EmptyString>;
    type: "string";
    pattern: string;
}
export declare function TemplateLiteral<T extends string>(syntax: T, options?: SchemaOptions): TTemplateLiteralSyntax<T>;
export declare function TemplateLiteral<T extends TTemplateLiteralKind[]>(kinds: [
    ...T
], options?: SchemaOptions): TTemplateLiteral<T>;
export type TTemplateLiteralToUnionLiteralArray<T extends string[], Acc extends TLiteral[] = [
]> = (T extends [
    infer L extends string,
    ...infer R extends string[]
] ? TTemplateLiteralToUnionLiteralArray<R, [
    ...Acc,
    TLiteral<L>
]> : Acc);
export type TTemplateLiteralToUnion<T extends TTemplateLiteral, U extends string[] = UnionToTuple<Static<T>>> = TUnionEvaluated<TTemplateLiteralToUnionLiteralArray<U>>;
export declare function TemplateLiteralToUnion<T extends TTemplateLiteral>(schema: TTemplateLiteral): TTemplateLiteralToUnion<T>;
type TFromTemplateLiteral<TemplateLiteral extends TTemplateLiteral, Keys extends string[] = TTemplateLiteralGenerate<TemplateLiteral>> = (Keys);
type TFromUnion<Types extends TSchema[], Result extends string[] = [
]> = (Types extends [
    infer Left extends TSchema,
    ...infer Right extends TSchema[]
] ? TFromUnion<Right, [
    ...Result,
    ...TIndexPropertyKeys<Left>
]> : Result);
type TFromLiteral<LiteralValue extends TLiteralValue> = (LiteralValue extends PropertyKey ? [
    `${LiteralValue}`
] : [
]);
export type TIndexPropertyKeys<Type extends TSchema> = (Type extends TTemplateLiteral ? TFromTemplateLiteral<Type> : Type extends TUnion<infer Types extends TSchema[]> ? TFromUnion<Types> : Type extends TLiteral<infer Value extends TLiteralValue> ? TFromLiteral<Value> : Type extends TNumber ? [
    "[number]"
] : Type extends TInteger ? [
    "[number]"
] : [
]);
export declare function IndexPropertyKeys<Type extends TSchema>(type: Type): TIndexPropertyKeys<Type>;
type TFromProperties$1<Type extends TSchema, Properties extends TProperties> = ({
    [K2 in keyof Properties]: TIndex<Type, TIndexPropertyKeys<Properties[K2]>>;
});
type TFromMappedResult$1<Type extends TSchema, MappedResult extends TMappedResult> = (TFromProperties$1<Type, MappedResult["properties"]>);
export type TIndexFromMappedResult<Type extends TSchema, MappedResult extends TMappedResult, Properties extends TProperties = TFromMappedResult$1<Type, MappedResult>> = (TMappedResult<Properties>);
export declare function IndexFromMappedResult<Type extends TSchema, MappedResult extends TMappedResult, Properties extends TProperties = TFromMappedResult$1<Type, MappedResult>>(type: Type, mappedResult: MappedResult, options?: SchemaOptions): TMappedResult<Properties>;
type TFromRest<Types extends TSchema[], Key extends PropertyKey, Result extends TSchema[] = [
]> = (Types extends [
    infer Left extends TSchema,
    ...infer Right extends TSchema[]
] ? TFromRest<Right, Key, [
    ...Result,
    Assert<TIndexFromPropertyKey<Left, Key>, TSchema>
]> : Result);
type TFromIntersectRest<Types extends TSchema[], Result extends TSchema[] = [
]> = (Types extends [
    infer Left extends TSchema,
    ...infer Right extends TSchema[]
] ? Left extends TNever ? TFromIntersectRest<Right, [
    ...Result
]> : TFromIntersectRest<Right, [
    ...Result,
    Left
]> : Result);
type TFromIntersect<Types extends TSchema[], Key extends PropertyKey> = (TIntersectEvaluated<TFromIntersectRest<TFromRest<Types, Key>>>);
type TFromUnionRest<Types extends TSchema[], Result extends TSchema[] = [
]> = Types extends [
    infer Left extends TSchema,
    ...infer Right extends TSchema[]
] ? Left extends TNever ? [
] : TFromUnionRest<Right, [
    Left,
    ...Result
]> : Result;
type TFromUnion$1<Types extends TSchema[], Key extends PropertyKey> = (TUnionEvaluated<TFromUnionRest<TFromRest<Types, Key>>>);
type TFromTuple<Types extends TSchema[], Key extends PropertyKey> = (Key extends keyof Types ? Types[Key] : Key extends "[number]" ? TUnionEvaluated<Types> : TNever);
type TFromArray<Type extends TSchema, Key extends PropertyKey> = (Key extends "[number]" ? Type : TNever);
type AssertPropertyKey<T> = Assert<T, string | number>;
type TFromProperty<Properties extends TProperties, Key extends PropertyKey> = (Key extends keyof Properties ? Properties[Key] : `${AssertPropertyKey<Key>}` extends `${AssertPropertyKey<keyof Properties>}` ? Properties[AssertPropertyKey<Key>] : TNever);
export type TIndexFromPropertyKey<Type extends TSchema, Key extends PropertyKey> = (Type extends TRecursive<infer Type extends TSchema> ? TIndexFromPropertyKey<Type, Key> : Type extends TIntersect<infer Types extends TSchema[]> ? TFromIntersect<Types, Key> : Type extends TUnion<infer Types extends TSchema[]> ? TFromUnion$1<Types, Key> : Type extends TTuple<infer Types extends TSchema[]> ? TFromTuple<Types, Key> : Type extends TArray<infer Type extends TSchema> ? TFromArray<Type, Key> : Type extends TObject<infer Properties extends TProperties> ? TFromProperty<Properties, Key> : TNever);
export declare function IndexFromPropertyKey<Type extends TSchema, Key extends PropertyKey>(type: Type, propertyKey: Key): TIndexFromPropertyKey<Type, Key>;
export type TIndexFromPropertyKeys<Type extends TSchema, PropertyKeys extends PropertyKey[], Result extends TSchema[] = [
]> = (PropertyKeys extends [
    infer Left extends PropertyKey,
    ...infer Right extends PropertyKey[]
] ? TIndexFromPropertyKeys<Type, Right, [
    ...Result,
    Assert<TIndexFromPropertyKey<Type, Left>, TSchema>
]> : Result);
export declare function IndexFromPropertyKeys<Type extends TSchema, PropertyKeys extends PropertyKey[]>(type: Type, propertyKeys: [
    ...PropertyKeys
]): TIndexFromPropertyKeys<Type, PropertyKeys>;
type FromSchema<Type extends TSchema, PropertyKeys extends PropertyKey[]> = (TUnionEvaluated<TIndexFromPropertyKeys<Type, PropertyKeys>>);
declare function FromSchema<Type extends TSchema, PropertyKeys extends PropertyKey[]>(type: Type, propertyKeys: [
    ...PropertyKeys
]): FromSchema<Type, PropertyKeys>;
export type TIndexFromComputed<Type extends TSchema, Key extends TSchema> = (TComputed<"Index", [
    Type,
    Key
]>);
export declare function IndexFromComputed<Type extends TSchema, Key extends TSchema>(type: Type, key: Key): TIndexFromComputed<Type, Key>;
export type TIndex<Type extends TSchema, PropertyKeys extends PropertyKey[]> = (FromSchema<Type, PropertyKeys>);
export declare function Index<Type extends TRef, Key extends TSchema>(type: Type, key: Key, options?: SchemaOptions): TIndexFromComputed<Type, Key>;
export declare function Index<Type extends TSchema, Key extends TRef>(type: Type, key: Key, options?: SchemaOptions): TIndexFromComputed<Type, Key>;
export declare function Index<Type extends TRef, Key extends TRef>(type: Type, key: Key, options?: SchemaOptions): TIndexFromComputed<Type, Key>;
export declare function Index<Type extends TSchema, MappedResult extends TMappedResult>(type: Type, mappedResult: MappedResult, options?: SchemaOptions): TIndexFromMappedResult<Type, MappedResult>;
export declare function Index<Type extends TSchema, MappedResult extends TMappedResult>(type: Type, mappedResult: MappedResult, options?: SchemaOptions): TIndexFromMappedResult<Type, MappedResult>;
export declare function Index<Type extends TSchema, MappedKey extends TMappedKey>(type: Type, mappedKey: MappedKey, options?: SchemaOptions): TIndexFromMappedKey<Type, MappedKey>;
export declare function Index<Type extends TSchema, Key extends TSchema, PropertyKeys extends PropertyKey[] = TIndexPropertyKeys<Key>>(T: Type, K: Key, options?: SchemaOptions): TIndex<Type, PropertyKeys>;
export declare function Index<Type extends TSchema, PropertyKeys extends PropertyKey[]>(type: Type, propertyKeys: readonly [
    ...PropertyKeys
], options?: SchemaOptions): TIndex<Type, PropertyKeys>;
type TMappedIndexPropertyKey<Type extends TSchema, Key extends PropertyKey> = {
    [_ in Key]: TIndex<Type, [
        Key
    ]>;
};
type TMappedIndexPropertyKeys<Type extends TSchema, PropertyKeys extends PropertyKey[], Result extends TProperties = {}> = (PropertyKeys extends [
    infer Left extends PropertyKey,
    ...infer Right extends PropertyKey[]
] ? TMappedIndexPropertyKeys<Type, Right, Result & TMappedIndexPropertyKey<Type, Left>> : Result);
type TMappedIndexProperties<Type extends TSchema, MappedKey extends TMappedKey> = Evaluate<TMappedIndexPropertyKeys<Type, MappedKey["keys"]>>;
export type TIndexFromMappedKey<Type extends TSchema, MappedKey extends TMappedKey, Properties extends TProperties = TMappedIndexProperties<Type, MappedKey>> = (Ensure<TMappedResult<Properties>>);
export declare function IndexFromMappedKey<Type extends TSchema, MappedKey extends TMappedKey, Properties extends TProperties = TMappedIndexProperties<Type, MappedKey>>(type: Type, mappedKey: MappedKey, options?: SchemaOptions): TMappedResult<Properties>;
export interface TIterator<T extends TSchema = TSchema> extends TSchema {
    [Kind]: "Iterator";
    static: IterableIterator<Static<T, this["params"]>>;
    type: "Iterator";
    items: T;
}
declare function Iterator$1<T extends TSchema>(items: T, options?: SchemaOptions): TIterator<T>;
export interface TPromise<T extends TSchema = TSchema> extends TSchema {
    [Kind]: "Promise";
    static: Promise<Static<T, this["params"]>>;
    type: "Promise";
    item: TSchema;
}
declare function Promise$1<T extends TSchema>(item: T, options?: SchemaOptions): TPromise<T>;
export type TSetIncludes<T extends PropertyKey[], S extends PropertyKey> = (T extends [
    infer L extends PropertyKey,
    ...infer R extends PropertyKey[]
] ? S extends L ? true : TSetIncludes<R, S> : false);
export declare function SetIncludes<T extends PropertyKey[], S extends PropertyKey>(T: [
    ...T
], S: S): TSetIncludes<T, S>;
export type TSetIsSubset<T extends PropertyKey[], S extends PropertyKey[]> = (T extends [
    infer L extends PropertyKey,
    ...infer R extends PropertyKey[]
] ? TSetIncludes<S, L> extends true ? TSetIsSubset<R, S> : false : true);
export declare function SetIsSubset<T extends PropertyKey[], S extends PropertyKey[]>(T: [
    ...T
], S: [
    ...S
]): TSetIsSubset<T, S>;
export type TSetDistinct<T extends PropertyKey[], Acc extends PropertyKey[] = [
]> = T extends [
    infer L extends PropertyKey,
    ...infer R extends PropertyKey[]
] ? TSetIncludes<Acc, L> extends false ? TSetDistinct<R, [
    ...Acc,
    L
]> : TSetDistinct<R, [
    ...Acc
]> : Acc;
export declare function SetDistinct<T extends PropertyKey[]>(T: [
    ...T
]): TSetDistinct<T>;
export type TSetIntersect<T extends PropertyKey[], S extends PropertyKey[], Acc extends PropertyKey[] = [
]> = (T extends [
    infer L extends PropertyKey,
    ...infer R extends PropertyKey[]
] ? TSetIncludes<S, L> extends true ? TSetIntersect<R, S, [
    ...Acc,
    L
]> : TSetIntersect<R, S, [
    ...Acc
]> : Acc);
export declare function SetIntersect<T extends PropertyKey[], S extends PropertyKey[]>(T: [
    ...T
], S: [
    ...S
]): TSetIntersect<T, S>;
export type TSetUnion<T extends PropertyKey[], S extends PropertyKey[]> = ([
    ...T,
    ...S
]);
export declare function SetUnion<T extends PropertyKey[], S extends PropertyKey[]>(T: [
    ...T
], S: [
    ...S
]): TSetUnion<T, S>;
export type TSetComplement<T extends PropertyKey[], S extends PropertyKey[], Acc extends PropertyKey[] = [
]> = (T extends [
    infer L extends PropertyKey,
    ...infer R extends PropertyKey[]
] ? TSetIncludes<S, L> extends true ? TSetComplement<R, S, [
    ...Acc
]> : TSetComplement<R, S, [
    ...Acc,
    L
]> : Acc);
export declare function SetComplement<T extends PropertyKey[], S extends PropertyKey[]>(T: [
    ...T
], S: [
    ...S
]): TSetComplement<T, S>;
type TSetIntersectManyResolve<T extends PropertyKey[][], Acc extends PropertyKey[]> = (T extends [
    infer L extends PropertyKey[],
    ...infer R extends PropertyKey[][]
] ? TSetIntersectManyResolve<R, TSetIntersect<Acc, L>> : Acc);
export type TSetIntersectMany<T extends PropertyKey[][]> = (T extends [
    infer L extends PropertyKey[]
] ? L : T extends [
    infer L extends PropertyKey[],
    ...infer R extends PropertyKey[][]
] ? TSetIntersectManyResolve<R, L> : [
]);
export declare function SetIntersectMany<T extends PropertyKey[][]>(T: [
    ...T
]): TSetIntersectMany<T>;
export type TSetUnionMany<T extends PropertyKey[][], Acc extends PropertyKey[] = [
]> = (T extends [
    infer L extends PropertyKey[],
    ...infer R extends PropertyKey[][]
] ? TSetUnionMany<R, TSetUnion<Acc, L>> : Acc);
export declare function SetUnionMany<T extends PropertyKey[][]>(T: [
    ...T
]): TSetUnionMany<T>;
type TFromMappedResult$2<K extends PropertyKey, P extends TProperties> = (K extends keyof P ? FromSchemaType<K, P[K]> : TMappedResult<P>);
type TMappedKeyToKnownMappedResultProperties<K extends PropertyKey> = {
    [_ in K]: TLiteral<Assert<K, TLiteralValue>>;
};
type TMappedKeyToUnknownMappedResultProperties<P extends PropertyKey[], Acc extends TProperties = {}> = (P extends [
    infer L extends PropertyKey,
    ...infer R extends PropertyKey[]
] ? TMappedKeyToUnknownMappedResultProperties<R, Acc & {
    [_ in L]: TLiteral<Assert<L, TLiteralValue>>;
}> : Acc);
type TMappedKeyToMappedResultProperties<K extends PropertyKey, P extends PropertyKey[]> = (TSetIncludes<P, K> extends true ? TMappedKeyToKnownMappedResultProperties<K> : TMappedKeyToUnknownMappedResultProperties<P>);
type TFromMappedKey<K extends PropertyKey, P extends PropertyKey[], R extends TProperties = TMappedKeyToMappedResultProperties<K, P>> = (TFromMappedResult$2<K, R>);
type TFromRest$1<K extends PropertyKey, T extends TSchema[], Acc extends TSchema[] = [
]> = (T extends [
    infer L extends TSchema,
    ...infer R extends TSchema[]
] ? TFromRest$1<K, R, [
    ...Acc,
    FromSchemaType<K, L>
]> : Acc);
type FromProperties<K extends PropertyKey, T extends TProperties, R extends TProperties = Evaluate<{
    [K2 in keyof T]: FromSchemaType<K, T[K2]>;
}>> = R;
declare function FromProperties<K extends PropertyKey, T extends TProperties>(K: K, T: T): FromProperties<K, T>;
type FromSchemaType<K extends PropertyKey, T extends TSchema> = (T extends TReadonly<infer S extends TSchema> ? TReadonly<FromSchemaType<K, S>> : T extends TOptional<infer S extends TSchema> ? TOptional<FromSchemaType<K, S>> : T extends TMappedResult<infer P extends TProperties> ? TFromMappedResult$2<K, P> : T extends TMappedKey<infer P extends PropertyKey[]> ? TFromMappedKey<K, P> : T extends TConstructor<infer S extends TSchema[], infer R extends TSchema> ? TConstructor<TFromRest$1<K, S>, FromSchemaType<K, R>> : T extends TFunction<infer S extends TSchema[], infer R extends TSchema> ? TFunction<TFromRest$1<K, S>, FromSchemaType<K, R>> : T extends TAsyncIterator<infer S extends TSchema> ? TAsyncIterator<FromSchemaType<K, S>> : T extends TIterator<infer S extends TSchema> ? TIterator<FromSchemaType<K, S>> : T extends TIntersect<infer S extends TSchema[]> ? TIntersect<TFromRest$1<K, S>> : T extends TEnum<infer S extends TEnumRecord> ? TEnum<S> : T extends TUnion<infer S extends TSchema[]> ? TUnion<TFromRest$1<K, S>> : T extends TTuple<infer S extends TSchema[]> ? TTuple<TFromRest$1<K, S>> : T extends TObject<infer S extends TProperties> ? TObject<FromProperties<K, S>> : T extends TArray<infer S extends TSchema> ? TArray<FromSchemaType<K, S>> : T extends TPromise<infer S extends TSchema> ? TPromise<FromSchemaType<K, S>> : T);
declare function FromSchemaType<K extends PropertyKey, T extends TSchema>(K: K, T: T): FromSchemaType<K, T>;
export type TMappedFunctionReturnType<K extends PropertyKey[], T extends TSchema, Acc extends TProperties = {}> = (K extends [
    infer L extends PropertyKey,
    ...infer R extends PropertyKey[]
] ? TMappedFunctionReturnType<R, T, Acc & {
    [_ in L]: FromSchemaType<L, T>;
}> : Acc);
export declare function MappedFunctionReturnType<K extends PropertyKey[], T extends TSchema>(K: [
    ...K
], T: T): TMappedFunctionReturnType<K, T>;
export type TMappedFunction<K extends PropertyKey[], I = TMappedKey<K>> = (T: I) => TSchema;
export type TMapped<K extends PropertyKey[], F extends TMappedFunction<K>, R extends TProperties = Evaluate<TMappedFunctionReturnType<K, ReturnType<F>>>> = Ensure<TObject<R>>;
export declare function Mapped<K extends TSchema, I extends PropertyKey[] = TIndexPropertyKeys<K>, F extends TMappedFunction<I> = TMappedFunction<I>, R extends TMapped<I, F> = TMapped<I, F>>(key: K, map: F, options?: ObjectOptions): R;
export declare function Mapped<K extends PropertyKey[], F extends TMappedFunction<K> = TMappedFunction<K>, R extends TMapped<K, F> = TMapped<K, F>>(key: [
    ...K
], map: F, options?: ObjectOptions): R;
type TRemoveOptional<T extends TSchema> = T extends TOptional<infer S> ? S : T;
type TAddOptional<T extends TSchema> = T extends TOptional<infer S> ? TOptional<S> : Ensure<TOptional<T>>;
export type TOptionalWithFlag<T extends TSchema, F extends boolean> = F extends false ? TRemoveOptional<T> : TAddOptional<T>;
export type TOptional<T extends TSchema> = T & {
    [OptionalKind]: "Optional";
};
export declare function Optional<T extends TMappedResult, F extends boolean>(schema: T, enable: F): TOptionalFromMappedResult<T, F>;
export declare function Optional<T extends TSchema, F extends boolean>(schema: T, enable: F): TOptionalWithFlag<T, F>;
export declare function Optional<T extends TMappedResult>(schema: T): TOptionalFromMappedResult<T, true>;
export declare function Optional<T extends TSchema>(schema: T): TOptionalWithFlag<T, true>;
type TFromProperties$2<P extends TProperties, F extends boolean> = ({
    [K2 in keyof P]: TOptionalWithFlag<P[K2], F>;
});
type TFromMappedResult$3<R extends TMappedResult, F extends boolean> = (TFromProperties$2<R["properties"], F>);
export type TOptionalFromMappedResult<R extends TMappedResult, F extends boolean, P extends TProperties = TFromMappedResult$3<R, F>> = (TMappedResult<P>);
export declare function OptionalFromMappedResult<R extends TMappedResult, F extends boolean, P extends TProperties = TFromMappedResult$3<R, F>>(R: R, F: F): TMappedResult<P>;
type TFromComputed<Target extends string, Parameters extends TSchema[]> = Ensure<(TComputed<"Awaited", [
    TComputed<Target, Parameters>
]>)>;
type TFromRef<Ref extends string> = Ensure<TComputed<"Awaited", [
    TRef<Ref>
]>>;
type TFromRest$2<Types extends TSchema[], Result extends TSchema[] = [
]> = (Types extends [
    infer Left extends TSchema,
    ...infer Right extends TSchema[]
] ? TFromRest$2<Right, [
    ...Result,
    TAwaited<Left>
]> : Result);
export type TAwaited<Type extends TSchema> = (Type extends TComputed<infer Target extends string, infer Parameters extends TSchema[]> ? TFromComputed<Target, Parameters> : Type extends TRef<infer Ref extends string> ? TFromRef<Ref> : Type extends TIntersect<infer Types extends TSchema[]> ? TIntersect<TFromRest$2<Types>> : Type extends TUnion<infer Types extends TSchema[]> ? TUnion<TFromRest$2<Types>> : Type extends TPromise<infer Type extends TSchema> ? TAwaited<Type> : Type);
declare function Awaited$1<T extends TSchema>(type: T, options?: SchemaOptions): TAwaited<T>;
type TFromRest$3<Types extends TSchema[], Result extends PropertyKey[][] = [
]> = (Types extends [
    infer L extends TSchema,
    ...infer R extends TSchema[]
] ? TFromRest$3<R, [
    ...Result,
    TKeyOfPropertyKeys<L>
]> : Result);
type TFromIntersect$1<Types extends TSchema[], PropertyKeysArray extends PropertyKey[][] = TFromRest$3<Types>, PropertyKeys extends PropertyKey[] = TSetUnionMany<PropertyKeysArray>> = PropertyKeys;
type TFromUnion$2<Types extends TSchema[], PropertyKeysArray extends PropertyKey[][] = TFromRest$3<Types>, PropertyKeys extends PropertyKey[] = TSetIntersectMany<PropertyKeysArray>> = PropertyKeys;
type TFromTuple$1<Types extends TSchema[], Indexer extends string = ZeroString, Acc extends PropertyKey[] = [
]> = Types extends [
    infer _ extends TSchema,
    ...infer R extends TSchema[]
] ? TFromTuple$1<R, TIncrement<Indexer>, [
    ...Acc,
    Indexer
]> : Acc;
type TFromArray$1<_ extends TSchema> = ([
    "[number]"
]);
type TFromProperties$3<Properties extends TProperties> = (UnionToTuple<keyof Properties>);
export type TKeyOfPropertyKeys<Type extends TSchema> = (Type extends TRecursive<infer Type extends TSchema> ? TKeyOfPropertyKeys<Type> : Type extends TIntersect<infer Types extends TSchema[]> ? TFromIntersect$1<Types> : Type extends TUnion<infer Types extends TSchema[]> ? TFromUnion$2<Types> : Type extends TTuple<infer Types extends TSchema[]> ? TFromTuple$1<Types> : Type extends TArray<infer Type extends TSchema> ? TFromArray$1<Type> : Type extends TObject<infer Properties extends TProperties> ? TFromProperties$3<Properties> : [
]);
export declare function KeyOfPropertyKeys<Type extends TSchema>(type: Type): TKeyOfPropertyKeys<Type>;
export declare function KeyOfPattern(schema: TSchema): string;
type TFromComputed$1<Target extends string, Parameters extends TSchema[]> = Ensure<TComputed<"KeyOf", [
    TComputed<Target, Parameters>
]>>;
type TFromRef$1<Ref extends string> = Ensure<TComputed<"KeyOf", [
    TRef<Ref>
]>>;
export type TKeyOfFromType<Type extends TSchema, PropertyKeys extends PropertyKey[] = TKeyOfPropertyKeys<Type>, PropertyKeyTypes extends TSchema[] = TKeyOfPropertyKeysToRest<PropertyKeys>, Result = TUnionEvaluated<PropertyKeyTypes>> = Ensure<Result>;
export type TKeyOfPropertyKeysToRest<PropertyKeys extends PropertyKey[], Result extends TSchema[] = [
]> = (PropertyKeys extends [
    infer L extends PropertyKey,
    ...infer R extends PropertyKey[]
] ? L extends "[number]" ? TKeyOfPropertyKeysToRest<R, [
    ...Result,
    TNumber
]> : TKeyOfPropertyKeysToRest<R, [
    ...Result,
    TLiteral<Assert<L, TLiteralValue>>
]> : Result);
export declare function KeyOfPropertyKeysToRest<PropertyKeys extends PropertyKey[]>(propertyKeys: [
    ...PropertyKeys
]): TKeyOfPropertyKeysToRest<PropertyKeys>;
export type TKeyOf<Type extends TSchema> = (Type extends TComputed<infer Target extends string, infer Parameters extends TSchema[]> ? TFromComputed$1<Target, Parameters> : Type extends TRef<infer Ref extends string> ? TFromRef$1<Ref> : Type extends TMappedResult ? TKeyOfFromMappedResult<Type> : TKeyOfFromType<Type>);
export declare function KeyOf<Type extends TSchema>(type: Type, options?: SchemaOptions): TKeyOf<Type>;
type TFromProperties$4<Properties extends TProperties> = ({
    [K2 in keyof Properties]: TKeyOfFromType<Properties[K2]>;
});
type TFromMappedResult$4<MappedResult extends TMappedResult> = (Evaluate<TFromProperties$4<MappedResult["properties"]>>);
export type TKeyOfFromMappedResult<MappedResult extends TMappedResult, Properties extends TProperties = TFromMappedResult$4<MappedResult>> = (Ensure<TMappedResult<Properties>>);
export declare function KeyOfFromMappedResult<MappedResult extends TMappedResult, Properties extends TProperties = TFromMappedResult$4<MappedResult>>(mappedResult: MappedResult, options?: SchemaOptions): TMappedResult<Properties>;
export declare function KeyOfPropertyEntries(schema: TSchema): [
    key: string,
    schema: TSchema
][];
type TFromProperties$5<Properties extends TProperties, PropertyKeys extends PropertyKey[]> = ({
    [K2 in keyof Properties]: TOmit<Properties[K2], PropertyKeys>;
});
type TFromMappedResult$5<MappedResult extends TMappedResult, PropertyKeys extends PropertyKey[]> = (Evaluate<TFromProperties$5<MappedResult["properties"], PropertyKeys>>);
export type TOmitFromMappedResult<MappedResult extends TMappedResult, PropertyKeys extends PropertyKey[], Properties extends TProperties = TFromMappedResult$5<MappedResult, PropertyKeys>> = (Ensure<TMappedResult<Properties>>);
export declare function OmitFromMappedResult<MappedResult extends TMappedResult, PropertyKeys extends PropertyKey[], Properties extends TProperties = TFromMappedResult$5<MappedResult, PropertyKeys>>(mappedResult: MappedResult, propertyKeys: [
    ...PropertyKeys
], options?: SchemaOptions): TMappedResult<Properties>;
type TFromIntersect$2<Types extends TSchema[], PropertyKeys extends PropertyKey[], Result extends TSchema[] = [
]> = (Types extends [
    infer L extends TSchema,
    ...infer R extends TSchema[]
] ? TFromIntersect$2<R, PropertyKeys, [
    ...Result,
    TOmit<L, PropertyKeys>
]> : Result);
type TFromUnion$3<T extends TSchema[], K extends PropertyKey[], Result extends TSchema[] = [
]> = (T extends [
    infer L extends TSchema,
    ...infer R extends TSchema[]
] ? TFromUnion$3<R, K, [
    ...Result,
    TOmit<L, K>
]> : Result);
type TFromProperties$6<Properties extends TProperties, PropertyKeys extends PropertyKey[], UnionKey extends PropertyKey = TupleToUnion<PropertyKeys>> = (Evaluate<Omit<Properties, UnionKey>>);
type TFromObject<_Type extends TObject, PropertyKeys extends PropertyKey[], Properties extends TProperties, MappedProperties extends TProperties = TFromProperties$6<Properties, PropertyKeys>, Result extends TSchema = TObject<MappedProperties>> = Result;
type TUnionFromPropertyKeys<PropertyKeys extends PropertyKey[], Result extends TLiteral[] = [
]> = (PropertyKeys extends [
    infer Key extends PropertyKey,
    ...infer Rest extends PropertyKey[]
] ? Key extends TLiteralValue ? TUnionFromPropertyKeys<Rest, [
    ...Result,
    TLiteral<Key>
]> : TUnionFromPropertyKeys<Rest, [
    ...Result
]> : TUnion<Result>);
export type TOmitResolve<Properties extends TProperties, PropertyKeys extends PropertyKey[]> = (Properties extends TRecursive<infer Types extends TSchema> ? TRecursive<TOmitResolve<Types, PropertyKeys>> : Properties extends TIntersect<infer Types extends TSchema[]> ? TIntersect<TFromIntersect$2<Types, PropertyKeys>> : Properties extends TUnion<infer Types extends TSchema[]> ? TUnion<TFromUnion$3<Types, PropertyKeys>> : Properties extends TObject<infer Properties extends TProperties> ? TFromObject<TObject, PropertyKeys, Properties> : TObject<{}>);
type TResolvePropertyKeys<Key extends TSchema | PropertyKey[]> = Key extends TSchema ? TIndexPropertyKeys<Key> : Key;
type TResolveTypeKey<Key extends TSchema | PropertyKey[]> = Key extends PropertyKey[] ? TUnionFromPropertyKeys<Key> : Key;
export type TOmit<Type extends TSchema, Key extends TSchema | PropertyKey[], IsTypeRef extends boolean = Type extends TRef ? true : false, IsKeyRef extends boolean = Key extends TRef ? true : false> = (Type extends TMappedResult ? TOmitFromMappedResult<Type, TResolvePropertyKeys<Key>> : Key extends TMappedKey ? TOmitFromMappedKey<Type, Key> : [
    IsTypeRef,
    IsKeyRef
] extends [
    true,
    true
] ? TComputed<"Omit", [
    Type,
    TResolveTypeKey<Key>
]> : [
    IsTypeRef,
    IsKeyRef
] extends [
    false,
    true
] ? TComputed<"Omit", [
    Type,
    TResolveTypeKey<Key>
]> : [
    IsTypeRef,
    IsKeyRef
] extends [
    true,
    false
] ? TComputed<"Omit", [
    Type,
    TResolveTypeKey<Key>
]> : TOmitResolve<Type, TResolvePropertyKeys<Key>>);
declare function Omit$1<Type extends TSchema, Key extends PropertyKey[]>(type: Type, key: readonly [
    ...Key
], options?: SchemaOptions): TOmit<Type, Key>;
declare function Omit$1<Type extends TSchema, Key extends TSchema>(type: Type, key: Key, options?: SchemaOptions): TOmit<Type, Key>;
type TFromPropertyKey<Type extends TSchema, Key extends PropertyKey> = {
    [_ in Key]: TOmit<Type, [
        Key
    ]>;
};
type TFromPropertyKeys<Type extends TSchema, PropertyKeys extends PropertyKey[], Result extends TProperties = {}> = (PropertyKeys extends [
    infer LK extends PropertyKey,
    ...infer RK extends PropertyKey[]
] ? TFromPropertyKeys<Type, RK, Result & TFromPropertyKey<Type, LK>> : Result);
type TFromMappedKey$1<Type extends TSchema, MappedKey extends TMappedKey> = (TFromPropertyKeys<Type, MappedKey["keys"]>);
export type TOmitFromMappedKey<Type extends TSchema, MappedKey extends TMappedKey, Properties extends TProperties = TFromMappedKey$1<Type, MappedKey>> = (TMappedResult<Properties>);
export declare function OmitFromMappedKey<Type extends TSchema, MappedKey extends TMappedKey, Properties extends TProperties = TFromMappedKey$1<Type, MappedKey>>(type: Type, mappedKey: MappedKey, options?: SchemaOptions): TMappedResult<Properties>;
type TFromProperties$7<Properties extends TProperties, PropertyKeys extends PropertyKey[]> = ({
    [K2 in keyof Properties]: TPick<Properties[K2], PropertyKeys>;
});
type TFromMappedResult$6<MappedResult extends TMappedResult, PropertyKeys extends PropertyKey[]> = (Evaluate<TFromProperties$7<MappedResult["properties"], PropertyKeys>>);
export type TPickFromMappedResult<MappedResult extends TMappedResult, PropertyKeys extends PropertyKey[], Properties extends TProperties = TFromMappedResult$6<MappedResult, PropertyKeys>> = (Ensure<TMappedResult<Properties>>);
export declare function PickFromMappedResult<MappedResult extends TMappedResult, PropertyKeys extends PropertyKey[], Properties extends TProperties = TFromMappedResult$6<MappedResult, PropertyKeys>>(mappedResult: MappedResult, propertyKeys: [
    ...PropertyKeys
], options?: SchemaOptions): TMappedResult<Properties>;
type TFromIntersect$3<Types extends TSchema[], PropertyKeys extends PropertyKey[], Result extends TSchema[] = [
]> = Types extends [
    infer L extends TSchema,
    ...infer R extends TSchema[]
] ? TFromIntersect$3<R, PropertyKeys, [
    ...Result,
    TPick<L, PropertyKeys>
]> : Result;
type TFromUnion$4<Types extends TSchema[], PropertyKeys extends PropertyKey[], Result extends TSchema[] = [
]> = Types extends [
    infer L extends TSchema,
    ...infer R extends TSchema[]
] ? TFromUnion$4<R, PropertyKeys, [
    ...Result,
    TPick<L, PropertyKeys>
]> : Result;
type TFromProperties$8<Properties extends TProperties, PropertyKeys extends PropertyKey[], UnionKeys extends PropertyKey = TupleToUnion<PropertyKeys>> = (Evaluate<Pick<Properties, UnionKeys & keyof Properties>>);
type TFromObject$1<_Type extends TObject, Keys extends PropertyKey[], Properties extends TProperties, MappedProperties extends TProperties = TFromProperties$8<Properties, Keys>, Result extends TSchema = TObject<MappedProperties>> = Result;
type TUnionFromPropertyKeys$1<PropertyKeys extends PropertyKey[], Result extends TLiteral[] = [
]> = (PropertyKeys extends [
    infer Key extends PropertyKey,
    ...infer Rest extends PropertyKey[]
] ? Key extends TLiteralValue ? TUnionFromPropertyKeys$1<Rest, [
    ...Result,
    TLiteral<Key>
]> : TUnionFromPropertyKeys$1<Rest, [
    ...Result
]> : TUnion<Result>);
export type TPickResolve<Type extends TProperties, PropertyKeys extends PropertyKey[]> = (Type extends TRecursive<infer Types extends TSchema> ? TRecursive<TPickResolve<Types, PropertyKeys>> : Type extends TIntersect<infer Types extends TSchema[]> ? TIntersect<TFromIntersect$3<Types, PropertyKeys>> : Type extends TUnion<infer Types extends TSchema[]> ? TUnion<TFromUnion$4<Types, PropertyKeys>> : Type extends TObject<infer Properties extends TProperties> ? TFromObject$1<TObject, PropertyKeys, Properties> : TObject<{}>);
type TResolvePropertyKeys$1<Key extends TSchema | PropertyKey[]> = Key extends TSchema ? TIndexPropertyKeys<Key> : Key;
type TResolveTypeKey$1<Key extends TSchema | PropertyKey[]> = Key extends PropertyKey[] ? TUnionFromPropertyKeys$1<Key> : Key;
export type TPick<Type extends TSchema, Key extends TSchema | PropertyKey[], IsTypeRef extends boolean = Type extends TRef ? true : false, IsKeyRef extends boolean = Key extends TRef ? true : false> = (Type extends TMappedResult ? TPickFromMappedResult<Type, TResolvePropertyKeys$1<Key>> : Key extends TMappedKey ? TPickFromMappedKey<Type, Key> : [
    IsTypeRef,
    IsKeyRef
] extends [
    true,
    true
] ? TComputed<"Pick", [
    Type,
    TResolveTypeKey$1<Key>
]> : [
    IsTypeRef,
    IsKeyRef
] extends [
    false,
    true
] ? TComputed<"Pick", [
    Type,
    TResolveTypeKey$1<Key>
]> : [
    IsTypeRef,
    IsKeyRef
] extends [
    true,
    false
] ? TComputed<"Pick", [
    Type,
    TResolveTypeKey$1<Key>
]> : TPickResolve<Type, TResolvePropertyKeys$1<Key>>);
declare function Pick$1<Type extends TSchema, Key extends PropertyKey[]>(type: Type, key: readonly [
    ...Key
], options?: SchemaOptions): TPick<Type, Key>;
declare function Pick$1<Type extends TSchema, Key extends TSchema>(type: Type, key: Key, options?: SchemaOptions): TPick<Type, Key>;
type TFromPropertyKey$1<Type extends TSchema, Key extends PropertyKey> = {
    [_ in Key]: TPick<Type, [
        Key
    ]>;
};
type TFromPropertyKeys$1<Type extends TSchema, PropertyKeys extends PropertyKey[], Result extends TProperties = {}> = (PropertyKeys extends [
    infer LeftKey extends PropertyKey,
    ...infer RightKeys extends PropertyKey[]
] ? TFromPropertyKeys$1<Type, RightKeys, Result & TFromPropertyKey$1<Type, LeftKey>> : Result);
type TFromMappedKey$2<Type extends TSchema, MappedKey extends TMappedKey> = (TFromPropertyKeys$1<Type, MappedKey["keys"]>);
export type TPickFromMappedKey<Type extends TSchema, MappedKey extends TMappedKey, Properties extends TProperties = TFromMappedKey$2<Type, MappedKey>> = (TMappedResult<Properties>);
export declare function PickFromMappedKey<Type extends TSchema, MappedKey extends TMappedKey, Properties extends TProperties = TFromMappedKey$2<Type, MappedKey>>(type: Type, mappedKey: MappedKey, options?: SchemaOptions): TMappedResult<Properties>;
export interface TNull extends TSchema {
    [Kind]: "Null";
    static: null;
    type: "null";
}
export declare function Null(options?: SchemaOptions): TNull;
export type TSymbolValue = string | number | undefined;
export interface TSymbol extends TSchema, SchemaOptions {
    [Kind]: "Symbol";
    static: symbol;
    type: "symbol";
}
declare function Symbol$1(options?: SchemaOptions): TSymbol;
export interface TUndefined extends TSchema {
    [Kind]: "Undefined";
    static: undefined;
    type: "undefined";
}
export declare function Undefined(options?: SchemaOptions): TUndefined;
type TFromComputed$2<Target extends string, Parameters extends TSchema[]> = Ensure<TComputed<"Partial", [
    TComputed<Target, Parameters>
]>>;
type TFromRef$2<Ref extends string> = Ensure<TComputed<"Partial", [
    TRef<Ref>
]>>;
type TFromProperties$9<Properties extends TProperties> = Evaluate<{
    [K in keyof Properties]: Properties[K] extends (TReadonlyOptional<infer S>) ? TReadonlyOptional<S> : Properties[K] extends (TReadonly<infer S>) ? TReadonlyOptional<S> : Properties[K] extends (TOptional<infer S>) ? TOptional<S> : TOptional<Properties[K]>;
}>;
type TFromObject$2<_Type extends TObject, Properties extends TProperties, MappedProperties extends TProperties = TFromProperties$9<Properties>, Result extends TSchema = TObject<MappedProperties>> = Result;
type TFromRest$4<Types extends TSchema[], Result extends TSchema[] = [
]> = (Types extends [
    infer L extends TSchema,
    ...infer R extends TSchema[]
] ? TFromRest$4<R, [
    ...Result,
    TPartial<L>
]> : Result);
export type TPartial<Type extends TSchema> = (Type extends TRecursive<infer Type extends TSchema> ? TRecursive<TPartial<Type>> : Type extends TComputed<infer Target extends string, infer Parameters extends TSchema[]> ? TFromComputed$2<Target, Parameters> : Type extends TRef<infer Ref extends string> ? TFromRef$2<Ref> : Type extends TIntersect<infer Types extends TSchema[]> ? TIntersect<TFromRest$4<Types>> : Type extends TUnion<infer Types extends TSchema[]> ? TUnion<TFromRest$4<Types>> : Type extends TObject<infer Properties extends TProperties> ? TFromObject$2<TObject, Properties> : Type extends TBigInt ? Type : Type extends TBoolean ? Type : Type extends TInteger ? Type : Type extends TLiteral ? Type : Type extends TNull ? Type : Type extends TNumber ? Type : Type extends TString ? Type : Type extends TSymbol ? Type : Type extends TUndefined ? Type : TObject<{}>);
declare function Partial$1<MappedResult extends TMappedResult>(type: MappedResult, options?: SchemaOptions): TPartialFromMappedResult<MappedResult>;
declare function Partial$1<Type extends TSchema>(type: Type, options?: SchemaOptions): TPartial<Type>;
type TFromProperties$10<P extends TProperties> = ({
    [K2 in keyof P]: TPartial<P[K2]>;
});
type TFromMappedResult$7<R extends TMappedResult> = (Evaluate<TFromProperties$10<R["properties"]>>);
export type TPartialFromMappedResult<R extends TMappedResult, P extends TProperties = TFromMappedResult$7<R>> = (Ensure<TMappedResult<P>>);
export declare function PartialFromMappedResult<R extends TMappedResult, P extends TProperties = TFromMappedResult$7<R>>(R: R, options?: SchemaOptions): TMappedResult<P>;
export interface RegExpOptions extends SchemaOptions {
    maxLength?: number;
    minLength?: number;
}
export interface TRegExp extends TSchema {
    [Kind]: "RegExp";
    static: `${string}`;
    type: "RegExp";
    source: string;
    flags: string;
}
declare function RegExp$1(pattern: string, options?: RegExpOptions): TRegExp;
declare function RegExp$1(regex: RegExp, options?: RegExpOptions): TRegExp;
type TFromTemplateLiteralKeyInfinite<Key extends TTemplateLiteral, Type extends TSchema> = Ensure<TRecord<Key, Type>>;
type TFromTemplateLiteralKeyFinite<Key extends TTemplateLiteral, Type extends TSchema, I extends string = Static<Key>> = (Ensure<TObject<Evaluate<{
    [_ in I]: Type;
}>>>);
type TFromTemplateLiteralKey<Key extends TTemplateLiteral, Type extends TSchema> = TIsTemplateLiteralFinite<Key> extends false ? TFromTemplateLiteralKeyInfinite<Key, Type> : TFromTemplateLiteralKeyFinite<Key, Type>;
type TFromEnumKey<Key extends Record<string, string | number>, Type extends TSchema> = Ensure<TObject<{
    [_ in Key[keyof Key]]: Type;
}>>;
type TFromUnionKeyLiteralString<Key extends TLiteral<string>, Type extends TSchema> = {
    [_ in Key["const"]]: Type;
};
type TFromUnionKeyLiteralNumber<Key extends TLiteral<number>, Type extends TSchema> = {
    [_ in Key["const"]]: Type;
};
type TFromUnionKeyVariants<Keys extends TSchema[], Type extends TSchema, Result extends TProperties = {}> = Keys extends [
    infer Left extends TSchema,
    ...infer Right extends TSchema[]
] ? (Left extends TUnion<infer Types extends TSchema[]> ? TFromUnionKeyVariants<Right, Type, Result & TFromUnionKeyVariants<Types, Type>> : Left extends TLiteral<string> ? TFromUnionKeyVariants<Right, Type, Result & TFromUnionKeyLiteralString<Left, Type>> : Left extends TLiteral<number> ? TFromUnionKeyVariants<Right, Type, Result & TFromUnionKeyLiteralNumber<Left, Type>> : {}) : Result;
type TFromUnionKey<Key extends TSchema[], Type extends TSchema, Properties extends TProperties = TFromUnionKeyVariants<Key, Type>> = (Ensure<TObject<Evaluate<Properties>>>);
type TFromLiteralKey<Key extends TLiteralValue, Type extends TSchema> = (Ensure<TObject<{
    [_ in Assert<Key, PropertyKey>]: Type;
}>>);
type TFromRegExpKey<_Key extends TRegExp, Type extends TSchema> = (Ensure<TRecord<TRegExp, Type>>);
type TFromStringKey<_Key extends TString, Type extends TSchema> = (Ensure<TRecord<TString, Type>>);
type TFromAnyKey<_Key extends TAny, Type extends TSchema> = (Ensure<TRecord<TAny, Type>>);
type TFromNeverKey<_Key extends TNever, Type extends TSchema> = (Ensure<TRecord<TNever, Type>>);
type TFromBooleanKey<_Key extends TBoolean, Type extends TSchema> = (Ensure<TObject<{
    true: Type;
    false: Type;
}>>);
type TFromIntegerKey<_Key extends TSchema, Type extends TSchema> = (Ensure<TRecord<TNumber, Type>>);
type TFromNumberKey<_Key extends TSchema, Type extends TSchema> = (Ensure<TRecord<TNumber, Type>>);
type RecordStatic<Key extends TSchema, Type extends TSchema, P extends unknown[]> = (Evaluate<{
    [_ in Assert<Static<Key>, PropertyKey>]: Static<Type, P>;
}>);
export interface TRecord<Key extends TSchema = TSchema, Type extends TSchema = TSchema> extends TSchema {
    [Kind]: "Record";
    static: RecordStatic<Key, Type, this["params"]>;
    type: "object";
    patternProperties: {
        [pattern: string]: Type;
    };
    additionalProperties: TAdditionalProperties;
}
export type TRecordOrObject<Key extends TSchema, Type extends TSchema> = (Key extends TTemplateLiteral ? TFromTemplateLiteralKey<Key, Type> : Key extends TEnum<infer Enum extends TEnumRecord> ? TFromEnumKey<Enum, Type> : Key extends TUnion<infer Types extends TSchema[]> ? TFromUnionKey<Types, Type> : Key extends TLiteral<infer Value extends TLiteralValue> ? TFromLiteralKey<Value, Type> : Key extends TBoolean ? TFromBooleanKey<Key, Type> : Key extends TInteger ? TFromIntegerKey<Key, Type> : Key extends TNumber ? TFromNumberKey<Key, Type> : Key extends TRegExp ? TFromRegExpKey<Key, Type> : Key extends TString ? TFromStringKey<Key, Type> : Key extends TAny ? TFromAnyKey<Key, Type> : Key extends TNever ? TFromNeverKey<Key, Type> : TNever);
declare function Record$1<Key extends TSchema, Type extends TSchema>(key: Key, type: Type, options?: ObjectOptions): TRecordOrObject<Key, Type>;
export declare function RecordPattern(record: TRecord): string;
export type TRecordKey<Type extends TRecord, Result extends TSchema = Type extends TRecord<infer Key extends TSchema, TSchema> ? (Key extends TNumber ? TNumber : Key extends TString ? TString : TString) : TString> = Result;
export declare function RecordKey<Type extends TRecord>(type: Type): TRecordKey<Type>;
export type TRecordValue<Type extends TRecord, Result extends TSchema = (Type extends TRecord<TSchema, infer Value extends TSchema> ? Value : TNever)> = Result;
export declare function RecordValue<Type extends TRecord>(type: Type): TRecordValue<Type>;
type TFromComputed$3<Target extends string, Parameters extends TSchema[]> = Ensure<TComputed<"Required", [
    TComputed<Target, Parameters>
]>>;
type TFromRef$3<Ref extends string> = Ensure<TComputed<"Required", [
    TRef<Ref>
]>>;
type TFromProperties$11<Properties extends TProperties> = Evaluate<{
    [K in keyof Properties]: Properties[K] extends (TReadonlyOptional<infer S>) ? TReadonly<S> : Properties[K] extends (TReadonly<infer S>) ? TReadonly<S> : Properties[K] extends (TOptional<infer S>) ? S : Properties[K];
}>;
type TFromObject$3<_Type extends TObject, Properties extends TProperties, MappedProperties extends TProperties = TFromProperties$11<Properties>, Result extends TSchema = TObject<MappedProperties>> = Result;
type TFromRest$5<Types extends TSchema[], Result extends TSchema[] = [
]> = (Types extends [
    infer L extends TSchema,
    ...infer R extends TSchema[]
] ? TFromRest$5<R, [
    ...Result,
    TRequired<L>
]> : Result);
export type TRequired<Type extends TSchema> = (Type extends TRecursive<infer Type extends TSchema> ? TRecursive<TRequired<Type>> : Type extends TComputed<infer Target extends string, infer Parameters extends TSchema[]> ? TFromComputed$3<Target, Parameters> : Type extends TRef<infer Ref extends string> ? TFromRef$3<Ref> : Type extends TIntersect<infer Types extends TSchema[]> ? TIntersect<TFromRest$5<Types>> : Type extends TUnion<infer Types extends TSchema[]> ? TUnion<TFromRest$5<Types>> : Type extends TObject<infer Properties extends TProperties> ? TFromObject$3<TObject, Properties> : Type extends TBigInt ? Type : Type extends TBoolean ? Type : Type extends TInteger ? Type : Type extends TLiteral ? Type : Type extends TNull ? Type : Type extends TNumber ? Type : Type extends TString ? Type : Type extends TSymbol ? Type : Type extends TUndefined ? Type : TObject<{}>);
declare function Required$1<MappedResult extends TMappedResult>(type: MappedResult, options?: SchemaOptions): TRequiredFromMappedResult<MappedResult>;
declare function Required$1<Type extends TSchema>(type: Type, options?: SchemaOptions): TRequired<Type>;
type TFromProperties$12<P extends TProperties> = ({
    [K2 in keyof P]: TRequired<P[K2]>;
});
type TFromMappedResult$8<R extends TMappedResult> = (Evaluate<TFromProperties$12<R["properties"]>>);
export type TRequiredFromMappedResult<R extends TMappedResult, P extends TProperties = TFromMappedResult$8<R>> = (Ensure<TMappedResult<P>>);
export declare function RequiredFromMappedResult<R extends TMappedResult, P extends TProperties = TFromMappedResult$8<R>>(R: R, options?: SchemaOptions): TMappedResult<P>;
export declare class TransformDecodeBuilder<T extends TSchema> {
    private readonly schema;
    constructor(schema: T);
    Decode<U extends unknown, D extends TransformFunction<StaticDecode<T>, U>>(decode: D): TransformEncodeBuilder<T, D>;
}
export declare class TransformEncodeBuilder<T extends TSchema, D extends TransformFunction> {
    private readonly schema;
    private readonly decode;
    constructor(schema: T, decode: D);
    private EncodeTransform;
    private EncodeSchema;
    Encode<E extends TransformFunction<ReturnType<D>, StaticDecode<T>>>(encode: E): TTransform<T, ReturnType<D>>;
}
type TransformStatic<T extends TSchema, P extends unknown[] = [
]> = T extends TTransform<infer _, infer S> ? S : Static<T, P>;
export type TransformFunction<T = any, U = any> = (value: T) => U;
export interface TransformOptions<I extends TSchema = TSchema, O extends unknown = unknown> {
    Decode: TransformFunction<StaticDecode<I>, O>;
    Encode: TransformFunction<O, StaticDecode<I>>;
}
export interface TTransform<I extends TSchema = TSchema, O extends unknown = unknown> extends TSchema {
    static: TransformStatic<I, this["params"]>;
    [TransformKind]: TransformOptions<I, O>;
    [key: string]: any;
}
export declare function Transform<I extends TSchema>(schema: I): TransformDecodeBuilder<I>;
type TDereferenceParameters<ModuleProperties extends TProperties, Types extends TSchema[], Result extends TSchema[] = [
]> = (Types extends [
    infer Left extends TSchema,
    ...infer Right extends TSchema[]
] ? Left extends TRef<infer Key extends string> ? TDereferenceParameters<ModuleProperties, Right, [
    ...Result,
    TDereference<ModuleProperties, Key>
]> : TDereferenceParameters<ModuleProperties, Right, [
    ...Result,
    TFromType<ModuleProperties, Left>
]> : Result);
type TDereference<ModuleProperties extends TProperties, Ref extends string, Result extends TSchema = (Ref extends keyof ModuleProperties ? ModuleProperties[Ref] extends TRef<infer Ref2 extends string> ? TDereference<ModuleProperties, Ref2> : TFromType<ModuleProperties, ModuleProperties[Ref]> : TNever)> = Result;
type TFromAwaited<Parameters extends TSchema[]> = (Parameters extends [
    infer T0 extends TSchema
] ? TAwaited<T0> : never);
type TFromIndex<Parameters extends TSchema[]> = (Parameters extends [
    infer T0 extends TSchema,
    infer T1 extends TSchema
] ? TIndex<T0, TIndexPropertyKeys<T1>> extends infer Result extends TSchema ? Result : never : never);
type TFromKeyOf<Parameters extends TSchema[]> = (Parameters extends [
    infer T0 extends TSchema
] ? TKeyOf<T0> : never);
type TFromPartial<Parameters extends TSchema[]> = (Parameters extends [
    infer T0 extends TSchema
] ? TPartial<T0> : never);
type TFromOmit<Parameters extends TSchema[]> = (Parameters extends [
    infer T0 extends TSchema,
    infer T1 extends TSchema
] ? TOmit<T0, T1> : never);
type TFromPick<Parameters extends TSchema[]> = (Parameters extends [
    infer T0 extends TSchema,
    infer T1 extends TSchema
] ? TPick<T0, T1> : never);
type TFromRequired<Parameters extends TSchema[]> = (Parameters extends [
    infer T0 extends TSchema
] ? TRequired<T0> : never);
type TFromComputed$4<ModuleProperties extends TProperties, Target extends string, Parameters extends TSchema[], Dereferenced extends TSchema[] = TDereferenceParameters<ModuleProperties, Parameters>> = (Target extends "Awaited" ? TFromAwaited<Dereferenced> : Target extends "Index" ? TFromIndex<Dereferenced> : Target extends "KeyOf" ? TFromKeyOf<Dereferenced> : Target extends "Partial" ? TFromPartial<Dereferenced> : Target extends "Omit" ? TFromOmit<Dereferenced> : Target extends "Pick" ? TFromPick<Dereferenced> : Target extends "Required" ? TFromRequired<Dereferenced> : TNever);
type TFromArray$2<ModuleProperties extends TProperties, Type extends TSchema> = (Ensure<TArray<TFromType<ModuleProperties, Type>>>);
type TFromAsyncIterator<ModuleProperties extends TProperties, Type extends TSchema> = (TAsyncIterator<TFromType<ModuleProperties, Type>>);
type TFromConstructor<ModuleProperties extends TProperties, Parameters extends TSchema[], InstanceType extends TSchema> = (TConstructor<TFromTypes<ModuleProperties, Parameters>, TFromType<ModuleProperties, InstanceType>>);
type TFromFunction<ModuleProperties extends TProperties, Parameters extends TSchema[], ReturnType extends TSchema> = Ensure<Ensure<TFunction<TFromTypes<ModuleProperties, Parameters>, TFromType<ModuleProperties, ReturnType>>>>;
type TFromIntersect$4<ModuleProperties extends TProperties, Types extends TSchema[]> = (Ensure<TIntersectEvaluated<TFromTypes<ModuleProperties, Types>>>);
type TFromIterator<ModuleProperties extends TProperties, Type extends TSchema> = (TIterator<TFromType<ModuleProperties, Type>>);
type TFromObject$4<ModuleProperties extends TProperties, Properties extends TProperties> = Ensure<TObject<Evaluate<{
    [Key in keyof Properties]: TFromType<ModuleProperties, Properties[Key]>;
}>>>;
type TFromRecord<ModuleProperties extends TProperties, Key extends TSchema, Value extends TSchema, Result extends TSchema = TRecordOrObject<Key, TFromType<ModuleProperties, Value>>> = Result;
type TFromTransform<ModuleProperties extends TProperties, Input extends TSchema, Output extends unknown, Result extends TSchema = Input extends TRef<infer Key extends string> ? TTransform<TDereference<ModuleProperties, Key>, Output> : TTransform<Input, Output>> = Result;
type TFromTuple$2<ModuleProperties extends TProperties, Types extends TSchema[]> = (Ensure<TTuple<TFromTypes<ModuleProperties, Types>>>);
type TFromUnion$5<ModuleProperties extends TProperties, Types extends TSchema[]> = (Ensure<TUnionEvaluated<TFromTypes<ModuleProperties, Types>>>);
type TFromTypes<ModuleProperties extends TProperties, Types extends TSchema[], Result extends TSchema[] = [
]> = (Types extends [
    infer Left extends TSchema,
    ...infer Right extends TSchema[]
] ? TFromTypes<ModuleProperties, Right, [
    ...Result,
    TFromType<ModuleProperties, Left>
]> : Result);
type TFromType<ModuleProperties extends TProperties, Type extends TSchema> = (Type extends TOptional<infer Type extends TSchema> ? TOptional<TFromType<ModuleProperties, Type>> : Type extends TReadonly<infer Type extends TSchema> ? TReadonly<TFromType<ModuleProperties, Type>> : Type extends TTransform<infer Input extends TSchema, infer Output extends unknown> ? TFromTransform<ModuleProperties, Input, Output> : Type extends TArray<infer Type extends TSchema> ? TFromArray$2<ModuleProperties, Type> : Type extends TAsyncIterator<infer Type extends TSchema> ? TFromAsyncIterator<ModuleProperties, Type> : Type extends TComputed<infer Target extends string, infer Parameters extends TSchema[]> ? TFromComputed$4<ModuleProperties, Target, Parameters> : Type extends TConstructor<infer Parameters extends TSchema[], infer InstanceType extends TSchema> ? TFromConstructor<ModuleProperties, Parameters, InstanceType> : Type extends TFunction<infer Parameters extends TSchema[], infer ReturnType extends TSchema> ? TFromFunction<ModuleProperties, Parameters, ReturnType> : Type extends TIntersect<infer Types extends TSchema[]> ? TFromIntersect$4<ModuleProperties, Types> : Type extends TIterator<infer Type extends TSchema> ? TFromIterator<ModuleProperties, Type> : Type extends TObject<infer Properties extends TProperties> ? TFromObject$4<ModuleProperties, Properties> : Type extends TRecord<infer Key extends TSchema, infer Value extends TSchema> ? TFromRecord<ModuleProperties, Key, Value> : Type extends TTuple<infer Types extends TSchema[]> ? TFromTuple$2<ModuleProperties, Types> : Type extends TEnum<infer _ extends TEnumRecord> ? Type : Type extends TUnion<infer Types extends TSchema[]> ? TFromUnion$5<ModuleProperties, Types> : Type);
type TComputeType<ModuleProperties extends TProperties, Key extends PropertyKey> = (Key extends keyof ModuleProperties ? TFromType<ModuleProperties, ModuleProperties[Key]> : TNever);
type TComputeModuleProperties<ModuleProperties extends TProperties> = Evaluate<{
    [Key in keyof ModuleProperties]: TComputeType<ModuleProperties, Key>;
}>;
type TInferArray<ModuleProperties extends TProperties, Type extends TSchema> = (Ensure<Array<TInfer<ModuleProperties, Type>>>);
type TInferAsyncIterator<ModuleProperties extends TProperties, Type extends TSchema> = (Ensure<AsyncIterableIterator<TInfer<ModuleProperties, Type>>>);
type TInferConstructor<ModuleProperties extends TProperties, Parameters extends TSchema[], InstanceType extends TSchema> = Ensure<new (...args: TInferTuple<ModuleProperties, Parameters>) => TInfer<ModuleProperties, InstanceType>>;
type TInferFunction<ModuleProperties extends TProperties, Parameters extends TSchema[], ReturnType extends TSchema> = Ensure<(...args: TInferTuple<ModuleProperties, Parameters>) => TInfer<ModuleProperties, ReturnType>>;
type TInferIterator<ModuleProperties extends TProperties, Type extends TSchema> = (Ensure<IterableIterator<TInfer<ModuleProperties, Type>>>);
type TInferIntersect<ModuleProperties extends TProperties, Types extends TSchema[], Result extends unknown = unknown> = (Types extends [
    infer Left extends TSchema,
    ...infer Right extends TSchema[]
] ? TInferIntersect<ModuleProperties, Right, Result & TInfer<ModuleProperties, Left>> : Result);
type ReadonlyOptionalPropertyKeys<Properties extends TProperties> = {
    [Key in keyof Properties]: Properties[Key] extends TReadonly<TSchema> ? (Properties[Key] extends TOptional<Properties[Key]> ? Key : never) : never;
}[keyof Properties];
type ReadonlyPropertyKeys<Source extends TProperties> = {
    [Key in keyof Source]: Source[Key] extends TReadonly<TSchema> ? (Source[Key] extends TOptional<Source[Key]> ? never : Key) : never;
}[keyof Source];
type OptionalPropertyKeys<Source extends TProperties> = {
    [Key in keyof Source]: Source[Key] extends TOptional<TSchema> ? (Source[Key] extends TReadonly<Source[Key]> ? never : Key) : never;
}[keyof Source];
type RequiredPropertyKeys<Source extends TProperties> = keyof Omit<Source, ReadonlyOptionalPropertyKeys<Source> | ReadonlyPropertyKeys<Source> | OptionalPropertyKeys<Source>>;
type InferPropertiesWithModifiers<Properties extends TProperties, Source extends Record<keyof any, unknown>> = Evaluate<(Readonly<Partial<Pick<Source, ReadonlyOptionalPropertyKeys<Properties>>>> & Readonly<Pick<Source, ReadonlyPropertyKeys<Properties>>> & Partial<Pick<Source, OptionalPropertyKeys<Properties>>> & Required<Pick<Source, RequiredPropertyKeys<Properties>>>)>;
type InferProperties<ModuleProperties extends TProperties, Properties extends TProperties> = InferPropertiesWithModifiers<Properties, {
    [K in keyof Properties]: TInfer<ModuleProperties, Properties[K]>;
}>;
type TInferObject<ModuleProperties extends TProperties, Properties extends TProperties> = (InferProperties<ModuleProperties, Properties>);
type TInferTuple<ModuleProperties extends TProperties, Types extends TSchema[], Result extends unknown[] = [
]> = (Types extends [
    infer L extends TSchema,
    ...infer R extends TSchema[]
] ? TInferTuple<ModuleProperties, R, [
    ...Result,
    TInfer<ModuleProperties, L>
]> : Result);
type TInferRecord<ModuleProperties extends TProperties, Key extends TSchema, Type extends TSchema, InferredKey extends PropertyKey = TInfer<ModuleProperties, Key> extends infer Key extends PropertyKey ? Key : never, InferedType extends unknown = TInfer<ModuleProperties, Type>> = Ensure<{
    [_ in InferredKey]: InferedType;
}>;
type TInferRef<ModuleProperties extends TProperties, Ref extends string> = (Ref extends keyof ModuleProperties ? TInfer<ModuleProperties, ModuleProperties[Ref]> : unknown);
type TInferUnion<ModuleProperties extends TProperties, Types extends TSchema[], Result extends unknown = never> = (Types extends [
    infer L extends TSchema,
    ...infer R extends TSchema[]
] ? TInferUnion<ModuleProperties, R, Result | TInfer<ModuleProperties, L>> : Result);
type TInfer<ModuleProperties extends TProperties, Type extends TSchema> = (Type extends TArray<infer Type extends TSchema> ? TInferArray<ModuleProperties, Type> : Type extends TAsyncIterator<infer Type extends TSchema> ? TInferAsyncIterator<ModuleProperties, Type> : Type extends TConstructor<infer Parameters extends TSchema[], infer InstanceType extends TSchema> ? TInferConstructor<ModuleProperties, Parameters, InstanceType> : Type extends TFunction<infer Parameters extends TSchema[], infer ReturnType extends TSchema> ? TInferFunction<ModuleProperties, Parameters, ReturnType> : Type extends TIntersect<infer Types extends TSchema[]> ? TInferIntersect<ModuleProperties, Types> : Type extends TIterator<infer Type extends TSchema> ? TInferIterator<ModuleProperties, Type> : Type extends TObject<infer Properties extends TProperties> ? TInferObject<ModuleProperties, Properties> : Type extends TRecord<infer Key extends TSchema, infer Type extends TSchema> ? TInferRecord<ModuleProperties, Key, Type> : Type extends TRef<infer Ref extends string> ? TInferRef<ModuleProperties, Ref> : Type extends TTuple<infer Types extends TSchema[]> ? TInferTuple<ModuleProperties, Types> : Type extends TEnum<infer _ extends TEnumRecord> ? Static<Type> : Type extends TUnion<infer Types extends TSchema[]> ? TInferUnion<ModuleProperties, Types> : Type extends TRecursive<infer Schema extends TSchema> ? TInfer<ModuleProperties, Schema> : Static<Type>);
type TInferFromModuleKey<ModuleProperties extends TProperties, Key extends PropertyKey> = (Key extends keyof ModuleProperties ? TInfer<ModuleProperties, ModuleProperties[Key]> : never);
export interface TDefinitions<ModuleProperties extends TProperties> extends TSchema {
    static: {
        [K in keyof ModuleProperties]: Static<ModuleProperties[K]>;
    };
    $defs: ModuleProperties;
}
export interface TImport<ModuleProperties extends TProperties = {}, Key extends keyof ModuleProperties = keyof ModuleProperties> extends TSchema {
    [Kind]: "Import";
    static: TInferFromModuleKey<ModuleProperties, Key>;
    $defs: ModuleProperties;
    $ref: Key;
}
export declare class TModule<ModuleProperties extends TProperties, ComputedModuleProperties extends TProperties = TComputeModuleProperties<ModuleProperties>> {
    private readonly $defs;
    constructor($defs: ModuleProperties);
    Import<Key extends keyof ComputedModuleProperties>(key: Key, options?: SchemaOptions): TImport<ComputedModuleProperties, Key>;
    private WithIdentifiers;
}
export declare function Module<Properties extends TProperties>(properties: Properties): TModule<Properties>;
export interface TNot<T extends TSchema = TSchema> extends TSchema {
    [Kind]: "Not";
    static: T extends TNot<infer U> ? Static<U> : unknown;
    not: T;
}
export declare function Not<Type extends TSchema>(type: Type, options?: SchemaOptions): TNot<Type>;
type TDecodeImport<ModuleProperties extends TProperties, Key extends PropertyKey> = (Key extends keyof ModuleProperties ? TDecodeType<ModuleProperties[Key]> extends infer Type extends TSchema ? Type extends TRef<infer Ref extends string> ? TDecodeImport<ModuleProperties, Ref> : Type : TNever : TNever);
type TDecodeProperties<Properties extends TProperties> = {
    [Key in keyof Properties]: TDecodeType<Properties[Key]>;
};
type TDecodeTypes<Types extends TSchema[], Result extends TSchema[] = [
]> = (Types extends [
    infer Left extends TSchema,
    ...infer Right extends TSchema[]
] ? TDecodeTypes<Right, [
    ...Result,
    TDecodeType<Left>
]> : Result);
export type TDecodeType<Type extends TSchema> = (Type extends TOptional<infer Type extends TSchema> ? TOptional<TDecodeType<Type>> : Type extends TReadonly<infer Type extends TSchema> ? TReadonly<TDecodeType<Type>> : Type extends TTransform<infer _Input extends TSchema, infer Output> ? TUnsafe<Output> : Type extends TArray<infer Type extends TSchema> ? TArray<TDecodeType<Type>> : Type extends TAsyncIterator<infer Type extends TSchema> ? TAsyncIterator<TDecodeType<Type>> : Type extends TConstructor<infer Parameters extends TSchema[], infer InstanceType extends TSchema> ? TConstructor<TDecodeTypes<Parameters>, TDecodeType<InstanceType>> : Type extends TEnum<infer Values> ? TEnum<Values> : Type extends TFunction<infer Parameters extends TSchema[], infer ReturnType extends TSchema> ? TFunction<TDecodeTypes<Parameters>, TDecodeType<ReturnType>> : Type extends TIntersect<infer Types extends TSchema[]> ? TIntersect<TDecodeTypes<Types>> : Type extends TImport<infer ModuleProperties extends TProperties, infer Key> ? TDecodeImport<ModuleProperties, Key> : Type extends TIterator<infer Type extends TSchema> ? TIterator<TDecodeType<Type>> : Type extends TNot<infer Type extends TSchema> ? TNot<TDecodeType<Type>> : Type extends TObject<infer Properties extends TProperties> ? TObject<Evaluate<TDecodeProperties<Properties>>> : Type extends TPromise<infer Type extends TSchema> ? TPromise<TDecodeType<Type>> : Type extends TRecord<infer Key extends TSchema, infer Value extends TSchema> ? TRecord<Key, TDecodeType<Value>> : Type extends TRecursive<infer Type extends TSchema> ? TRecursive<TDecodeType<Type>> : Type extends TRef<infer Ref extends string> ? TRef<Ref> : Type extends TTuple<infer Types extends TSchema[]> ? TTuple<TDecodeTypes<Types>> : Type extends TUnion<infer Types extends TSchema[]> ? TUnion<TDecodeTypes<Types>> : Type);
export type StaticDecodeIsAny<Type> = boolean extends (Type extends TSchema ? true : false) ? true : false;
export type StaticDecode<Type extends TSchema, Params extends unknown[] = [
], Result = StaticDecodeIsAny<Type> extends true ? unknown : Static<TDecodeType<Type>, Params>> = Result;
export type StaticEncode<Type extends TSchema, Params extends unknown[] = [
], Result = Static<Type, Params>> = Result;
export type Static<Type extends TSchema, Params extends unknown[] = [
], Result = (Type & {
    params: Params;
})["static"]> = Result;
type ReadonlyOptionalPropertyKeys$1<T extends TProperties> = {
    [K in keyof T]: T[K] extends TReadonly<TSchema> ? (T[K] extends TOptional<T[K]> ? K : never) : never;
}[keyof T];
type ReadonlyPropertyKeys$1<T extends TProperties> = {
    [K in keyof T]: T[K] extends TReadonly<TSchema> ? (T[K] extends TOptional<T[K]> ? never : K) : never;
}[keyof T];
type OptionalPropertyKeys$1<T extends TProperties> = {
    [K in keyof T]: T[K] extends TOptional<TSchema> ? (T[K] extends TReadonly<T[K]> ? never : K) : never;
}[keyof T];
type RequiredPropertyKeys$1<T extends TProperties> = keyof Omit<T, ReadonlyOptionalPropertyKeys$1<T> | ReadonlyPropertyKeys$1<T> | OptionalPropertyKeys$1<T>>;
type ObjectStaticProperties<T extends TProperties, R extends Record<keyof any, unknown>> = Evaluate<(Readonly<Partial<Pick<R, ReadonlyOptionalPropertyKeys$1<T>>>> & Readonly<Pick<R, ReadonlyPropertyKeys$1<T>>> & Partial<Pick<R, OptionalPropertyKeys$1<T>>> & Required<Pick<R, RequiredPropertyKeys$1<T>>>)>;
type ObjectStatic<T extends TProperties, P extends unknown[]> = ObjectStaticProperties<T, {
    [K in keyof T]: Static<T[K], P>;
}>;
export type TPropertyKey = string | number;
export type TProperties = Record<TPropertyKey, TSchema>;
type TIsLiteralString<Type extends string> = ([
    Type
] extends [
    string
] ? [
    string
] extends [
    Type
] ? false : true : false);
type IsRequiredArrayLiteralConstant<RequiredTuple extends string[]> = (RequiredTuple extends [
    infer Left extends string,
    ...infer _ extends string[]
] ? TIsLiteralString<Left> : false);
type TRequiredArray<Properties extends TProperties, RequiredProperties extends TProperties = {
    [Key in keyof Properties as Properties[Key] extends TOptional<Properties[Key]> ? never : Key]: Properties[Key];
}, RequiredUnion extends string = Extract<keyof RequiredProperties, string>, RequiredTuple extends string[] = UnionToTuple<RequiredUnion>, Result extends string[] | undefined = (IsRequiredArrayLiteralConstant<RequiredTuple> extends true ? RequiredTuple : string[] | undefined)> = Result;
export type TAdditionalProperties = undefined | TSchema | boolean;
export interface ObjectOptions extends SchemaOptions {
    additionalProperties?: TAdditionalProperties;
    minProperties?: number;
    maxProperties?: number;
}
export interface TObject<T extends TProperties = TProperties> extends TSchema, ObjectOptions {
    [Kind]: "Object";
    static: ObjectStatic<T, this["params"]>;
    additionalProperties?: TAdditionalProperties;
    type: "object";
    properties: T;
    required: TRequiredArray<T>;
}
declare function _Object_<T extends TProperties>(properties: T, options?: ObjectOptions): TObject<T>;
declare var Object$1: typeof _Object_;
export type TupleToIntersect<T extends any[]> = T extends [
    infer I
] ? I : T extends [
    infer I,
    ...infer R
] ? I & TupleToIntersect<R> : never;
export type TupleToUnion<T extends any[]> = {
    [K in keyof T]: T[K];
}[number];
export type UnionToIntersect<U> = (U extends unknown ? (arg: U) => 0 : never) extends (arg: infer I) => 0 ? I : never;
export type UnionLast<U> = UnionToIntersect<U extends unknown ? (x: U) => 0 : never> extends (x: infer L) => 0 ? L : never;
export type UnionToTuple<U, Acc extends unknown[] = [
], R = UnionLast<U>> = [
    U
] extends [
    never
] ? Acc : UnionToTuple<Exclude<U, R>, [
    Extract<U, R>,
    ...Acc
]>;
export type Trim<T> = T extends `${" "}${infer U}` ? Trim<U> : T extends `${infer U}${" "}` ? Trim<U> : T;
export type Assert<T, E> = T extends E ? T : never;
export type Evaluate<T> = T extends infer O ? {
    [K in keyof O]: O[K];
} : never;
export type Ensure<T> = T extends infer U ? U : never;
export type EmptyString = "";
export type ZeroString = "0";
type IncrementBase = {
    m: "9";
    t: "01";
    "0": "1";
    "1": "2";
    "2": "3";
    "3": "4";
    "4": "5";
    "5": "6";
    "6": "7";
    "7": "8";
    "8": "9";
    "9": "0";
};
type IncrementTake<T extends keyof IncrementBase> = IncrementBase[T];
type IncrementStep<T extends string> = T extends IncrementBase["m"] ? IncrementBase["t"] : T extends `${infer L extends keyof IncrementBase}${infer R}` ? L extends IncrementBase["m"] ? `${IncrementTake<L>}${IncrementStep<R>}` : `${IncrementTake<L>}${R}` : never;
type IncrementReverse<T extends string> = T extends `${infer L}${infer R}` ? `${IncrementReverse<R>}${L}` : T;
export type TIncrement<T extends string> = IncrementReverse<IncrementStep<IncrementReverse<T>>>;
export declare function Increment<T extends string>(T: T): TIncrement<T>;
export type AssertProperties<T> = T extends TProperties ? T : TProperties;
export type AssertRest<T, E extends TSchema[] = TSchema[]> = T extends E ? T : [
];
export type AssertType<T, E extends TSchema = TSchema> = T extends E ? T : TNever;
export interface ArrayOptions extends SchemaOptions {
    minItems?: number;
    maxItems?: number;
    uniqueItems?: boolean;
    contains?: TSchema;
    minContains?: number;
    maxContains?: number;
}
type ArrayStatic<T extends TSchema, P extends unknown[]> = Ensure<Static<T, P>[]>;
export interface TArray<T extends TSchema = TSchema> extends TSchema, ArrayOptions {
    [Kind]: "Array";
    static: ArrayStatic<T, this["params"]>;
    type: "array";
    items: T;
}
declare function Array$1<Type extends TSchema>(items: Type, options?: ArrayOptions): TArray<Type>;
export interface DateOptions extends SchemaOptions {
    exclusiveMaximumTimestamp?: number;
    exclusiveMinimumTimestamp?: number;
    maximumTimestamp?: number;
    minimumTimestamp?: number;
    multipleOfTimestamp?: number;
}
export interface TDate extends TSchema, DateOptions {
    [Kind]: "Date";
    static: Date;
    type: "date";
}
declare function Date$1(options?: DateOptions): TDate;
export interface Uint8ArrayOptions extends SchemaOptions {
    maxByteLength?: number;
    minByteLength?: number;
}
export interface TUint8Array extends TSchema, Uint8ArrayOptions {
    [Kind]: "Uint8Array";
    static: Uint8Array;
    type: "uint8array";
}
declare function Uint8Array$1(options?: Uint8ArrayOptions): TUint8Array;
export interface TUnknown extends TSchema {
    [Kind]: "Unknown";
    static: unknown;
}
export declare function Unknown(options?: SchemaOptions): TUnknown;
export interface TVoid extends TSchema {
    [Kind]: "Void";
    static: void;
    type: "void";
}
export declare function Void(options?: SchemaOptions): TVoid;
export interface SchemaOptions {
    $schema?: string;
    $id?: string;
    title?: string;
    description?: string;
    default?: any;
    examples?: any;
    readOnly?: boolean;
    writeOnly?: boolean;
    [prop: string]: any;
}
export interface TKind {
    [Kind]: string;
}
export interface TSchema extends TKind, SchemaOptions {
    [ReadonlyKind]?: string;
    [OptionalKind]?: string;
    [Hint]?: string;
    params: unknown[];
    static: unknown;
}
export type TAnySchema = TSchema | TAny | TArray | TAsyncIterator | TBigInt | TBoolean | TConstructor | TDate | TEnum | TFunction | TInteger | TIntersect | TIterator | TLiteral | TNot | TNull | TNumber | TObject | TPromise | TRecord | TRef | TRegExp | TString | TSymbol | TTemplateLiteral | TThis | TTuple | TUndefined | TUnion | TUint8Array | TUnknown | TVoid;
export declare function CloneRest<T extends TSchema[]>(schemas: T): T;
export declare function CloneType<T extends TSchema>(schema: T, options?: SchemaOptions): T;
export declare function Clone<T>(value: T): T;
export declare function CreateType(schema: Record<any, unknown>, options?: SchemaOptions): unknown;
export interface TArgument<Index extends number = number> extends TSchema {
    [Kind]: "Argument";
    static: unknown;
    index: Index;
}
export declare function Argument<Index extends number>(index: Index): TArgument<Index>;
declare function IsReadonly<T extends TSchema>(value: T): value is TReadonly<T>;
declare function IsOptional<T extends TSchema>(value: T): value is TOptional<T>;
declare function IsAny(value: unknown): value is TAny;
declare function IsArgument(value: unknown): value is TArgument;
declare function IsArray(value: unknown): value is TArray;
declare function IsAsyncIterator(value: unknown): value is TAsyncIterator;
declare function IsBigInt(value: unknown): value is TBigInt;
declare function IsBoolean(value: unknown): value is TBoolean;
declare function IsComputed(value: unknown): value is TComputed;
declare function IsConstructor(value: unknown): value is TConstructor;
declare function IsDate(value: unknown): value is TDate;
declare function IsFunction(value: unknown): value is TFunction;
declare function IsImport(value: unknown): value is TImport;
declare function IsInteger(value: unknown): value is TInteger;
declare function IsProperties(value: unknown): value is TProperties;
declare function IsIntersect(value: unknown): value is TIntersect;
declare function IsIterator(value: unknown): value is TIterator;
declare function IsKindOf<T extends string>(value: unknown, kind: T): value is Record<PropertyKey, unknown> & {
    [Kind]: T;
};
declare function IsLiteralString(value: unknown): value is TLiteral<string>;
declare function IsLiteralNumber(value: unknown): value is TLiteral<number>;
declare function IsLiteralBoolean(value: unknown): value is TLiteral<boolean>;
declare function IsLiteralValue(value: unknown): value is TLiteralValue;
declare function IsLiteral(value: unknown): value is TLiteral;
declare function IsMappedKey(value: unknown): value is TMappedKey;
declare function IsMappedResult(value: unknown): value is TMappedResult;
declare function IsNever(value: unknown): value is TNever;
declare function IsNot(value: unknown): value is TNot;
declare function IsNull(value: unknown): value is TNull;
declare function IsNumber(value: unknown): value is TNumber;
declare function IsObject(value: unknown): value is TObject;
declare function IsPromise(value: unknown): value is TPromise;
declare function IsRecord(value: unknown): value is TRecord;
declare function IsRecursive(value: unknown): value is {
    [Hint]: "Recursive";
};
declare function IsRef(value: unknown): value is TRef;
declare function IsRegExp(value: unknown): value is TRegExp;
declare function IsString(value: unknown): value is TString;
declare function IsSymbol(value: unknown): value is TSymbol;
declare function IsTemplateLiteral(value: unknown): value is TTemplateLiteral;
declare function IsThis(value: unknown): value is TThis;
declare function IsTransform(value: unknown): value is {
    [TransformKind]: TransformOptions;
};
declare function IsTuple(value: unknown): value is TTuple;
declare function IsUndefined(value: unknown): value is TUndefined;
declare function IsUnion(value: unknown): value is TUnion;
declare function IsUint8Array(value: unknown): value is TUint8Array;
declare function IsUnknown(value: unknown): value is TUnknown;
declare function IsUnsafe(value: unknown): value is TUnsafe<unknown>;
declare function IsVoid(value: unknown): value is TVoid;
declare function IsKind(value: unknown): value is Record<PropertyKey, unknown> & {
    [Kind]: string;
};
declare function IsSchema(value: unknown): value is TSchema;
declare class TypeGuardUnknownTypeError extends TypeBoxError {
}
declare function IsReadonly$1<T extends TSchema>(value: T): value is TReadonly<T>;
declare function IsOptional$1<T extends TSchema>(value: T): value is TOptional<T>;
declare function IsAny$1(value: unknown): value is TAny;
declare function IsArgument$1(value: unknown): value is TArgument;
declare function IsArray$1(value: unknown): value is TArray;
declare function IsAsyncIterator$1(value: unknown): value is TAsyncIterator;
declare function IsBigInt$1(value: unknown): value is TBigInt;
declare function IsBoolean$1(value: unknown): value is TBoolean;
declare function IsComputed$1(value: unknown): value is TComputed;
declare function IsConstructor$1(value: unknown): value is TConstructor;
declare function IsDate$1(value: unknown): value is TDate;
declare function IsFunction$1(value: unknown): value is TFunction;
declare function IsImport$1(value: unknown): value is TImport;
declare function IsInteger$1(value: unknown): value is TInteger;
declare function IsProperties$1(value: unknown): value is TProperties;
declare function IsIntersect$1(value: unknown): value is TIntersect;
declare function IsIterator$1(value: unknown): value is TIterator;
declare function IsKindOf$1<T extends string>(value: unknown, kind: T): value is Record<PropertyKey, unknown> & {
    [Kind]: T;
};
declare function IsLiteralString$1(value: unknown): value is TLiteral<string>;
declare function IsLiteralNumber$1(value: unknown): value is TLiteral<number>;
declare function IsLiteralBoolean$1(value: unknown): value is TLiteral<boolean>;
declare function IsLiteral$1(value: unknown): value is TLiteral;
declare function IsLiteralValue$1(value: unknown): value is TLiteralValue;
declare function IsMappedKey$1(value: unknown): value is TMappedKey;
declare function IsMappedResult$1(value: unknown): value is TMappedResult;
declare function IsNever$1(value: unknown): value is TNever;
declare function IsNot$1(value: unknown): value is TNot;
declare function IsNull$1(value: unknown): value is TNull;
declare function IsNumber$1(value: unknown): value is TNumber;
declare function IsObject$1(value: unknown): value is TObject;
declare function IsPromise$1(value: unknown): value is TPromise;
declare function IsRecord$1(value: unknown): value is TRecord;
declare function IsRecursive$1(value: unknown): value is {
    [Hint]: "Recursive";
};
declare function IsRef$1(value: unknown): value is TRef;
declare function IsRegExp$1(value: unknown): value is TRegExp;
declare function IsString$1(value: unknown): value is TString;
declare function IsSymbol$1(value: unknown): value is TSymbol;
declare function IsTemplateLiteral$1(value: unknown): value is TTemplateLiteral;
declare function IsThis$1(value: unknown): value is TThis;
declare function IsTransform$1(value: unknown): value is {
    [TransformKind]: TransformOptions;
};
declare function IsTuple$1(value: unknown): value is TTuple;
declare function IsUndefined$1(value: unknown): value is TUndefined;
declare function IsUnionLiteral(value: unknown): value is TUnion<TLiteral[]>;
declare function IsUnion$1(value: unknown): value is TUnion;
declare function IsUint8Array$1(value: unknown): value is TUint8Array;
declare function IsUnknown$1(value: unknown): value is TUnknown;
declare function IsUnsafe$1(value: unknown): value is TUnsafe<unknown>;
declare function IsVoid$1(value: unknown): value is TVoid;
declare function IsKind$1(value: unknown): value is Record<PropertyKey, unknown> & {
    [Kind]: string;
};
declare function IsSchema$1(value: unknown): value is TSchema;
declare function HasPropertyKey<K extends PropertyKey>(value: Record<any, unknown>, key: K): value is Record<PropertyKey, unknown> & {
    [_ in K]: unknown;
};
declare function IsAsyncIterator$2(value: unknown): value is AsyncIterableIterator<unknown>;
declare function IsArray$2(value: unknown): value is unknown[];
declare function IsBigInt$2(value: unknown): value is bigint;
declare function IsBoolean$2(value: unknown): value is boolean;
declare function IsDate$2(value: unknown): value is Date;
declare function IsFunction$2(value: unknown): value is Function;
declare function IsIterator$2(value: unknown): value is IterableIterator<unknown>;
declare function IsNull$2(value: unknown): value is null;
declare function IsNumber$2(value: unknown): value is number;
declare function IsObject$2(value: unknown): value is Record<PropertyKey, unknown>;
declare function IsRegExp$2(value: unknown): value is RegExp;
declare function IsString$2(value: unknown): value is string;
declare function IsSymbol$2(value: unknown): value is symbol;
declare function IsUint8Array$2(value: unknown): value is Uint8Array;
declare function IsUndefined$2(value: unknown): value is undefined;
export declare const PatternBoolean = "(true|false)";
export declare const PatternNumber = "(0|[1-9][0-9]*)";
export declare const PatternString = "(.*)";
export declare const PatternNever = "(?!.*)";
export declare const PatternBooleanExact = "^(true|false)$";
export declare const PatternNumberExact = "^(0|[1-9][0-9]*)$";
export declare const PatternStringExact = "^(.*)$";
export declare const PatternNeverExact = "^(?!.*)$";
type FormatRegistryValidationFunction = (value: string) => boolean;
declare function Entries(): Map<string, FormatRegistryValidationFunction>;
declare function Clear(): void;
declare function Delete(format: string): boolean;
declare function Has(format: string): boolean;
declare function Set$1(format: string, func: FormatRegistryValidationFunction): void;
declare function Get(format: string): FormatRegistryValidationFunction | undefined;
type TypeRegistryValidationFunction<TSchema> = (schema: TSchema, value: unknown) => boolean;
declare function Entries$1(): Map<string, TypeRegistryValidationFunction<any>>;
declare function Clear$1(): void;
declare function Delete$1(kind: string): boolean;
declare function Has$1(kind: string): boolean;
declare function Set$1<TSchema = unknown>(kind: string, func: TypeRegistryValidationFunction<TSchema>): void;
declare function Get$1(kind: string): TypeRegistryValidationFunction<any> | undefined;
type TCompositeKeys<T extends TSchema[], Acc extends PropertyKey[] = [
]> = (T extends [
    infer L extends TSchema,
    ...infer R extends TSchema[]
] ? TCompositeKeys<R, [
    ...Acc,
    ...TKeyOfPropertyKeys<L>
]> : TSetDistinct<Acc>);
type TFilterNever<T extends TSchema[], Acc extends TSchema[] = [
]> = (T extends [
    infer L extends TSchema,
    ...infer R extends TSchema[]
] ? L extends TNever ? TFilterNever<R, [
    ...Acc
]> : TFilterNever<R, [
    ...Acc,
    L
]> : Acc);
type TCompositeProperty<T extends TSchema[], K extends PropertyKey, Acc extends TSchema[] = [
]> = (T extends [
    infer L extends TSchema,
    ...infer R extends TSchema[]
] ? TCompositeProperty<R, K, [
    ...Acc,
    ...TIndexFromPropertyKeys<L, [
        K
    ]>
]> : TFilterNever<Acc>);
type TCompositeProperties<T extends TSchema[], K extends PropertyKey[], Acc = {}> = (K extends [
    infer L extends PropertyKey,
    ...infer R extends PropertyKey[]
] ? TCompositeProperties<T, R, Acc & {
    [_ in L]: TIntersectEvaluated<TCompositeProperty<T, L>>;
}> : Acc);
type TCompositeEvaluate<T extends TSchema[], K extends PropertyKey[] = TCompositeKeys<T>, P extends TProperties = Evaluate<TCompositeProperties<T, K>>, R extends TSchema = TObject<P>> = R;
export type TComposite<T extends TSchema[]> = TCompositeEvaluate<T>;
export declare function Composite<T extends TSchema[]>(T: [
    ...T
], options?: ObjectOptions): TComposite<T>;
type TFromArray$3<T extends readonly unknown[]> = T extends readonly [
    infer L extends unknown,
    ...infer R extends unknown[]
] ? [
    FromValue<L, false>,
    ...TFromArray$3<R>
] : T;
type TFromProperties$13<T extends Record<PropertyKey, unknown>> = {
    -readonly [K in keyof T]: FromValue<T[K], false> extends infer R extends TSchema ? TReadonly<R> : TReadonly<TNever>;
};
type TConditionalReadonly<T extends TSchema, Root extends boolean> = Root extends true ? T : TReadonly<T>;
type FromValue<T, Root extends boolean> = T extends AsyncIterableIterator<unknown> ? TConditionalReadonly<TAny, Root> : T extends IterableIterator<unknown> ? TConditionalReadonly<TAny, Root> : T extends readonly unknown[] ? TReadonly<TTuple<AssertRest<TFromArray$3<T>>>> : T extends Uint8Array ? TUint8Array : T extends Date ? TDate : T extends Record<PropertyKey, unknown> ? TConditionalReadonly<TObject<Evaluate<TFromProperties$13<T>>>, Root> : T extends Function ? TConditionalReadonly<TFunction<[
], TUnknown>, Root> : T extends undefined ? TUndefined : T extends null ? TNull : T extends symbol ? TSymbol : T extends number ? TLiteral<T> : T extends boolean ? TLiteral<T> : T extends string ? TLiteral<T> : T extends bigint ? TBigInt : TObject<{}>;
declare function FromValue<T, Root extends boolean>(value: T, root: Root): FromValue<T, Root>;
export type TConst<T> = FromValue<T, true>;
export declare function Const<T>(T: T, options?: SchemaOptions): TConst<T>;
export type TConstructorParameters<Type extends TSchema> = (Type extends TConstructor<infer Parameters extends TSchema[], infer _InstanceType extends TSchema> ? TTuple<Parameters> : TNever);
declare function ConstructorParameters$1<Type extends TSchema>(schema: Type, options?: SchemaOptions): TConstructorParameters<Type>;
export type TExcludeFromTemplateLiteral<L extends TTemplateLiteral, R extends TSchema> = (TExclude<TTemplateLiteralToUnion<L>, R>);
export declare function ExcludeFromTemplateLiteral<L extends TTemplateLiteral, R extends TSchema>(L: L, R: R): TExcludeFromTemplateLiteral<L, R>;
type TExcludeRest<L extends TSchema[], R extends TSchema> = AssertRest<UnionToTuple<{
    [K in keyof L]: Static<AssertType<L[K]>> extends Static<R> ? never : L[K];
}[number]>> extends infer R extends TSchema[] ? TUnionEvaluated<R> : never;
export type TExclude<L extends TSchema, R extends TSchema> = (L extends TUnion<infer S> ? TExcludeRest<S, R> : L extends R ? TNever : L);
declare function Exclude$1<L extends TMappedResult, R extends TSchema>(unionType: L, excludedMembers: R, options?: SchemaOptions): TExcludeFromMappedResult<L, R>;
declare function Exclude$1<L extends TTemplateLiteral, R extends TSchema>(unionType: L, excludedMembers: R, options?: SchemaOptions): TExcludeFromTemplateLiteral<L, R>;
declare function Exclude$1<L extends TSchema, R extends TSchema>(unionType: L, excludedMembers: R, options?: SchemaOptions): TExclude<L, R>;
type TFromProperties$14<K extends TProperties, T extends TSchema> = ({
    [K2 in keyof K]: TExclude<K[K2], T>;
});
type TFromMappedResult$9<R extends TMappedResult, T extends TSchema> = (TFromProperties$14<R["properties"], T>);
export type TExcludeFromMappedResult<R extends TMappedResult, T extends TSchema, P extends TProperties = TFromMappedResult$9<R, T>> = (TMappedResult<P>);
export declare function ExcludeFromMappedResult<R extends TMappedResult, T extends TSchema, P extends TProperties = TFromMappedResult$9<R, T>>(R: R, T: T): TMappedResult<P>;
export declare class ExtendsResolverError extends TypeBoxError {
}
export declare enum ExtendsResult {
    Union = 0,
    True = 1,
    False = 2
}
export declare function ExtendsCheck(left: TSchema, right: TSchema): ExtendsResult;
type TFromProperties$15<P extends TProperties, Right extends TSchema, False extends TSchema, True extends TSchema> = ({
    [K2 in keyof P]: TExtends<P[K2], Right, False, True>;
});
type TFromMappedResult$10<Left extends TMappedResult, Right extends TSchema, True extends TSchema, False extends TSchema> = (TFromProperties$15<Left["properties"], Right, True, False>);
export type TExtendsFromMappedResult<Left extends TMappedResult, Right extends TSchema, True extends TSchema, False extends TSchema, P extends TProperties = TFromMappedResult$10<Left, Right, True, False>> = (TMappedResult<P>);
export declare function ExtendsFromMappedResult<Left extends TMappedResult, Right extends TSchema, True extends TSchema, False extends TSchema, P extends TProperties = TFromMappedResult$10<Left, Right, True, False>>(Left: Left, Right: Right, True: True, False: False, options?: SchemaOptions): TMappedResult<P>;
type TExtendsResolve<L extends TSchema, R extends TSchema, T extends TSchema, U extends TSchema> = ((Static<L> extends Static<R> ? T : U) extends infer O extends TSchema ? UnionToTuple<O> extends [
    infer X extends TSchema,
    infer Y extends TSchema
] ? TUnion<[
    X,
    Y
]> : O : never);
export type TExtends<L extends TSchema, R extends TSchema, T extends TSchema, F extends TSchema> = TExtendsResolve<L, R, T, F>;
export declare function Extends<L extends TMappedResult, R extends TSchema, T extends TSchema, F extends TSchema>(L: L, R: R, T: T, F: F, options?: SchemaOptions): TExtendsFromMappedResult<L, R, T, F>;
export declare function Extends<L extends TMappedKey, R extends TSchema, T extends TSchema, F extends TSchema>(L: L, R: R, T: T, F: F, options?: SchemaOptions): TExtendsFromMappedKey<L, R, T, F>;
export declare function Extends<L extends TSchema, R extends TSchema, T extends TSchema, F extends TSchema>(L: L, R: R, T: T, F: F, options?: SchemaOptions): TExtends<L, R, T, F>;
type TFromPropertyKey$2<K extends PropertyKey, U extends TSchema, L extends TSchema, R extends TSchema> = {
    [_ in K]: TExtends<TLiteral<Assert<K, TLiteralValue>>, U, L, R>;
};
type TFromPropertyKeys$2<K extends PropertyKey[], U extends TSchema, L extends TSchema, R extends TSchema, Acc extends TProperties = {}> = (K extends [
    infer LK extends PropertyKey,
    ...infer RK extends PropertyKey[]
] ? TFromPropertyKeys$2<RK, U, L, R, Acc & TFromPropertyKey$2<LK, U, L, R>> : Acc);
type TFromMappedKey$3<K extends TMappedKey, U extends TSchema, L extends TSchema, R extends TSchema> = (TFromPropertyKeys$2<K["keys"], U, L, R>);
export type TExtendsFromMappedKey<T extends TMappedKey, U extends TSchema, L extends TSchema, R extends TSchema, P extends TProperties = TFromMappedKey$3<T, U, L, R>> = (TMappedResult<P>);
export declare function ExtendsFromMappedKey<T extends TMappedKey, U extends TSchema, L extends TSchema, R extends TSchema, P extends TProperties = TFromMappedKey$3<T, U, L, R>>(T: T, U: U, L: L, R: R, options?: SchemaOptions): TMappedResult<P>;
export declare function ExtendsUndefinedCheck(schema: TSchema): boolean;
export type TExtractFromTemplateLiteral<L extends TTemplateLiteral, R extends TSchema> = (TExtract<TTemplateLiteralToUnion<L>, R>);
export declare function ExtractFromTemplateLiteral<L extends TTemplateLiteral, R extends TSchema>(L: L, R: R): TExtractFromTemplateLiteral<L, R>;
type TExtractRest<L extends TSchema[], R extends TSchema> = AssertRest<UnionToTuple<{
    [K in keyof L]: Static<AssertType<L[K]>> extends Static<R> ? L[K] : never;
}[number]>> extends infer R extends TSchema[] ? TUnionEvaluated<R> : never;
export type TExtract<L extends TSchema, U extends TSchema> = (L extends TUnion<infer S> ? TExtractRest<S, U> : L extends U ? L : TNever);
declare function Extract$1<L extends TMappedResult, R extends TSchema>(type: L, union: R, options?: SchemaOptions): TExtractFromMappedResult<L, R>;
declare function Extract$1<L extends TTemplateLiteral, R extends TSchema>(type: L, union: R, options?: SchemaOptions): TExtractFromTemplateLiteral<L, R>;
declare function Extract$1<L extends TSchema, R extends TSchema>(type: L, union: R, options?: SchemaOptions): TExtract<L, R>;
type TFromProperties$16<P extends TProperties, T extends TSchema> = ({
    [K2 in keyof P]: TExtract<P[K2], T>;
});
type TFromMappedResult$11<R extends TMappedResult, T extends TSchema> = (TFromProperties$16<R["properties"], T>);
export type TExtractFromMappedResult<R extends TMappedResult, T extends TSchema, P extends TProperties = TFromMappedResult$11<R, T>> = (TMappedResult<P>);
export declare function ExtractFromMappedResult<R extends TMappedResult, T extends TSchema, P extends TProperties = TFromMappedResult$11<R, T>>(R: R, T: T): TMappedResult<P>;
export type TInstanceType<Type extends TSchema, Result extends TSchema = Type extends TConstructor<infer _Parameters extends TSchema[], infer InstanceType extends TSchema> ? InstanceType : TNever> = Result;
declare function InstanceType$1<Type extends TSchema>(schema: Type, options?: SchemaOptions): TInstanceType<Type>;
type TFromConstructor$1<Args extends TSchema[], Parameters extends TSchema[], InstanceType extends TSchema, Result extends TConstructor = TConstructor<TFromTypes$1<Args, Parameters>, TFromType$1<Args, InstanceType>>> = Result;
type TFromFunction$1<Args extends TSchema[], Parameters extends TSchema[], ReturnType extends TSchema, Result extends TFunction = TFunction<TFromTypes$1<Args, Parameters>, TFromType$1<Args, ReturnType>>> = Result;
type TFromIntersect$5<Args extends TSchema[], Types extends TSchema[], Result extends TIntersect = TIntersect<TFromTypes$1<Args, Types>>> = Result;
type TFromUnion$6<Args extends TSchema[], Types extends TSchema[], Result extends TUnion = TUnion<TFromTypes$1<Args, Types>>> = Result;
type TFromTuple$3<Args extends TSchema[], Types extends TSchema[], Result extends TTuple = TTuple<TFromTypes$1<Args, Types>>> = Result;
type TFromArray$4<Args extends TSchema[], Type extends TSchema, Result extends TArray = TArray<TFromType$1<Args, Type>>> = Result;
type TFromAsyncIterator$1<Args extends TSchema[], Type extends TSchema, Result extends TAsyncIterator = TAsyncIterator<TFromType$1<Args, Type>>> = Result;
type TFromIterator$1<Args extends TSchema[], Type extends TSchema, Result extends TIterator = TIterator<TFromType$1<Args, Type>>> = Result;
type TFromPromise<Args extends TSchema[], Type extends TSchema, Result extends TPromise = TPromise<TFromType$1<Args, Type>>> = Result;
type TFromObject$5<Args extends TSchema[], Properties extends TProperties, MappedProperties extends TProperties = TFromProperties$17<Args, Properties>, Result extends TSchema = TObject<MappedProperties>> = Result;
type TFromRecord$1<Args extends TSchema[], Key extends TSchema, Value extends TSchema, MappedKey extends TSchema = TFromType$1<Args, Key>, MappedValue extends TSchema = TFromType$1<Args, Value>, Result extends TSchema = TRecordOrObject<MappedKey, MappedValue>> = Result;
type TFromArgument<Args extends TSchema[], Index extends number, Result extends TSchema = Index extends keyof Args[Index] ? Args[Index] : TUnknown> = Result;
type TFromProperty$1<Args extends TSchema[], Type extends TSchema, IsReadonly extends boolean = Type extends TReadonly<Type> ? true : false, IsOptional extends boolean = Type extends TOptional<Type> ? true : false, Mapped extends TSchema = TFromType$1<Args, Type>, Result extends TSchema = ([
    IsReadonly,
    IsOptional
] extends [
    true,
    true
] ? TReadonlyOptional<Mapped> : [
    IsReadonly,
    IsOptional
] extends [
    true,
    false
] ? TReadonly<Mapped> : [
    IsReadonly,
    IsOptional
] extends [
    false,
    true
] ? TOptional<Mapped> : Mapped)> = Result;
type TFromProperties$17<Args extends TSchema[], Properties extends TProperties, Result extends TProperties = {
    [Key in keyof Properties]: TFromProperty$1<Args, Properties[Key]>;
}> = Result;
type TFromTypes$1<Args extends TSchema[], Types extends TSchema[], Result extends TSchema[] = [
]> = (Types extends [
    infer Left extends TSchema,
    ...infer Right extends TSchema[]
] ? TFromTypes$1<Args, Right, [
    ...Result,
    TFromType$1<Args, Left>
]> : Result);
export declare function FromTypes<Args extends TSchema[], Types extends TSchema[]>(args: [
    ...Args
], types: [
    ...Types
]): TFromTypes$1<Args, Types>;
type TFromType$1<Args extends TSchema[], Type extends TSchema> = (Type extends TConstructor<infer Parameters extends TSchema[], infer InstanceType extends TSchema> ? TFromConstructor$1<Args, Parameters, InstanceType> : Type extends TFunction<infer Parameters extends TSchema[], infer ReturnType extends TSchema> ? TFromFunction$1<Args, Parameters, ReturnType> : Type extends TIntersect<infer Types extends TSchema[]> ? TFromIntersect$5<Args, Types> : Type extends TUnion<infer Types extends TSchema[]> ? TFromUnion$6<Args, Types> : Type extends TTuple<infer Types extends TSchema[]> ? TFromTuple$3<Args, Types> : Type extends TArray<infer Type extends TSchema> ? TFromArray$4<Args, Type> : Type extends TAsyncIterator<infer Type extends TSchema> ? TFromAsyncIterator$1<Args, Type> : Type extends TIterator<infer Type extends TSchema> ? TFromIterator$1<Args, Type> : Type extends TPromise<infer Type extends TSchema> ? TFromPromise<Args, Type> : Type extends TObject<infer Properties extends TProperties> ? TFromObject$5<Args, Properties> : Type extends TRecord<infer Key extends TSchema, infer Value extends TSchema> ? TFromRecord$1<Args, Key, Value> : Type extends TArgument<infer Index extends number> ? TFromArgument<Args, Index> : Type);
export type TInstantiate<Type extends TSchema, Args extends TSchema[], Result extends TSchema = TFromType$1<Args, Type>> = Result;
export declare function Instantiate<Type extends TSchema, Args extends TSchema[]>(type: Type, args: [
    ...Args
]): TInstantiate<Type, Args>;
type TMappedIntrinsicPropertyKey<K extends PropertyKey, M extends IntrinsicMode> = {
    [_ in K]: TIntrinsic<TLiteral<Assert<K, TLiteralValue>>, M>;
};
type TMappedIntrinsicPropertyKeys<K extends PropertyKey[], M extends IntrinsicMode, Acc extends TProperties = {}> = (K extends [
    infer L extends PropertyKey,
    ...infer R extends PropertyKey[]
] ? TMappedIntrinsicPropertyKeys<R, M, Acc & TMappedIntrinsicPropertyKey<L, M>> : Acc);
type TMappedIntrinsicProperties<K extends TMappedKey, M extends IntrinsicMode> = (TMappedIntrinsicPropertyKeys<K["keys"], M>);
export type TIntrinsicFromMappedKey<K extends TMappedKey, M extends IntrinsicMode, P extends TProperties = TMappedIntrinsicProperties<K, M>> = (TMappedResult<P>);
export declare function IntrinsicFromMappedKey<K extends TMappedKey, M extends IntrinsicMode, P extends TProperties = TMappedIntrinsicProperties<K, M>>(T: K, M: M, options: SchemaOptions): TMappedResult<P>;
export type IntrinsicMode = "Uppercase" | "Lowercase" | "Capitalize" | "Uncapitalize";
type TFromTemplateLiteral$1<T extends TTemplateLiteralKind[], M extends IntrinsicMode> = M extends IntrinsicMode ? T extends [
    infer L extends TTemplateLiteralKind,
    ...infer R extends TTemplateLiteralKind[]
] ? [
    TIntrinsic<L, M>,
    ...TFromTemplateLiteral$1<R, M>
] : T : T;
type TFromLiteralValue<T, M extends IntrinsicMode> = (T extends string ? M extends "Uncapitalize" ? Uncapitalize<T> : M extends "Capitalize" ? Capitalize<T> : M extends "Uppercase" ? Uppercase<T> : M extends "Lowercase" ? Lowercase<T> : string : T);
type TFromRest$6<T extends TSchema[], M extends IntrinsicMode, Acc extends TSchema[] = [
]> = T extends [
    infer L extends TSchema,
    ...infer R extends TSchema[]
] ? TFromRest$6<R, M, [
    ...Acc,
    TIntrinsic<L, M>
]> : Acc;
export type TIntrinsic<T extends TSchema, M extends IntrinsicMode> = T extends TMappedKey ? TIntrinsicFromMappedKey<T, M> : T extends TTemplateLiteral<infer S> ? TTemplateLiteral<TFromTemplateLiteral$1<S, M>> : T extends TUnion<infer S> ? TUnion<TFromRest$6<S, M>> : T extends TLiteral<infer S> ? TLiteral<TFromLiteralValue<S, M>> : T;
export declare function Intrinsic<T extends TMappedKey, M extends IntrinsicMode>(schema: T, mode: M, options?: SchemaOptions): TIntrinsicFromMappedKey<T, M>;
export declare function Intrinsic<T extends TSchema, M extends IntrinsicMode>(schema: T, mode: M, options?: SchemaOptions): TIntrinsic<T, M>;
export type TCapitalize<T extends TSchema> = TIntrinsic<T, "Capitalize">;
declare function Capitalize$1<T extends TSchema>(T: T, options?: SchemaOptions): TCapitalize<T>;
export type TLowercase<T extends TSchema> = TIntrinsic<T, "Lowercase">;
declare function Lowercase$1<T extends TSchema>(T: T, options?: SchemaOptions): TLowercase<T>;
export type TUncapitalize<T extends TSchema> = TIntrinsic<T, "Uncapitalize">;
declare function Uncapitalize$1<T extends TSchema>(T: T, options?: SchemaOptions): TUncapitalize<T>;
export type TUppercase<T extends TSchema> = TIntrinsic<T, "Uppercase">;
declare function Uppercase$1<T extends TSchema>(T: T, options?: SchemaOptions): TUppercase<T>;
export type TParameters<Type extends TSchema> = (Type extends TFunction<infer Parameters extends TSchema[], infer _ReturnType extends TSchema> ? TTuple<Parameters> : TNever);
declare function Parameters$1<Type extends TSchema>(schema: Type, options?: SchemaOptions): TParameters<Type>;
type TRestResolve<T extends TSchema> = T extends TIntersect<infer S extends TSchema[]> ? S : T extends TUnion<infer S extends TSchema[]> ? S : T extends TTuple<infer S extends TSchema[]> ? S : [
];
export type TRest<T extends TSchema> = TRestResolve<T>;
export declare function Rest<T extends TSchema>(T: T): TRest<T>;
export type TReturnType<Type extends TSchema, Result extends TSchema = Type extends TFunction<infer _Parameters extends TSchema[], infer ReturnType extends TSchema> ? ReturnType : TNever> = Result;
declare function ReturnType$1<Type extends TSchema>(schema: Type, options?: SchemaOptions): TReturnType<Type>;
export declare class JsonTypeBuilder {
    ReadonlyOptional<Type extends TSchema>(type: Type): TReadonlyOptional<Type>;
    Readonly<Type extends TMappedResult, Flag extends boolean>(type: Type, enable: Flag): TReadonlyFromMappedResult<Type, Flag>;
    Readonly<Type extends TSchema, Flag extends boolean>(type: Type, enable: Flag): TReadonlyWithFlag<Type, Flag>;
    Readonly<Type extends TMappedResult>(type: Type): TReadonlyFromMappedResult<Type, true>;
    Readonly<Type extends TSchema>(type: Type): TReadonlyWithFlag<Type, true>;
    Optional<Type extends TMappedResult, Flag extends boolean>(type: Type, enable: Flag): TOptionalFromMappedResult<Type, Flag>;
    Optional<Type extends TSchema, Flag extends boolean>(type: Type, enable: Flag): TOptionalWithFlag<Type, Flag>;
    Optional<Type extends TMappedResult>(type: Type): TOptionalFromMappedResult<Type, true>;
    Optional<Type extends TSchema>(type: Type): TOptionalWithFlag<Type, true>;
    Any(options?: SchemaOptions): TAny;
    Array<Type extends TSchema>(items: Type, options?: ArrayOptions): TArray<Type>;
    Boolean(options?: SchemaOptions): TBoolean;
    Capitalize<T extends TSchema>(schema: T, options?: SchemaOptions): TCapitalize<T>;
    Composite<T extends TSchema[]>(schemas: [
        ...T
    ], options?: ObjectOptions): TComposite<T>;
    Const<T>(value: T, options?: SchemaOptions): TConst<T>;
    Enum<V extends TEnumValue, T extends Record<TEnumKey, V>>(item: T, options?: SchemaOptions): TEnum<T>;
    Exclude<L extends TMappedResult, R extends TSchema>(unionType: L, excludedMembers: R, options?: SchemaOptions): TExcludeFromMappedResult<L, R>;
    Exclude<L extends TTemplateLiteral, R extends TSchema>(unionType: L, excludedMembers: R, options?: SchemaOptions): TExcludeFromTemplateLiteral<L, R>;
    Exclude<L extends TSchema, R extends TSchema>(unionType: L, excludedMembers: R, options?: SchemaOptions): TExclude<L, R>;
    Extends<L extends TMappedResult, R extends TSchema, T extends TSchema, F extends TSchema>(L: L, R: R, T: T, F: F, options?: SchemaOptions): TExtendsFromMappedResult<L, R, T, F>;
    Extends<L extends TMappedKey, R extends TSchema, T extends TSchema, F extends TSchema>(L: L, R: R, T: T, F: F, options?: SchemaOptions): TExtendsFromMappedKey<L, R, T, F>;
    Extends<L extends TSchema, R extends TSchema, T extends TSchema, F extends TSchema>(L: L, R: R, T: T, F: F, options?: SchemaOptions): TExtends<L, R, T, F>;
    Extract<L extends TMappedResult, R extends TSchema>(type: L, union: R, options?: SchemaOptions): TExtractFromMappedResult<L, R>;
    Extract<L extends TTemplateLiteral, R extends TSchema>(type: L, union: R, options?: SchemaOptions): TExtractFromTemplateLiteral<L, R>;
    Extract<L extends TSchema, R extends TSchema>(type: L, union: R, options?: SchemaOptions): TExtract<L, R>;
    Index<Type extends TRef, Key extends TSchema>(type: Type, key: Key, options?: SchemaOptions): TIndexFromComputed<Type, Key>;
    Index<Type extends TSchema, Key extends TRef>(type: Type, key: Key, options?: SchemaOptions): TIndexFromComputed<Type, Key>;
    Index<Type extends TRef, Key extends TRef>(type: Type, key: Key, options?: SchemaOptions): TIndexFromComputed<Type, Key>;
    Index<Type extends TSchema, MappedResult extends TMappedResult>(type: Type, mappedResult: MappedResult, options?: SchemaOptions): TIndexFromMappedResult<Type, MappedResult>;
    Index<Type extends TSchema, MappedKey extends TMappedKey>(type: Type, mappedKey: MappedKey, options?: SchemaOptions): TIndexFromMappedKey<Type, MappedKey>;
    Index<Type extends TSchema, Key extends TSchema, PropertyKeys extends PropertyKey[] = TIndexPropertyKeys<Key>>(T: Type, K: Key, options?: SchemaOptions): TIndex<Type, PropertyKeys>;
    Index<Type extends TSchema, PropertyKeys extends PropertyKey[]>(type: Type, propertyKeys: readonly [
        ...PropertyKeys
    ], options?: SchemaOptions): TIndex<Type, PropertyKeys>;
    Integer(options?: IntegerOptions): TInteger;
    Intersect<Types extends TSchema[]>(types: [
        ...Types
    ], options?: IntersectOptions): Intersect<Types>;
    KeyOf<Type extends TSchema>(type: Type, options?: SchemaOptions): TKeyOf<Type>;
    Literal<LiteralValue extends TLiteralValue>(literalValue: LiteralValue, options?: SchemaOptions): TLiteral<LiteralValue>;
    Lowercase<Type extends TSchema>(type: Type, options?: SchemaOptions): TLowercase<Type>;
    Mapped<K extends TSchema, I extends PropertyKey[] = TIndexPropertyKeys<K>, F extends TMappedFunction<I> = TMappedFunction<I>, R extends TMapped<I, F> = TMapped<I, F>>(key: K, map: F, options?: ObjectOptions): R;
    Mapped<K extends PropertyKey[], F extends TMappedFunction<K> = TMappedFunction<K>, R extends TMapped<K, F> = TMapped<K, F>>(key: [
        ...K
    ], map: F, options?: ObjectOptions): R;
    Module<Properties extends TProperties>(properties: Properties): TModule<Properties>;
    Never(options?: SchemaOptions): TNever;
    Not<T extends TSchema>(type: T, options?: SchemaOptions): TNot<T>;
    Null(options?: SchemaOptions): TNull;
    Number(options?: NumberOptions): TNumber;
    Object<T extends TProperties>(properties: T, options?: ObjectOptions): TObject<T>;
    Omit<Type extends TSchema, Key extends PropertyKey[]>(type: Type, key: readonly [
        ...Key
    ], options?: SchemaOptions): TOmit<Type, Key>;
    Omit<Type extends TSchema, Key extends TSchema>(type: Type, key: Key, options?: SchemaOptions): TOmit<Type, Key>;
    Partial<MappedResult extends TMappedResult>(type: MappedResult, options?: SchemaOptions): TPartialFromMappedResult<MappedResult>;
    Partial<Type extends TSchema>(type: Type, options?: SchemaOptions): TPartial<Type>;
    Pick<Type extends TSchema, Key extends PropertyKey[]>(type: Type, key: readonly [
        ...Key
    ], options?: SchemaOptions): TPick<Type, Key>;
    Pick<Type extends TSchema, Key extends TSchema>(type: Type, key: Key, options?: SchemaOptions): TPick<Type, Key>;
    Record<Key extends TSchema, Value extends TSchema>(key: Key, value: Value, options?: ObjectOptions): TRecordOrObject<Key, Value>;
    Recursive<T extends TSchema>(callback: (thisType: TThis) => T, options?: SchemaOptions): TRecursive<T>;
    Ref<Ref extends string>($ref: Ref, options?: SchemaOptions): TRef<Ref>;
    Ref<Type extends TSchema>(type: Type, options?: SchemaOptions): TRefUnsafe<Type>;
    Required<MappedResult extends TMappedResult>(type: MappedResult, options?: SchemaOptions): TRequiredFromMappedResult<MappedResult>;
    Required<Type extends TSchema>(type: Type, options?: SchemaOptions): TRequired<Type>;
    Rest<Type extends TSchema>(type: Type): TRest<Type>;
    String(options?: StringOptions): TString;
    TemplateLiteral<Syntax extends string>(syntax: Syntax, options?: SchemaOptions): TTemplateLiteralSyntax<Syntax>;
    TemplateLiteral<Kinds extends TTemplateLiteralKind[]>(kinds: [
        ...Kinds
    ], options?: SchemaOptions): TTemplateLiteral<Kinds>;
    Transform<Type extends TSchema>(type: Type): TransformDecodeBuilder<Type>;
    Tuple<Types extends TSchema[]>(types: [
        ...Types
    ], options?: SchemaOptions): TTuple<Types>;
    Uncapitalize<Type extends TSchema>(type: Type, options?: SchemaOptions): TUncapitalize<Type>;
    Union<Types extends TSchema[]>(types: [
        ...Types
    ], options?: SchemaOptions): Union<Types>;
    Unknown(options?: SchemaOptions): TUnknown;
    Unsafe<T>(options?: UnsafeOptions): TUnsafe<T>;
    Uppercase<T extends TSchema>(schema: T, options?: SchemaOptions): TUppercase<T>;
}
export declare class JavaScriptTypeBuilder extends JsonTypeBuilder {
    Argument<Index extends number>(index: Index): TArgument<Index>;
    AsyncIterator<Type extends TSchema>(items: Type, options?: SchemaOptions): TAsyncIterator<Type>;
    Awaited<Type extends TSchema>(schema: Type, options?: SchemaOptions): TAwaited<Type>;
    BigInt(options?: BigIntOptions): TBigInt;
    ConstructorParameters<Type extends TSchema>(schema: Type, options?: SchemaOptions): TConstructorParameters<Type>;
    Constructor<Parameters extends TSchema[], InstanceType extends TSchema>(parameters: [
        ...Parameters
    ], instanceType: InstanceType, options?: SchemaOptions): TConstructor<Parameters, InstanceType>;
    Date(options?: DateOptions): TDate;
    Function<Parameters extends TSchema[], ReturnType extends TSchema>(parameters: [
        ...Parameters
    ], returnType: ReturnType, options?: SchemaOptions): TFunction<Parameters, ReturnType>;
    InstanceType<Type extends TSchema>(schema: Type, options?: SchemaOptions): TInstanceType<Type>;
    Instantiate<Type extends TSchema, Parameters extends TSchema[]>(schema: Type, parameters: [
        ...Parameters
    ]): TInstantiate<Type, Parameters>;
    Iterator<Type extends TSchema>(items: Type, options?: SchemaOptions): TIterator<Type>;
    Parameters<Type extends TSchema>(schema: Type, options?: SchemaOptions): TParameters<Type>;
    Promise<Type extends TSchema>(item: Type, options?: SchemaOptions): TPromise<Type>;
    RegExp(pattern: string, options?: RegExpOptions): TRegExp;
    RegExp(regex: RegExp, options?: RegExpOptions): TRegExp;
    ReturnType<Type extends TSchema>(type: Type, options?: SchemaOptions): TReturnType<Type>;
    Symbol(options?: SchemaOptions): TSymbol;
    Undefined(options?: SchemaOptions): TUndefined;
    Uint8Array(options?: Uint8ArrayOptions): TUint8Array;
    Void(options?: SchemaOptions): TVoid;
}
export declare const Type: InstanceType<typeof JavaScriptTypeBuilder>;
declare namespace ValueGuard {
    export { HasPropertyKey, IsArray$2 as IsArray, IsAsyncIterator$2 as IsAsyncIterator, IsBigInt$2 as IsBigInt, IsBoolean$2 as IsBoolean, IsDate$2 as IsDate, IsFunction$2 as IsFunction, IsIterator$2 as IsIterator, IsNull$2 as IsNull, IsNumber$2 as IsNumber, IsObject$2 as IsObject, IsRegExp$2 as IsRegExp, IsString$2 as IsString, IsSymbol$2 as IsSymbol, IsUint8Array$2 as IsUint8Array, IsUndefined$2 as IsUndefined };
}
declare namespace TypeRegistry {
    export { Clear$1 as Clear, Delete$1 as Delete, Entries$1 as Entries, Get$1 as Get, Has$1 as Has, Set$1 as Set, TypeRegistryValidationFunction };
}
declare namespace KindGuard {
    export { IsAny, IsArgument, IsArray, IsAsyncIterator, IsBigInt, IsBoolean, IsComputed, IsConstructor, IsDate, IsFunction, IsImport, IsInteger, IsIntersect, IsIterator, IsKind, IsKindOf, IsLiteral, IsLiteralBoolean, IsLiteralNumber, IsLiteralString, IsLiteralValue, IsMappedKey, IsMappedResult, IsNever, IsNot, IsNull, IsNumber, IsObject, IsOptional, IsPromise, IsProperties, IsReadonly, IsRecord, IsRecursive, IsRef, IsRegExp, IsSchema, IsString, IsSymbol, IsTemplateLiteral, IsThis, IsTransform, IsTuple, IsUint8Array, IsUndefined, IsUnion, IsUnknown, IsUnsafe, IsVoid };
}
declare namespace TypeGuard {
    export { IsAny$1 as IsAny, IsArgument$1 as IsArgument, IsArray$1 as IsArray, IsAsyncIterator$1 as IsAsyncIterator, IsBigInt$1 as IsBigInt, IsBoolean$1 as IsBoolean, IsComputed$1 as IsComputed, IsConstructor$1 as IsConstructor, IsDate$1 as IsDate, IsFunction$1 as IsFunction, IsImport$1 as IsImport, IsInteger$1 as IsInteger, IsIntersect$1 as IsIntersect, IsIterator$1 as IsIterator, IsKind$1 as IsKind, IsKindOf$1 as IsKindOf, IsLiteral$1 as IsLiteral, IsLiteralBoolean$1 as IsLiteralBoolean, IsLiteralNumber$1 as IsLiteralNumber, IsLiteralString$1 as IsLiteralString, IsLiteralValue$1 as IsLiteralValue, IsMappedKey$1 as IsMappedKey, IsMappedResult$1 as IsMappedResult, IsNever$1 as IsNever, IsNot$1 as IsNot, IsNull$1 as IsNull, IsNumber$1 as IsNumber, IsObject$1 as IsObject, IsOptional$1 as IsOptional, IsPromise$1 as IsPromise, IsProperties$1 as IsProperties, IsReadonly$1 as IsReadonly, IsRecord$1 as IsRecord, IsRecursive$1 as IsRecursive, IsRef$1 as IsRef, IsRegExp$1 as IsRegExp, IsSchema$1 as IsSchema, IsString$1 as IsString, IsSymbol$1 as IsSymbol, IsTemplateLiteral$1 as IsTemplateLiteral, IsThis$1 as IsThis, IsTransform$1 as IsTransform, IsTuple$1 as IsTuple, IsUint8Array$1 as IsUint8Array, IsUndefined$1 as IsUndefined, IsUnion$1 as IsUnion, IsUnionLiteral, IsUnknown$1 as IsUnknown, IsUnsafe$1 as IsUnsafe, IsVoid$1 as IsVoid, TypeGuardUnknownTypeError };
}
declare namespace FormatRegistry {
    export { Clear, Delete, Entries, FormatRegistryValidationFunction, Get, Has, Set$1 as Set };
}
export { Array$1 as Array, AsyncIterator$1 as AsyncIterator, Awaited$1 as Awaited, BigInt$1 as BigInt, Boolean$1 as Boolean, Capitalize$1 as Capitalize, ConstructorParameters$1 as ConstructorParameters, Date$1 as Date, Exclude$1 as Exclude, Extract$1 as Extract, FormatRegistry, Function$1 as Function, InstanceType$1 as InstanceType, Iterator$1 as Iterator, KindGuard, Lowercase$1 as Lowercase, Number$1 as Number, Object$1 as Object, Omit$1 as Omit, Parameters$1 as Parameters, Partial$1 as Partial, Pick$1 as Pick, Promise$1 as Promise, Readonly$1 as Readonly, Record$1 as Record, RegExp$1 as RegExp, Required$1 as Required, ReturnType$1 as ReturnType, String$1 as String, Symbol$1 as Symbol, TFromType$1 as TFromType, TFromTypes$1 as TFromTypes, TypeGuard, TypeRegistry, Uint8Array$1 as Uint8Array, Uncapitalize$1 as Uncapitalize, Uppercase$1 as Uppercase, ValueGuard, };
export {};
