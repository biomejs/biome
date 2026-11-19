declare const matcher: unique symbol;
type matcher = typeof matcher;
declare const unset: unique symbol;
type unset = typeof unset;
declare const isVariadic: unique symbol;
type isVariadic = typeof isVariadic;
declare const anonymousSelectKey = "@ts-pattern/anonymous-select-key";
type anonymousSelectKey = typeof anonymousSelectKey;
declare const override: unique symbol;
type override = typeof override;
type ValueOf<a> = a extends readonly any[] ? a[number] : a[keyof a];
type Values<a extends object> = UnionToTuple<ValueOf<a>>;
type LeastUpperBound<a, b> = b extends a ? b : a extends b ? a : never;
type Contains<a, b> = a extends any ? 'exclude' extends {
    [k in keyof a]-?: Equal<a[k], b> extends true ? 'exclude' : 'include';
}[keyof a] ? true : false : never;
type UnionToIntersection<union> = (union extends any ? (k: union) => void : never) extends (k: infer intersection) => void ? intersection : never;
type IsUnion<a> = [
    a
] extends [
    UnionToIntersection<a>
] ? false : true;
type UnionToTuple<union, output extends any[] = [
]> = UnionToIntersection<union extends any ? (t: union) => union : never> extends (_: any) => infer elem ? UnionToTuple<Exclude<union, elem>, [
    elem,
    ...output
]> : output;
type Flatten<xs extends readonly any[], output extends any[] = [
]> = xs extends readonly [
    infer head,
    ...infer tail
] ? Flatten<tail, [
    ...output,
    ...Extract<head, readonly any[]>
]> : output;
type Equal<a, b> = (<T>() => T extends a ? 1 : 2) extends <T>() => T extends b ? 1 : 2 ? true : false;
type IsAny<a> = 0 extends 1 & a ? true : false;
type IsNever<T> = [
    T
] extends [
    never
] ? true : false;
type Length<it extends readonly any[]> = it['length'];
type Iterator<n extends number, it extends any[] = [
]> = it['length'] extends n ? it : Iterator<n, [
    any,
    ...it
]>;
type Prev<it extends any[]> = it extends readonly [
    any,
    ...infer tail
] ? tail : [
];
type UpdateAt<tail extends readonly any[], n extends any[], value, inits extends readonly any[] = [
]> = Length<n> extends 0 ? tail extends readonly [
    any,
    ...infer tail
] ? [
    ...inits,
    value,
    ...tail
] : inits : tail extends readonly [
    infer head,
    ...infer tail
] ? UpdateAt<tail, Prev<n>, value, [
    ...inits,
    head
]> : inits;
type BuiltInObjects = Function | Date | RegExp | Generator | {
    readonly [Symbol.toStringTag]: string;
} | any[];
type IsPlainObject<o, excludeUnion = BuiltInObjects> = o extends object ? o extends string | excludeUnion ? false : true : false;
type Compute<a extends any> = a extends BuiltInObjects ? a : {
    [k in keyof a]: a[k];
};
type WithDefault$1<a, def> = [
    a
] extends [
    never
] ? def : a;
type IsLiteral<a> = [
    a
] extends [
    null | undefined
] ? true : [
    a
] extends [
    string
] ? string extends a ? false : true : [
    a
] extends [
    number
] ? number extends a ? false : true : [
    a
] extends [
    boolean
] ? boolean extends a ? false : true : [
    a
] extends [
    symbol
] ? symbol extends a ? false : true : [
    a
] extends [
    bigint
] ? bigint extends a ? false : true : false;
type Primitives = number | boolean | string | undefined | null | symbol | bigint;
type NonLiteralPrimitive = Exclude<Primitives, undefined | null>;
type Union<a, b> = [
    b
] extends [
    a
] ? a : [
    a
] extends [
    b
] ? b : a | b;
type GuardValue<fn> = fn extends (value: any) => value is infer b ? b : fn extends (value: infer a) => unknown ? a : never;
type All<bools extends boolean[]> = bools[number] extends true ? true : false;
type Extends<a, b> = [
    a
] extends [
    b
] ? true : false;
type Not<a extends boolean> = a extends true ? false : true;
type AllKeys<a> = a extends any ? keyof a : never;
type MergeUnion<a> = {
    [k in AllKeys<a>]: a extends any ? k extends keyof a ? a[k] : never : never;
} | never;
type IsTuple<a> = a extends readonly [
] | readonly [
    any,
    ...any
] | readonly [
    ...any,
    any
] ? true : false;
type IsStrictArray<a extends readonly any[]> = Not<IsTuple<a>>;
type IsReadonlyArray<a> = a extends readonly any[] ? a extends any[] ? false : true : false;
type MaybeAddReadonly<a, shouldAdd extends boolean> = shouldAdd extends true ? Readonly<a> : a;
type MapKey<T> = T extends Map<infer K, any> ? K : never;
type MapValue<T> = T extends Map<any, infer V> ? V : never;
type SetValue<T> = T extends Set<infer V> ? V : never;
type RecordKey<T> = T extends Record<infer K, any> ? K : never;
type RecordValue<T> = T extends Record<any, infer V> ? V : never;
type ReadonlyArrayValue<T> = T extends ReadonlyArray<infer V> ? V : never;
type ExtractPlainObject<T> = T extends any ? IsPlainObject<T> extends true ? T : never : never;
type GetKey<O, K> = O extends any ? K extends keyof O ? O[K] : never : never;
interface Fn {
    input: unknown;
    output: unknown;
}
type Call<fn extends Fn, input> = (fn & {
    input: input;
})['output'];
type IsOptionalKeysOf<obj, key extends keyof obj> = {} extends Pick<obj, key> ? true : false;
type SelectionsRecord = Record<string, [
    unknown,
    unknown[]
]>;
type None = {
    type: 'none';
};
type Some<key extends string> = {
    type: 'some';
    key: key;
};
type SelectionType = None | Some<string>;
type MapOptional<selections> = {
    [k in keyof selections]: selections[k] extends [
        infer v,
        infer subpath
    ] ? [
        v | undefined,
        subpath
    ] : never;
};
type MapList<selections> = {
    [k in keyof selections]: selections[k] extends [
        infer v,
        infer subpath
    ] ? [
        v[],
        subpath
    ] : never;
};
type ReduceFindSelectionUnion<i, ps extends readonly any[], output = never> = ps extends readonly [
    infer head,
    ...infer tail
] ? ReduceFindSelectionUnion<i, tail, output | FindSelectionUnion<i, head>> : output;
type FindSelectionUnionInArray<i, p, path extends any[] = [
], output = never> = i extends readonly (infer iItem)[] ? p extends readonly [
] ? output : p extends readonly [
    infer p1,
    ...infer pRest
] ? i extends readonly [
    infer i1,
    ...infer iRest
] ? FindSelectionUnionInArray<iRest, pRest, [
    ...path,
    p['length']
], output | FindSelectionUnion<i1, p1, [
    ...path,
    p['length']
]>> : FindSelectionUnionInArray<iItem[], pRest, [
    ...path,
    p['length']
], output | FindSelectionUnion<iItem, p1, [
    ...path,
    p['length']
]>> : p extends readonly [
    ...infer pInit,
    infer p1
] ? i extends readonly [
    ...infer iInit,
    infer i1
] ? FindSelectionUnionInArray<iInit, pInit, [
    ...path,
    p['length']
], output | FindSelectionUnion<i1, p1, [
    ...path,
    p['length']
]>> : FindSelectionUnionInArray<iItem[], pInit, [
    ...path,
    p['length']
], output | FindSelectionUnion<iItem, p1, [
    ...path,
    p['length']
]>> : p extends readonly [
    ...(readonly (infer pRest & AnyMatcher)[])
] ? output | FindSelectionUnion<i, pRest, [
    ...path,
    p['length']
]> : output | FindSelectionUnion<iItem, ValueOf<p>, [
    ...path,
    Extract<p, readonly any[]>['length']
]> : output;
type FindSelectionUnion<i, p, path extends any[] = [
]> = 0 extends 1 & i ? never : 0 extends 1 & p ? never : p extends Primitives ? never : p extends Matcher<any, infer pattern, infer matcherType, infer sel> ? {
    select: sel extends Some<infer k> ? {
        [kk in k]: [
            i,
            path
        ];
    } | FindSelectionUnion<i, pattern, path> : never;
    array: i extends readonly (infer iItem)[] ? MapList<FindSelectionUnion<iItem, pattern>> : never;
    record: [
        i,
        pattern
    ] extends [
        Record<infer k, infer v>,
        [
            infer pkey,
            infer pvalue
        ]
    ] ? MapList<FindSelectionUnion<k, pkey, path>> | MapList<FindSelectionUnion<v, pvalue, path>> : never;
    map: [
        i,
        pattern
    ] extends [
        Map<infer k, infer v>,
        [
            infer pkey,
            infer pvalue
        ]
    ] ? MapList<FindSelectionUnion<k, pkey, path>> | MapList<FindSelectionUnion<v, pvalue, path>> : never;
    set: i extends Set<infer v> ? MapList<FindSelectionUnion<v, pattern, path>> : never;
    optional: MapOptional<FindSelectionUnion<i, pattern>>;
    or: MapOptional<ReduceFindSelectionUnion<i, Extract<pattern, readonly any[]>>>;
    and: ReduceFindSelectionUnion<i, Extract<pattern, readonly any[]>>;
    not: never;
    default: sel extends Some<infer k> ? {
        [kk in k]: [
            i,
            path
        ];
    } : never;
    custom: never;
}[matcherType] : p extends readonly any[] ? FindSelectionUnionInArray<i, p> : p extends {} ? i extends {} ? {
    [k in keyof p]: k extends keyof i ? FindSelectionUnion<i[k], p[k], [
        ...path,
        k
    ]> : never;
}[keyof p] : never : never;
type SeveralAnonymousSelectError<a = 'You can only use a single anonymous selection (with `select()`) in your pattern. If you need to select multiple values, give them names with `select(<name>)` instead'> = {
    __error: never;
} & a;
type MixedNamedAndAnonymousSelectError<a = 'Mixing named selections (`select("name")`) and anonymous selections (`select()`) is forbiden. Please, only use named selections.'> = {
    __error: never;
} & a;
type SelectionToArgs<selections extends SelectionsRecord> = anonymousSelectKey extends keyof selections ? IsUnion<selections[anonymousSelectKey][1]> extends true ? SeveralAnonymousSelectError : keyof selections extends anonymousSelectKey ? selections[anonymousSelectKey][0] : MixedNamedAndAnonymousSelectError : {
    [k in keyof selections]: selections[k][0];
};
type Selections<i, p> = FindSelectionUnion<i, p> extends infer u ? [
    u
] extends [
    never
] ? i : SelectionToArgs<Extract<MergeUnion<u>, SelectionsRecord>> : i;
type FindSelected<i, p> = Equal<p, Pattern<i>> extends true ? i : Selections<i, p>;
type MatcherType = 'not' | 'optional' | 'or' | 'and' | 'array' | 'record' | 'map' | 'set' | 'select' | 'default' | 'custom';
type MatcherProtocol<input, narrowed, matcherType extends MatcherType, selections extends SelectionType, excluded> = {
    match: <I>(value: I | input) => MatchResult;
    getSelectionKeys?: () => string[];
    matcherType?: matcherType;
};
type MatchResult = {
    matched: boolean;
    selections?: Record<string, any>;
};
interface Matcher<input, narrowed, matcherType extends MatcherType = 'default', selections extends SelectionType = None, excluded = narrowed> {
    [matcher](): MatcherProtocol<input, narrowed, matcherType, selections, excluded>;
    [isVariadic]?: boolean;
}
type PatternMatcher<input> = Matcher<input, unknown, any, any>;
type MatchedValue<a, invpattern> = WithDefault$1<ExtractPreciseValue<a, invpattern>, a>;
type AnyMatcher = Matcher<any, any, any, any, any>;
type UnknownMatcher = PatternMatcher<unknown>;
type CustomP<input, pattern, narrowedOrFn> = Matcher<input, pattern, 'custom', None, narrowedOrFn>;
type ArrayP<input, p> = Matcher<input, p, 'array'>;
type RecordP<input, pkey, pvalue> = Matcher<input, [
    pkey,
    pvalue
], 'record'>;
type OptionalP<input, p> = Matcher<input, p, 'optional'>;
type MapP<input, pkey, pvalue> = Matcher<input, [
    pkey,
    pvalue
], 'map'>;
type SetP<input, p> = Matcher<input, p, 'set'>;
type AndP<input, ps> = Matcher<input, ps, 'and'>;
type OrP<input, ps> = Matcher<input, ps, 'or'>;
type NotP<input, p> = Matcher<input, p, 'not'>;
type GuardP<input, narrowed> = Matcher<input, narrowed>;
type GuardExcludeP<input, narrowed, excluded> = Matcher<input, narrowed, 'default', None, excluded>;
type SelectP<key extends string, input = unknown, p = Matcher<unknown, unknown>> = Matcher<input, p, 'select', Some<key>>;
type AnonymousSelectP = SelectP<anonymousSelectKey>;
interface Override<a> {
    [override]: a;
}
type UnknownProperties = {
    readonly [k: PropertyKey]: unknown;
};
type UnknownValuePattern = readonly [
] | readonly [
    unknown,
    ...unknown[]
] | readonly [
    ...unknown[],
    unknown
] | UnknownProperties | Primitives | UnknownMatcher;
type Pattern<a = unknown> = unknown extends a ? UnknownValuePattern : KnownPattern<a>;
type KnownPattern<a> = KnownPatternInternal<a>;
type KnownPatternInternal<a, objs = Exclude<a, Primitives | Map<any, any> | Set<any> | readonly any[]>, arrays = Extract<a, readonly any[]>, primitives = Extract<a, Primitives>> = primitives | PatternMatcher<a> | ([
    objs
] extends [
    never
] ? never : ObjectPattern<Readonly<MergeUnion<objs>>>) | ([
    arrays
] extends [
    never
] ? never : ArrayPattern<arrays>);
type ObjectPattern<a> = {
    readonly [k in keyof a]?: Pattern<a[k]>;
} | never;
type ArrayPattern<a> = a extends readonly (infer i)[] ? a extends readonly [
    any,
    ...any
] ? {
    readonly [index in keyof a]: Pattern<a[index]>;
} : readonly [
] | readonly [
    Pattern<i>,
    ...Pattern<i>[]
] | readonly [
    ...Pattern<i>[],
    Pattern<i>
] : never;
type UnknownPattern = Chainable<GuardP<unknown, unknown>, never>;
type StringPattern = StringChainable<GuardP<unknown, string>, never>;
type NumberPattern = NumberChainable<GuardP<unknown, number>, never>;
type BooleanPattern = Chainable<GuardP<unknown, boolean>, never>;
type BigIntPattern = BigIntChainable<GuardP<unknown, bigint>, never>;
type SymbolPattern = Chainable<GuardP<unknown, symbol>, never>;
type NullishPattern = Chainable<GuardP<unknown, null | undefined>, never>;
type NonNullablePattern = Chainable<GuardP<unknown, {}>, never>;
type MergeGuards<input, guard1, guard2> = [
    guard1,
    guard2
] extends [
    GuardExcludeP<any, infer narrowed1, infer excluded1>,
    GuardExcludeP<any, infer narrowed2, infer excluded2>
] ? GuardExcludeP<input, narrowed1 & narrowed2, excluded1 & excluded2> : never;
type Chainable<p, omitted extends string = never> = p & Omit<{
    optional<input>(): Chainable<OptionalP<input, p>, omitted | 'optional'>;
    and<input, const p2 extends Pattern<input>>(pattern: p2): Chainable<AndP<input, [
        p,
        p2
    ]>, omitted>;
    or<input, const p2 extends Pattern<input>>(pattern: p2): Chainable<OrP<input, [
        p,
        p2
    ]>, omitted>;
    select<input>(): Chainable<SelectP<anonymousSelectKey, input, p>, omitted | 'select' | 'or' | 'and'>;
    select<input, k extends string>(key: k): Chainable<SelectP<k, input, p>, omitted | 'select' | 'or' | 'and'>;
}, omitted>;
type StringChainable<p extends Matcher<any, any, any, any, any>, omitted extends string = never> = Chainable<p, omitted> & Omit<{
    startsWith<input, const start extends string>(start: start): StringChainable<MergeGuards<input, p, GuardP<unknown, `${start}${string}`>>, omitted | 'startsWith'>;
    endsWith<input, const end extends string>(end: end): StringChainable<MergeGuards<input, p, GuardP<unknown, `${string}${end}`>>, omitted | 'endsWith'>;
    minLength<input, const min extends number>(min: min): StringChainable<MergeGuards<input, p, GuardExcludeP<unknown, string, never>>, omitted | 'minLength'>;
    length<input, const len extends number>(len: len): StringChainable<MergeGuards<input, p, GuardExcludeP<unknown, string, never>>, omitted | 'length' | 'minLength' | 'maxLength'>;
    maxLength<input, const max extends number>(max: max): StringChainable<MergeGuards<input, p, GuardExcludeP<unknown, string, never>>, omitted | 'maxLength'>;
    includes<input, const substr extends string>(substr: substr): StringChainable<MergeGuards<input, p, GuardExcludeP<unknown, string, never>>, omitted>;
    regex<input, const expr extends string | RegExp>(expr: expr): StringChainable<MergeGuards<input, p, GuardExcludeP<unknown, string, never>>, omitted>;
}, omitted>;
type NumberChainable<p, omitted extends string = never> = Chainable<p, omitted> & Omit<{
    between<input, const min extends number, const max extends number>(min: min, max: max): NumberChainable<MergeGuards<input, p, GuardExcludeP<unknown, number, never>>, omitted>;
    lt<input, const max extends number>(max: max): NumberChainable<MergeGuards<input, p, GuardExcludeP<unknown, number, never>>, omitted>;
    gt<input, const min extends number>(min: min): NumberChainable<MergeGuards<input, p, GuardExcludeP<unknown, number, never>>, omitted>;
    lte<input, const max extends number>(max: max): NumberChainable<MergeGuards<input, p, GuardExcludeP<unknown, number, never>>, omitted>;
    gte<input, const min extends number>(min: min): NumberChainable<MergeGuards<input, p, GuardExcludeP<unknown, number, never>>, omitted>;
    int<input>(): NumberChainable<MergeGuards<input, p, GuardExcludeP<unknown, number, never>>, omitted | 'int'>;
    finite<input>(): NumberChainable<MergeGuards<input, p, GuardExcludeP<unknown, number, never>>, omitted | 'finite'>;
    positive<input>(): NumberChainable<MergeGuards<input, p, GuardExcludeP<unknown, number, never>>, omitted | 'positive' | 'negative'>;
    negative<input>(): NumberChainable<MergeGuards<input, p, GuardExcludeP<unknown, number, never>>, omitted | 'positive' | 'negative' | 'negative'>;
}, omitted>;
type BigIntChainable<p, omitted extends string = never> = Chainable<p, omitted> & Omit<{
    between<input, const min extends bigint, const max extends bigint>(min: min, max: max): BigIntChainable<MergeGuards<input, p, GuardExcludeP<unknown, bigint, never>>, omitted>;
    lt<input, const max extends bigint>(max: max): BigIntChainable<MergeGuards<input, p, GuardExcludeP<unknown, bigint, never>>, omitted>;
    gt<input, const min extends bigint>(min: min): BigIntChainable<MergeGuards<input, p, GuardExcludeP<unknown, bigint, never>>, omitted>;
    lte<input, const max extends bigint>(max: max): BigIntChainable<MergeGuards<input, p, GuardExcludeP<unknown, bigint, never>>, omitted>;
    gte<input, const min extends bigint>(min: min): BigIntChainable<MergeGuards<input, p, GuardExcludeP<unknown, bigint, never>>, omitted>;
    positive<input>(): BigIntChainable<MergeGuards<input, p, GuardExcludeP<unknown, bigint, never>>, omitted | 'positive' | 'negative'>;
    negative<input>(): BigIntChainable<MergeGuards<input, p, GuardExcludeP<unknown, bigint, never>>, omitted | 'positive' | 'negative' | 'negative'>;
}, omitted>;
type Variadic<pattern> = pattern & Iterable<pattern>;
type ArrayChainable<pattern, omitted extends string = never> = Variadic<pattern> & Omit<{
    optional<input>(): ArrayChainable<OptionalP<input, pattern>, omitted | 'optional'>;
    select<input>(): ArrayChainable<SelectP<anonymousSelectKey, input, pattern>, omitted | 'select'>;
    select<input, k extends string>(key: k): ArrayChainable<SelectP<k, input, pattern>, omitted | 'select'>;
}, omitted>;
type ExtractPreciseValue<a, b> = b extends Override<infer b1> ? b1 : unknown extends b ? a : 0 extends 1 & b ? a : 0 extends 1 & a ? b : b extends readonly any[] ? ExtractPreciseArrayValue<a, b, IsReadonlyArray<a>> : b extends Map<infer bk, infer bv> ? a extends Map<infer ak, infer av> ? Map<ExtractPreciseValue<ak, bk>, ExtractPreciseValue<av, bv>> : LeastUpperBound<a, b> : b extends Set<infer bv> ? a extends Set<infer av> ? Set<ExtractPreciseValue<av, bv>> : LeastUpperBound<a, b> : IsPlainObject<b, BuiltInObjects | Error> extends true ? a extends object ? a extends b ? a : b extends a ? Contains<b, never> extends true ? never : Contains<Omit<b, keyof a>, {}> extends true ? never : [
    Exclude<keyof a, keyof b>
] extends [
    never
] ? b : Compute<b & Omit<a, keyof b>> : [
    keyof a & keyof b
] extends [
    never
] ? never : Compute<{
    [k in keyof a as k extends keyof b ? never : k]: a[k];
} & {
    [k in keyof b]: k extends keyof a ? ExtractPreciseValue<a[k], b[k]> : b[k];
}> extends infer result ? Contains<Pick<result, keyof result & keyof b>, never> extends true ? never : result : never : LeastUpperBound<a, b> : LeastUpperBound<a, b>;
type ExtractPreciseArrayValue<a, b, isReadonly extends boolean, startOutput extends any[] = [
], endOutput extends any[] = [
]> = a extends readonly (infer aItem)[] ? b extends readonly [
] ? MaybeAddReadonly<[
    ...startOutput,
    ...endOutput
], isReadonly> : b extends readonly [
    infer b1,
    ...infer bRest
] ? a extends readonly [
    infer a1,
    ...infer aRest
] ? ExtractPreciseValue<a1, b1> extends infer currentValue ? [
    currentValue
] extends [
    never
] ? never : ExtractPreciseArrayValue<aRest, bRest, isReadonly, [
    ...startOutput,
    currentValue
], endOutput> : never : ExtractPreciseValue<aItem, b1> extends infer currentValue ? [
    currentValue
] extends [
    never
] ? never : ExtractPreciseArrayValue<aItem[], bRest, isReadonly, [
    ...startOutput,
    currentValue
], endOutput> : never : b extends readonly [
    ...infer bInit,
    infer b1
] ? a extends readonly [
    ...infer aInit,
    infer a1
] ? ExtractPreciseValue<a1, b1> extends infer currentValue ? [
    currentValue
] extends [
    never
] ? never : ExtractPreciseArrayValue<aInit, bInit, isReadonly, startOutput, [
    ...endOutput,
    currentValue
]> : never : ExtractPreciseValue<aItem, b1> extends infer currentValue ? [
    currentValue
] extends [
    never
] ? never : ExtractPreciseArrayValue<aItem[], bInit, isReadonly, startOutput, [
    ...endOutput,
    currentValue
]> : never : ExtractPreciseValue<aItem, ValueOf<b>> extends infer currentValue ? [
    currentValue
] extends [
    never
] ? never : MaybeAddReadonly<[
    ...startOutput,
    ...currentValue[],
    ...endOutput
], isReadonly> : never : LeastUpperBound<a, b>;
type BuildMany<data, xs extends readonly any[]> = xs extends any ? BuildOne<data, xs> : never;
type BuildOne<data, xs extends readonly any[]> = xs extends [
    [
        infer value,
        infer path
    ],
    ...infer tail
] ? BuildOne<SetDeep<data, value, path>, tail> : data;
type SetDeep<data, value, path> = path extends readonly [
    infer head,
    ...infer tail
] ? data extends readonly any[] ? data extends readonly [
    any,
    ...any
] ? head extends number ? UpdateAt<data, Iterator<head>, SetDeep<data[head], value, tail>> : never : SetDeep<ValueOf<data>, value, tail>[] : data extends Set<infer a> ? Set<SetDeep<a, value, tail>> : data extends Map<infer k, infer v> ? Map<k, SetDeep<v, value, tail>> : head extends keyof data ? [
    IsOptionalKeysOf<data, head>,
    tail,
    undefined
] extends [
    true,
    [
    ],
    value
] ? {
    [k in keyof data]: k extends head ? value : data[k];
} : {
    [k in keyof data]-?: k extends head ? SetDeep<data[head], value, tail> : data[k];
} : data : value;
type IsMatchingTuple<a extends readonly any[], b extends readonly any[]> = [
    a,
    b
] extends [
    readonly [
    ],
    readonly [
    ]
] ? true : [
    a,
    b
] extends [
    readonly [
        infer a1,
        ...infer aRest
    ],
    readonly [
        infer b1,
        ...infer bRest
    ]
] ? IsMatching<a1, b1> extends true ? IsMatchingTuple<aRest, bRest> : false : false;
type IsMatchingArray<a extends readonly any[], b extends readonly any[]> = b extends readonly [
] ? true : b extends readonly [
    infer b1,
    ...infer bRest
] ? a extends readonly [
    infer a1,
    ...infer aRest
] ? IsMatching<a1, b1> extends true ? IsMatchingArray<aRest, bRest> : false : a extends readonly [
] ? false : IsMatching<ValueOf<a>, b1> extends true ? IsMatchingArray<a, bRest> : false : b extends readonly [
    ...infer bInit,
    infer b1
] ? a extends readonly [
    ...infer aInit,
    infer a1
] ? IsMatching<a1, b1> extends true ? IsMatchingArray<aInit, bInit> : false : a extends readonly [
] ? false : IsMatching<ValueOf<a>, b1> extends true ? IsMatchingArray<a, bInit> : false : IsMatching<ValueOf<a>, ValueOf<b>>;
type IsMatching<a, b> = true extends IsUnion<a> | IsUnion<b> ? true extends (b extends any ? (a extends any ? IsMatching<a, b> : never) : never) ? true : false : unknown extends b ? true : {} extends b ? true : b extends Primitives ? a extends b ? true : b extends a ? true : false : Equal<a, b> extends true ? true : b extends readonly any[] ? a extends readonly any[] ? All<[
    IsLiteral<Length<a>>,
    IsLiteral<Length<b>>
]> extends true ? Equal<Length<a>, Length<b>> extends false ? false : IsMatchingTuple<a, b> : IsMatchingArray<a, b> : false : IsPlainObject<b> extends true ? true extends (a extends any ? [
    keyof b & keyof a
] extends [
    never
] ? false : {
    [k in keyof b & keyof a]: IsMatching<a[k], b[k]>;
}[keyof b & keyof a] extends true ? true : false : never) ? true : false : b extends a ? true : false;
type DistributeMatchingUnions<a, p> = IsAny<a> extends true ? any : BuildMany<a, Distribute<FindUnionsMany<a, p>>>;
type FindUnionsMany<a, p, path extends PropertyKey[] = [
]> = UnionToTuple<(p extends any ? IsMatching<a, p> extends true ? FindUnions<a, p, path> : [
] : never) extends readonly (infer T)[] ? T : never>;
type FindUnions<a, p, path extends PropertyKey[] = [
]> = unknown extends p ? [
] : IsAny<p> extends true ? [
] : Length<path> extends 5 ? [
] : IsUnion<a> extends true ? [
    {
        cases: a extends any ? {
            value: a;
            subUnions: FindUnionsMany<a, p, path>;
        } : never;
        path: path;
    }
] : [
    a,
    p
] extends [
    readonly any[],
    readonly any[]
] ? [
    a,
    p
] extends [
    readonly [
        infer a1,
        infer a2,
        infer a3,
        infer a4,
        infer a5
    ],
    readonly [
        infer p1,
        infer p2,
        infer p3,
        infer p4,
        infer p5
    ]
] ? [
    ...FindUnions<a1, p1, [
        ...path,
        0
    ]>,
    ...FindUnions<a2, p2, [
        ...path,
        1
    ]>,
    ...FindUnions<a3, p3, [
        ...path,
        2
    ]>,
    ...FindUnions<a4, p4, [
        ...path,
        3
    ]>,
    ...FindUnions<a5, p5, [
        ...path,
        4
    ]>
] : [
    a,
    p
] extends [
    readonly [
        infer a1,
        infer a2,
        infer a3,
        infer a4
    ],
    readonly [
        infer p1,
        infer p2,
        infer p3,
        infer p4
    ]
] ? [
    ...FindUnions<a1, p1, [
        ...path,
        0
    ]>,
    ...FindUnions<a2, p2, [
        ...path,
        1
    ]>,
    ...FindUnions<a3, p3, [
        ...path,
        2
    ]>,
    ...FindUnions<a4, p4, [
        ...path,
        3
    ]>
] : [
    a,
    p
] extends [
    readonly [
        infer a1,
        infer a2,
        infer a3
    ],
    readonly [
        infer p1,
        infer p2,
        infer p3
    ]
] ? [
    ...FindUnions<a1, p1, [
        ...path,
        0
    ]>,
    ...FindUnions<a2, p2, [
        ...path,
        1
    ]>,
    ...FindUnions<a3, p3, [
        ...path,
        2
    ]>
] : [
    a,
    p
] extends [
    readonly [
        infer a1,
        infer a2
    ],
    readonly [
        infer p1,
        infer p2
    ]
] ? [
    ...FindUnions<a1, p1, [
        ...path,
        0
    ]>,
    ...FindUnions<a2, p2, [
        ...path,
        1
    ]>
] : [
    a,
    p
] extends [
    readonly [
        infer a1
    ],
    readonly [
        infer p1
    ]
] ? FindUnions<a1, p1, [
    ...path,
    0
]> : p extends readonly [
] | readonly [
    any,
    ...any
] | readonly [
    ...any,
    any
] ? IsStrictArray<Extract<a, readonly any[]>> extends false ? [
] : [
    ArrayToVariadicUnion<a, p> extends infer aUnion ? {
        cases: aUnion extends any ? {
            value: aUnion;
            subUnions: [
            ];
        } : never;
        path: path;
    } : never
] : [
] : a extends Set<any> ? [
] : a extends Map<any, any> ? [
] : [
    IsPlainObject<a>,
    IsPlainObject<p>
] extends [
    true,
    true
] ? Flatten<Values<{
    [k in keyof a & keyof p]: FindUnions<a[k], p[k], [
        ...path,
        k
    ]>;
}>> : [
];
type ArrayToVariadicUnion<input, excluded> = MaybeAddReadonly<(input extends readonly [
    any,
    ...any
] | readonly [
    ...any,
    any
] ? never : [
]) | (excluded extends readonly [
    ...any,
    any
] ? [
    ...Extract<input, readonly any[]>,
    ValueOf<input>
] : [
    ValueOf<input>,
    ...Extract<input, readonly any[]>
]), IsReadonlyArray<input>>;
type Distribute<unions extends readonly any[]> = unions extends readonly [
    {
        cases: infer cases;
        path: infer path;
    },
    ...infer tail
] ? cases extends {
    value: infer value;
    subUnions: infer subUnions;
} ? [
    [
        value,
        path
    ],
    ...Distribute<Extract<subUnions, readonly any[]>>,
    ...Distribute<tail>
] : never : [
];
type DeepExclude<a, b> = Exclude<DistributeMatchingUnions<a, b>, b>;
type OptionalKeys<p> = ValueOf<{
    [k in keyof p]: 0 extends 1 & p[k] ? never : p[k] extends Matcher<any, any, infer matcherType> ? matcherType extends 'optional' ? k : never : never;
}>;
type ReduceUnion<tuple extends readonly any[], i, output = never> = tuple extends readonly [
    infer p,
    ...infer tail
] ? ReduceUnion<tail, i, output | InvertPatternInternal<p, i>> : output;
type ReduceIntersection<tuple extends readonly any[], i, output = unknown> = tuple extends readonly [
    infer p,
    ...infer tail
] ? ReduceIntersection<tail, i, output & InvertPatternInternal<p, i>> : output;
type InvertArrayPattern<p, i, startOutput extends any[] = [
], endOutput extends any[] = [
]> = i extends readonly (infer ii)[] ? p extends readonly [
] ? [
    ...startOutput,
    ...endOutput
] : p extends readonly [
    infer p1,
    ...infer pRest
] ? i extends readonly [
    infer i1,
    ...infer iRest
] ? InvertArrayPattern<pRest, iRest, [
    ...startOutput,
    InvertPatternInternal<p1, i1>
], endOutput> : InvertArrayPattern<pRest, ii[], [
    ...startOutput,
    InvertPatternInternal<p1, ii>
], endOutput> : p extends readonly [
    ...infer pInit,
    infer p1
] ? i extends readonly [
    ...infer iInit,
    infer i1
] ? InvertArrayPattern<pInit, iInit, startOutput, [
    ...endOutput,
    InvertPatternInternal<p1, i1>
]> : InvertArrayPattern<pInit, ii[], startOutput, [
    ...endOutput,
    InvertPatternInternal<p1, ii>
]> : p extends readonly [
    ...(readonly (infer pRest & AnyMatcher)[])
] ? [
    ...startOutput,
    ...Extract<InvertPatternInternal<pRest, i>, readonly any[]>,
    ...endOutput
] : [
    ...startOutput,
    ...InvertPatternInternal<ValueOf<p>, ii>[],
    ...endOutput
] : never;
type InvertPattern<p, input> = Equal<Pattern<input>, p> extends true ? never : InvertPatternInternal<p, input>;
type InvertPatternInternal<p, input> = 0 extends 1 & p ? never : p extends Matcher<infer _input, infer subpattern, infer matcherType, any, infer narrowedOrFn> ? {
    not: DeepExclude<input, InvertPatternInternal<subpattern, input>>;
    select: InvertPatternInternal<subpattern, input>;
    array: InvertPatternInternal<subpattern, ReadonlyArrayValue<input>>[];
    record: subpattern extends [
        infer pk,
        infer pv
    ] ? Record<Extract<InvertPatternInternal<pk, RecordKey<input>>, PropertyKey>, InvertPatternInternal<pv, RecordValue<input>>> : never;
    map: subpattern extends [
        infer pk,
        infer pv
    ] ? Map<InvertPatternInternal<pk, MapKey<Extract<input, Map<any, any>>>>, InvertPatternInternal<pv, MapValue<Extract<input, Map<any, any>>>>> : never;
    set: Set<InvertPatternInternal<subpattern, SetValue<Extract<input, Set<any>>>>>;
    optional: InvertPatternInternal<subpattern, Exclude<input, undefined>> | undefined;
    and: ReduceIntersection<Extract<subpattern, readonly any[]>, input>;
    or: ReduceUnion<Extract<subpattern, readonly any[]>, input>;
    default: [
        subpattern
    ] extends [
        never
    ] ? input : subpattern;
    custom: Override<narrowedOrFn extends Fn ? Call<narrowedOrFn, input> : narrowedOrFn>;
}[matcherType] : p extends Primitives ? p : p extends readonly any[] ? InvertArrayPattern<p, WithDefault$1<Extract<input, readonly any[]>, unknown[]>> : IsPlainObject<p> extends true ? OptionalKeys<p> extends infer optKeys ? [
    optKeys
] extends [
    never
] ? {
    [k in Exclude<keyof p, optKeys>]: InvertPatternInternal<p[k], WithDefault$1<GetKey<ExtractPlainObject<input>, k>, unknown>>;
} : Compute<{
    [k in Exclude<keyof p, optKeys>]: InvertPatternInternal<p[k], WithDefault$1<GetKey<ExtractPlainObject<input>, k>, unknown>>;
} & {
    [k in Extract<optKeys, keyof p>]?: InvertPatternInternal<p[k], WithDefault$1<GetKey<ExtractPlainObject<input>, k>, unknown>>;
}> : never : p;
type ReduceIntersectionForExclude<tuple extends readonly any[], i, output = unknown> = tuple extends readonly [
    infer p,
    ...infer tail
] ? ReduceIntersectionForExclude<tail, i, output & InvertPatternForExcludeInternal<p, i, unknown>> : output;
type ReduceUnionForExclude<tuple extends readonly any[], i, output = never> = tuple extends readonly [
    infer p,
    ...infer tail
] ? ReduceUnionForExclude<tail, i, output | InvertPatternForExcludeInternal<p, i, never>> : output;
type ExcludeIfExists<a, b> = [
    b
] extends [
    never
] ? never : unknown extends a ? never : All<[
    Extends<a, NonLiteralPrimitive>,
    Not<IsLiteral<a>>,
    IsLiteral<b>
]> extends true ? never : DeepExclude<a, b>;
type InvertArrayPatternForExclude<p, i, empty, isReadonly extends boolean, startOutput extends any[] = [
], endOutput extends any[] = [
]> = i extends readonly (infer ii)[] ? p extends readonly [
] ? MaybeAddReadonly<[
    ...startOutput,
    ...endOutput
], isReadonly> : p extends readonly [
    infer p1,
    ...infer pRest
] ? i extends readonly [
    infer i1,
    ...infer iRest
] ? InvertArrayPatternForExclude<pRest, iRest, empty, isReadonly, [
    ...startOutput,
    InvertPatternForExcludeInternal<p1, i1, empty>
], endOutput> : InvertArrayPatternForExclude<pRest, ii[], empty, isReadonly, [
    ...startOutput,
    InvertPatternForExcludeInternal<p1, ii, empty>
], endOutput> : p extends readonly [
    ...infer pInit,
    infer p1
] ? i extends readonly [
    ...infer iInit,
    infer i1
] ? InvertArrayPatternForExclude<pInit, iInit, empty, isReadonly, startOutput, [
    ...endOutput,
    InvertPatternForExcludeInternal<p1, i1, empty>
]> : InvertArrayPatternForExclude<pInit, ii[], empty, isReadonly, startOutput, [
    ...endOutput,
    InvertPatternForExcludeInternal<p1, ii, empty>
]> : p extends readonly [
    ...(readonly (infer pRest & AnyMatcher)[])
] ? MaybeAddReadonly<[
    ...startOutput,
    ...Extract<InvertPatternForExcludeInternal<pRest, i, empty>, readonly any[]>,
    ...endOutput
], isReadonly> : MaybeAddReadonly<[
    ...startOutput,
    ...InvertPatternForExcludeInternal<ValueOf<p>, ii, empty>[],
    ...endOutput
], isReadonly> : empty;
type InvertPatternForExclude<p, i> = Equal<Pattern<i>, p> extends true ? never : InvertPatternForExcludeInternal<p, i>;
type InvertPatternForExcludeInternal<p, i, empty = never> = unknown extends p ? i : [
    p
] extends [
    Primitives
] ? IsLiteral<p> extends true ? p : IsLiteral<i> extends true ? p : empty : p extends Matcher<infer matchableInput, infer subpattern, infer matcherType, any, infer excluded> ? {
    select: InvertPatternForExcludeInternal<subpattern, i, empty>;
    array: i extends readonly (infer ii)[] ? MaybeAddReadonly<InvertPatternForExcludeInternal<subpattern, ii, empty>[], IsReadonlyArray<i>> : empty;
    record: subpattern extends [
        infer pk,
        infer pv
    ] ? Record<Extract<InvertPatternForExcludeInternal<pk, RecordKey<i>, empty>, PropertyKey>, InvertPatternForExcludeInternal<pv, RecordValue<i>, empty>> : empty;
    map: subpattern extends [
        infer pk,
        infer pv
    ] ? i extends Map<infer ik, infer iv> ? Map<InvertPatternForExcludeInternal<pk, ik, empty>, InvertPatternForExcludeInternal<pv, iv, empty>> : empty : empty;
    set: i extends Set<infer iv> ? Set<InvertPatternForExcludeInternal<subpattern, iv, empty>> : empty;
    optional: InvertPatternForExcludeInternal<subpattern, i, empty> | undefined;
    and: ReduceIntersectionForExclude<Extract<subpattern, readonly any[]>, i>;
    or: ReduceUnionForExclude<Extract<subpattern, readonly any[]>, i>;
    not: ExcludeIfExists<unknown extends matchableInput ? i : matchableInput, InvertPatternForExcludeInternal<subpattern, i>>;
    default: excluded;
    custom: excluded extends infer narrowedOrFn extends Fn ? Call<narrowedOrFn, i> : excluded;
}[matcherType] : p extends readonly any[] ? Extract<i, readonly any[]> extends infer arrayInput ? InvertArrayPatternForExclude<p, arrayInput, empty, IsReadonlyArray<arrayInput>> : never : IsPlainObject<p> extends true ? Equal<{}, p> extends true ? {} : i extends object ? [
    keyof p & keyof i
] extends [
    never
] ? empty : OptionalKeys<p> extends infer optKeys ? [
    optKeys
] extends [
    never
] ? {
    readonly [k in keyof p]: k extends keyof i ? InvertPatternForExcludeInternal<p[k], i[k], empty> : InvertPatternInternal<p[k], unknown>;
} : Compute<{
    readonly [k in Exclude<keyof p, optKeys>]: k extends keyof i ? InvertPatternForExcludeInternal<p[k], i[k], empty> : InvertPatternInternal<p[k], unknown>;
} & {
    readonly [k in Extract<optKeys, keyof p>]?: k extends keyof i ? InvertPatternForExcludeInternal<p[k], i[k], empty> : InvertPatternInternal<p[k], unknown>;
}> : empty : empty : empty;
type unstable_Matchable<narrowedOrFn, input = unknown, pattern = never> = CustomP<input, pattern, narrowedOrFn>;
type unstable_Matcher<narrowedOrFn, input = unknown, pattern = never> = ReturnType<CustomP<input, pattern, narrowedOrFn>[matcher]>;
type InferPattern<pattern> = InvertPattern<NoInfer<pattern>, unknown>;
type narrow<input, pattern> = ExtractPreciseValue<input, InvertPattern<pattern, input>>;
declare function optional<input, const pattern extends unknown extends input ? UnknownValuePattern : Pattern<input>>(pattern: pattern): Chainable<OptionalP<input, pattern>, 'optional'>;
type UnwrapArray<xs> = xs extends readonly (infer x)[] ? x : never;
type UnwrapSet<xs> = xs extends Set<infer x> ? x : never;
type UnwrapMapKey<xs> = xs extends Map<infer k, any> ? k : never;
type UnwrapMapValue<xs> = xs extends Map<any, infer v> ? v : never;
type UnwrapRecordKey<xs> = xs extends Record<infer k, any> ? k : never;
type UnwrapRecordValue<xs> = xs extends Record<any, infer v> ? v : never;
type WithDefault<a, b> = [
    a
] extends [
    never
] ? b : a;
declare function array<input>(): ArrayChainable<ArrayP<input, unknown>>;
declare function array<input, const pattern extends Pattern<WithDefault<UnwrapArray<input>, unknown>>>(pattern: pattern): ArrayChainable<ArrayP<input, pattern>>;
declare function set<input>(): Chainable<SetP<input, unknown>>;
declare function set<input, const pattern extends Pattern<WithDefault<UnwrapSet<input>, unknown>>>(pattern: pattern): Chainable<SetP<input, pattern>>;
declare function map<input>(): Chainable<MapP<input, unknown, unknown>>;
declare function map<input, const pkey extends Pattern<WithDefault<UnwrapMapKey<input>, unknown>>, const pvalue extends Pattern<WithDefault<UnwrapMapValue<input>, unknown>>>(patternKey: pkey, patternValue: pvalue): Chainable<MapP<input, pkey, pvalue>>;
declare function record<input, const pvalue extends Pattern<UnwrapRecordValue<input>>>(patternValue: pvalue): Chainable<RecordP<input, StringPattern, pvalue>>;
declare function record<input, const pkey extends Pattern<WithDefault<UnwrapRecordKey<input>, PropertyKey>>, const pvalue extends Pattern<WithDefault<UnwrapRecordValue<input>, unknown>>>(patternKey: pkey, patternValue?: pvalue): Chainable<RecordP<input, pkey, pvalue>>;
declare function intersection<input, const patterns extends readonly [
    Pattern<input>,
    ...Pattern<input>[]
]>(...patterns: patterns): Chainable<AndP<input, patterns>>;
declare function union<input, const patterns extends readonly [
    Pattern<input>,
    ...Pattern<input>[]
]>(...patterns: patterns): Chainable<OrP<input, patterns>>;
declare function not<input, const pattern extends Pattern<input> | UnknownValuePattern>(pattern: pattern): Chainable<NotP<input, pattern>>;
declare function when<input, predicate extends (value: input) => unknown>(predicate: predicate): GuardP<input, predicate extends (value: any) => value is infer narrowed ? narrowed : never>;
declare function when<input, narrowed extends input, excluded>(predicate: (input: input) => input is narrowed): GuardExcludeP<input, narrowed, excluded>;
declare function select(): Chainable<AnonymousSelectP, 'select' | 'or' | 'and'>;
declare function select<input, const patternOrKey extends string | (unknown extends input ? UnknownValuePattern : Pattern<input>)>(patternOrKey: patternOrKey): patternOrKey extends string ? Chainable<SelectP<patternOrKey, 'select' | 'or' | 'and'>> : Chainable<SelectP<anonymousSelectKey, input, patternOrKey>, 'select' | 'or' | 'and'>;
declare function select<input, const pattern extends unknown extends input ? UnknownValuePattern : Pattern<input>, const k extends string>(key: k, pattern: pattern): Chainable<SelectP<k, input, pattern>, 'select' | 'or' | 'and'>;
type AnyConstructor = abstract new (...args: any[]) => any;
declare const any: UnknownPattern;
declare const unknown: UnknownPattern;
declare const _: UnknownPattern;
declare const string: StringPattern;
declare const number: NumberPattern;
declare const bigint: BigIntPattern;
declare const boolean: BooleanPattern;
declare const symbol: SymbolPattern;
declare const nullish: NullishPattern;
declare const nonNullable: NonNullablePattern;
declare function instanceOf<T extends AnyConstructor>(classConstructor: T): Chainable<GuardP<unknown, InstanceType<T>>>;
declare function shape<input, const pattern extends Pattern<input>>(pattern: pattern): Chainable<GuardP<input, InvertPattern<pattern, input>>>;
type patterns_d_Pattern<a = unknown> = Pattern<a>;
declare const patterns_d__: typeof _;
declare const patterns_d_any: typeof any;
declare const patterns_d_array: typeof array;
declare const patterns_d_bigint: typeof bigint;
declare const patterns_d_boolean: typeof boolean;
type patterns_d_infer<pattern> = InferPattern<pattern>;
declare const patterns_d_instanceOf: typeof instanceOf;
declare const patterns_d_intersection: typeof intersection;
declare const patterns_d_map: typeof map;
type patterns_d_matcher = matcher;
type patterns_d_narrow<input, pattern> = narrow<input, pattern>;
declare const patterns_d_nonNullable: typeof nonNullable;
declare const patterns_d_not: typeof not;
declare const patterns_d_nullish: typeof nullish;
declare const patterns_d_number: typeof number;
declare const patterns_d_optional: typeof optional;
declare const patterns_d_record: typeof record;
declare const patterns_d_select: typeof select;
declare const patterns_d_set: typeof set;
declare const patterns_d_shape: typeof shape;
declare const patterns_d_string: typeof string;
declare const patterns_d_symbol: typeof symbol;
declare const patterns_d_union: typeof union;
declare const patterns_d_unknown: typeof unknown;
type patterns_d_unstable_Matchable<narrowedOrFn, input = unknown, pattern = never> = unstable_Matchable<narrowedOrFn, input, pattern>;
type patterns_d_unstable_Matcher<narrowedOrFn, input = unknown, pattern = never> = unstable_Matcher<narrowedOrFn, input, pattern>;
declare const patterns_d_when: typeof when;
declare namespace patterns_d {
    export { patterns_d__ as _, patterns_d_any as any, patterns_d_array as array, patterns_d_bigint as bigint, patterns_d_boolean as boolean, patterns_d_instanceOf as instanceOf, patterns_d_intersection as intersection, patterns_d_map as map, patterns_d_nonNullable as nonNullable, patterns_d_not as not, patterns_d_nullish as nullish, patterns_d_number as number, patterns_d_optional as optional, patterns_d_record as record, patterns_d_select as select, patterns_d_set as set, patterns_d_shape as shape, patterns_d_string as string, patterns_d_symbol as symbol, patterns_d_union as union, patterns_d_unknown as unknown, patterns_d_when as when };
    export type { patterns_d_Pattern as Pattern, patterns_d_infer as infer, patterns_d_matcher as matcher, patterns_d_narrow as narrow, Fn as unstable_Fn, patterns_d_unstable_Matchable as unstable_Matchable, patterns_d_unstable_Matcher as unstable_Matcher };
}
type PickReturnValue<a, b> = a extends unset ? b : a;
interface NonExhaustiveError$1<i> {
    __nonExhaustive: never;
}
interface TSPatternError<i> {
    __nonExhaustive: never;
}
type Match<i, o, handledCases extends any[] = [
], inferredOutput = never> = {
    with<const p extends Pattern<i>, c, value extends MatchedValue<i, InvertPattern<p, i>>>(pattern: IsNever<p> extends true ? Pattern<i> : p, handler: (selections: FindSelected<value, p>, value: value) => PickReturnValue<o, c>): InvertPatternForExclude<p, value> extends infer excluded ? Match<Exclude<i, excluded>, o, [
        ...handledCases,
        excluded
    ], Union<inferredOutput, c>> : never;
    with<const p1 extends Pattern<i>, const p2 extends Pattern<i>, c, p extends p1 | p2, value extends p extends any ? MatchedValue<i, InvertPattern<p, i>> : never>(p1: p1, p2: p2, handler: (value: value) => PickReturnValue<o, c>): [
        InvertPatternForExclude<p1, value>,
        InvertPatternForExclude<p2, value>
    ] extends [
        infer excluded1,
        infer excluded2
    ] ? Match<Exclude<i, excluded1 | excluded2>, o, [
        ...handledCases,
        excluded1,
        excluded2
    ], Union<inferredOutput, c>> : never;
    with<const p1 extends Pattern<i>, const p2 extends Pattern<i>, const p3 extends Pattern<i>, const ps extends readonly Pattern<i>[], c, p extends p1 | p2 | p3 | ps[number], value extends MatchedValue<i, InvertPattern<p, i>>>(...args: [
        p1: p1,
        p2: p2,
        p3: p3,
        ...patterns: ps,
        handler: (value: value) => PickReturnValue<o, c>
    ]): [
        InvertPatternForExclude<p1, value>,
        InvertPatternForExclude<p2, value>,
        InvertPatternForExclude<p3, value>,
        MakeTuples<ps, value>
    ] extends [
        infer excluded1,
        infer excluded2,
        infer excluded3,
        infer excludedRest
    ] ? Match<Exclude<i, excluded1 | excluded2 | excluded3 | Extract<excludedRest, any[]>[number]>, o, [
        ...handledCases,
        excluded1,
        excluded2,
        excluded3,
        ...Extract<excludedRest, any[]>
    ], Union<inferredOutput, c>> : never;
    with<const pat extends Pattern<i>, pred extends (value: MatchedValue<i, InvertPattern<pat, i>>) => unknown, c, value extends GuardValue<pred>>(pattern: pat, predicate: pred, handler: (selections: FindSelected<value, pat>, value: value) => PickReturnValue<o, c>): pred extends (value: any) => value is infer narrowed ? Match<Exclude<i, narrowed>, o, [
        ...handledCases,
        narrowed
    ], Union<inferredOutput, c>> : Match<i, o, handledCases, Union<inferredOutput, c>>;
    when<pred extends (value: i) => unknown, c, value extends GuardValue<pred>>(predicate: pred, handler: (value: value) => PickReturnValue<o, c>): pred extends (value: any) => value is infer narrowed ? Match<Exclude<i, narrowed>, o, [
        ...handledCases,
        narrowed
    ], Union<inferredOutput, c>> : Match<i, o, handledCases, Union<inferredOutput, c>>;
    otherwise<c>(handler: (value: i) => PickReturnValue<o, c>): PickReturnValue<o, Union<inferredOutput, c>>;
    exhaustive: DeepExcludeAll<i, handledCases> extends infer remainingCases ? [
        remainingCases
    ] extends [
        never
    ] ? Exhaustive<o, inferredOutput> : NonExhaustiveError$1<remainingCases> : never;
    run(): PickReturnValue<o, inferredOutput>;
    returnType: [
        inferredOutput
    ] extends [
        never
    ] ? <output>() => Match<i, output, handledCases> : TSPatternError<'calling `.returnType<T>()` is only allowed directly after `match(...)`.'>;
    narrow(): Match<DeepExcludeAll<i, handledCases>, o, [
    ], inferredOutput>;
};
type DeepExcludeAll<a, tupleList extends any[]> = [
    a
] extends [
    never
] ? never : tupleList extends [
    infer excluded,
    ...infer tail
] ? DeepExcludeAll<DeepExclude<a, excluded>, tail> : a;
type MakeTuples<ps extends readonly any[], value> = {
    -readonly [index in keyof ps]: InvertPatternForExclude<ps[index], value>;
};
type Exhaustive<output, inferredOutput> = {
    (): PickReturnValue<output, inferredOutput>;
    <otherOutput>(handler: (unexpectedValue: unknown) => PickReturnValue<output, otherOutput>): PickReturnValue<output, Union<inferredOutput, otherOutput>>;
};
declare function match<const input, output = unset>(value: input): Match<input, output>;
type PatternConstraint<T> = T extends readonly any[] ? Pattern<T> : T extends object ? Pattern<T> & UnknownProperties : Pattern<T>;
declare function isMatching<const p extends Pattern<unknown>>(pattern: p): (value: unknown) => value is InferPattern<p>;
declare function isMatching<const T, const P extends PatternConstraint<T>>(pattern: P, value: T): value is T & WithDefault$1<patterns_d.narrow<T, P>, patterns_d.infer<P>>;
declare class NonExhaustiveError extends Error {
    input: unknown;
    constructor(input: unknown);
}
export { NonExhaustiveError, patterns_d as P, isMatching, match };
