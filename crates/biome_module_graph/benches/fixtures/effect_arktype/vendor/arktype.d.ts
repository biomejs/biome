type Fn<args extends readonly any[] = readonly any[], returns = unknown> = (...args: args) => returns;
type Thunk<ret = unknown> = () => ret;
type CallableOptions<attachments extends object> = {
    attach?: attachments;
    bind?: object;
};
interface Callable<fn extends Fn, attachments extends object> extends fn, attachments {
}
declare class Callable<fn extends Fn, attachments extends object = {}> {
    constructor(fn: fn, ...[opts]: {} extends attachments ? [
        opts?: CallableOptions<attachments>
    ] : [
        opts: CallableOptions<attachments>
    ]);
}
type GuardablePredicate<input = unknown, narrowed extends input = input> = ((In: input) => In is narrowed) | ((In: input) => boolean);
declare const ecmascriptConstructors: {
    Array: ArrayConstructor;
    Boolean: BooleanConstructor;
    Date: DateConstructor;
    Error: ErrorConstructor;
    Function: FunctionConstructor;
    Map: MapConstructor;
    Number: NumberConstructor;
    Promise: PromiseConstructor;
    RegExp: RegExpConstructor;
    Set: SetConstructor;
    String: StringConstructor;
    WeakMap: WeakMapConstructor;
    WeakSet: WeakSetConstructor;
};
type ecmascriptConstructors = typeof ecmascriptConstructors;
type EcmascriptObjects = satisfy<instantiateConstructors<keyof ecmascriptConstructors>, {
    Array: Array<unknown>;
    Boolean: Boolean;
    Date: Date;
    Error: Error;
    Function: Function;
    Map: Map<unknown, unknown>;
    Number: Number;
    RegExp: RegExp;
    Set: Set<unknown>;
    String: String;
    WeakMap: WeakMap<object, unknown>;
    WeakSet: WeakSet<object>;
    Promise: Promise<unknown>;
}>;
type platformConstructors = {
    ArrayBuffer: ArrayBufferConstructor;
    Blob: typeof Blob;
    File: typeof File;
    FormData: typeof FormData;
    Headers: typeof Headers;
    Request: typeof Request;
    Response: typeof Response;
    URL: typeof URL;
};
declare const platformConstructors: platformConstructors;
type PlatformObjects = instantiateConstructors<keyof platformConstructors>;
declare const builtinConstructors: {
    String: StringConstructor;
    Number: NumberConstructor;
    Boolean: BooleanConstructor;
    Int8Array: Int8ArrayConstructor;
    Uint8Array: Uint8ArrayConstructor;
    Uint8ClampedArray: Uint8ClampedArrayConstructor;
    Int16Array: Int16ArrayConstructor;
    Uint16Array: Uint16ArrayConstructor;
    Int32Array: Int32ArrayConstructor;
    Uint32Array: Uint32ArrayConstructor;
    Float32Array: Float32ArrayConstructor;
    Float64Array: Float64ArrayConstructor;
    BigInt64Array: BigInt64ArrayConstructor;
    BigUint64Array: BigUint64ArrayConstructor;
    ArrayBuffer: ArrayBufferConstructor;
    Blob: typeof Blob;
    File: typeof File;
    FormData: typeof FormData;
    Headers: typeof Headers;
    Request: typeof Request;
    Response: typeof Response;
    URL: typeof URL;
    Array: ArrayConstructor;
    Date: DateConstructor;
    Error: ErrorConstructor;
    Function: FunctionConstructor;
    Map: MapConstructor;
    Promise: PromiseConstructor;
    RegExp: RegExpConstructor;
    Set: SetConstructor;
    WeakMap: WeakMapConstructor;
    WeakSet: WeakSetConstructor;
};
type builtinConstructors = typeof builtinConstructors;
type BuiltinObjectKind = keyof builtinConstructors;
type GlobalName = keyof typeof globalThis;
type instantiateConstructors<kind extends BuiltinObjectKind> = {
    [k in kind]: k extends GlobalName ? InstanceType<(typeof globalThis)[k]> : `${k}Constructor` extends GlobalName ? InstanceType<(typeof globalThis)[`${k}Constructor`]> : never;
};
type describeObject<o extends object, opts extends DescribeOptions = {}> = objectKindOf<o> extends string ? [
    opts["includeArticles"]
] extends [
    true
] ? objectKindDescriptions[objectKindOf<o>] : objectKindOf<o> : [
    opts["includeArticles"]
] extends [
    true
] ? domainDescriptions["object"] : "object";
type instantiableObjectKind<data extends object> = {
    [kind in keyof builtinConstructors]: data extends (InstanceType<builtinConstructors[kind]>) ? kind : never;
}[keyof builtinConstructors];
type objectKindOf<data extends object> = object extends data ? keyof builtinConstructors | undefined : data extends Fn ? "Function" : instantiableObjectKind<data> extends never ? undefined : instantiableObjectKind<data>;
declare const objectKindOf: <data extends object>(data: data) => objectKindOf<data> | undefined;
declare const objectKindOrDomainOf: <data>(data: data) => (objectKindOf<data & object> & {}) | domainOf<data>;
type objectKindOrDomainOf<data> = data extends object ? objectKindOf<data> extends undefined ? "object" : objectKindOf<data> : domainOf<data>;
declare const objectKindDescriptions: {
    readonly Int8Array: "an Int8Array";
    readonly Uint8Array: "a Uint8Array";
    readonly Uint8ClampedArray: "a Uint8ClampedArray";
    readonly Int16Array: "an Int16Array";
    readonly Uint16Array: "a Uint16Array";
    readonly Int32Array: "an Int32Array";
    readonly Uint32Array: "a Uint32Array";
    readonly Float32Array: "a Float32Array";
    readonly Float64Array: "a Float64Array";
    readonly BigInt64Array: "a BigInt64Array";
    readonly BigUint64Array: "a BigUint64Array";
    readonly ArrayBuffer: string;
    readonly Blob: string;
    readonly File: string;
    readonly FormData: string;
    readonly Headers: string;
    readonly Request: string;
    readonly Response: string;
    readonly URL: string;
    readonly Array: "an array";
    readonly Function: "a function";
    readonly Date: "a Date";
    readonly RegExp: "a RegExp";
    readonly Error: "an Error";
    readonly Map: "a Map";
    readonly Set: "a Set";
    readonly String: "a String object";
    readonly Number: "a Number object";
    readonly Boolean: "a Boolean object";
    readonly Promise: "a Promise";
    readonly WeakMap: "a WeakMap";
    readonly WeakSet: "a WeakSet";
};
type objectKindDescriptions = typeof objectKindDescriptions;
type Constructor<instance = {}> = abstract new (...args: never[]) => instance;
type stringifyUnion<t extends string, delimiter extends string = ", "> = join<unionToTuple<t>, delimiter>;
type unionToTuple<t> = _unionToTuple<t, [
]> extends infer result ? conform<result, t[]> : never;
type _unionToTuple<t, result extends unknown[]> = getLastBranch<t> extends infer current ? [
    t
] extends [
    never
] ? result : _unionToTuple<Exclude<t, current>, [
    current,
    ...result
]> : never;
type getLastBranch<t> = intersectUnion<t extends unknown ? (x: t) => void : never> extends ((x: infer branch) => void) ? branch : never;
type intersectUnion<t> = (t extends unknown ? (_: t) => void : never) extends ((_: infer intersection) => void) ? intersection : never;
type DescribeOptions = {
    includeArticles?: boolean;
    branchDelimiter?: string;
};
type typeToString<t, opts extends DescribeOptions = {}> = stringifyUnion<[
    t
] extends [
    anyOrNever
] ? unknown extends t ? "any" : "never" : unknown extends t ? "unknown" : boolean extends t ? "boolean" | ([
    t
] extends [
    boolean
] ? never : typeToString<Exclude<t, boolean>, opts>) : t extends array ? arrayTypeToString<t, opts> : t extends object ? describeObject<t, opts> : t extends Stringifiable ? stringifiableToString<t, opts> : describeDomainOf<t, opts>, opts["branchDelimiter"] extends string ? opts["branchDelimiter"] : describeDefaults["branchDelimiter"]>;
type stringifiableToString<t extends Stringifiable, opts extends DescribeOptions> = inferDomain<domainOf<t>> extends t ? describeDomainOf<t, opts> : `${t}`;
type describe<t> = typeToString<t, {
    includeArticles: true;
    branchDelimiter: " or ";
}>;
type arrayTypeToString<t extends array, opts extends DescribeOptions> = typeToString<t[number], opts> extends infer element extends string ? opts["includeArticles"] extends true ? describeArrayOf<element> : includesDelimiter<element, opts> extends true ? `(${element})[]` : `${element}[]` : never;
type describeArrayOf<element extends string> = element extends "unknown" ? "an array" : `an array of ${element}`;
type includesDelimiter<s extends string, opts extends DescribeOptions> = s extends (`${string}${opts["branchDelimiter"] extends string ? opts["branchDelimiter"] : describeDefaults["branchDelimiter"]}${string}`) ? true : false;
type describeDefaults = satisfy<Required<DescribeOptions>, {
    includeArticles: false;
    branchDelimiter: " | ";
}>;
type TypesByDomain = {
    bigint: bigint;
    boolean: boolean;
    number: number;
    object: object;
    string: string;
    symbol: symbol;
    undefined: undefined;
    null: null;
};
type inferDomain<kind extends Domain$1> = Domain$1 extends kind ? unknown : TypesByDomain[kind];
type Domain$1 = show<keyof TypesByDomain>;
type PrimitiveDomain = Exclude<Domain$1, "object">;
type Primitive = inferDomain<PrimitiveDomain>;
type domainOf<data> = unknown extends data ? Domain$1 : data extends object ? "object" : data extends string ? "string" : data extends number ? "number" : data extends boolean ? "boolean" : data extends undefined ? "undefined" : data extends null ? "null" : data extends bigint ? "bigint" : data extends symbol ? "symbol" : never;
declare const domainOf: <data>(data: data) => domainOf<data>;
declare const domainDescriptions: {
    readonly boolean: "boolean";
    readonly null: "null";
    readonly undefined: "undefined";
    readonly bigint: "a bigint";
    readonly number: "a number";
    readonly object: "an object";
    readonly string: "a string";
    readonly symbol: "a symbol";
};
type domainDescriptions = typeof domainDescriptions;
type describeDomainOf<t, opts extends DescribeOptions = {}> = stringifyUnion<opts["includeArticles"] extends true ? domainDescriptions[domainOf<t>] : domainOf<t>, opts["branchDelimiter"] extends string ? opts["branchDelimiter"] : describeDefaults["branchDelimiter"]>;
type Digit = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9;
type NumberLiteral<n extends number = number> = `${n}`;
type BigintLiteral<n extends bigint = bigint> = `${n}n`;
type NonNegativeIntegerLiteral<n extends bigint = bigint> = `${Digit}` | (`${Exclude<Digit, 0>}${string}` & `${n}`);
type NumericLiteralKind = "number" | "bigint" | "integer";
declare const numericLiteralDescriptions: {
    readonly number: "a number";
    readonly bigint: "a bigint";
    readonly integer: "an integer";
};
type numericLiteralDescriptions = typeof numericLiteralDescriptions;
type writeMalformedNumericLiteralMessage<def extends string, kind extends NumericLiteralKind> = `'${def}' was parsed as ${numericLiteralDescriptions[kind]} but could not be narrowed to a literal value. Avoid unnecessary leading or trailing zeros and other abnormal notation`;
declare const writeMalformedNumericLiteralMessage: <def extends string, kind extends NumericLiteralKind>(def: def, kind: kind) => writeMalformedNumericLiteralMessage<def, kind>;
type parseInteger<token extends string> = token extends `${bigint}` ? token extends `${infer n extends number}` ? n : never : never;
type parseNonNegativeInteger<token extends string> = token extends `-${string}` ? never : parseInteger<token>;
type Key = string | symbol;
type toArkKey<o, k extends keyof o> = k extends number ? [
    o,
    number
] extends [
    array,
    k
] ? NonNegativeIntegerLiteral : `${k}` : k;
type arkIndexableOf<o> = arkKeyOf<o> extends infer k ? k extends `${infer index extends number}` ? index | k : k : never;
type arkKeyOf<o> = [
    o
] extends [
    object
] ? [
    o
] extends [
    array
] ? arkArrayKeyOf<o> : arkObjectLiteralKeyOf<o> : never;
type arkArrayKeyOf<a extends array> = number extends a["length"] ? NonNegativeIntegerLiteral : keyof a extends infer i ? i extends `${number}` ? i : never : never;
type arkObjectLiteralKeyOf<o extends object> = keyof o extends infer k ? k extends number ? `${k}` : k : never;
type arkGet<o, k extends arkIndexableOf<o>> = o[k extends keyof o ? k : NonNegativeIntegerLiteral extends k ? number & keyof o : k extends number ? `${k}` & keyof o : never];
type propwiseXor<a, b> = show<a & {
    [k in keyof b]?: undefined;
}> | show<b & {
    [k in keyof a]?: undefined;
}>;
type requireKeys<o, key extends keyof o> = o & {
    [requiredKey in key]-?: defined<o[requiredKey]>;
};
type isSafelyMappable<t> = {
    [k in keyof t]: t[k];
} extends t ? true : false;
type KeySet<key extends string = string> = {
    readonly [_ in key]?: 1;
};
type keySetOf<o extends object> = KeySet<Extract<keyof o, string>>;
type mutable<o, maxDepth extends number = 1> = _mutable<o, [
], maxDepth>;
type _mutable<o, depth extends 1[], maxDepth extends number> = depth["length"] extends maxDepth ? o : o extends Primitive ? o : o extends Fn ? o : {
    -readonly [k in keyof o]: _mutable<o[k], [
        ...depth,
        1
    ], maxDepth>;
};
type Entry<key extends PropertyKey = PropertyKey, value = unknown> = readonly [
    key: key,
    value: value
];
type unionKeyOf<t> = t extends unknown ? keyof t : never;
type requiredKeyOf<o> = keyof o extends infer k ? k extends keyof o ? o extends {
    [_ in k]-?: o[k];
} ? k : never : never : never;
type optionalKeyOf<o> = Exclude<keyof o, requiredKeyOf<o>>;
type merge<base, props> = base extends unknown ? props extends unknown ? keyof base & keyof props extends never ? show<base & props> : show<omit<base, keyof props & keyof base> & props> : never : never;
type propValueOf<o> = o[keyof o];
declare class Covariant<t> {
    private " covariant"?;
}
interface DynamicBase<t extends object> extends t, Covariant<t> {
}
declare class DynamicBase<t extends object> {
    constructor(properties: t);
}
declare const NoopBase: new <t extends object>() => t;
declare class CastableBase<t extends object> extends NoopBase<t> {
    private " covariant"?;
}
type pick<o, key extends keyof o> = o extends unknown ? {
    [k in keyof o as k extends key ? k : never]: o[k];
} : never;
declare const pick: <o extends object, keys extends keySetOf<o>>(o: o, keys: keys) => pick<o, keyof keys & keyof o>;
type omit<o, key extends keyof o> = {
    [k in keyof o as k extends key ? never : k]: o[k];
};
declare const omit: <o extends object, keys extends keySetOf<o>>(o: o, keys: keys) => omit<o, keyof keys & keyof o>;
type ifEmptyObjectLiteral<t, onTrue = true, onFalse = false> = [
    unknown,
    t & (null | undefined)
] extends [
    t | null | undefined,
    never
] ? onTrue : onFalse;
type EmptyObject = Record<PropertyKey, never>;
declare const unset: " unset​";
type unset = typeof unset;
declare const noSuggest: <s extends string>(s: s) => noSuggest<s>;
type noSuggest<s extends string = string> = ` ${s}`;
declare const ZeroWidthSpace = "\u200B";
type ZeroWidthSpace = typeof ZeroWidthSpace;
type ErrorMessage<message extends string = string> = `${message}${ZeroWidthSpace}`;
interface ErrorType<ctx extends {} = {}> extends CastableBase<ctx> {
    [brand]: "ErrorType";
}
type Completion<text extends string = string> = `${text}${ZeroWidthSpace}${ZeroWidthSpace}`;
type Stringifiable = string | boolean | number | bigint | null | undefined;
type show<t> = {
    [k in keyof t]: t[k];
} & unknown;
type get<t, k extends PropertyKey> = t[k & keyof t];
type andPreserveUnknown<l, r> = unknown extends l & r ? unknown : show<l & r>;
type anyOrNever = " anyOrNever";
type conform<t, base> = t extends base ? t : base;
type equals<l, r> = [
    l,
    r
] extends [
    r,
    l
] ? true : false;
declare const brand: " brand";
type Brand<t = unknown, id = unknown> = t & {
    readonly [brand]: [
        t,
        id
    ];
};
type satisfy<base, t extends base> = t;
type defined<t> = t & ({} | null);
type autocomplete<suggestions extends string> = suggestions | (string & {});
declare const inferred: " arkInferred";
type inferred = typeof inferred;
declare const args: " args";
type args = typeof args;
declare abstract class Hkt<constraints extends unknown[] = any> {
    [args]: unknown[];
    constraints: constraints;
    args: this[args] extends infer args extends unknown[] ? args : never;
    0: this[args] extends [
        infer arg,
        ...any
    ] ? arg : never;
    1: this[args] extends [
        any,
        infer arg,
        ...any
    ] ? arg : never;
    2: this[args] extends [
        any,
        any,
        infer arg,
        ...any
    ] ? arg : never;
    3: this[args] extends [
        any,
        any,
        any,
        infer arg,
        ...any
    ] ? arg : never;
    abstract body: unknown;
    description?: string;
    constructor();
}
declare namespace Hkt {
    type constructor<constraints extends unknown[] = any> = new () => Hkt<constraints>;
    type args = typeof args;
    type apply<hkt extends Hkt, args extends {
        [i in keyof args]: hkt["constraints"][i];
    }> = (hkt & {
        [args]: args;
    })["body"];
}
interface AndPreserveUnknown extends Hkt<[
    unknown,
    unknown
]> {
    body: andPreserveUnknown<this[0], this[1]>;
}
type SequenceIntersectionKind = "array" | "parameters";
type intersectArrays<l extends array, r extends array, operator extends Hkt = AndPreserveUnknown> = intersectSequences<l, r, [
], [
], operator, "array">;
type intersectSequences<l extends array, r extends array, acc extends array, postfix extends array, operation extends Hkt, kind extends SequenceIntersectionKind> = l extends readonly [
] ? kind extends "array" ? [
] extends r ? [
    ...acc,
    ...postfix
] : never : [
    ...acc,
    ...r,
    ...postfix
] : r extends readonly [
] ? kind extends "array" ? [
] extends l ? [
    ...acc,
    ...postfix
] : never : [
    ...acc,
    ...l,
    ...postfix
] : [
    l,
    r
] extends ([
    readonly [
        (infer lHead)?,
        ...infer lTail
    ],
    readonly [
        (infer rHead)?,
        ...infer rTail
    ]
]) ? [
    "0",
    lHead,
    rHead
] extends [
    keyof l | keyof r,
    l[0],
    r[0]
] ? intersectSequences<lTail, rTail, [
    [
    ],
    [
    ]
] extends [
    l,
    r
] ? [
    ...acc,
    Hkt.apply<operation, [
        lHead,
        rHead
    ]>?
] : [
    ...acc,
    Hkt.apply<operation, [
        lHead,
        rHead
    ]>
], postfix, operation, kind> : l extends readonly [
    ...infer lInit,
    infer lLast
] ? r extends readonly [
    ...infer rInit,
    infer rLast
] ? intersectSequences<lInit, rInit, acc, [
    Hkt.apply<operation, [
        lLast,
        rLast
    ]>,
    ...postfix
], operation, kind> : intersectSequences<lInit, r, acc, [
    Hkt.apply<operation, [
        lLast,
        r[number]
    ]>,
    ...postfix
], operation, kind> : r extends readonly [
    ...infer rInit,
    infer rLast
] ? intersectSequences<l, rInit, acc, [
    Hkt.apply<operation, [
        l[number],
        rLast
    ]>,
    ...postfix
], operation, kind> : [
    ...acc,
    ...Hkt.apply<operation, [
        lHead,
        rHead
    ]>[],
    ...postfix
] : never;
type isDisjoint<l, r> = overlaps<l, r> extends true ? false : true;
type overlaps<l, r> = l & r extends never ? false : domainOf<l> & domainOf<r> extends never ? false : [
    l,
    r
] extends [
    object,
    object
] ? false extends (propValueOf<{
    [k in Extract<keyof l & keyof r, requiredKeyOf<l> | requiredKeyOf<r>>]: overlaps<l[k], r[k]>;
}>) ? false : true : true;
declare const join: <segments extends array<string>, delimiter extends string>(segments: segments, delimiter: delimiter) => join<segments, delimiter>;
type join<segments extends array<string>, delimiter extends string, result extends string = ""> = segments extends (readonly [
    infer head extends string,
    ...infer tail extends string[]
]) ? join<tail, delimiter, result extends "" ? head : `${result}${delimiter}${head}`> : result;
type array<t = unknown> = readonly t[];
declare namespace array {
    type multiply<t extends array, count extends number> = _multiply<t, [
    ], count, [
    ]>;
    type _multiply<base extends array, result extends array, count extends number, i extends 1[]> = i["length"] extends count ? result : _multiply<base, [
        ...result,
        ...base
    ], count, [
        ...i,
        1
    ]>;
    type repeat<element, count extends number> = buildFromSegments<element, [
    ], exponentials.max<count>, count>;
    type buildFromSegments<element, result extends 1[], segments extends 1[][], count extends number, next extends 1[] = [
        ...result,
        ...segments[0]
    ]> = next["length"] extends count ? {
        [i in keyof next]: element;
    } : `${count}` extends keyof next ? buildFromSegments<element, result, nextSegments<segments>, count> : buildFromSegments<element, next, nextSegments<segments>, count>;
    type nextSegments<segments extends 1[][]> = segments extends [
        unknown,
        ...infer nextSegments extends 1[][]
    ] ? nextSegments : never;
    type minLength<element, minLength extends number> = readonly [
        ...multiply<[
            element
        ], minLength>,
        ...element[]
    ];
}
type listable<t> = t | readonly t[];
type flattenListable<t> = t extends array<infer element> ? element : t;
type tailOf<t extends array> = t extends readonly [
    unknown,
    ...infer tail
] ? tail : never;
type lastIndexOf<t extends array> = tailOf<t>["length"];
type lastOf<t extends array> = t[lastIndexOf<t>];
type numericStringKeyOf<t extends array> = Extract<keyof t, `${number}`>;
type liftArray<t> = t extends array ? [
    t
] extends [
    anyOrNever
] ? t[] : t : t[];
declare const liftArray: <t>(data: t) => liftArray<t>;
declare const ReadonlyArray$1: new <T>(...args: ConstructorParameters<typeof Array<T>>) => ReadonlyArray<T>;
type applyElementLabels<t extends readonly unknown[], labels extends readonly unknown[]> = labels extends [
    unknown,
    ...infer labelsTail
] ? t extends readonly [
    infer head,
    ...infer tail
] ? readonly [
    ...labelElement<head, labels>,
    ...applyElementLabels<tail, labelsTail>
] : applyOptionalElementLabels<Required<t>, labels> : t;
type applyOptionalElementLabels<t extends readonly unknown[], labels extends readonly unknown[]> = labels extends readonly [
    unknown,
    ...infer labelsTail
] ? t extends readonly [
    infer head,
    ...infer tail
] ? [
    ...labelOptionalElement<head, labels>,
    ...applyOptionalElementLabels<tail, labelsTail>
] : applyRestElementLabels<t, labels> : t;
type applyRestElementLabels<t extends readonly unknown[], labels extends readonly unknown[]> = t extends readonly [
] ? [
] : labels extends readonly [
    unknown,
    ...infer tail
] ? [
    ...labelOptionalElement<t[0], labels>,
    ...applyRestElementLabels<t, tail>
] : t;
type labelElement<element, labels extends readonly unknown[]> = labels extends readonly [
    unknown
] ? {
    [K in keyof labels]: element;
} : labels extends readonly [
    ...infer head,
    unknown
] ? labelElement<element, head> : [
    _: element
];
type labelOptionalElement<element, label extends readonly unknown[]> = label extends readonly [
    unknown
] ? {
    [K in keyof label]?: element;
} : label extends readonly [
    ...infer head,
    unknown
] ? labelOptionalElement<element, head> : [
    _?: element
];
type setIndex<arr extends readonly unknown[], i extends number, to extends arr[number]> = arr extends arr[number][] ? _setIndex<arr, i, to, [
]> : Readonly<_setIndex<arr, i, to, [
]>>;
type _setIndex<arr extends readonly unknown[], i extends number, to extends arr[number], result extends arr[number][]> = arr extends readonly [
    infer head,
    ...infer tail
] ? _setIndex<tail, i, to, [
    ...result,
    result["length"] extends i ? to : head
]> : result;
type zero = [
];
type one = [
    1
];
type two = [
    1,
    1
];
type three = [
    ...two,
    ...two
];
type four = [
    ...three,
    ...three
];
type five = [
    ...four,
    ...four
];
type six = [
    ...five,
    ...five
];
type seven = [
    ...six,
    ...six
];
type eight = [
    ...seven,
    ...seven
];
type nine = [
    ...eight,
    ...eight
];
type ten = [
    ...nine,
    ...nine
];
type eleven = [
    ...ten,
    ...ten
];
type twelve = [
    ...eleven,
    ...eleven
];
type thirteen = [
    ...twelve,
    ...twelve
];
type fourteen = [
    ...thirteen,
    ...thirteen
];
type exponentials = [
    fourteen,
    thirteen,
    twelve,
    eleven,
    ten,
    nine,
    eight,
    seven,
    six,
    five,
    four,
    three,
    two,
    one,
    zero
];
declare namespace exponentials {
    type max<n extends number> = _max<n, exponentials>;
    type _max<n extends number, filtered extends unknown[]> = `${n}` extends keyof filtered[0] ? _max<n, tailOf<filtered>> : filtered;
}
type JsonStructure = JsonObject | JsonArray;
interface JsonObject {
    [k: string]: Json;
}
type JsonArray = Json[];
type JsonPrimitive = string | boolean | number | null;
type Json = JsonStructure | JsonPrimitive;
declare class ReadonlyPath extends ReadonlyArray$1<PropertyKey> {
    private cache;
    constructor(...items: array<PropertyKey>);
    toJSON(): JsonArray;
    stringify(): string;
    stringifyAncestors(): readonly string[];
}
type SerializedString<value extends string = string> = `"${value}"`;
type SerializedPrimitives = {
    string: SerializedString;
    number: `${number}`;
    bigint: BigintLiteral;
    boolean: "true" | "false";
    null: "null";
    undefined: "undefined";
};
type SerializedPrimitive = SerializedPrimitives[keyof SerializedPrimitives];
declare const initialRegistryContents: {
    version: string;
    filename: string;
    FileConstructor: typeof File;
};
type InitialRegistryContents = typeof initialRegistryContents;
interface ArkRegistry extends InitialRegistryContents {
    [k: string]: unknown;
}
declare global {
    export interface ArkEnv {
        prototypes(): never;
    }
    export namespace ArkEnv {
        type prototypes = ReturnType<ArkEnv["prototypes"]>;
    }
}
type contains<s extends string, sub extends string> = s extends `${string}${sub}${string}` ? true : false;
declare const Backslash = "\\";
type Backslash = typeof Backslash;
declare const whitespaceChars: {
    readonly " ": 1;
    readonly "\n": 1;
    readonly "\t": 1;
};
type WhitespaceChar = keyof typeof whitespaceChars;
type trim$1<s extends string> = trimEnd<trimStart<s>>;
type trimStart<s extends string> = s extends `${WhitespaceChar}${infer tail}` ? trimEnd<tail> : s;
type trimEnd<s extends string> = s extends `${infer init}${WhitespaceChar}` ? trimEnd<init> : s;
declare class Scanner<lookahead extends string = string> {
    chars: string[];
    i: number;
    def: string;
    constructor(def: string);
    shift(): this["lookahead"];
    get lookahead(): lookahead;
    get nextLookahead(): string;
    get length(): number;
    shiftUntil(condition: Scanner.UntilCondition): string;
    shiftUntilEscapable(condition: Scanner.UntilCondition, escapeEscape?: typeof Backslash | ""): string;
    shiftUntilLookahead(charOrSet: string | KeySet): string;
    shiftUntilNonWhitespace(): string;
    jumpToIndex(i: number): void;
    jumpForward(count: number): void;
    get location(): number;
    get unscanned(): string;
    get scanned(): string;
    sliceChars(start: number, end?: number): string;
    lookaheadIs<char extends lookahead>(char: char): this is Scanner<char>;
    lookaheadIsIn<keySet extends KeySet>(tokens: keySet): this is Scanner<Extract<keyof keySet, string>>;
}
declare namespace Scanner {
    type UntilCondition = (scanner: Scanner, shifted: string) => boolean;
    type shift<lookahead extends string, unscanned extends string> = `${lookahead}${unscanned}`;
    type shiftUntil<unscanned extends string, terminator extends string, appendTo extends string = ""> = unscanned extends shift<infer lookahead, infer nextUnscanned> ? lookahead extends terminator ? [
        appendTo,
        unscanned
    ] : shiftUntil<nextUnscanned, terminator, `${appendTo}${lookahead}`> : [
        appendTo,
        ""
    ];
    type shiftUntilEscapable<unscanned extends string, terminator extends string, escapeEscape extends Backslash | "", appendTo extends string = ""> = unscanned extends shift<infer lookahead, infer nextUnscanned> ? lookahead extends terminator ? [
        appendTo,
        unscanned
    ] : lookahead extends Backslash ? nextUnscanned extends (shift<infer nextLookahead, infer postEscapedUnscanned>) ? shiftUntilEscapable<postEscapedUnscanned, terminator, escapeEscape, `${appendTo}${nextLookahead extends terminator ? "" : nextLookahead extends Backslash ? escapeEscape : Backslash}${nextLookahead}`> : [
        `${appendTo}${Backslash}`,
        ""
    ] : shiftUntilEscapable<nextUnscanned, terminator, escapeEscape, `${appendTo}${lookahead}`> : [
        appendTo,
        ""
    ];
    type shiftUntilNot<unscanned extends string, nonTerminator extends string, appendTo extends string = ""> = unscanned extends shift<infer lookahead, infer nextUnscanned> ? lookahead extends nonTerminator ? shiftUntilNot<nextUnscanned, nonTerminator, `${appendTo}${lookahead}`> : [
        appendTo,
        unscanned
    ] : [
        appendTo,
        ""
    ];
    type skipWhitespace<unscanned extends string> = shiftUntilNot<unscanned, WhitespaceChar>[1];
    type shiftResult<scanned extends string, unscanned extends string> = [
        scanned,
        unscanned
    ];
}
declare const writeUnmatchedGroupCloseMessage: <char extends string, unscanned extends string>(char: char, unscanned: unscanned) => writeUnmatchedGroupCloseMessage<char, unscanned>;
type writeUnmatchedGroupCloseMessage<char extends string, unscanned extends string> = `Unmatched ${char}${unscanned extends "" ? "" : ` before ${unscanned}`}`;
declare const writeUnclosedGroupMessage: <missingChar extends string>(missingChar: missingChar) => writeUnclosedGroupMessage<missingChar>;
type writeUnclosedGroupMessage<missingChar extends string> = `Missing ${missingChar}`;
declare const intrinsic: {
    emptyStructure: StructureNode;
    jsonPrimitive: BaseRoot<InternalRootDeclaration>;
    jsonObject: BaseRoot<InternalRootDeclaration>;
    jsonData: BaseRoot<InternalRootDeclaration>;
    integer: BaseRoot<InternalRootDeclaration>;
    lengthBoundable: BaseRoot<InternalRootDeclaration>;
    key: BaseRoot<InternalRootDeclaration>;
    nonNegativeIntegerString: BaseRoot<InternalRootDeclaration>;
    string: BaseRoot<InternalRootDeclaration>;
    number: BaseRoot<InternalRootDeclaration>;
    bigint: BaseRoot<InternalRootDeclaration>;
    boolean: BaseRoot<InternalRootDeclaration>;
    symbol: BaseRoot<InternalRootDeclaration>;
    undefined: BaseRoot<InternalRootDeclaration>;
    object: BaseRoot<InternalRootDeclaration>;
    null: BaseRoot<InternalRootDeclaration>;
    Array: BaseRoot<InternalRootDeclaration>;
    Date: BaseRoot<InternalRootDeclaration>;
    false: BaseRoot<InternalRootDeclaration>;
    never: BaseRoot<InternalRootDeclaration>;
    true: BaseRoot<InternalRootDeclaration>;
    unknown: BaseRoot<InternalRootDeclaration>;
};
type JsonSchemaOrBoolean = listable<JsonSchema.Branch>;
type JsonSchema = JsonSchema.NonBooleanBranch;
declare namespace JsonSchema {
    type TypeName = "string" | "integer" | "number" | "object" | "array" | "boolean" | "null";
    interface Meta<t = unknown> extends UniversalMeta<t> {
        $schema?: string;
        $defs?: Record<string, JsonSchema>;
    }
    type Format = autocomplete<"date-time" | "date" | "time" | "email" | "ipv4" | "ipv6" | "uri" | "uuid" | "regex">;
    interface UniversalMeta<t = unknown> {
        title?: string;
        description?: string;
        format?: Format;
        deprecated?: true;
        default?: t;
        examples?: readonly t[];
    }
    type Composition = Union | OneOf | Intersection | Not;
    type NonBooleanBranch = Constrainable | Const | Composition | Enum | String | Numeric | Object | Array | Ref;
    type Branch = boolean | JsonSchema;
    type RefString = `#/$defs/${string}`;
    interface Ref extends Meta {
        $ref: RefString;
        type?: never;
    }
    interface Constrainable extends Meta {
        type?: listable<TypeName>;
    }
    interface Intersection extends Meta {
        allOf: readonly JsonSchema[];
    }
    interface Not extends Meta {
        not: JsonSchema;
    }
    interface OneOf extends Meta {
        oneOf: readonly JsonSchema[];
    }
    interface Union extends Meta {
        anyOf: readonly JsonSchema[];
    }
    interface Const extends Meta {
        const: unknown;
    }
    interface Enum extends Meta {
        enum: array;
    }
    interface String extends Meta<string> {
        type: "string";
        minLength?: number;
        maxLength?: number;
        pattern?: string;
        format?: string;
    }
    interface Numeric extends Meta<number> {
        type: "number" | "integer";
        multipleOf?: number;
        minimum?: number;
        exclusiveMinimum?: number;
        maximum?: number;
        exclusiveMaximum?: number;
    }
    interface Object extends Meta<JsonObject> {
        type: "object";
        properties?: Record<string, JsonSchema>;
        required?: string[];
        patternProperties?: Record<string, JsonSchema>;
        additionalProperties?: JsonSchemaOrBoolean;
        maxProperties?: number;
        minProperties?: number;
        propertyNames?: String;
    }
    interface Array extends Meta<JsonArray> {
        type: "array";
        additionalItems?: JsonSchemaOrBoolean;
        contains?: JsonSchemaOrBoolean;
        uniqueItems?: boolean;
        minItems?: number;
        maxItems?: number;
        items?: JsonSchemaOrBoolean;
        prefixItems?: readonly Branch[];
    }
    type LengthBoundable = String | Array;
    type Structure = Object | Array;
}
declare class DivisorNode extends InternalPrimitiveConstraint<Divisor.Declaration> {
    traverseAllows: TraverseAllows<number>;
    readonly compiledCondition: string;
    readonly compiledNegation: string;
    readonly impliedBasis: BaseRoot;
    readonly expression: string;
    reduceJsonSchema(schema: JsonSchema.Numeric): JsonSchema.Numeric;
}
declare namespace Divisor {
    interface Inner {
        readonly rule: number;
    }
    interface NormalizedSchema extends BaseNormalizedSchema {
        readonly rule: number;
    }
    type Schema = NormalizedSchema | number;
    interface ErrorContext extends BaseErrorContext<"divisor">, Inner {
    }
    interface Declaration extends declareNode<{
        kind: "divisor";
        schema: Schema;
        normalizedSchema: NormalizedSchema;
        inner: Inner;
        prerequisite: number;
        errorContext: ErrorContext;
    }> {
    }
    type Node = DivisorNode;
}
declare const Divisor: {
    implementation: nodeImplementationOf<Divisor.Declaration>;
    Node: typeof DivisorNode;
};
declare const writeIndivisibleMessage: (t: BaseRoot) => string;
type writeIndivisibleMessage<actual> = writeInvalidOperandMessage<"divisor", actual>;
interface BaseRangeDeclaration extends BaseNodeDeclaration {
    kind: RangeKind;
    inner: BaseRangeInner;
    normalizedSchema: UnknownExpandedRangeSchema;
}
declare abstract class BaseRange<d extends BaseRangeDeclaration> extends InternalPrimitiveConstraint<d> {
    readonly exclusive?: true;
    readonly boundOperandKind: OperandKindsByBoundKind[d["kind"]];
    readonly compiledActual: string;
    readonly comparator: RelativeComparator;
    readonly numericLimit: number;
    readonly expression: string;
    readonly compiledCondition: string;
    readonly compiledNegation: string;
    readonly stringLimit: string;
    readonly limitKind: LimitKind;
    isStricterThan(r: nodeOfKind<d["kind"] | pairedRangeKind<d["kind"]>>): boolean;
    overlapsRange(r: nodeOfKind<pairedRangeKind<d["kind"]>>): boolean;
    overlapIsUnit(r: nodeOfKind<pairedRangeKind<d["kind"]>>): boolean;
}
interface BaseRangeInner {
    readonly rule: LimitValue;
}
type LimitValue = Date | number;
type LimitSchemaValue = Date | number | string;
interface UnknownExpandedRangeSchema extends BaseNormalizedSchema {
    readonly rule: LimitSchemaValue;
    readonly exclusive?: boolean;
}
interface UnknownNormalizedRangeSchema extends BaseNormalizedSchema {
    readonly rule: LimitSchemaValue;
}
interface ExclusiveExpandedDateRangeSchema extends BaseNormalizedSchema {
    rule: LimitSchemaValue;
    exclusive?: true;
}
type ExclusiveDateRangeSchema = LimitSchemaValue | ExclusiveExpandedDateRangeSchema;
interface InclusiveExpandedDateRangeSchema extends BaseNormalizedSchema {
    rule: LimitSchemaValue;
    exclusive?: false;
}
type InclusiveDateRangeSchema = LimitSchemaValue | InclusiveExpandedDateRangeSchema;
interface ExclusiveNormalizedNumericRangeSchema extends BaseNormalizedSchema {
    rule: number;
    exclusive?: true;
}
type ExclusiveNumericRangeSchema = number | ExclusiveNormalizedNumericRangeSchema;
interface InclusiveNormalizedNumericRangeSchema extends BaseNormalizedSchema {
    rule: number;
    exclusive?: false;
}
type InclusiveNumericRangeSchema = number | InclusiveNormalizedNumericRangeSchema;
type LimitKind = "lower" | "upper";
type RelativeComparator<kind extends LimitKind = LimitKind> = {
    lower: ">" | ">=";
    upper: "<" | "<=";
}[kind];
declare const boundKindPairsByLower: BoundKindPairsByLower;
type BoundKindPairsByLower = {
    min: "max";
    minLength: "maxLength";
    after: "before";
};
type BoundKindPairsByUpper = {
    max: "min";
    maxLength: "minLength";
    before: "after";
};
type pairedRangeKind<kind extends RangeKind> = kind extends LowerBoundKind ? BoundKindPairsByLower[kind] : BoundKindPairsByUpper[kind & UpperBoundKind];
type LowerBoundKind = keyof typeof boundKindPairsByLower;
type UpperBoundKind = propValueOf<typeof boundKindPairsByLower>;
type OperandKindsByBoundKind = satisfy<Record<RangeKind, BoundOperandKind>, {
    min: "value";
    max: "value";
    minLength: "length";
    maxLength: "length";
    after: "date";
    before: "date";
}>;
type BoundOperandKind = "value" | "length" | "date";
type LengthBoundableData = string | array;
declare const writeUnboundableMessage: <root extends string>(root: root) => writeUnboundableMessage<root>;
type writeUnboundableMessage<root extends string> = `Bounded expression ${root} must be exactly one of number, string, Array, or Date`;
declare class ExactLengthNode extends InternalPrimitiveConstraint<ExactLength.Declaration> {
    traverseAllows: TraverseAllows<LengthBoundableData>;
    readonly compiledCondition: string;
    readonly compiledNegation: string;
    readonly impliedBasis: BaseRoot;
    readonly expression: string;
    reduceJsonSchema(schema: JsonSchema.LengthBoundable): JsonSchema.LengthBoundable;
}
declare namespace ExactLength {
    interface Inner {
        readonly rule: number;
    }
    interface NormalizedSchema extends BaseNormalizedSchema {
        readonly rule: number;
    }
    type Schema = NormalizedSchema | number;
    interface ErrorContext extends BaseErrorContext<"exactLength">, Inner {
    }
    type Declaration = declareNode<{
        kind: "exactLength";
        schema: Schema;
        normalizedSchema: NormalizedSchema;
        inner: Inner;
        prerequisite: LengthBoundableData;
        errorContext: ErrorContext;
    }>;
    type Node = ExactLengthNode;
}
declare const ExactLength: {
    implementation: nodeImplementationOf<{
        intersectionIsOpen: false;
        childKind: never;
        reducibleTo: "exactLength";
        kind: "exactLength";
        schema: ExactLength.Schema;
        normalizedSchema: ExactLength.NormalizedSchema;
        inner: ExactLength.Inner;
        prerequisite: LengthBoundableData;
        errorContext: ExactLength.ErrorContext;
    }>;
    Node: typeof ExactLengthNode;
};
interface StandardTypedV1<Input = unknown, Output = Input> {
    readonly "~standard": StandardTypedV1.Props<Input, Output>;
}
declare namespace StandardTypedV1 {
    interface Props<Input = unknown, Output = Input> {
        readonly version: 1;
        readonly vendor: string;
        readonly types?: Types<Input, Output> | undefined;
    }
    interface Types<Input = unknown, Output = Input> {
        readonly input: Input;
        readonly output: Output;
    }
    type InferInput<Schema extends StandardTypedV1> = NonNullable<Schema["~standard"]["types"]>["input"];
    type InferOutput<Schema extends StandardTypedV1> = NonNullable<Schema["~standard"]["types"]>["output"];
}
interface StandardSchemaV1<Input = unknown, Output = Input> {
    readonly "~standard": StandardSchemaV1.Props<Input, Output>;
}
declare namespace StandardSchemaV1 {
    interface Props<Input = unknown, Output = Input> extends StandardTypedV1.Props<Input, Output> {
        readonly validate: (value: unknown, options?: StandardSchemaV1.Options | undefined) => Result<Output> | Promise<Result<Output>>;
    }
    type Result<Output> = SuccessResult<Output> | FailureResult;
    interface SuccessResult<Output> {
        readonly value: Output;
        readonly issues?: undefined;
    }
    interface Options {
        readonly libraryOptions?: Record<string, unknown> | undefined;
    }
    interface FailureResult {
        readonly issues: ReadonlyArray<Issue>;
    }
    interface Issue {
        readonly message: string;
        readonly path?: ReadonlyArray<PropertyKey | PathSegment> | undefined;
    }
    interface PathSegment {
        readonly key: PropertyKey;
    }
    interface Types<Input = unknown, Output = Input> extends StandardTypedV1.Types<Input, Output> {
    }
    type InferInput<Schema extends StandardTypedV1> = StandardTypedV1.InferInput<Schema>;
    type InferOutput<Schema extends StandardTypedV1> = StandardTypedV1.InferOutput<Schema>;
    interface ArkTypeProps<Input = unknown, Output = Input> extends Props<Input, Output>, StandardJSONSchemaV1.Props<Input, Output> {
        vendor: "arktype";
    }
}
interface StandardJSONSchemaV1<Input = unknown, Output = Input> {
    readonly "~standard": StandardJSONSchemaV1.Props<Input, Output>;
}
declare namespace StandardJSONSchemaV1 {
    interface Props<Input = unknown, Output = Input> extends StandardTypedV1.Props<Input, Output> {
        readonly jsonSchema: StandardJSONSchemaV1.Converter;
    }
    interface Converter {
        readonly input: (options: StandardJSONSchemaV1.Options) => Record<string, unknown>;
        readonly output: (options: StandardJSONSchemaV1.Options) => Record<string, unknown>;
    }
    type Target = "draft-2020-12" | "draft-07" | "openapi-3.0" | ({} & string);
    interface Options {
        readonly target: Target;
        readonly libraryOptions?: Record<string, unknown> | undefined;
    }
    interface Types<Input = unknown, Output = Input> extends StandardTypedV1.Types<Input, Output> {
    }
    type InferInput<Schema extends StandardTypedV1> = StandardTypedV1.InferInput<Schema>;
    type InferOutput<Schema extends StandardTypedV1> = StandardTypedV1.InferOutput<Schema>;
}
declare class ToJsonSchemaError<code extends ToJsonSchema.Code = ToJsonSchema.Code> extends Error {
    readonly name = "ToJsonSchemaError";
    readonly code: code;
    readonly context: ToJsonSchema.ContextByCode[code];
    constructor(code: code, context: ToJsonSchema.ContextByCode[code]);
    hasCode<code extends ToJsonSchema.Code>(code: code): this is ToJsonSchemaError<code>;
}
declare const ToJsonSchema: {
    Error: typeof ToJsonSchemaError;
    throw: (code: keyof ToJsonSchema.ContextByCode, context: ToJsonSchema.MorphContext | ToJsonSchema.UnitContext | ToJsonSchema.ProtoContext | ToJsonSchema.DomainContext | ToJsonSchema.PredicateContext | ToJsonSchema.DateContext | ToJsonSchema.ArrayObjectContext | ToJsonSchema.ArrayPostfixContext | ToJsonSchema.DefaultValueContext | ToJsonSchema.PatternIntersectionContext | ToJsonSchema.SymbolKeyContext) => never;
    throwInternalOperandError: (kind: ConstraintKind, schema: JsonSchema) => never;
    defaultConfig: ToJsonSchema.Context;
};
declare namespace ToJsonSchema {
    type Unjsonifiable = object | symbol | bigint | undefined;
    type Error = InstanceType<typeof ToJsonSchema.Error>;
    interface BaseContext<code extends Code, base extends JsonSchema = JsonSchema> {
        code: code;
        base: base;
    }
    interface ArrayObjectContext extends BaseContext<"arrayObject", JsonSchema.Array> {
        object: JsonSchema.Object;
    }
    interface ArrayPostfixContext extends BaseContext<"arrayPostfix", VariadicArraySchema> {
        elements: readonly JsonSchema[];
    }
    interface DefaultValueContext extends BaseContext<"defaultValue", JsonSchema> {
        value: Unjsonifiable;
    }
    interface DomainContext extends BaseContext<"domain", JsonSchema> {
        domain: satisfy<Domain$1, "symbol" | "bigint" | "undefined">;
    }
    interface MorphContext extends BaseContext<"morph", JsonSchema> {
        out: JsonSchema | null;
    }
    interface PatternIntersectionContext extends BaseContext<"patternIntersection", StringSchemaWithPattern> {
        pattern: string;
    }
    interface PredicateContext extends BaseContext<"predicate", JsonSchema> {
        predicate: Predicate;
    }
    interface ProtoContext extends BaseContext<"proto", JsonSchema> {
        proto: Constructor;
    }
    type SymbolKeyContext = IndexSymbolKeyContext | RequiredSymbolKeyContext | OptionalSymbolKeyContext;
    interface IndexSymbolKeyContext extends BaseContext<"symbolKey", JsonSchema.Object> {
        key: null;
        value: JsonSchema;
        optional: false;
    }
    interface RequiredSymbolKeyContext extends BaseContext<"symbolKey", JsonSchema.Object> {
        key: symbol;
        value: JsonSchema;
        optional: false;
    }
    interface OptionalSymbolKeyContext extends BaseContext<"symbolKey", JsonSchema.Object> {
        key: symbol;
        value: JsonSchema;
        optional: true;
        default?: Json;
    }
    interface UnitContext extends BaseContext<"unit", JsonSchema> {
        unit: Unjsonifiable;
    }
    interface DateContext extends BaseContext<"date", JsonSchema> {
        before?: Date;
        after?: Date;
    }
    interface ContextByCode {
        arrayObject: ArrayObjectContext;
        arrayPostfix: ArrayPostfixContext;
        defaultValue: DefaultValueContext;
        domain: DomainContext;
        morph: MorphContext;
        patternIntersection: PatternIntersectionContext;
        predicate: PredicateContext;
        proto: ProtoContext;
        symbolKey: SymbolKeyContext;
        unit: UnitContext;
        date: DateContext;
    }
    type Code = keyof ContextByCode;
    type FallbackContext = ContextByCode[Code];
    type HandlerByCode = satisfy<{
        [code in Code]: (ctx: ContextByCode[code]) => unknown;
    }, {
        arrayObject: (ctx: ArrayObjectContext) => JsonSchema.Structure;
        arrayPostfix: (ctx: ArrayPostfixContext) => VariadicArraySchema;
        defaultValue: (ctx: DefaultValueContext) => JsonSchema;
        domain: (ctx: DomainContext) => JsonSchema;
        morph: (ctx: MorphContext) => JsonSchema;
        patternIntersection: (ctx: PatternIntersectionContext) => JsonSchema.String;
        predicate: (ctx: PredicateContext) => JsonSchema;
        proto: (ctx: ProtoContext) => JsonSchema;
        symbolKey: (ctx: SymbolKeyContext) => JsonSchema.Object;
        unit: (ctx: UnitContext) => JsonSchema;
        date: (ctx: DateContext) => JsonSchema;
    }>;
    type VariadicArraySchema = requireKeys<JsonSchema.Array, "items">;
    type StringSchemaWithPattern = requireKeys<JsonSchema.String, "pattern">;
    type UniversalFallback = (ctx: FallbackContext) => JsonSchema;
    interface FallbackObject extends Partial<HandlerByCode> {
        default?: UniversalFallback;
    }
    type FallbackOption = UniversalFallback | FallbackObject;
    type Target = satisfy<StandardJSONSchemaV1.Target, "draft-2020-12" | "draft-07">;
    interface Options {
        dialect?: string | null;
        target?: Target;
        useRefs?: boolean;
        fallback?: FallbackOption;
    }
    interface Context extends Required<Options> {
        fallback: HandlerByCode;
    }
}
declare class PatternNode extends InternalPrimitiveConstraint<Pattern.Declaration> {
    readonly instance: RegExp;
    readonly expression: string;
    traverseAllows: (string: string) => boolean;
    readonly compiledCondition: string;
    readonly compiledNegation: string;
    readonly impliedBasis: BaseRoot;
    reduceJsonSchema(base: JsonSchema.String, ctx: ToJsonSchema.Context): JsonSchema.String;
}
declare namespace Pattern {
    interface NormalizedSchema extends BaseNormalizedSchema {
        readonly rule: string;
        readonly flags?: string;
    }
    interface Inner {
        readonly rule: string;
        readonly flags?: string;
    }
    type Schema = NormalizedSchema | string | RegExp;
    interface ErrorContext extends BaseErrorContext<"pattern">, Inner {
    }
    interface Declaration extends declareNode<{
        kind: "pattern";
        schema: Schema;
        normalizedSchema: NormalizedSchema;
        inner: Inner;
        intersectionIsOpen: true;
        prerequisite: string;
        errorContext: ErrorContext;
    }> {
    }
    type Node = PatternNode;
}
declare const Pattern: {
    implementation: nodeImplementationOf<Pattern.Declaration>;
    Node: typeof PatternNode;
};
type PreparsedNodeResolution = {
    [arkKind]: "generic" | "module";
};
declare class RootModule<exports extends {} = {}> extends DynamicBase<exports> {
    get [arkKind](): "module";
}
interface InternalModule<exports extends InternalResolutions = InternalResolutions> extends RootModule<exports> {
    root?: BaseRoot;
}
type exportSchemaScope<$> = {
    [k in keyof $]: instantiateRoot<$[k]>;
};
type instantiateRoot<t> = t extends InternalResolution ? [
    t
] extends [
    anyOrNever
] ? BaseRoot : t : BaseRoot;
declare const SchemaModule: new <$ = {}>(types: exportSchemaScope<$>) => SchemaModule<$>;
interface SchemaModule<$ = {}> extends RootModule<exportSchemaScope<$>> {
}
declare class ArkError<code extends ArkErrorCode = ArkErrorCode> extends CastableBase<ArkErrorContextInput<code>> {
    readonly [arkKind] = "error";
    path: ReadonlyPath;
    data: Prerequisite<code>;
    private nodeConfig;
    protected input: ArkErrorContextInput<code>;
    protected ctx: Traversal;
    constructor(input: ArkErrorContextInput<code>, ctx: Traversal);
    transform(f: (input: ArkErrorContextInput<code>) => ArkErrorContextInput): ArkError;
    hasCode<code extends ArkErrorCode>(code: code): this is ArkError<code>;
    get propString(): string;
    get expected(): string;
    get actual(): string;
    get problem(): string;
    get message(): string;
    get flat(): ArkError[];
    toJSON(): JsonObject;
    toString(): string;
    throw(): never;
}
declare namespace ArkErrors {
    type Handler<returns = unknown> = (errors: ArkErrors) => returns;
}
declare class ArkErrors extends ReadonlyArray$1<ArkError> implements StandardSchemaV1.FailureResult {
    readonly [arkKind] = "errors";
    static get [Symbol.species](): ArrayConstructor;
    protected ctx: Traversal;
    constructor(ctx: Traversal);
    byPath: Record<string, ArkError>;
    get flatByPath(): Record<string, ArkError[]>;
    get flatProblemsByPath(): Record<string, string[]>;
    byAncestorPath: Record<string, ArkError[]>;
    count: number;
    private mutable;
    throw(): never;
    toTraversalError(): TraversalError;
    add(error: ArkError): void;
    transform(f: (e: ArkError) => ArkError): ArkErrors;
    merge(errors: ArkErrors): void;
    affectsPath(path: ReadonlyPath): boolean;
    get summary(): string;
    get issues(): this;
    toJSON(): JsonArray;
    toString(): string;
    private addAncestorPaths;
}
declare class TraversalError extends Error {
    readonly name = "TraversalError";
    arkErrors: ArkErrors;
    constructor(errors: ArkErrors);
}
interface DerivableErrorContext<code extends ArkErrorCode = ArkErrorCode> {
    expected: string;
    actual: string;
    problem: string;
    message: string;
    data: Prerequisite<code>;
    path: array<PropertyKey>;
    propString: string;
}
type DerivableErrorContextInput<code extends ArkErrorCode = ArkErrorCode> = Partial<DerivableErrorContext<code>> & propwiseXor<{
    path?: array<PropertyKey>;
}, {
    relativePath?: array<PropertyKey>;
    prefixPath?: array<PropertyKey>;
}>;
type ArkErrorCode = {
    [kind in NodeKind]: errorContext<kind> extends null ? never : kind;
}[NodeKind];
type ArkErrorContextInputsByCode = {
    [code in ArkErrorCode]: errorContext<code> & DerivableErrorContextInput<code>;
};
type ArkErrorContextInput<code extends ArkErrorCode = ArkErrorCode> = merge<ArkErrorContextInputsByCode[code], {
    meta?: ArkEnv.meta;
}>;
type NodeErrorContextInput<code extends ArkErrorCode = ArkErrorCode> = ArkErrorContextInputsByCode[code] & {
    meta: ArkEnv.meta;
};
type MessageContext<code extends ArkErrorCode = ArkErrorCode> = Omit<ArkError<code>, "message">;
type ProblemContext<code extends ArkErrorCode = ArkErrorCode> = Omit<MessageContext<code>, "problem">;
type CustomErrorInput = show<{
    code?: undefined;
} & DerivableErrorContextInput>;
type ArkErrorInput = string | ArkErrorContextInput | CustomErrorInput;
type ProblemConfig<code extends ArkErrorCode = ArkErrorCode> = string | ProblemWriter<code>;
type ProblemWriter<code extends ArkErrorCode = ArkErrorCode> = (context: ProblemContext<code>) => string;
type MessageConfig<code extends ArkErrorCode = ArkErrorCode> = string | MessageWriter<code>;
type MessageWriter<code extends ArkErrorCode = ArkErrorCode> = (context: MessageContext<code>) => string;
type getAssociatedDataForError<code extends ArkErrorCode> = code extends NodeKind ? Prerequisite<code> : unknown;
type ExpectedConfig<code extends ArkErrorCode = ArkErrorCode> = string | ExpectedWriter<code>;
type ExpectedWriter<code extends ArkErrorCode = ArkErrorCode> = (source: errorContext<code>) => string;
type ActualConfig<code extends ArkErrorCode = ArkErrorCode> = string | ActualWriter<code>;
type ActualWriter<code extends ArkErrorCode = ArkErrorCode> = (data: getAssociatedDataForError<code>) => string;
declare const makeRootAndArrayPropertiesMutable: <o extends object>(o: o) => makeRootAndArrayPropertiesMutable<o>;
type makeRootAndArrayPropertiesMutable<inner> = {
    -readonly [k in keyof inner]: inner[k] extends array | undefined ? mutable<inner[k]> : inner[k];
} & unknown;
type arkKind = typeof arkKind;
declare const arkKind: " arkKind";
type unwrapDefault<thunkableValue> = thunkableValue extends Thunk<infer returnValue> ? returnValue : thunkableValue;
type GenericParamAst<name extends string = string, constraint = unknown> = [
    name: name,
    constraint: constraint
];
type GenericParamDef<name extends string = string> = name | readonly [
    name,
    unknown
];
type genericParamNames<params extends array<GenericParamAst>> = {
    [i in keyof params]: params[i][0];
};
type GenericArgResolutions<params extends array<GenericParamAst> = array<GenericParamAst>> = {
    [i in keyof params as params[i & `${number}`][0]]: BaseRoot;
};
declare class LazyGenericBody<argResolutions = {}, returns = unknown> extends Callable<(args: argResolutions) => returns> {
}
interface GenericAst<params extends array<GenericParamAst> = array<GenericParamAst>, bodyDef = unknown, $ = unknown, arg$ = $> {
    [arkKind]: "generic";
    paramsAst: params;
    bodyDef: bodyDef;
    $: $;
    arg$: arg$;
    names: genericParamNames<params>;
    t: this;
}
declare class GenericRoot<params extends array<GenericParamAst> = array<GenericParamAst>, bodyDef = unknown> extends Callable<(...args: {
    [i in keyof params]: BaseRoot;
}) => BaseRoot> {
    readonly [arkKind] = "generic";
    readonly paramsAst: params;
    readonly t: GenericAst<params, bodyDef, {}, {}>;
    paramDefs: array<GenericParamDef>;
    bodyDef: bodyDef;
    $: BaseScope;
    arg$: BaseScope;
    baseInstantiation: BaseRoot;
    hkt: Hkt.constructor | null;
    description: string;
    constructor(paramDefs: array<GenericParamDef>, bodyDef: bodyDef, $: BaseScope, arg$: BaseScope, hkt: Hkt.constructor | null);
    defIsLazy(): this is GenericRoot<params, LazyGenericBody>;
    protected cacheGetter<name extends keyof this>(name: name, value: this[name]): this[name];
    get json(): JsonStructure;
    get params(): {
        [i in keyof params]: [
            params[i][0],
            BaseRoot
        ];
    };
    get names(): genericParamNames<params>;
    get constraints(): {
        [i in keyof params]: BaseRoot;
    };
    get internal(): this;
    get referencesById(): Record<string, BaseNode>;
    get references(): BaseNode[];
}
type genericParamSchemasToAst<schemas extends readonly GenericParamDef[]> = {
    [i in keyof schemas]: schemas[i] extends GenericParamDef<infer name> ? [
        name,
        unknown
    ] : never;
};
type genericHktToConstraints<hkt extends abstract new () => Hkt> = InstanceType<hkt>["constraints"];
type GenericRootParser = <const paramsDef extends readonly GenericParamDef[]>(...params: paramsDef) => GenericRootBodyParser<genericParamSchemasToAst<paramsDef>>;
type GenericRootBodyParser<params extends array<GenericParamAst>> = {
    <const body>(body: RootSchema): GenericRoot<params, body>;
    <hkt extends Hkt.constructor>(instantiateDef: LazyGenericBody<GenericArgResolutions<params>>, hkt: hkt): GenericRoot<{
        [i in keyof params]: [
            params[i][0],
            genericHktToConstraints<hkt>[i]
        ];
    }, InstanceType<hkt>>;
};
declare const writeUnsatisfiedParameterConstraintMessage: <name extends string, constraint extends string, arg extends string>(name: name, constraint: constraint, arg: arg) => writeUnsatisfiedParameterConstraintMessage<name, constraint, arg>;
type writeUnsatisfiedParameterConstraintMessage<name extends string, constraint extends string, arg extends string> = `${name} must be assignable to ${constraint} (was ${arg})`;
declare class AliasNode extends BaseRoot<Alias.Declaration> {
    readonly expression: string;
    readonly structure: undefined;
    get resolution(): BaseRoot;
    protected _resolve(): BaseRoot;
    get resolutionId(): NodeId;
    get defaultShortDescription(): string;
    protected innerToJsonSchema(ctx: ToJsonSchema.Context): JsonSchema;
    traverseAllows: TraverseAllows;
    traverseApply: TraverseApply;
    compile(js: NodeCompiler): void;
}
declare namespace Alias {
    type Schema<alias extends string = string> = `$${alias}` | NormalizedSchema<alias>;
    interface NormalizedSchema<alias extends string = string> extends BaseNormalizedSchema {
        readonly reference: alias;
        readonly resolve?: () => BaseRoot;
    }
    interface Inner<alias extends string = string> {
        readonly reference: alias;
        readonly resolve?: () => BaseRoot;
    }
    interface Declaration extends declareNode<{
        kind: "alias";
        schema: Schema;
        normalizedSchema: NormalizedSchema;
        inner: Inner;
    }> {
    }
    type Node = AliasNode;
}
declare const Alias: {
    implementation: nodeImplementationOf<Alias.Declaration>;
    Node: typeof AliasNode;
};
declare const $ark: ArkSchemaRegistry;
type RegisteredReference<to extends string = string> = `$ark${"" | NonNegativeIntegerLiteral}.${to}`;
type InternalResolutions = Record<string, InternalResolution | undefined>;
type exportedNameOf<$> = Exclude<keyof $ & string, PrivateDeclaration>;
type resolvableReferenceIn<$> = {
    [k in keyof $]: k extends string ? k extends PrivateDeclaration<infer alias> ? alias : k extends noSuggest | "root" ? never : k : never;
}[keyof $];
type flatResolutionsOf<$> = show<intersectUnion<resolvableReferenceIn<$> extends infer k ? k extends keyof $ & string ? resolutionsOfReference<k, $[k]> : unknown : unknown>>;
type resolutionsOfReference<k extends string, v> = [
    v
] extends [
    {
        [arkKind]: "module";
    }
] ? [
    v
] extends [
    anyOrNever
] ? {
    [_ in k]: v;
} : prefixKeys<flatResolutionsOf<v>, k> & {
    [innerKey in keyof v as innerKey extends "root" ? k : never]: v[innerKey];
} : {
    [_ in k]: v;
};
type prefixKeys<o, prefix extends string> = {
    [k in keyof o & string as `${prefix}.${k}`]: o[k];
} & unknown;
type PrivateDeclaration<key extends string = string> = `#${key}`;
type InternalResolution = BaseRoot | GenericRoot | InternalModule;
type toInternalScope<$> = BaseScope<{
    [k in keyof $]: $[k] extends {
        [arkKind]: infer kind;
    } ? [
        $[k]
    ] extends [
        anyOrNever
    ] ? BaseRoot : kind extends "generic" ? GenericRoot : kind extends "module" ? InternalModule : never : BaseRoot;
}>;
type CachedResolution = NodeId | BaseRoot | GenericRoot;
declare const writeDuplicateAliasError: <alias extends string>(alias: alias) => writeDuplicateAliasError<alias>;
type writeDuplicateAliasError<alias extends string> = `#${alias} duplicates public alias ${alias}`;
type AliasDefEntry = [
    name: string,
    defValue: unknown
];
type GlobalOnlyConfigOptionName = satisfy<keyof ArkSchemaConfig, "dateAllowsInvalid" | "numberAllowsNaN" | "onUndeclaredKey" | "keywords">;
interface ScopeOnlyConfigOptions {
    name?: string;
    prereducedAliases?: boolean;
}
interface ArkSchemaScopeConfig extends Omit<ArkSchemaConfig, GlobalOnlyConfigOptionName>, ScopeOnlyConfigOptions {
}
interface ResolvedScopeConfig extends ResolvedConfig, ScopeOnlyConfigOptions {
}
declare abstract class BaseScope<$ extends {} = {}> {
    readonly config: ArkSchemaScopeConfig;
    readonly resolvedConfig: ResolvedScopeConfig;
    readonly name: string;
    get [arkKind](): "scope";
    readonly referencesById: {
        [id: string]: BaseNode;
    };
    references: readonly BaseNode[];
    readonly resolutions: {
        [alias: string]: CachedResolution | undefined;
    };
    exportedNames: string[];
    readonly aliases: Record<string, unknown>;
    protected resolved: boolean;
    readonly nodesByHash: Record<string, BaseNode>;
    readonly intrinsic: Omit<typeof $ark.intrinsic, `json${string}`>;
    constructor(def: Record<string, unknown>, config?: ArkSchemaScopeConfig);
    protected cacheGetter<name extends keyof this>(name: name, value: this[name]): this[name];
    get internal(): this;
    private _json;
    get json(): JsonStructure;
    defineSchema<def extends RootSchema>(def: def): def;
    generic: GenericRootParser;
    units: (values: array, opts?: BaseParseOptions) => BaseRoot;
    protected lazyResolutions: Alias.Node[];
    lazilyResolve(resolve: () => BaseRoot, syntheticAlias?: string): Alias.Node;
    schema: InternalSchemaParser;
    parseSchema: InternalSchemaParser;
    protected preparseNode(kinds: NodeKind | listable<RootKind>, schema: unknown, opts: BaseParseOptions): BaseNode | NodeParseContextInput;
    bindReference<reference extends BaseNode | GenericRoot>(reference: reference): reference;
    resolveRoot(name: string): BaseRoot;
    maybeResolveRoot(name: string): BaseRoot | undefined;
    protected maybeResolveSubalias(name: string): BaseRoot | GenericRoot | undefined;
    get ambient(): InternalModule;
    maybeResolve(name: string): Exclude<CachedResolution, string> | undefined;
    protected createParseContext<input extends BaseParseContextInput>(input: input): input & AttachedParseContext;
    traversal(root: unknown): Traversal;
    import(): SchemaModule<{
        [k in exportedNameOf<$> as PrivateDeclaration<k>]: $[k];
    }>;
    import<names extends exportedNameOf<$>[]>(...names: names): SchemaModule<{
        [k in names[number] as PrivateDeclaration<k>]: $[k];
    } & unknown>;
    precompilation: string | undefined;
    private _exportedResolutions;
    private _exports;
    export(): SchemaModule<{
        [k in exportedNameOf<$>]: $[k];
    }>;
    export<names extends exportedNameOf<$>[]>(...names: names): SchemaModule<{
        [k in names[number]]: $[k];
    } & unknown>;
    resolve<name extends exportedNameOf<$>>(name: name): instantiateRoot<$[name]>;
    node: <kinds extends NodeKind | array<RootKind>, prereduced extends boolean = false>(kinds: kinds, nodeSchema: NodeSchema<flattenListable<kinds>>, opts?: BaseParseOptions<prereduced>) => nodeOfKind<prereduced extends true ? flattenListable<kinds> : reducibleKindOf<flattenListable<kinds>>>;
    parse: (def: unknown, opts?: BaseParseOptions) => BaseRoot;
    parseDefinition(def: unknown, opts?: BaseParseOptions): BaseRoot;
    finalize<node extends BaseRoot>(node: node): node;
    protected abstract preparseOwnDefinitionFormat(def: unknown, opts: BaseParseOptions): BaseRoot | BaseParseContextInput;
    abstract parseOwnDefinitionFormat(def: unknown, ctx: BaseParseContext): BaseRoot;
    protected abstract preparseOwnAliasEntry(k: string, v: unknown): AliasDefEntry;
    protected abstract normalizeRootScopeValue(resolution: unknown): unknown;
}
type InternalSchemaParser = (schema: RootSchema, opts?: BaseParseOptions) => BaseRoot;
declare const writeUnresolvableMessage: <token extends string>(token: token) => writeUnresolvableMessage<token>;
type writeUnresolvableMessage<token extends string> = `'${token}' is unresolvable`;
declare const writeNonSubmoduleDotMessage: <name extends string>(name: name) => writeNonSubmoduleDotMessage<name>;
type writeNonSubmoduleDotMessage<name extends string> = `'${name}' must reference a module to be accessed using dot syntax`;
declare const writeMissingSubmoduleAccessMessage: <name extends string>(name: name) => writeMissingSubmoduleAccessMessage<name>;
type writeMissingSubmoduleAccessMessage<name extends string> = `Reference to submodule '${name}' must specify an alias`;
declare class OptionalNode extends BaseProp<"optional"> {
    constructor(...args: ConstructorParameters<typeof BaseProp>);
    get rawIn(): OptionalNode;
    get outProp(): Prop.Node;
    expression: string;
    defaultValueMorph: Morph | undefined;
    defaultValueMorphRef: string | undefined;
}
declare namespace Optional {
    interface Schema extends Prop.Schema {
        default?: unknown;
    }
    interface Inner extends Prop.Inner {
        default?: unknown;
    }
    type Declaration = declareNode<Prop.Declaration<"optional"> & {
        schema: Schema;
        normalizedSchema: Schema;
        inner: Inner;
    }>;
    type Node = OptionalNode;
    namespace Node {
        type withDefault = requireKeys<Node, "default" | "defaultValueMorph" | "defaultValueMorphRef">;
    }
}
declare const Optional: {
    implementation: nodeImplementationOf<{
        reducibleTo: "optional";
        errorContext: null;
        kind: "optional";
        prerequisite: object;
        intersectionIsOpen: true;
        childKind: RootKind;
        schema: Optional.Schema;
        normalizedSchema: Optional.Schema;
        inner: Optional.Inner;
    }>;
    Node: typeof OptionalNode;
};
type writeUnassignableDefaultValueMessage<baseDef extends string, defaultValue extends string> = `Default value ${defaultValue} must be assignable to ${baseDef}`;
declare class RequiredNode extends BaseProp<"required"> {
    expression: string;
    errorContext: NodeErrorContextInput<"required">;
    compiledErrorContext: string;
}
declare namespace Required$2 {
    interface ErrorContext extends BaseErrorContext<"required"> {
        missingValueDescription: string;
    }
    interface Schema extends Prop.Schema {
    }
    interface Inner extends Prop.Inner {
    }
    type Declaration = declareNode<Prop.Declaration<"required"> & {
        schema: Schema;
        normalizedSchema: Schema;
        inner: Inner;
        errorContext: ErrorContext;
    }>;
    type Node = RequiredNode;
}
declare const Required$2: {
    implementation: nodeImplementationOf<{
        reducibleTo: "required";
        kind: "required";
        prerequisite: object;
        intersectionIsOpen: true;
        childKind: RootKind;
        schema: Required$2.Schema;
        normalizedSchema: Required$2.Schema;
        inner: Required$2.Inner;
        errorContext: Required$2.ErrorContext;
    }>;
    Node: typeof RequiredNode;
};
declare namespace Prop {
    type Kind = "required" | "optional";
    type Node = nodeOfKind<Kind>;
    interface Schema extends BaseNormalizedSchema {
        readonly key: Key;
        readonly value: RootSchema;
    }
    interface Inner {
        readonly key: Key;
        readonly value: BaseRoot;
    }
    interface Declaration<kind extends Kind = Kind> {
        kind: kind;
        prerequisite: object;
        intersectionIsOpen: true;
        childKind: RootKind;
    }
}
declare abstract class BaseProp<kind extends Prop.Kind = Prop.Kind> extends BaseConstraint<kind extends "required" ? Required$2.Declaration : Optional.Declaration> {
    required: boolean;
    optional: boolean;
    impliedBasis: BaseRoot;
    serializedKey: string;
    compiledKey: string;
    flatRefs: FlatRef[];
    protected _transform(mapper: DeepNodeTransformation, ctx: DeepNodeTransformContext): BaseNode | null;
    hasDefault(): this is Optional.Node.withDefault;
    traverseAllows: TraverseAllows<object>;
    traverseApply: TraverseApply<object>;
    compile(js: NodeCompiler): void;
}
declare class IndexNode extends BaseConstraint<Index.Declaration> {
    impliedBasis: BaseRoot;
    expression: string;
    flatRefs: FlatRef<BaseRoot<InternalRootDeclaration>>[];
    traverseAllows: TraverseAllows<object>;
    traverseApply: TraverseApply<object>;
    protected _transform(mapper: DeepNodeTransformation, ctx: DeepNodeTransformContext): BaseNode | null;
    compile(): void;
}
declare namespace Index {
    type KeyKind = Exclude<RootKind, "unit">;
    type KeyNode = nodeOfKind<KeyKind>;
    interface Schema extends BaseNormalizedSchema {
        readonly signature: RootSchema<KeyKind>;
        readonly value: RootSchema;
    }
    interface Inner {
        readonly signature: KeyNode;
        readonly value: BaseRoot;
    }
    interface Declaration extends declareNode<{
        kind: "index";
        schema: Schema;
        normalizedSchema: Schema;
        inner: Inner;
        prerequisite: object;
        intersectionIsOpen: true;
        childKind: RootKind;
    }> {
    }
    type Node = IndexNode;
}
declare const Index: {
    implementation: nodeImplementationOf<Index.Declaration>;
    Node: typeof IndexNode;
};
declare const writeInvalidPropertyKeyMessage: <indexSchema extends string>(indexSchema: indexSchema) => writeInvalidPropertyKeyMessage<indexSchema>;
type writeInvalidPropertyKeyMessage<indexSchema extends string> = `Indexed key definition '${indexSchema}' must be a string or symbol`;
declare class MaxLengthNode extends BaseRange<MaxLength.Declaration> {
    readonly impliedBasis: BaseRoot;
    traverseAllows: TraverseAllows<LengthBoundableData>;
    reduceJsonSchema(schema: JsonSchema.LengthBoundable): JsonSchema.LengthBoundable;
}
declare namespace MaxLength {
    interface Inner extends BaseRangeInner {
        rule: number;
    }
    interface NormalizedSchema extends UnknownNormalizedRangeSchema {
        rule: number;
    }
    interface ExpandedSchema extends UnknownExpandedRangeSchema {
        rule: number;
    }
    type Schema = ExpandedSchema | number;
    interface ErrorContext extends BaseErrorContext<"maxLength">, Inner {
    }
    interface Declaration extends declareNode<{
        kind: "maxLength";
        schema: Schema;
        reducibleTo: "exactLength";
        normalizedSchema: NormalizedSchema;
        inner: Inner;
        prerequisite: LengthBoundableData;
        errorContext: ErrorContext;
    }> {
    }
    type Node = MaxLengthNode;
}
declare const MaxLength: {
    implementation: nodeImplementationOf<MaxLength.Declaration>;
    Node: typeof MaxLengthNode;
};
declare class MinLengthNode extends BaseRange<MinLength.Declaration> {
    readonly impliedBasis: BaseRoot;
    traverseAllows: TraverseAllows<LengthBoundableData>;
    reduceJsonSchema(schema: JsonSchema.LengthBoundable): JsonSchema.LengthBoundable;
}
declare namespace MinLength {
    interface Inner extends BaseRangeInner {
        rule: number;
    }
    interface NormalizedSchema extends UnknownNormalizedRangeSchema {
        rule: number;
    }
    interface ExpandedSchema extends UnknownExpandedRangeSchema {
        rule: number;
    }
    type Schema = ExpandedSchema | number;
    interface ErrorContext extends BaseErrorContext<"minLength">, Inner {
    }
    interface Declaration extends declareNode<{
        kind: "minLength";
        schema: Schema;
        normalizedSchema: NormalizedSchema;
        inner: Inner;
        prerequisite: LengthBoundableData;
        reducibleTo: "intersection";
        errorContext: ErrorContext;
    }> {
    }
    type Node = MinLengthNode;
}
declare const MinLength: {
    implementation: nodeImplementationOf<MinLength.Declaration>;
    Node: typeof MinLengthNode;
};
declare class SequenceNode extends BaseConstraint<Sequence.Declaration> {
    impliedBasis: BaseRoot;
    tuple: SequenceTuple;
    prefixLength: number;
    defaultablesLength: number;
    optionalsLength: number;
    postfixLength: number;
    defaultablesAndOptionals: BaseRoot[];
    prevariadic: array<PrevariadicSequenceElement>;
    variadicOrPostfix: array<BaseRoot>;
    flatRefs: FlatRef[];
    protected addFlatRefs(): FlatRef[];
    isVariadicOnly: boolean;
    minVariadicLength: number;
    minLength: number;
    minLengthNode: MinLengthNode | null;
    maxLength: number | null;
    maxLengthNode: MaxLengthNode | ExactLengthNode | null;
    impliedSiblings: array<MaxLengthNode | MinLengthNode | ExactLengthNode>;
    defaultValueMorphs: Morph[];
    defaultValueMorphsReference: `$ark.${string}` | `$ark0.${string}` | `$ark${`2${string}` & `${bigint}`}.${string}` | `$ark${`1${string}` & `${bigint}`}.${string}` | `$ark${`3${string}` & `${bigint}`}.${string}` | `$ark${`4${string}` & `${bigint}`}.${string}` | `$ark${`5${string}` & `${bigint}`}.${string}` | `$ark${`6${string}` & `${bigint}`}.${string}` | `$ark${`7${string}` & `${bigint}`}.${string}` | `$ark${`8${string}` & `${bigint}`}.${string}` | `$ark${`9${string}` & `${bigint}`}.${string}` | undefined;
    optionalize(): SequenceNode;
    require(): SequenceNode;
    protected elementAtIndex(data: array, index: number): SequenceElement;
    traverseAllows: TraverseAllows<array>;
    traverseApply: TraverseApply<array>;
    get element(): BaseRoot;
    compile(js: NodeCompiler): void;
    protected _transform(mapper: DeepNodeTransformation, ctx: DeepNodeTransformContext): BaseNode | null;
    expression: string;
    reduceJsonSchema(schema: JsonSchema.Array, ctx: ToJsonSchema.Context): JsonSchema.Array;
}
declare namespace Sequence {
    interface NormalizedSchema extends BaseNormalizedSchema {
        readonly prefix?: array<RootSchema>;
        readonly defaultables?: array<DefaultableSchema>;
        readonly optionals?: array<RootSchema>;
        readonly variadic?: RootSchema;
        readonly minVariadicLength?: number;
        readonly postfix?: array<RootSchema>;
    }
    type Schema = NormalizedSchema | RootSchema;
    type DefaultableSchema = [
        schema: RootSchema,
        defaultValue: unknown
    ];
    type DefaultableElement = [
        node: BaseRoot,
        defaultValue: unknown
    ];
    interface Inner {
        readonly prefix?: array<BaseRoot>;
        readonly defaultables?: array<DefaultableElement>;
        readonly optionals?: array<BaseRoot>;
        readonly variadic?: BaseRoot;
        readonly minVariadicLength?: number;
        readonly postfix?: array<BaseRoot>;
    }
    interface Declaration extends declareNode<{
        kind: "sequence";
        schema: Schema;
        normalizedSchema: NormalizedSchema;
        inner: Inner;
        prerequisite: array;
        reducibleTo: "sequence";
        childKind: RootKind;
    }> {
    }
    type Node = SequenceNode;
}
declare const Sequence: {
    implementation: nodeImplementationOf<Sequence.Declaration>;
    Node: typeof SequenceNode;
};
declare const postfixAfterOptionalOrDefaultableMessage = "A postfix required element cannot follow an optional or defaultable element";
type postfixAfterOptionalOrDefaultableMessage = typeof postfixAfterOptionalOrDefaultableMessage;
type SequenceElement = PrevariadicSequenceElement | VariadicSequenceElement | PostfixSequenceElement;
type PrevariadicSequenceElement = PrefixSequenceElement | DefaultableSequenceElement | OptionalSequenceElement;
type PrefixSequenceElement = {
    kind: "prefix";
    node: BaseRoot;
};
type OptionalSequenceElement = {
    kind: "optionals";
    node: BaseRoot;
};
type PostfixSequenceElement = {
    kind: "postfix";
    node: BaseRoot;
};
type VariadicSequenceElement = {
    kind: "variadic";
    node: BaseRoot;
};
type DefaultableSequenceElement = {
    kind: "defaultables";
    node: BaseRoot;
    default: unknown;
};
type SequenceTuple = array<SequenceElement>;
type UndeclaredKeyBehavior = "ignore" | UndeclaredKeyHandling;
type UndeclaredKeyHandling = "reject" | "delete";
declare class StructureNode extends BaseConstraint<Structure.Declaration> {
    impliedBasis: BaseRoot;
    impliedSiblings: BaseConstraint<Constraint.Declaration>[];
    props: array<Prop.Node>;
    propsByKey: Record<Key, Prop.Node | undefined>;
    propsByKeyReference: RegisteredReference;
    expression: string;
    requiredKeys: Key[];
    optionalKeys: Key[];
    literalKeys: Key[];
    _keyof: BaseRoot | undefined;
    keyof(): BaseRoot;
    map(flatMapProp: PropFlatMapper): StructureNode;
    assertHasKeys(keys: array<KeyOrKeyNode>): void;
    get(indexer: GettableKeyOrNode, ...path: array<GettableKeyOrNode>): BaseRoot;
    pick(...keys: KeyOrKeyNode[]): StructureNode;
    omit(...keys: KeyOrKeyNode[]): StructureNode;
    optionalize(): StructureNode;
    require(): StructureNode;
    merge(r: StructureNode): StructureNode;
    private filterKeys;
    traverseAllows: TraverseAllows<object>;
    traverseApply: TraverseApply<object>;
    protected _traverse: (traversalKind: TraversalKind, data: object, ctx: InternalTraversal) => boolean;
    get defaultable(): Optional.Node.withDefault[];
    declaresKey: (k: Key) => boolean;
    _compileDeclaresKey(js: NodeCompiler): string;
    get structuralMorph(): Morph | undefined;
    structuralMorphRef: RegisteredReference | undefined;
    compile(js: NodeCompiler): unknown;
    protected compileExhaustiveEntry(js: NodeCompiler): NodeCompiler;
    reduceJsonSchema(schema: JsonSchema.Structure, ctx: ToJsonSchema.Context): JsonSchema.Structure;
    reduceObjectJsonSchema(schema: JsonSchema.Object, ctx: ToJsonSchema.Context): JsonSchema.Object;
}
type PropFlatMapper = (entry: Prop.Node) => listable<MappedPropInner>;
type MappedPropInner = BaseMappedPropInner | OptionalMappedPropInner;
interface BaseMappedPropInner extends Required$2.Schema {
    kind?: "required" | "optional";
}
interface OptionalMappedPropInner extends Optional.Schema {
    kind: "optional";
}
declare namespace Structure {
    interface Schema extends BaseNormalizedSchema {
        readonly optional?: readonly Optional.Schema[];
        readonly required?: readonly Required$2.Schema[];
        readonly index?: readonly Index.Schema[];
        readonly sequence?: Sequence.Schema;
        readonly undeclared?: UndeclaredKeyBehavior;
    }
    interface Inner {
        readonly optional?: readonly Optional.Node[];
        readonly required?: readonly Required$2.Node[];
        readonly index?: readonly Index.Node[];
        readonly sequence?: Sequence.Node;
        readonly undeclared?: UndeclaredKeyHandling;
    }
    namespace Inner {
        type mutable = makeRootAndArrayPropertiesMutable<Inner>;
    }
    interface Declaration extends declareNode<{
        kind: "structure";
        schema: Schema;
        normalizedSchema: Schema;
        inner: Inner;
        prerequisite: object;
        childKind: StructuralKind;
    }> {
    }
    type Node = StructureNode;
}
declare const Structure: {
    implementation: nodeImplementationOf<Structure.Declaration>;
    Node: typeof StructureNode;
};
declare abstract class InternalBasis<d extends InternalRootDeclaration = InternalRootDeclaration> extends BaseRoot<d> {
    abstract compiledCondition: string;
    abstract compiledNegation: string;
    structure: undefined;
    traverseApply: TraverseApply<d["prerequisite"]>;
    get errorContext(): d["errorContext"];
    get compiledErrorContext(): string;
    compile(js: NodeCompiler): void;
}
declare class UnitNode extends InternalBasis<Unit.Declaration> {
    compiledValue: JsonPrimitive;
    serializedValue: string;
    compiledCondition: string;
    compiledNegation: string;
    expression: string;
    domain: Domain$1;
    get defaultShortDescription(): string;
    protected innerToJsonSchema(ctx: ToJsonSchema.Context): JsonSchema;
    traverseAllows: TraverseAllows;
}
declare namespace Unit {
    interface Schema<value = unknown> extends BaseNormalizedSchema {
        readonly unit: value;
    }
    interface Inner<value = unknown> {
        readonly unit: value;
    }
    interface ErrorContext<value = unknown> extends BaseErrorContext<"unit">, Inner<value> {
    }
    interface Declaration extends declareNode<{
        kind: "unit";
        schema: Schema;
        normalizedSchema: Schema;
        inner: Inner;
        errorContext: ErrorContext;
    }> {
    }
    type Node = UnitNode;
}
declare const Unit: {
    implementation: nodeImplementationOf<Unit.Declaration>;
    Node: typeof UnitNode;
};
declare class DomainNode extends InternalBasis<Domain.Declaration> {
    private readonly requiresNaNCheck;
    readonly traverseAllows: TraverseAllows;
    readonly compiledCondition: string;
    readonly compiledNegation: string;
    readonly expression: string;
    get nestableExpression(): string;
    get defaultShortDescription(): string;
    protected innerToJsonSchema(ctx: ToJsonSchema.Context): JsonSchema.Constrainable;
}
type Domain = Domain$1;
declare namespace Domain {
    type Enumerable = "undefined" | "null" | "boolean";
    type NonEnumerable = Exclude<Domain, Enumerable>;
    interface Inner<domain extends NonEnumerable = NonEnumerable> {
        readonly domain: domain;
        readonly numberAllowsNaN?: boolean;
    }
    interface NormalizedSchema<domain extends NonEnumerable = NonEnumerable> extends BaseNormalizedSchema, Inner<domain> {
    }
    type Schema<domain extends NonEnumerable = NonEnumerable> = domain | NormalizedSchema<domain>;
    interface ErrorContext extends BaseErrorContext<"domain">, Inner {
    }
    interface Declaration extends declareNode<{
        kind: "domain";
        schema: Schema;
        normalizedSchema: NormalizedSchema;
        inner: Inner;
        errorContext: ErrorContext;
    }> {
    }
    type Node = DomainNode;
}
declare const Domain: {
    implementation: nodeImplementationOf<Domain.Declaration>;
    Node: typeof DomainNode;
    writeBadAllowNanMessage: (actual: Exclude<Domain.NonEnumerable, "number">) => string;
};
declare class UnionNode extends BaseRoot<Union.Declaration> {
    isBoolean: boolean;
    get branchGroups(): BaseRoot[];
    unitBranches: (MorphNode | UnitNode)[];
    discriminant: Discriminant<DiscriminantKind> | null;
    discriminantJson: JsonStructure | null;
    expression: string;
    createBranchedOptimisticRootApply(): BaseNode["rootApply"];
    get shallowMorphs(): array<Morph>;
    get defaultShortDescription(): string;
    protected innerToJsonSchema(ctx: ToJsonSchema.Context): JsonSchema;
    traverseAllows: TraverseAllows;
    traverseApply: TraverseApply;
    traverseOptimistic: (data: unknown) => unknown;
    compile(js: NodeCompiler): void;
    private compileIndiscriminable;
    get nestableExpression(): string;
    discriminate(): Discriminant | null;
}
declare namespace Union {
    type ChildKind = UnionChildKind;
    type ChildSchema = NodeSchema<ChildKind>;
    type ChildNode = nodeOfKind<ChildKind>;
    type Schema = NormalizedSchema | readonly RootSchema[];
    interface NormalizedSchema extends BaseNormalizedSchema {
        readonly branches: array<RootSchema>;
        readonly ordered?: true;
    }
    interface Inner {
        readonly branches: readonly ChildNode[];
        readonly ordered?: true;
    }
    interface ErrorContext extends BaseErrorContext<"union"> {
        errors: readonly ArkError[];
    }
    interface Declaration extends declareNode<{
        kind: "union";
        schema: Schema;
        normalizedSchema: NormalizedSchema;
        inner: Inner;
        errorContext: ErrorContext;
        reducibleTo: RootKind;
        childKind: UnionChildKind;
    }> {
    }
    type Node = UnionNode;
}
declare const Union: {
    implementation: nodeImplementationOf<Union.Declaration>;
    Node: typeof UnionNode;
};
type CaseKey<kind extends DiscriminantKind = DiscriminantKind> = DiscriminantKind extends kind ? string : DiscriminantKinds[kind] | "default";
type DiscriminantLocation<kind extends DiscriminantKind = DiscriminantKind> = {
    path: PropertyKey[];
    optionallyChainedPropString: string;
    kind: kind;
};
interface Discriminant<kind extends DiscriminantKind = DiscriminantKind> extends DiscriminantLocation<kind> {
    cases: DiscriminatedCases<kind>;
}
type DiscriminatedCases<kind extends DiscriminantKind = DiscriminantKind> = {
    [caseKey in CaseKey<kind>]: BaseRoot | true;
};
type DiscriminantKinds = {
    domain: Domain;
    unit: SerializedPrimitive | RegisteredReference;
};
type DiscriminantKind = show<keyof DiscriminantKinds>;
interface InternalRootDeclaration extends BaseNodeDeclaration {
    kind: RootKind;
}
declare abstract class BaseRoot<out d extends InternalRootDeclaration = InternalRootDeclaration> extends BaseNode<d> implements StandardSchemaV1, StandardJSONSchemaV1 {
    readonly [arkKind]: "root";
    readonly [inferred]: unknown;
    constructor(attachments: UnknownAttachments, $: BaseScope);
    get rawIn(): BaseRoot;
    get rawOut(): BaseRoot;
    get internal(): this;
    get "~standard"(): StandardSchemaV1.ArkTypeProps;
    as(): this;
    brand(name: string): this;
    readonly(): this;
    readonly branches: readonly nodeOfKind<Union.ChildKind>[];
    distribute<mapOut, reduceOut = mapOut[]>(mapBranch: (branch: nodeOfKind<Union.ChildKind>, i: number, branches: array<nodeOfKind<Union.ChildKind>>) => mapOut, reduceMapped?: (mappedBranches: mapOut[]) => reduceOut): reduceOut;
    abstract get defaultShortDescription(): string;
    get shortDescription(): string;
    toJsonSchema(opts?: ToJsonSchema.Options): JsonSchema;
    toJsonSchemaRecurse(ctx: ToJsonSchema.Context): JsonSchema;
    get alwaysExpandJsonSchema(): boolean;
    protected toResolvedJsonSchema(ctx: ToJsonSchema.Context): JsonSchema;
    protected abstract innerToJsonSchema(ctx: ToJsonSchema.Context): JsonSchema;
    intersect(r: unknown): BaseRoot | Disjoint;
    rawIntersect(r: BaseRoot): BaseRoot;
    toNeverIfDisjoint(): BaseRoot;
    and(r: unknown): BaseRoot;
    rawAnd(r: BaseRoot): BaseRoot;
    or(r: unknown): BaseRoot;
    rawOr(r: BaseRoot): BaseRoot;
    map(flatMapEntry: PropFlatMapper): BaseRoot;
    pick(...keys: KeyOrKeyNode[]): BaseRoot;
    omit(...keys: KeyOrKeyNode[]): BaseRoot;
    required(): BaseRoot;
    partial(): BaseRoot;
    private _keyof?;
    keyof(): BaseRoot;
    get props(): Prop.Node[];
    merge(r: unknown): BaseRoot;
    private applyStructuralOperation;
    get(...path: GettableKeyOrNode[]): BaseRoot;
    extract(r: unknown): BaseRoot;
    exclude(r: unknown): BaseRoot;
    array(): BaseRoot;
    overlaps(r: unknown): boolean;
    extends(r: unknown): boolean;
    ifExtends(r: unknown): BaseRoot | undefined;
    subsumes(r: unknown): boolean;
    configure(meta: TypeMeta.MappableInput, selector?: NodeSelector): this;
    describe(description: string, selector?: NodeSelector): this;
    optional(): [
        this,
        "?"
    ];
    default(thunkableValue: unknown): [
        this,
        "=",
        unknown
    ];
    from(input: unknown): unknown;
    protected _pipe(...morphs: Morph[]): BaseRoot;
    protected tryPipe(...morphs: Morph[]): BaseRoot;
    pipe: ((...morphs: Morph[]) => BaseRoot) & {
        try: (...morphs: Morph[]) => BaseRoot;
    };
    to(def: unknown): BaseRoot;
    private toNode;
    rawPipeOnce(morph: Morph): BaseRoot;
    narrow(predicate: Predicate): BaseRoot;
    constrain<kind extends Constraint.PrimitiveKind>(kind: kind, schema: NodeSchema<kind>): BaseRoot;
    constrainIn<kind extends Constraint.PrimitiveKind>(kind: kind, schema: NodeSchema<kind>): BaseRoot;
    constrainOut<kind extends Constraint.PrimitiveKind>(kind: kind, schema: NodeSchema<kind>): BaseRoot;
    private _constrain;
    onUndeclaredKey(cfg: UndeclaredKeyBehavior | UndeclaredKeyConfig): BaseRoot;
    hasEqualMorphs(r: BaseRoot): boolean;
    onDeepUndeclaredKey(behavior: UndeclaredKeyBehavior): BaseRoot;
    filter(predicate: Predicate): BaseRoot;
    divisibleBy(schema: Divisor.Schema): BaseRoot;
    matching(schema: Pattern.Schema): BaseRoot;
    atLeast(schema: InclusiveNumericRangeSchema): BaseRoot;
    atMost(schema: InclusiveNumericRangeSchema): BaseRoot;
    moreThan(schema: ExclusiveNumericRangeSchema): BaseRoot;
    lessThan(schema: ExclusiveNumericRangeSchema): BaseRoot;
    atLeastLength(schema: InclusiveNumericRangeSchema): BaseRoot;
    atMostLength(schema: InclusiveNumericRangeSchema): BaseRoot;
    moreThanLength(schema: ExclusiveNumericRangeSchema): BaseRoot;
    lessThanLength(schema: ExclusiveNumericRangeSchema): BaseRoot;
    exactlyLength(schema: ExactLength.Schema): BaseRoot;
    atOrAfter(schema: InclusiveDateRangeSchema): BaseRoot;
    atOrBefore(schema: InclusiveDateRangeSchema): BaseRoot;
    laterThan(schema: ExclusiveDateRangeSchema): BaseRoot;
    earlierThan(schema: ExclusiveDateRangeSchema): BaseRoot;
}
type UndeclaredKeyConfig = {
    rule: UndeclaredKeyBehavior;
    deep?: boolean;
};
declare const emptyBrandNameMessage = "Expected a non-empty brand name after #";
type emptyBrandNameMessage = typeof emptyBrandNameMessage;
type schemaKindRightOf<kind extends RootKind> = Extract<kindRightOf<kind>, RootKind>;
type schemaKindOrRightOf<kind extends RootKind> = kind | schemaKindRightOf<kind>;
type StructuralOperationBranchResultByName = {
    keyof: Union.ChildNode;
    pick: Union.ChildNode;
    omit: Union.ChildNode;
    get: Union.ChildNode;
    map: Union.ChildNode;
    required: Union.ChildNode;
    partial: Union.ChildNode;
    merge: Union.ChildNode;
    props: array<Prop.Node>;
};
type StructuralOperationName = keyof StructuralOperationBranchResultByName;
declare const writeNonStructuralOperandMessage: <operation extends StructuralOperationName, operand extends string>(operation: operation, operand: operand) => writeNonStructuralOperandMessage<operation, operand>;
type writeNonStructuralOperandMessage<operation extends StructuralOperationName, operand extends string> = `${operation} operand must be an object (was ${operand})`;
declare const basisKinds: readonly [
    "unit",
    "proto",
    "domain"
];
type BasisKind = (typeof basisKinds)[number];
declare const structuralKinds: readonly [
    "required",
    "optional",
    "index",
    "sequence"
];
type StructuralKind = (typeof structuralKinds)[number];
declare const prestructuralKinds: readonly [
    "pattern",
    "divisor",
    "exactLength",
    "max",
    "min",
    "maxLength",
    "minLength",
    "before",
    "after"
];
type PrestructuralKind = (typeof prestructuralKinds)[number];
declare const refinementKinds: readonly [
    "pattern",
    "divisor",
    "exactLength",
    "max",
    "min",
    "maxLength",
    "minLength",
    "before",
    "after",
    "structure",
    "predicate"
];
type RefinementKind = (typeof refinementKinds)[number];
declare const constraintKinds: readonly [
    "pattern",
    "divisor",
    "exactLength",
    "max",
    "min",
    "maxLength",
    "minLength",
    "before",
    "after",
    "structure",
    "predicate",
    "required",
    "optional",
    "index",
    "sequence"
];
type ConstraintKind = (typeof constraintKinds)[number];
declare const rootKinds: readonly [
    "alias",
    "union",
    "morph",
    "unit",
    "intersection",
    "proto",
    "domain"
];
type RootKind = (typeof rootKinds)[number];
type NodeKind = RootKind | ConstraintKind;
type orderedNodeKinds = [
    ...typeof rootKinds,
    ...typeof constraintKinds
];
declare const nodeKinds: orderedNodeKinds;
type OpenNodeKind = {
    [k in NodeKind]: Declaration<k>["intersectionIsOpen"] extends true ? k : never;
}[NodeKind];
type OrderedNodeKinds = typeof nodeKinds;
type RightsByKind = accumulateRightKinds<OrderedNodeKinds, {}>;
type kindOrRightOf<kind extends NodeKind> = kind | kindRightOf<kind>;
type accumulateRightKinds<remaining extends readonly NodeKind[], result> = remaining extends (readonly [
    infer head extends NodeKind,
    ...infer tail extends NodeKind[]
]) ? accumulateRightKinds<tail, result & {
    [k in head]: tail[number];
}> : result;
interface InternalIntersectionOptions {
    pipe: boolean;
}
interface IntersectionContext extends InternalIntersectionOptions {
    $: BaseScope;
    invert: boolean;
}
type ConstraintIntersection<lKind extends ConstraintKind, rKind extends kindOrRightOf<lKind>> = (l: nodeOfKind<lKind>, r: nodeOfKind<rKind>, ctx: IntersectionContext) => BaseNode | Disjoint | null;
type ConstraintIntersectionMap<kind extends ConstraintKind> = show<{
    [_ in kind]: ConstraintIntersection<kind, kind>;
} & {
    [rKind in kindRightOf<kind>]?: ConstraintIntersection<kind, rKind>;
}>;
type RootIntersection<lKind extends RootKind, rKind extends schemaKindOrRightOf<lKind>> = (l: nodeOfKind<lKind>, r: nodeOfKind<rKind>, ctx: IntersectionContext) => BaseRoot | Disjoint;
type TypeIntersectionMap<kind extends RootKind> = {
    [rKind in schemaKindOrRightOf<kind>]: RootIntersection<kind, rKind>;
};
type IntersectionMap<kind extends NodeKind> = kind extends RootKind ? TypeIntersectionMap<kind> : ConstraintIntersectionMap<kind & ConstraintKind>;
type UnknownIntersectionMap = {
    [k in NodeKind]?: (l: BaseNode, r: BaseNode, ctx: IntersectionContext) => UnknownIntersectionResult;
};
type UnknownIntersectionResult = BaseNode | Disjoint | null;
type kindRightOf<kind extends NodeKind> = RightsByKind[kind];
declare const unionChildKinds: readonly [
    ...("intersection" | "morph" | "unit" | "proto" | "domain")[],
    "alias"
];
type UnionChildKind = (typeof unionChildKinds)[number];
type keySchemaDefinitions<d extends BaseNodeDeclaration> = {
    [k in keyRequiringSchemaDefinition<d>]: NodeKeyImplementation<d, k>;
};
type keyRequiringSchemaDefinition<d extends BaseNodeDeclaration> = Exclude<keyof d["normalizedSchema"], keyof BaseNormalizedSchema>;
type NodeKeyImplementation<d extends BaseNodeDeclaration, k extends keyof d["normalizedSchema"], instantiated = k extends keyof d["inner"] ? Exclude<d["inner"][k], undefined> : never> = requireKeys<{
    preserveUndefined?: true;
    child?: boolean | ((value: instantiated) => BaseNode[]);
    serialize?: (schema: instantiated) => Json;
    reduceIo?: (ioKind: "in" | "out", inner: makeRootAndArrayPropertiesMutable<d["inner"]>, value: d["inner"][k]) => void;
    parse?: (schema: Exclude<d["normalizedSchema"][k], undefined>, ctx: NodeParseContext<d["kind"]>) => instantiated | undefined;
}, (d["normalizedSchema"][k] extends instantiated | undefined ? never : "parse") | ([
    instantiated
] extends [
    listable<BaseNode>
] ? "child" : never)>;
interface CommonNodeImplementationInput<d extends BaseNodeDeclaration> {
    kind: d["kind"];
    keys: keySchemaDefinitions<d>;
    normalize: (schema: d["schema"], $: BaseScope) => d["normalizedSchema"];
    applyConfig?: (schema: d["normalizedSchema"], config: ResolvedScopeConfig) => d["normalizedSchema"];
    hasAssociatedError: d["errorContext"] extends null ? false : true;
    finalizeInnerJson?: (json: {
        [k in keyof d["inner"]]: Json;
    }) => JsonStructure;
    collapsibleKey?: keyof d["inner"];
    reduce?: (inner: d["inner"], $: BaseScope) => nodeOfKind<d["reducibleTo"]> | Disjoint | undefined;
    obviatesBasisDescription?: d["kind"] extends RefinementKind ? true : never;
    obviatesBasisExpression?: d["kind"] extends RefinementKind ? true : never;
}
interface UnknownNodeImplementation extends CommonNodeImplementationInput<BaseNodeDeclaration> {
    defaults: ResolvedUnknownNodeConfig;
    intersectionIsOpen: boolean;
    intersections: UnknownIntersectionMap;
    keys: Record<string, NodeKeyImplementation<any, any>>;
}
type nodeImplementationOf<d extends BaseNodeDeclaration> = nodeImplementationInputOf<d> & {
    intersections: IntersectionMap<d["kind"]>;
    intersectionIsOpen: d["intersectionIsOpen"];
    defaults: Required<NodeConfig<d["kind"]>>;
};
type nodeImplementationInputOf<d extends BaseNodeDeclaration> = CommonNodeImplementationInput<d> & {
    intersections: IntersectionMap<d["kind"]>;
    defaults: nodeSchemaaultsImplementationInputFor<d["kind"]>;
} & (d["intersectionIsOpen"] extends true ? {
    intersectionIsOpen: true;
} : {}) & (d["reducibleTo"] extends d["kind"] ? {} : {
    reduce: {};
});
type nodeSchemaaultsImplementationInputFor<kind extends NodeKind> = requireKeys<NodeConfig<kind>, "description" | (Inner<kind> extends (Omit<errorContext<kind>, keyof BaseErrorContext | "description">) ? never : "expected" & keyof NodeConfig<kind>)>;
type DescriptionWriter<kind extends NodeKind = NodeKind> = (node: nodeOfKind<kind>) => string;
interface UnknownAttachments {
    readonly kind: NodeKind;
    readonly impl: UnknownNodeImplementation;
    readonly id: NodeId;
    readonly inner: Record<string, any>;
    readonly innerEntries: readonly Entry<string>[];
    readonly innerJson: object;
    readonly innerHash: string;
    readonly meta: ArkEnv.meta;
    readonly metaJson: object;
    readonly json: object;
    readonly hash: string;
    readonly collapsibleJson: Json;
    readonly children: BaseNode[];
}
interface NarrowedAttachments<d extends BaseNodeDeclaration> extends UnknownAttachments {
    kind: d["kind"];
    inner: d["inner"];
    json: JsonStructure;
    innerJson: JsonStructure;
    collapsibleJson: Json;
    children: nodeOfKind<d["childKind"]>[];
}
declare class AfterNode extends BaseRange<After.Declaration> {
    impliedBasis: BaseRoot;
    collapsibleLimitString: string;
    traverseAllows: TraverseAllows<Date>;
    reduceJsonSchema(base: JsonSchema, ctx: ToJsonSchema.Context): JsonSchema;
}
declare namespace After {
    interface Inner extends BaseRangeInner {
        rule: Date;
    }
    interface NormalizedSchema extends UnknownNormalizedRangeSchema {
        rule: LimitSchemaValue;
    }
    interface ExpandedSchema extends UnknownExpandedRangeSchema {
        rule: LimitSchemaValue;
    }
    type Schema = ExpandedSchema | LimitSchemaValue;
    interface ErrorContext extends BaseErrorContext<"after">, Inner {
    }
    interface Declaration extends declareNode<{
        kind: "after";
        schema: Schema;
        normalizedSchema: NormalizedSchema;
        inner: Inner;
        prerequisite: Date;
        errorContext: ErrorContext;
    }> {
    }
    type Node = AfterNode;
}
declare const After: {
    implementation: nodeImplementationOf<After.Declaration>;
    Node: typeof AfterNode;
};
declare class BeforeNode extends BaseRange<Before.Declaration> {
    collapsibleLimitString: string;
    traverseAllows: TraverseAllows<Date>;
    impliedBasis: BaseRoot;
    reduceJsonSchema(base: JsonSchema, ctx: ToJsonSchema.Context): JsonSchema;
}
declare namespace Before {
    interface Inner extends BaseRangeInner {
        rule: Date;
    }
    interface NormalizedSchema extends UnknownNormalizedRangeSchema {
        rule: LimitSchemaValue;
    }
    interface ExpandedSchema extends UnknownExpandedRangeSchema {
        rule: LimitSchemaValue;
    }
    type Schema = ExpandedSchema | LimitSchemaValue;
    interface ErrorContext extends BaseErrorContext<"before">, Inner {
    }
    interface Declaration extends declareNode<{
        kind: "before";
        schema: Schema;
        normalizedSchema: NormalizedSchema;
        inner: Inner;
        prerequisite: Date;
        errorContext: ErrorContext;
    }> {
    }
    type Node = BeforeNode;
}
declare const Before: {
    implementation: nodeImplementationOf<Before.Declaration>;
    Node: typeof BeforeNode;
};
declare class MaxNode extends BaseRange<Max.Declaration> {
    impliedBasis: BaseRoot;
    traverseAllows: TraverseAllows<number>;
    reduceJsonSchema(schema: JsonSchema.Numeric): JsonSchema.Numeric;
}
declare namespace Max {
    interface Inner extends BaseRangeInner {
        rule: number;
        exclusive?: true;
    }
    interface NormalizedSchema extends UnknownExpandedRangeSchema {
        rule: number;
    }
    type Schema = NormalizedSchema | number;
    interface ErrorContext extends BaseErrorContext<"max">, Inner {
    }
    interface Declaration extends declareNode<{
        kind: "max";
        schema: Schema;
        normalizedSchema: NormalizedSchema;
        inner: Inner;
        prerequisite: number;
        errorContext: ErrorContext;
    }> {
    }
    type Node = MaxNode;
}
declare const Max: {
    implementation: nodeImplementationOf<Max.Declaration>;
    Node: typeof MaxNode;
};
declare class MinNode extends BaseRange<Min.Declaration> {
    readonly impliedBasis: BaseRoot;
    traverseAllows: TraverseAllows<number>;
    reduceJsonSchema(schema: JsonSchema.Numeric): JsonSchema.Numeric;
}
declare namespace Min {
    interface Inner extends BaseRangeInner {
        rule: number;
        exclusive?: true;
    }
    interface NormalizedSchema extends UnknownExpandedRangeSchema {
        rule: number;
    }
    type Schema = NormalizedSchema | number;
    interface ErrorContext extends BaseErrorContext<"min">, Inner {
    }
    interface Declaration extends declareNode<{
        kind: "min";
        schema: Schema;
        normalizedSchema: NormalizedSchema;
        inner: Inner;
        prerequisite: number;
        errorContext: ErrorContext;
    }> {
    }
    type Node = MinNode;
}
declare const Min: {
    implementation: nodeImplementationOf<Min.Declaration>;
    Node: typeof MinNode;
};
interface BoundDeclarations {
    min: Min.Declaration;
    max: Max.Declaration;
    minLength: MinLength.Declaration;
    maxLength: MaxLength.Declaration;
    exactLength: ExactLength.Declaration;
    after: After.Declaration;
    before: Before.Declaration;
}
interface BoundNodesByKind {
    min: Min.Node;
    max: Max.Node;
    minLength: MinLength.Node;
    maxLength: MaxLength.Node;
    exactLength: ExactLength.Node;
    after: After.Node;
    before: Before.Node;
}
type BoundKind = keyof BoundDeclarations;
type RangeKind = Exclude<BoundKind, "exactLength">;
interface DisjointEntry<kind extends DisjointKind = DisjointKind> {
    kind: kind;
    l: OperandsByDisjointKind[kind];
    r: OperandsByDisjointKind[kind];
    path: Key[];
    optional: boolean;
}
type OperandsByDisjointKind = {
    domain: nodeOfKind<"domain"> | Domain.Enumerable;
    unit: nodeOfKind<"unit">;
    proto: nodeOfKind<"proto">;
    presence: BaseRoot;
    range: nodeOfKind<BoundKind>;
    assignability: BaseNode;
    union: readonly BaseRoot[];
};
type DisjointEntryContext = {
    path?: Key[];
    optional?: true;
};
declare class Disjoint extends Array<DisjointEntry> {
    static init<kind extends DisjointKind>(kind: kind, l: OperandsByDisjointKind[kind], r: OperandsByDisjointKind[kind], ctx?: DisjointEntryContext): Disjoint;
    add<kind extends DisjointKind>(kind: kind, l: OperandsByDisjointKind[kind], r: OperandsByDisjointKind[kind], ctx?: DisjointEntryContext): Disjoint;
    get summary(): string;
    describeReasons(): string;
    throw(): never;
    invert(): Disjoint;
    withPrefixKey(key: PropertyKey, kind: Prop.Kind): Disjoint;
    toNeverIfDisjoint(): BaseRoot;
}
type DisjointKind = keyof OperandsByDisjointKind;
type withMetaPrefixedKeys<o> = {
    [k in keyof o as k extends string ? `meta.${k}` : never]: o[k];
};
interface DefaultArkEnv {
    meta(): {};
    onFail(errors: ArkErrors): ArkErrors;
}
interface NodeMeta extends JsonSchema.UniversalMeta, UnknownErrorConfigs {
    alias?: string;
    onFail?: ArkErrors.Handler;
}
declare global {
    export interface ArkEnv extends DefaultArkEnv {
    }
    export namespace ArkEnv {
        type meta = show<NodeMeta & ReturnType<ArkEnv["meta"]>>;
        type onFail = ReturnType<ArkEnv["onFail"]>;
    }
}
type TypeMeta = Omit<ArkEnv.meta, "onFail">;
declare namespace TypeMeta {
    type Collapsible<meta extends TypeMeta = TypeMeta> = meta | string;
    type Mapper<meta extends TypeMeta = TypeMeta> = (existing: Readonly<meta>) => meta;
    type MappableInput<meta extends TypeMeta = TypeMeta> = Collapsible<meta> | Mapper<meta>;
    namespace MappableInput {
        type Internal = MappableInput<ArkEnv.meta>;
    }
}
interface BaseNormalizedSchema extends withMetaPrefixedKeys<TypeMeta> {
    readonly meta?: ArkEnv.meta | string;
}
interface DeclarationInput {
    kind: NodeKind;
    schema: unknown;
    normalizedSchema: BaseNormalizedSchema;
    inner: object;
    errorContext?: BaseErrorContext;
    reducibleTo?: NodeKind;
    intersectionIsOpen?: true;
    prerequisite?: unknown;
    childKind?: NodeKind;
}
interface BaseErrorContext<kind extends NodeKind = NodeKind> {
    readonly description?: string;
    readonly code: kind;
    readonly meta: ArkEnv.meta;
}
type declareNode<d extends {
    [k in keyof d]: k extends keyof DeclarationInput ? DeclarationInput[k] : never;
} & DeclarationInput> = merge<{
    intersectionIsOpen: false;
    prerequisite: prerequisiteOf<d>;
    childKind: never;
    reducibleTo: d["kind"];
    errorContext: null;
}, d>;
type prerequisiteOf<d extends DeclarationInput> = "prerequisite" extends keyof d ? d["prerequisite"] : unknown;
type attachmentsOf<d extends BaseNodeDeclaration> = NarrowedAttachments<d> & attachedInner<d>;
type attachedInner<d extends BaseNodeDeclaration> = "intersection" & d["kind"] extends never ? d["inner"] : {};
interface BaseNodeDeclaration {
    kind: NodeKind;
    schema: unknown;
    normalizedSchema: BaseNormalizedSchema;
    inner: {};
    reducibleTo: NodeKind;
    prerequisite: any;
    intersectionIsOpen: boolean;
    childKind: NodeKind;
    errorContext: BaseErrorContext | null;
}
declare class MorphNode extends BaseRoot<Morph.Declaration> {
    serializedMorphs: string[];
    compiledMorphs: string;
    lastMorph: Morph | BaseRoot | undefined;
    lastMorphIfNode: BaseRoot | undefined;
    introspectableIn: BaseRoot | undefined;
    introspectableOut: BaseRoot | undefined;
    get shallowMorphs(): array<Morph>;
    get rawIn(): BaseRoot;
    get rawOut(): BaseRoot;
    declareIn(declaredIn: BaseRoot): MorphNode;
    declareOut(declaredOut: BaseRoot): MorphNode;
    expression: string;
    get defaultShortDescription(): string;
    protected innerToJsonSchema(ctx: ToJsonSchema.Context): JsonSchema;
    compile(js: NodeCompiler): void;
    traverseAllows: TraverseAllows;
    traverseApply: TraverseApply;
    hasEqualMorphs(r: MorphNode): boolean;
}
declare namespace Morph {
    interface Inner {
        readonly in?: BaseRoot;
        readonly morphs: array<Morph | BaseRoot>;
        readonly declaredIn?: BaseRoot;
        readonly declaredOut?: BaseRoot;
    }
    interface Schema extends BaseNormalizedSchema {
        readonly in?: RootSchema;
        readonly morphs: listable<Morph | BaseRoot>;
        readonly declaredIn?: BaseRoot;
        readonly declaredOut?: BaseRoot;
    }
    interface Declaration extends declareNode<{
        kind: "morph";
        schema: Schema;
        normalizedSchema: Schema;
        inner: Inner;
        childKind: RootKind;
    }> {
    }
    type Node = MorphNode;
    type In<morph extends Morph> = morph extends Morph<infer i> ? i : never;
    type Out<morph extends Morph> = morph extends Morph<never, infer o> ? o : never;
    type ContextFree<i = never, o = unknown> = (In: i) => o;
}
type Morph<i = never, o = unknown> = (In: i, ctx: Traversal) => o;
declare const Morph: {
    implementation: nodeImplementationOf<Morph.Declaration>;
    Node: typeof MorphNode;
};
type MorphsAtPath = {
    path: ReadonlyPath;
    morphs: array<Morph>;
};
type BranchTraversal = {
    error: ArkError | undefined;
    queuedMorphs: MorphsAtPath[];
};
type InternalTraversal = Omit<Traversal, "error" | "mustBe" | "reject">;
declare class Traversal {
    path: PropertyKey[];
    errors: ArkErrors;
    root: unknown;
    config: ResolvedConfig;
    queuedMorphs: MorphsAtPath[];
    branches: BranchTraversal[];
    seen: {
        [id in string]?: unknown[];
    };
    constructor(root: unknown, config: ResolvedConfig);
    get data(): unknown;
    get propString(): string;
    reject(input: ArkErrorInput): false;
    mustBe(expected: string): false;
    error<input extends ArkErrorInput>(input: input): ArkError<input extends {
        code: ArkErrorCode;
    } ? input["code"] : "predicate">;
    hasError(): boolean;
    get currentBranch(): BranchTraversal | undefined;
    queueMorphs(morphs: array<Morph>): void;
    finalize(onFail?: ArkErrors.Handler | null): unknown;
    get currentErrorCount(): number;
    get failFast(): boolean;
    pushBranch(): void;
    popBranch(): BranchTraversal | undefined;
    get external(): this;
    errorFromNodeContext<input extends NodeErrorContextInput>(input: input): ArkError<input["code"]>;
    private errorFromContext;
    private applyQueuedMorphs;
    private applyMorphsAtPath;
}
type TraversalMethodsByKind<input = unknown> = {
    Allows: TraverseAllows<input>;
    Apply: TraverseApply<input>;
    Optimistic: TraverseApply<input>;
};
type TraversalKind = keyof TraversalMethodsByKind & {};
type TraverseAllows<data = unknown> = (data: data, ctx: InternalTraversal) => boolean;
type TraverseApply<data = unknown> = (data: data, ctx: InternalTraversal) => void;
type CoercibleValue = string | number | boolean | null | undefined;
declare class CompiledFunction<compiledSignature = (...args: unknown[]) => unknown, args extends readonly string[] = readonly string[]> extends CastableBase<{
    [k in args[number]]: k;
}> {
    readonly argNames: args;
    readonly body = "";
    constructor(...args: args);
    indentation: number;
    indent(): this;
    dedent(): this;
    prop(key: PropertyKey, optional?: boolean): string;
    index(key: string | number, optional?: boolean): string;
    line(statement: string): this;
    const(identifier: string, expression: CoercibleValue): this;
    let(identifier: string, expression: CoercibleValue): this;
    set(identifier: string, expression: CoercibleValue): this;
    if(condition: string, then: (self: this) => this): this;
    elseIf(condition: string, then: (self: this) => this): this;
    else(then: (self: this) => this): this;
    for(until: string, body: (self: this) => this, initialValue?: CoercibleValue): this;
    forIn(object: string, body: (self: this) => this): this;
    block(prefix: string, contents: (self: this) => this, suffix?: string): this;
    return(expression?: CoercibleValue): this;
    write(name?: string, indent?: number): string;
    compile(): compiledSignature;
}
interface InvokeOptions extends ReferenceOptions {
    arg?: string;
}
interface ReferenceOptions {
    kind?: TraversalKind;
    bind?: string;
}
declare namespace NodeCompiler {
    interface Context {
        kind: TraversalKind;
        optimistic?: true;
    }
}
declare class NodeCompiler extends CompiledFunction<Fn, [
    "data",
    "ctx"
]> {
    traversalKind: TraversalKind;
    optimistic: boolean;
    constructor(ctx: NodeCompiler.Context);
    invoke(node: BaseNode | NodeId, opts?: InvokeOptions): string;
    referenceToId(id: NodeId, opts?: ReferenceOptions): string;
    requiresContextFor(node: BaseNode): boolean;
    initializeErrorCount(): this;
    returnIfFail(): this;
    returnIfFailFast(): this;
    traverseKey(keyExpression: string, accessExpression: string, node: BaseNode): this;
    check(node: BaseNode, opts?: InvokeOptions): this;
}
declare class PredicateNode extends BaseConstraint<Predicate.Declaration> {
    serializedPredicate: RegisteredReference;
    compiledCondition: string;
    compiledNegation: string;
    impliedBasis: null;
    expression: string;
    traverseAllows: TraverseAllows;
    errorContext: Predicate.ErrorContext;
    compiledErrorContext: string;
    traverseApply: TraverseApply;
    compile(js: NodeCompiler): void;
    reduceJsonSchema(base: JsonSchema.Constrainable, ctx: ToJsonSchema.Context): JsonSchema;
}
declare namespace Predicate {
    type Schema<predicate extends Predicate = Predicate> = NormalizedSchema<predicate> | predicate;
    interface NormalizedSchema<predicate extends Predicate = Predicate> extends BaseNormalizedSchema {
        readonly predicate: predicate;
    }
    interface Inner<predicate extends Predicate = Predicate> {
        readonly predicate: predicate;
    }
    interface ErrorContext extends BaseErrorContext<"predicate"> {
        readonly predicate?: Predicate;
    }
    interface Declaration extends declareNode<{
        kind: "predicate";
        schema: Schema;
        normalizedSchema: NormalizedSchema;
        inner: Inner;
        intersectionIsOpen: true;
        errorContext: ErrorContext;
    }> {
    }
    type Node = PredicateNode;
}
declare const Predicate: {
    implementation: nodeImplementationOf<Predicate.Declaration>;
    Node: typeof PredicateNode;
};
type Predicate<data = any> = (data: data, ctx: Traversal) => boolean;
declare namespace Predicate {
    type Casted<input = never, narrowed extends input = input> = (input: input, ctx: Traversal) => input is narrowed;
    type Castable<input = never, narrowed extends input = input> = Predicate<input> | Casted<input, narrowed>;
}
declare class ProtoNode extends InternalBasis<Proto.Declaration> {
    builtinName: BuiltinObjectKind | null;
    serializedConstructor: string;
    private readonly requiresInvalidDateCheck;
    private readonly isArrayProto;
    traverseAllows: TraverseAllows;
    compiledCondition: string;
    compiledNegation: string;
    protected innerToJsonSchema(ctx: ToJsonSchema.Context): JsonSchema;
    expression: string;
    get nestableExpression(): string;
    readonly domain = "object";
    get defaultShortDescription(): string;
}
declare namespace Proto {
    type Reference = Constructor | BuiltinObjectKind;
    type Schema<proto extends Reference = Reference> = proto | ExpandedSchema<proto>;
    interface NormalizedSchema<proto extends Constructor = Constructor> extends BaseNormalizedSchema {
        readonly proto: proto;
        readonly dateAllowsInvalid?: boolean;
    }
    interface ExpandedSchema<proto extends Reference = Reference> {
        readonly proto: proto;
        readonly dateAllowsInvalid?: boolean;
    }
    interface Inner<proto extends Constructor = Constructor> {
        readonly proto: proto;
        readonly dateAllowsInvalid?: boolean;
    }
    interface ErrorContext extends BaseErrorContext<"proto">, Inner {
    }
    interface Declaration extends declareNode<{
        kind: "proto";
        schema: Schema;
        normalizedSchema: NormalizedSchema;
        inner: Inner;
        errorContext: ErrorContext;
    }> {
    }
    type Node = ProtoNode;
}
declare const Proto: {
    implementation: nodeImplementationOf<Proto.Declaration>;
    Node: typeof ProtoNode;
    writeBadInvalidDateMessage: (actual: Constructor) => string;
    writeInvalidSchemaMessage: (actual: unknown) => string;
};
declare class IntersectionNode extends BaseRoot<Intersection.Declaration> {
    basis: nodeOfKind<Intersection.BasisKind> | null;
    prestructurals: array<nodeOfKind<PrestructuralKind>>;
    refinements: array<nodeOfKind<RefinementKind>>;
    structure: Structure.Node | undefined;
    expression: string;
    get shallowMorphs(): array<Morph>;
    get defaultShortDescription(): string;
    protected innerToJsonSchema(ctx: ToJsonSchema.Context): JsonSchema;
    traverseAllows: TraverseAllows;
    traverseApply: TraverseApply;
    compile(js: NodeCompiler): void;
}
declare namespace Intersection {
    type BasisKind = "domain" | "proto";
    type ChildKind = BasisKind | RefinementKind;
    type FlattenedChildKind = ChildKind | StructuralKind;
    type RefinementsInner = {
        [k in RefinementKind]?: intersectionChildInnerValueOf<k>;
    };
    interface Inner extends RefinementsInner {
        domain?: Domain.Node;
        proto?: Proto.Node;
        structure?: Structure.Node;
        predicate?: array<PredicateNode>;
    }
    namespace Inner {
        type mutable = makeRootAndArrayPropertiesMutable<Inner>;
    }
    type ConstraintsSchema<inferredBasis = any> = show<BaseNormalizedSchema & {
        domain?: Domain.Schema;
        proto?: Proto.Schema;
    } & conditionalRootOf<inferredBasis>>;
    type NormalizedSchema = Omit<ConstraintsSchema, StructuralKind | "undeclared">;
    type Schema<inferredBasis = any> = ConstraintsSchema<inferredBasis>;
    interface AstSchema extends BaseNormalizedSchema {
        intersection: readonly RootSchema[];
    }
    interface ErrorContext extends BaseErrorContext<"intersection">, Inner {
        errors: readonly ArkError[];
    }
    type Declaration = declareNode<{
        kind: "intersection";
        schema: Schema;
        normalizedSchema: NormalizedSchema;
        inner: Inner;
        reducibleTo: "intersection" | BasisKind;
        errorContext: ErrorContext;
        childKind: ChildKind;
    }>;
    type Node = IntersectionNode;
}
declare const Intersection: {
    implementation: nodeImplementationOf<{
        intersectionIsOpen: false;
        prerequisite: unknown;
        kind: "intersection";
        schema: Intersection.Schema;
        normalizedSchema: Intersection.NormalizedSchema;
        inner: Intersection.Inner;
        reducibleTo: "intersection" | Intersection.BasisKind;
        errorContext: Intersection.ErrorContext;
        childKind: Intersection.ChildKind;
    }>;
    Node: typeof IntersectionNode;
};
type ConditionalTerminalIntersectionRoot = {
    undeclared?: UndeclaredKeyBehavior;
};
type ConditionalTerminalIntersectionKey = keyof ConditionalTerminalIntersectionRoot;
type ConditionalIntersectionKey = ConstraintKind | ConditionalTerminalIntersectionKey;
type constraintKindOf<t> = {
    [k in ConstraintKind]: t extends Prerequisite<k> ? k : never;
}[ConstraintKind];
type conditionalIntersectionKeyOf<t> = constraintKindOf<t> | (t extends object ? "undeclared" : never);
type intersectionChildSchemaValueOf<k extends Intersection.FlattenedChildKind> = k extends OpenNodeKind ? listable<NodeSchema<k>> : NodeSchema<k>;
type conditionalSchemaValueOfKey<k extends ConditionalIntersectionKey> = k extends Intersection.FlattenedChildKind ? intersectionChildSchemaValueOf<k> : ConditionalTerminalIntersectionRoot[k & ConditionalTerminalIntersectionKey];
type intersectionChildInnerValueOf<k extends Intersection.FlattenedChildKind> = k extends OpenNodeKind ? readonly nodeOfKind<k>[] : nodeOfKind<k>;
type conditionalRootOf<t> = {
    [k in conditionalIntersectionKeyOf<t>]?: conditionalSchemaValueOfKey<k>;
};
declare namespace Constraint {
    interface Declaration extends BaseNodeDeclaration {
        kind: ConstraintKind;
    }
    type ReductionResult = BaseRoot | Disjoint | Intersection.Inner.mutable;
    interface Attachments {
        impliedBasis: BaseRoot | null;
        impliedSiblings?: array<BaseConstraint> | null;
    }
    type PrimitiveKind = Exclude<ConstraintKind, StructuralKind>;
}
declare abstract class BaseConstraint<out d extends Constraint.Declaration = Constraint.Declaration> extends BaseNode<d> {
    readonly [arkKind]: "constraint";
    constructor(attachments: UnknownAttachments, $: BaseScope);
    abstract readonly impliedBasis: BaseRoot | null;
    readonly impliedSiblings?: array<BaseConstraint>;
    intersect<r extends BaseConstraint>(r: r): intersectConstraintKinds<d["kind"], r["kind"]>;
}
declare abstract class InternalPrimitiveConstraint<d extends Constraint.Declaration> extends BaseConstraint<d> {
    abstract traverseAllows: TraverseAllows<d["prerequisite"]>;
    abstract readonly compiledCondition: string;
    abstract readonly compiledNegation: string;
    abstract reduceJsonSchema(base: JsonSchema.Constrainable, ctx: ToJsonSchema.Context): JsonSchema.Constrainable;
    traverseApply: TraverseApply<d["prerequisite"]>;
    compile(js: NodeCompiler): void;
    get errorContext(): d["errorContext"];
    get compiledErrorContext(): string;
}
type intersectConstraintKinds<l extends ConstraintKind, r extends ConstraintKind> = nodeOfKind<l | r | "unit" | "union"> | Disjoint | null;
declare const writeInvalidOperandMessage: <kind extends ConstraintKind, expected extends BaseRoot, actual extends BaseRoot>(kind: kind, expected: expected, actual: actual) => string;
type writeInvalidOperandMessage<kind extends ConstraintKind, actual> = `${Capitalize<kind>} operand must be ${describe<Prerequisite<kind>>} (was ${describe<Exclude<actual, Prerequisite<kind>>>})`;
declare abstract class BaseNode<out d extends BaseNodeDeclaration = BaseNodeDeclaration> extends Callable<(data: d["prerequisite"], ctx?: Traversal, onFail?: ArkErrors.Handler | null) => unknown, attachmentsOf<d>> {
    attachments: UnknownAttachments;
    $: BaseScope;
    onFail: ArkErrors.Handler | null;
    includesTransform: boolean;
    includesContextualPredicate: boolean;
    isCyclic: boolean;
    allowsRequiresContext: boolean;
    rootApplyStrategy: "allows" | "contextual" | "optimistic" | "branchedOptimistic";
    contextFreeMorph: ((data: unknown) => unknown) | undefined;
    rootApply: (data: unknown, onFail: ArkErrors.Handler | null) => unknown;
    referencesById: Record<string, BaseNode>;
    shallowReferences: BaseNode[];
    flatRefs: FlatRef[];
    flatMorphs: FlatRef<Morph.Node | Intersection.Node>[];
    allows: (data: d["prerequisite"]) => boolean;
    get shallowMorphs(): array<Morph>;
    constructor(attachments: UnknownAttachments, $: BaseScope);
    protected createRootApply(): this["rootApply"];
    abstract traverseAllows: TraverseAllows<d["prerequisite"]>;
    abstract traverseApply: TraverseApply<d["prerequisite"]>;
    abstract expression: string;
    abstract compile(js: NodeCompiler): void;
    readonly compiledMeta: string;
    protected cacheGetter<name extends keyof this>(name: name, value: this[name]): this[name];
    get description(): string;
    get references(): BaseNode[];
    readonly precedence: number;
    precompilation: string | undefined;
    assert: (data: d["prerequisite"], pipedFromCtx?: Traversal) => unknown;
    traverse(data: d["prerequisite"], pipedFromCtx?: Traversal): ArkErrors | {} | null | undefined;
    get in(): unknown;
    get rawIn(): BaseNode;
    get out(): unknown;
    get rawOut(): BaseNode;
    getIo(ioKind: "in" | "out"): BaseNode;
    toJSON(): JsonStructure;
    toString(): string;
    equals(r: unknown): boolean;
    ifEquals(r: unknown): BaseNode | undefined;
    hasKind<kind extends NodeKind>(kind: kind): this is nodeOfKind<kind>;
    assertHasKind<kind extends NodeKind>(kind: kind): nodeOfKind<kind>;
    hasKindIn<kinds extends NodeKind[]>(...kinds: kinds): this is nodeOfKind<kinds[number]>;
    assertHasKindIn<kinds extends NodeKind[]>(...kinds: kinds): nodeOfKind<kinds[number]>;
    isBasis(): this is nodeOfKind<BasisKind>;
    isConstraint(): this is BaseConstraint;
    isStructural(): this is nodeOfKind<StructuralKind>;
    isRefinement(): this is nodeOfKind<RefinementKind>;
    isRoot(): this is BaseRoot;
    isUnknown(): boolean;
    isNever(): boolean;
    hasUnit<value>(value: unknown): this is Unit.Node & {
        unit: value;
    };
    hasOpenIntersection(): this is nodeOfKind<OpenNodeKind>;
    get nestableExpression(): string;
    select<const selector extends NodeSelector.CompositeInput, predicate extends GuardablePredicate<NodeSelector.inferSelectKind<d["kind"], selector>>>(selector: NodeSelector.validateComposite<selector, predicate>): NodeSelector.infer<d["kind"], selector>;
    select<const selector extends NodeSelector.Single>(selector: selector): NodeSelector.infer<d["kind"], selector>;
    private _select;
    transform<mapper extends DeepNodeTransformation>(mapper: mapper, opts?: DeepNodeTransformOptions): nodeOfKind<reducibleKindOf<this["kind"]>> | Extract<ReturnType<mapper>, null>;
    protected _createTransformContext(opts: DeepNodeTransformOptions | undefined): DeepNodeTransformContext;
    protected _transform(mapper: DeepNodeTransformation, ctx: DeepNodeTransformContext): BaseNode | null;
    configureReferences(meta: TypeMeta.MappableInput.Internal, selector?: NodeSelector): this;
}
type KeyOrKeyNode = Key | BaseRoot;
type GettableKeyOrNode = KeyOrKeyNode | number;
type FlatRef<root extends BaseRoot = BaseRoot> = {
    path: array<KeyOrKeyNode>;
    node: root;
    propString: string;
};
type NodeSelector = NodeSelector.Single | NodeSelector.Composite;
declare namespace NodeSelector {
    type SelectableFn<input, returns, kind extends NodeKind = NodeKind> = {
        <const selector extends NodeSelector.CompositeInput, predicate extends GuardablePredicate<NodeSelector.inferSelectKind<kind, selector>>>(input: input, selector?: NodeSelector.validateComposite<selector, predicate>): returns;
        <const selector extends NodeSelector.Single>(input: input, selector?: selector): returns;
    };
    type Single = NodeSelector.Boundary | NodeSelector.Kind | GuardablePredicate<BaseNode>;
    type Boundary = "self" | "child" | "shallow" | "references";
    type Kind = NodeKind;
    type Method = "filter" | "assertFilter" | "find" | "assertFind";
    interface Composite {
        method?: Method;
        boundary?: Boundary;
        kind?: Kind;
        where?: GuardablePredicate<BaseNode>;
    }
    type Normalized = requireKeys<Composite, "method" | "boundary">;
    type CompositeInput = Omit<Composite, "where">;
    type BaseResult = BaseNode | BaseNode[] | undefined;
    type validateComposite<selector, predicate> = {
        [k in keyof selector]: k extends "where" ? predicate : conform<selector[k], CompositeInput[k & keyof CompositeInput]>;
    };
    type infer<selfKind extends NodeKind, selector> = applyMethod<selector extends NodeSelector.WhereCastInput<any, infer narrowed> ? narrowed : NodeSelector.inferSelectKind<selfKind, selector>, selector>;
    type BoundaryInput<b extends Boundary> = b | {
        boundary: b;
    };
    type KindInput<k extends Kind> = k | {
        kind: k;
    };
    type WhereCastInput<kindNode extends BaseNode, narrowed extends kindNode> = ((In: kindNode) => In is narrowed) | {
        where: (In: kindNode) => In is narrowed;
    };
    type inferSelectKind<selfKind extends NodeKind, selector> = selectKind<selfKind, selector> extends infer kind extends NodeKind ? NodeKind extends kind ? BaseNode : nodeOfKind<kind> : never;
    type selectKind<selfKind extends NodeKind, selector> = selector extends BoundaryInput<"self"> ? selfKind : selector extends KindInput<infer kind> ? kind : selector extends BoundaryInput<"child"> ? selfKind | childKindOf<selfKind> : NodeKind;
    type applyMethod<t, selector> = selector extends {
        method: infer method extends Method;
    } ? method extends "filter" ? t[] : method extends "assertFilter" ? [
        t,
        ...t[]
    ] : method extends "find" ? t | undefined : method extends "assertFind" ? t : never : t[];
}
type DeepNodeTransformOptions = {
    shouldTransform?: ShouldTransformFn;
    bindScope?: BaseScope;
    prereduced?: boolean;
    selected?: readonly BaseNode[] | undefined;
};
type ShouldTransformFn = (node: BaseNode, ctx: DeepNodeTransformContext) => boolean;
interface DeepNodeTransformContext extends DeepNodeTransformOptions {
    root: BaseNode;
    selected: readonly BaseNode[] | undefined;
    path: mutable<array<KeyOrKeyNode>>;
    seen: {
        [originalId: string]: (() => BaseNode | undefined) | undefined;
    };
    parseOptions: BaseParseOptions;
    undeclaredKeyHandling: UndeclaredKeyHandling | undefined;
}
type DeepNodeTransformation = <kind extends NodeKind>(kind: kind, innerWithMeta: Inner<kind> & {
    meta: ArkEnv.meta;
}, ctx: DeepNodeTransformContext) => NormalizedSchema<kind> | null;
interface NodeDeclarationsByKind extends BoundDeclarations {
    alias: Alias.Declaration;
    domain: Domain.Declaration;
    unit: Unit.Declaration;
    proto: Proto.Declaration;
    union: Union.Declaration;
    morph: Morph.Declaration;
    intersection: Intersection.Declaration;
    sequence: Sequence.Declaration;
    divisor: Divisor.Declaration;
    required: Required$2.Declaration;
    optional: Optional.Declaration;
    index: Index.Declaration;
    pattern: Pattern.Declaration;
    predicate: Predicate.Declaration;
    structure: Structure.Declaration;
}
interface NodesByKind extends BoundNodesByKind {
    alias: Alias.Node;
    union: Union.Node;
    morph: Morph.Node;
    intersection: Intersection.Node;
    unit: Unit.Node;
    proto: Proto.Node;
    domain: Domain.Node;
    divisor: Divisor.Node;
    pattern: Pattern.Node;
    predicate: Predicate.Node;
    required: Required$2.Node;
    optional: Optional.Node;
    index: Index.Node;
    sequence: Sequence.Node;
    structure: Structure.Node;
}
type nodeOfKind<kind extends NodeKind> = NodesByKind[kind];
type Declaration<kind extends NodeKind> = NodeDeclarationsByKind[kind];
type NodeSchema<kind extends NodeKind> = Declaration<kind>["schema"];
type RootSchema<kind extends RootKind = RootKind> = NodeSchema<kind>;
type NormalizedSchema<kind extends NodeKind> = Declaration<kind>["normalizedSchema"];
type childKindOf<kind extends NodeKind> = Declaration<kind>["childKind"];
type Prerequisite<kind extends NodeKind> = Declaration<kind>["prerequisite"];
type reducibleKindOf<kind extends NodeKind> = Declaration<kind>["reducibleTo"] extends NodeKind ? Declaration<kind>["reducibleTo"] : kind;
type Inner<kind extends NodeKind> = Declaration<kind>["inner"];
type errorContext<kind extends NodeKind> = Declaration<kind>["errorContext"];
type ContextualArgs = Record<string, BaseRoot | NodeId>;
type BaseParseOptions<prereduced extends boolean = boolean> = {
    alias?: string;
    prereduced?: prereduced;
    args?: ContextualArgs;
    id?: NodeId;
};
interface BaseParseContextInput extends BaseParseOptions {
    prefix: string;
    def: unknown;
}
interface AttachedParseContext {
    [arkKind]: "context";
    $: BaseScope;
    id: NodeId;
    phase: "unresolved" | "resolving" | "resolved";
}
interface BaseParseContext extends BaseParseContextInput, AttachedParseContext {
    id: NodeId;
}
interface NodeParseContextInput<kind extends NodeKind = NodeKind> extends BaseParseContextInput {
    kind: kind;
    def: NormalizedSchema<kind>;
}
interface NodeParseContext<kind extends NodeKind = NodeKind> extends NodeParseContextInput<kind>, AttachedParseContext {
    id: NodeId;
}
type NodeId = Brand<string, "NodeId">;
declare const nodesByRegisteredId: Record<NodeId, BaseNode | BaseParseContext | undefined>;
interface ArkSchemaRegistry extends ArkRegistry {
    intrinsic: typeof intrinsic;
    config: ArkSchemaConfig;
    defaultConfig: ResolvedConfig;
    resolvedConfig: ResolvedConfig;
    nodesByRegisteredId: typeof nodesByRegisteredId;
}
type nodeConfigForKind<kind extends NodeKind> = Readonly<show<{
    description?: DescriptionWriter<kind>;
} & (kind extends ArkErrorCode ? {
    expected?: ExpectedConfig<kind>;
    actual?: ActualConfig<kind>;
    problem?: ProblemConfig<kind>;
    message?: MessageConfig<kind>;
} : {})>>;
type NodeConfigsByKind = {
    [kind in NodeKind]: nodeConfigForKind<kind>;
};
type NodeConfig<kind extends NodeKind = NodeKind> = NodeConfigsByKind[kind];
interface UnknownErrorConfigs {
    expected?: ExpectedConfig;
    actual?: ActualConfig;
    problem?: ProblemConfig;
    message?: MessageConfig;
}
interface UnknownNodeConfig extends UnknownErrorConfigs {
    description?: DescriptionWriter;
}
type ResolvedUnknownNodeConfig = requireKeys<UnknownNodeConfig, "description">;
type CloneImplementation = <original extends object>(original: original) => original;
interface ArkSchemaConfig extends Partial<Readonly<NodeConfigsByKind>> {
    readonly jitless?: boolean;
    readonly clone?: boolean | CloneImplementation;
    readonly onUndeclaredKey?: UndeclaredKeyBehavior;
    readonly numberAllowsNaN?: boolean;
    readonly dateAllowsInvalid?: boolean;
    readonly exactOptionalPropertyTypes?: boolean;
    readonly onFail?: ArkErrors.Handler | null;
    readonly keywords?: Record<string, TypeMeta.Collapsible | undefined>;
    readonly toJsonSchema?: ToJsonSchema.Options;
}
type resolveConfig<config extends ArkSchemaConfig> = show<{
    [k in keyof ArkSchemaConfig]-?: k extends NodeKind ? Required<config[k]> : k extends "clone" ? CloneImplementation | false : k extends "keywords" ? Record<string, TypeMeta | undefined> : k extends "toJsonSchema" ? ToJsonSchema.Context : config[k];
} & Omit<config, keyof ArkSchemaConfig>>;
type ResolvedConfig = resolveConfig<ArkSchemaConfig>;
interface RegexExecArray<patternAndCaptures extends IndexedCaptures, namedCaptures extends NamedCaptures, flags extends Flags> extends DynamicBase<patternAndCaptures> {
    index: number;
    input: patternAndCaptures[0];
    indices: flags extends `${string}d${string}` ? RegexIndicesArray<patternAndCaptures, namedCaptures> : undefined;
    groups: keyof namedCaptures extends never ? undefined : namedCaptures;
}
type RegexIndexRange = [
    start: number,
    end: number
];
interface RegexIndicesArray<patternAndCaptures extends IndexedCaptures, namedCaptures extends NamedCaptures> extends DynamicBase<{
    [i in keyof patternAndCaptures]: RegexIndexRange;
}> {
    groups: keyof namedCaptures extends never ? undefined : {
        [k in keyof namedCaptures]: RegexIndexRange;
    };
}
type parseBuiltinQuantifier<s extends State, quantifier extends QuantifyingChar, unscanned extends string> = s["root"] extends "" ? s.error<writeUnmatchedQuantifierError<quantifier>> : quantifyBuiltin<s, quantifier, unscanned extends Scanner.shift<"?", infer lazyUnscanned> ? lazyUnscanned : unscanned>;
type quantifyBuiltin<s extends State, quantifier extends QuantifyingChar, unscanned extends string> = quantifier extends "?" ? s.pushQuantifier<s, 0, 1, unscanned> : quantifier extends "+" ? s.pushQuantifier<s, 1, null, unscanned> : quantifier extends "*" ? s.pushQuantifier<s, 0, null, unscanned> : never;
type ParsedRange = {
    min: number;
    max: number | null;
    unscanned: string;
};
declare namespace ParsedRange {
    type from<r extends ParsedRange> = r;
}
type skipPossibleQuestionMark<unscanned extends string> = unscanned extends `?${infer next}` ? next : unscanned;
type parsePossibleRangeString<unscanned extends string> = unscanned extends (`${infer l extends `${number}`},${infer r extends `${number}`}}${infer next}`) ? ParsedRange.from<{
    min: parseNonNegativeInteger<l>;
    max: parseNonNegativeInteger<r>;
    unscanned: skipPossibleQuestionMark<next>;
}> : unscanned extends `${infer l extends `${number}`},}${infer next}` ? ParsedRange.from<{
    min: parseNonNegativeInteger<l>;
    max: null;
    unscanned: skipPossibleQuestionMark<next>;
}> : unscanned extends `${infer l extends `${number}`}}${infer next}` ? ParsedRange.from<{
    min: parseNonNegativeInteger<l>;
    max: parseNonNegativeInteger<l>;
    unscanned: skipPossibleQuestionMark<next>;
}> : null;
type parseQuantifier<unscanned extends string, parsed extends ParsedRange> = unscanned extends `${infer range}${parsed["unscanned"]}` ? `{${range}` : never;
type parsePossibleRange<s extends State, unscanned extends string, parsed extends ParsedRange | null = parsePossibleRangeString<unscanned>> = parsed extends ParsedRange ? s["root"] extends "" ? s.error<writeUnmatchedQuantifierError<parseQuantifier<unscanned, parsed>>> : [
    parsed["min"],
    parsed["max"]
] extends ([
    never,
    unknown
] | [
    unknown,
    never
]) ? s.error<writeUnnaturalNumberQuantifierError<parseQuantifier<unscanned, parsed>>> : s.pushQuantifier<s, parsed["min"], parsed["max"], parsed["unscanned"] extends Scanner.shift<"?", infer lazyUnscanned> ? lazyUnscanned : parsed["unscanned"]> : s.shiftQuantifiable<s, "{", unscanned>;
type quantify<pattern extends string, min extends number, max extends number | null> = tryFastPath<pattern, min, max>;
type tryFastPath<pattern extends string, min extends number, max extends number | null> = max extends 0 ? "" : string extends pattern ? string : `${number}` extends pattern ? min extends 0 ? "" | `${number}` : `${number}` : min extends 0 ? max extends 1 ? "" | pattern : max extends number ? loopFromZero<pattern, max, "", [
]> : "" | `${pattern}${string}` : loopUntilMin<pattern, min, max, "", [
]>;
type loopFromZero<base extends string, max extends number, acc extends string, repetitions extends 1[]> = repetitions["length"] extends max ? acc : loopFromZero<base, max, acc | `${acc}${base}`, [
    ...repetitions,
    1
]>;
type loopUntilMin<base extends string, min extends number, max extends number | null, acc extends string, repetitions extends 1[]> = repetitions["length"] extends min ? max extends number ? loopUntilMax<base, min, max, acc, repetitions> : repetitions["length"] extends 0 ? acc | `${acc}${base}${string}` : `${acc}${string}` : loopUntilMin<base, min, max, `${acc}${base}`, [
    ...repetitions,
    1
]>;
type loopUntilMax<base extends string, min extends number, max extends number, acc extends string, repetitions extends 1[]> = repetitions["length"] extends max ? acc : loopUntilMax<base, min, max, acc | `${acc}${base}`, [
    ...repetitions,
    1
]>;
type QuantifyingChar = "*" | "+" | "?";
declare const writeUnmatchedQuantifierError: <quantifier extends string>(quantifier: quantifier) => writeUnmatchedQuantifierError<quantifier>;
type writeUnmatchedQuantifierError<quantifier extends string> = `Quantifier ${quantifier} requires a preceding token`;
declare const writeUnnaturalNumberQuantifierError: <quantifier extends string>(quantifier: quantifier) => writeUnnaturalNumberQuantifierError<quantifier>;
type writeUnnaturalNumberQuantifierError<quantifier extends string> = `Quantifier ${quantifier} must use natural numbers`;
interface State extends State.Group {
    unscanned: string;
    groups: State.Group[];
    flags: Flags;
}
declare namespace State {
    type from<s extends State> = s;
    type initialize<source extends string, flags extends Flags> = from<{
        unscanned: source;
        groups: [
        ];
        capture: never;
        branches: [
        ];
        sequence: SequenceTree.Empty;
        root: "";
        caseInsensitive: contains<flags, "i">;
        flags: flags;
    }>;
    enum UnnamedCaptureKind {
        indexed,
        lookaround,
        noncapturing
    }
    type CaptureKind = string | UnnamedCaptureKind;
    type Group = {
        capture: CaptureKind;
        branches: RegexAst[];
        sequence: RegexAst;
        root: RegexAst;
        caseInsensitive: boolean;
    };
    namespace Group {
        type from<g extends Group> = g;
        type pop<init extends Group, last extends Group[]> = [
            ...last,
            init
        ];
        type finalize<g extends Group> = g["branches"] extends [
        ] ? pushQuantifiable<g["sequence"], g["root"]> : [
            ...g["branches"],
            pushQuantifiable<g["sequence"], g["root"]>
        ] extends (infer branches extends RegexAst[]) ? finalizeUnion<branches, [
        ]> : never;
        type finalizeUnion<remaining extends RegexAst[], flattened extends RegexAst[]> = remaining extends ([
            infer head extends RegexAst,
            ...infer tail extends RegexAst[]
        ]) ? head extends UnionTree<infer headBranches> ? finalizeUnion<tail, [
            ...flattened,
            ...headBranches
        ]> : finalizeUnion<tail, [
            ...flattened,
            head
        ]> : UnionTree<flattened>;
    }
}
type Boundary = Anchor | "(" | ")" | "[" | "]";
type Anchor = "^" | "$";
type Control = QuantifyingChar | Boundary | "|" | "." | "{" | "-" | "\\";
type AnchorMarker<inner extends Anchor = Anchor> = `<${ZeroWidthSpace}${inner}${ZeroWidthSpace}>`;
type StartAnchorMarker = AnchorMarker<"^">;
type EndAnchorMarker = AnchorMarker<"$">;
type RegexAst = string | ReferenceNode | UnionTree | SequenceTree | GroupTree | QuantifierTree;
interface ReferenceNode<to extends string = string> {
    kind: "reference";
    to: to;
}
declare namespace ReferenceNode {
    type finalize<self extends ReferenceNode, ctx extends FinalizationContext, to extends string = self["to"]> = to extends NumberLiteral & keyof ctx["captures"] ? ctx["captures"][to] extends IncompleteCaptureGroup ? FinalizationResult.error<ctx, writeIncompleteReferenceError<to>> : FinalizationResult.from<{
        pattern: inferReference<ctx["captures"][to]>;
        ctx: ctx;
    }> : to extends keyof ctx["names"] ? ctx["names"][to] extends IncompleteCaptureGroup ? FinalizationResult.error<ctx, writeIncompleteReferenceError<to>> : FinalizationResult.from<{
        pattern: inferReference<ctx["names"][to]>;
        ctx: ctx;
    }> : FinalizationResult.error<ctx, writeUnresolvableBackreferenceMessage<to>>;
    type inferReference<to extends string | undefined> = to extends string ? to : "";
}
declare const writeIncompleteReferenceError: <ref extends string>(ref: ref) => writeIncompleteReferenceError<ref>;
type writeIncompleteReferenceError<ref extends string> = `Reference to incomplete group '${ref}' has no effect`;
interface SequenceTree<ast extends RegexAst[] = RegexAst[]> {
    kind: "sequence";
    ast: ast;
}
declare namespace SequenceTree {
    type Empty = SequenceTree<[
    ]>;
    type finalize<self extends SequenceTree, ctx extends FinalizationContext> = _finalize<self["ast"], "", ctx>;
    type _finalize<tree extends unknown[], pattern extends string, ctx extends FinalizationContext> = tree extends [
        infer head,
        ...infer tail
    ] ? finalizeTree<head, ctx> extends infer r ? r extends FinalizationResult ? _finalize<tail, appendNonRedundant<pattern, r["pattern"]>, r["ctx"]> : never : never : FinalizationResult.from<{
        pattern: pattern;
        ctx: ctx;
    }>;
}
interface UnionTree<ast extends RegexAst[] = RegexAst[]> {
    kind: "union";
    ast: ast;
}
declare namespace UnionTree {
    type finalize<self extends UnionTree, ctx extends FinalizationContext> = _finalize<self["ast"], [
    ], ctx>;
    type FinalizedBranch = {
        pattern: string;
        captures: IndexedCaptures;
        names: NamedCaptures;
    };
    namespace FinalizedBranch {
        type from<b extends FinalizedBranch> = b;
    }
    type _finalize<astBranches extends unknown[], acc extends FinalizedBranch[], ctx extends FinalizationContext> = astBranches extends [
        infer head,
        ...infer tail
    ] ? finalizeTree<head, ctx> extends infer r ? r extends FinalizationResult ? _finalize<tail, finalizeBranch<acc, ctx, r>, ctx> : never : never : finalizeBranches<keyof acc, acc, ctx>;
    type finalizeBranch<acc extends FinalizedBranch[], ctx extends FinalizationContext, r extends FinalizationResult> = [
        ...acc,
        FinalizedBranch.from<{
            pattern: r["pattern"];
            captures: finalizeBranchCaptures<acc, ctx, r>;
            names: r["ctx"]["names"];
        }>
    ];
    type finalizeBranchCaptures<acc extends FinalizedBranch[], ctx extends FinalizationContext, r extends FinalizationResult, branchCaptures extends IndexedCaptures = extractNewCaptures<ctx["captures"], r["ctx"]["captures"]>> = acc extends [
    ] ? branchCaptures : acc[0]["captures"] extends (infer firstCaptureBranch extends IndexedCaptures) ? branchCaptures extends [
    ] ? {
        [i in keyof firstCaptureBranch]: undefined;
    } : [
        ...{
            [i in keyof firstCaptureBranch]: undefined;
        },
        ...branchCaptures
    ] : never;
    type finalizeBranches<i, acc extends FinalizedBranch[], ctx extends FinalizationContext> = i extends keyof acc & NumberLiteral ? FinalizationResult.from<{
        pattern: acc[i]["pattern"];
        ctx: {
            flags: ctx["flags"];
            captures: [
                ...ctx["captures"],
                ...acc[i]["captures"]
            ];
            names: {
                [k in unionKeyOf<acc[number]["names"]>]: k extends (keyof acc[i]["names"]) ? acc[i]["names"][k] : undefined;
            };
            errors: ctx["errors"];
        };
    }> : never;
}
type CapturedGroupKind = string | State.UnnamedCaptureKind.indexed;
type IncompleteCaptureGroup = noSuggest<"incompleteCaptureGroup">;
interface GroupTree<ast extends RegexAst = RegexAst, capture extends CapturedGroupKind = CapturedGroupKind> {
    kind: "group";
    capture: capture;
    ast: ast;
}
declare namespace GroupTree {
    type finalize<self extends GroupTree, ctx extends FinalizationContext> = finalizeGroupAst<self, ctx> extends infer r ? r extends FinalizationResult ? finalizeGroupResult<self, ctx, r> : never : never;
    type finalizeGroupAst<self extends GroupTree, ctx extends FinalizationContext> = finalizeTree<self["ast"], self["capture"] extends string ? {
        captures: [
            ...ctx["captures"],
            IncompleteCaptureGroup
        ];
        names: ctx["names"] & {
            [_ in self["capture"]]: IncompleteCaptureGroup;
        };
        flags: ctx["flags"];
        errors: ctx["errors"];
    } : self["capture"] extends State.UnnamedCaptureKind.indexed ? {
        captures: [
            ...ctx["captures"],
            IncompleteCaptureGroup
        ];
        names: ctx["names"];
        flags: ctx["flags"];
        errors: ctx["errors"];
    } : ctx>;
    type finalizeGroupResult<self extends GroupTree, ctx extends FinalizationContext, r extends FinalizationResult> = FinalizationResult.from<{
        pattern: r["pattern"];
        ctx: self["capture"] extends string ? finalizeNamedCapture<self["capture"], ctx["captures"]["length"], r["pattern"], r["ctx"]> : self["capture"] extends State.UnnamedCaptureKind.indexed ? finalizeUnnamedCapture<ctx["captures"]["length"], r["pattern"], r["ctx"]> : r["ctx"];
    }>;
    type finalizeNamedCapture<name extends string, index extends number, pattern extends string, ctx extends FinalizationContext> = FinalizationContext.from<{
        captures: setIndex<ctx["captures"], index, anchorsAway<pattern>>;
        names: {
            [k in keyof ctx["names"]]: k extends name ? anchorsAway<pattern> : ctx["names"][k];
        };
        flags: ctx["flags"];
        errors: ctx["errors"];
    }>;
    type finalizeUnnamedCapture<index extends number, pattern extends string, ctx extends FinalizationContext> = FinalizationContext.from<{
        captures: setIndex<ctx["captures"], index, anchorsAway<pattern>>;
        names: ctx["names"];
        flags: ctx["flags"];
        errors: ctx["errors"];
    }>;
}
interface QuantifierTree<ast extends RegexAst = RegexAst> {
    kind: "quantifier";
    ast: ast;
    min: number;
    max: number | null;
}
declare namespace QuantifierTree {
    type finalize<self extends QuantifierTree, ctx extends FinalizationContext> = finalizeTree<self["ast"], ctx> extends infer r extends FinalizationResult ? finalizeQuantifierResult<self, ctx, r> : never;
    type finalizeQuantifierResult<self extends QuantifierTree, ctx extends FinalizationContext, r extends FinalizationResult, quantifiedCaptures extends IndexedCaptures = extractNewCaptures<ctx["captures"], r["ctx"]["captures"]>> = self["min"] extends 0 ? quantifiedCaptures extends [
    ] ? finalizeNonZeroMinQuantified<self, r> : finalizeZeroMinQuantifiedWithCaptures<self, ctx, r, quantifiedCaptures> : finalizeNonZeroMinQuantified<self, r>;
    type finalizeNonZeroMinQuantified<self extends QuantifierTree, r extends FinalizationResult> = FinalizationResult.from<{
        pattern: quantify<r["pattern"], self["min"], self["max"]>;
        ctx: r["ctx"];
    }>;
    type finalizeZeroMinQuantifiedWithCaptures<self extends QuantifierTree, ctx extends FinalizationContext, r extends FinalizationResult, quantifiedCaptures extends IndexedCaptures> = finalizeZeroQuantified<ctx, r, quantifiedCaptures> | finalizeOnePlusQuantified<self["max"], r>;
    type finalizeZeroQuantified<ctx extends FinalizationContext, r extends FinalizationResult, quantifiedCaptures extends IndexedCaptures> = FinalizationResult.from<{
        pattern: "";
        ctx: {
            captures: [
                ...ctx["captures"],
                ...{
                    [i in keyof quantifiedCaptures]: undefined;
                }
            ];
            flags: r["ctx"]["flags"];
            names: zeroQuantifiedNames<ctx["names"], r["ctx"]["names"]>;
            errors: r["ctx"]["errors"];
        };
    }>;
    type zeroQuantifiedNames<base extends NamedCaptures, result extends NamedCaptures> = {
        [k in keyof result]: k extends keyof base ? result[k] : undefined;
    } & unknown;
    type finalizeOnePlusQuantified<max extends number | null, r extends FinalizationResult> = max extends 1 ? r : FinalizationResult.from<{
        pattern: quantify<r["pattern"], 1, max>;
        ctx: r["ctx"];
    }>;
}
type pushQuantifiable<sequence extends RegexAst, root extends RegexAst> = root extends "" ? sequence : sequence extends string ? sequence extends "" ? root : root extends string ? appendNonRedundant<sequence, root> : SequenceTree<[
    sequence,
    root
]> : sequence extends SequenceTree ? pushToSequence<sequence, root> : SequenceTree<[
    sequence,
    root
]>;
type pushToSequence<sequence extends SequenceTree, root extends RegexAst> = sequence extends SequenceTree.Empty ? root : root extends SequenceTree ? SequenceTree<[
    ...sequence["ast"],
    ...root["ast"]
]> : SequenceTree<[
    ...sequence["ast"],
    root
]>;
type extractNewCaptures<base extends IndexedCaptures, result extends IndexedCaptures> = result extends readonly [
    ...base,
    ...infer elements extends IndexedCaptures
] ? elements : [
];
interface FinalizationContext extends Required<RegexContext> {
    errors: ErrorMessage[];
}
declare namespace FinalizationContext {
    type from<ctx extends FinalizationContext> = ctx;
}
type FinalizationResult = {
    pattern: string;
    ctx: FinalizationContext;
};
declare namespace FinalizationResult {
    type from<r extends FinalizationResult> = r;
    type error<ctx extends FinalizationContext, message extends string> = from<{
        pattern: string;
        ctx: {
            captures: ctx["captures"];
            names: ctx["names"];
            flags: ctx["flags"];
            errors: [
                ...ctx["errors"],
                ErrorMessage<message>
            ];
        };
    }>;
}
type finalizeTree<tree, ctx extends FinalizationContext> = tree extends string ? FinalizationResult.from<{
    pattern: tree;
    ctx: ctx;
}> : tree extends SequenceTree ? SequenceTree.finalize<tree, ctx> : tree extends UnionTree ? UnionTree.finalize<tree, ctx> : tree extends GroupTree ? GroupTree.finalize<tree, ctx> : tree extends QuantifierTree ? QuantifierTree.finalize<tree, ctx> : tree extends ReferenceNode ? ReferenceNode.finalize<tree, ctx> : never;
type anchorsAway<pattern extends string> = pattern extends `${StartAnchorMarker}${infer startStripped}` ? startStripped extends `${infer bothStripped}${EndAnchorMarker}` ? bothStripped : startStripped : pattern extends `${infer endStripped}${EndAnchorMarker}` ? endStripped : pattern;
type appendNonRedundant<base extends string, suffix extends string> = string extends base ? string extends suffix ? string : `${base}${suffix}` : `${number}` extends base ? `${number}` extends suffix ? `${number}` : `${base}${suffix}` : `${base}${suffix}`;
type parseEscape<s extends State, unscanned extends string> = unscanned extends Scanner.shift<infer char, infer nextUnscanned> ? char extends NonZeroDigit ? parseNumericBackreference<s, unscanned> : char extends "k" ? parseNamedBackreference<s, nextUnscanned> : char extends UnicodePropertyChar ? parseUnicodeProperty<s, char, nextUnscanned> : parseSingleEscapedCharacter<s, char, nextUnscanned> : s.error<trailingBackslashMessage>;
type parseNumericBackreference<s extends State, fullUnscanned extends string> = Scanner.shiftUntilNot<fullUnscanned, StringDigit> extends (Scanner.shiftResult<infer ref, infer remaining>) ? s.shiftQuantifiable<s, ReferenceNode<ref>, remaining> : never;
type parseNamedBackreference<s extends State, unscanned extends string> = unscanned extends `<${infer ref}>${infer following}` ? s.shiftQuantifiable<s, ReferenceNode<ref>, following> : s.error<missingBackreferenceNameMessage>;
type parseUnicodeProperty<s extends State, char extends UnicodePropertyChar, unscanned extends string> = unscanned extends `{${string}}${infer following}` ? s.shiftQuantifiable<s, string, following> : s.error<writeInvalidUnicodePropertyMessage<char>>;
type parseSingleEscapedCharacter<s extends State, char extends string, remaining extends string> = parseEscapedChar<char> extends infer result extends string ? result extends ErrorMessage ? s.error<result> : s.shiftQuantifiable<s, result, remaining> : never;
type parseEscapedChar<char extends string> = char extends RegexClassChar ? string : char extends "d" ? `${number}` : char extends "s" ? WhitespaceChar : char extends BoundaryChar ? "" : char extends Control ? char : char extends "c" ? ErrorMessage<caretNotationMessage> : char extends StringEscapableChar ? ErrorMessage<writeStringEscapableMessage<char>> : ErrorMessage<writeUnnecessaryEscapeMessage<char>>;
declare const trailingBackslashMessage = "A regex cannot end with \\";
type trailingBackslashMessage = typeof trailingBackslashMessage;
declare const writeUnresolvableBackreferenceMessage: <ref extends string | number>(ref: ref) => writeUnresolvableBackreferenceMessage<ref>;
type writeUnresolvableBackreferenceMessage<ref extends string | number> = `Group ${ref} does not exist`;
declare const missingBackreferenceNameMessage = "\\k must be followed by a named reference like <name>";
type missingBackreferenceNameMessage = typeof missingBackreferenceNameMessage;
declare const writeInvalidUnicodePropertyMessage: <char extends UnicodePropertyChar>(char: char) => writeInvalidUnicodePropertyMessage<char>;
type writeInvalidUnicodePropertyMessage<char extends UnicodePropertyChar> = `\\${char} must be followed by a property like \\${char}{Emoji_Presentation}`;
declare const writeUnnecessaryEscapeMessage: <char extends string>(char: char) => writeUnnecessaryEscapeMessage<char>;
type writeUnnecessaryEscapeMessage<char extends string> = `Escape preceding ${char} is unnecessary and should be removed.`;
declare const writeStringEscapableMessage: (char: StringEscapableChar) => "\\v should be specified with a single backslash like regex('\\n')" | "\\u should be specified with a single backslash like regex('\\n')" | "\\0 should be specified with a single backslash like regex('\\n')" | "\\t should be specified with a single backslash like regex('\\n')" | "\\n should be specified with a single backslash like regex('\\n')" | "\\r should be specified with a single backslash like regex('\\n')" | "\\f should be specified with a single backslash like regex('\\n')" | "\\x should be specified with a single backslash like regex('\\n')";
type writeStringEscapableMessage<char extends StringEscapableChar> = `\\${char} should be specified with a single backslash like regex('\n')`;
declare const caretNotationMessage = "\\\\cX notation is not supported. Use hex (\\\\x) or unicode (\\\\u) instead.";
type caretNotationMessage = "\\cX notation is not supported. Use hex (\\x) or unicode (\\u) instead.";
type StringEscapableChar = "t" | "n" | "r" | "f" | "v" | "0" | "x" | "u";
type RegexClassChar = "w" | "W" | "D" | "S";
type BoundaryChar = "b" | "B";
type UnicodePropertyChar = "p" | "P";
type NonZeroDigit = "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9";
type StringDigit = "0" | NonZeroDigit;
type parseCharset<s extends State, unscanned extends string> = Scanner.shiftUntilEscapable<unscanned, "]", Backslash> extends (Scanner.shiftResult<infer scanned, infer nextUnscanned>) ? nextUnscanned extends `]${infer remaining}` ? scanned extends Scanner.shift<"^", string> ? s.shiftQuantifiable<s, string, remaining> : parseNonNegatedCharset<scanned, never, null> extends (infer result extends string) ? [
    result
] extends [
    never
] ? s.error<emptyCharacterSetMessage> : s.shiftQuantifiable<s, result, remaining> : never : s.error<writeUnclosedGroupMessage<"]">> : never;
type parseNonNegatedCharset<chars extends string, set extends string, lastChar extends string | null> = parseChar<chars> extends Scanner.shiftResult<infer result, infer unscanned> ? result extends UnescapedDashMarker ? parseDash<unscanned, set, lastChar> : result extends ErrorMessage ? result : parseNonNegatedCharset<unscanned, set | result, result> : set;
type parseDash<unscanned extends string, set extends string, lastChar extends string | null> = lastChar extends string ? parseChar<unscanned> extends (Scanner.shiftResult<infer rangeEnd, infer next>) ? parseNonNegatedCharset<next, set | inferRange<lastChar, rangeEnd>, null> : set | "-" : parseNonNegatedCharset<unscanned, set | "-", "-">;
type inferRange<start extends string, end extends string> = start | end extends StringDigit ? `${number}` : string;
type UnescapedDashMarker = noSuggest<"dash">;
type parseChar<unscanned extends string> = unscanned extends Scanner.shift<infer lookahead, infer next> ? lookahead extends Backslash ? next extends Scanner.shift<infer escaped, infer postEscaped> ? Scanner.shiftResult<parseEscapedChar<escaped>, postEscaped> : never : Scanner.shiftResult<lookahead extends "-" ? UnescapedDashMarker : lookahead, next> : null;
declare const emptyCharacterSetMessage = "Empty character set [] is unsatisfiable";
type emptyCharacterSetMessage = typeof emptyCharacterSetMessage;
type LookaroundChar = "=" | "!";
type ModifiableFlag = "i" | "m" | "s";
type parseGroup<s extends State, unscanned extends string> = unscanned extends Scanner.shift<infer lookahead, infer next> ? lookahead extends "?" ? parseNonCapturingGroup<s, next> : s.pushGroup<s, State.UnnamedCaptureKind.indexed, unscanned, undefined> : s.error<writeUnclosedGroupMessage<")">>;
type parseNonCapturingGroup<s extends State, unscanned extends string> = unscanned extends Scanner.shift<infer lookahead, infer next> ? lookahead extends ":" ? s.pushGroup<s, State.UnnamedCaptureKind.noncapturing, next, undefined> : lookahead extends LookaroundChar ? s.pushGroup<s, State.UnnamedCaptureKind.lookaround, next, undefined> : lookahead extends "<" ? parseNamedGroupOrLookbehind<s, next> : shiftModifiers<unscanned> extends (ShiftedModifiers<infer flags, infer negated, infer following>) ? following extends ErrorMessage<infer message> ? s.error<message> : s.pushGroup<s, State.UnnamedCaptureKind.noncapturing, following, "i" extends flags ? true : "i" extends negated ? false : undefined> : never : s.error<writeUnclosedGroupMessage<")">>;
type ShiftedModifiers<flags extends ModifiableFlag = ModifiableFlag, negated extends ModifiableFlag = ModifiableFlag, unscanned extends string = string> = [
    ParsedModifiers<flags, negated>,
    unscanned
];
type ParsedModifiers<flags extends ModifiableFlag = ModifiableFlag, negated extends ModifiableFlag = ModifiableFlag> = {
    flags: flags;
    negated: negated;
};
type shiftModifiers<unscanned extends string> = Scanner.shiftUntil<unscanned, ":" | ")"> extends (Scanner.shiftResult<infer scanned, infer next>) ? next extends Scanner.shift<infer terminator, infer following> ? terminator extends ":" ? parseModifiers<scanned> extends (ParsedModifiers<infer flags, infer negated>) ? ShiftedModifiers<flags, negated, following> : ShiftedModifiers<never, never, ErrorMessage<parseModifiers<scanned> & string>> : ShiftedModifiers<never, never, ErrorMessage<unescapedLiteralQuestionMarkMessage>> : ShiftedModifiers<never, never, ErrorMessage<writeUnclosedGroupMessage<")">>> : never;
type parseModifiers<unscanned extends string> = _parseModifiers<unscanned, never, never>;
type _parseModifiers<unscanned extends string, flags extends ModifiableFlag, negated extends ModifiableFlag> = unscanned extends Scanner.shift<infer lookahead, infer next> ? lookahead extends "-" ? [
    negated
] extends [
    never
] ? next extends Scanner.shift<infer modifier, infer next> ? modifier extends ModifiableFlag ? modifier extends flags | negated ? writeDuplicateModifierMessage<modifier> : _parseModifiers<next, flags, negated | modifier> : writeInvalidModifierMessage<modifier> : missingNegatedModifierMessage : multipleModifierDashesMessage : lookahead extends ModifiableFlag ? lookahead extends flags | negated ? writeDuplicateModifierMessage<lookahead> : [
    negated
] extends [
    never
] ? _parseModifiers<next, flags | lookahead, negated> : _parseModifiers<next, flags, negated | lookahead> : writeInvalidModifierMessage<lookahead> : ParsedModifiers<flags, negated>;
declare const writeDuplicateModifierMessage: <modifier extends ModifiableFlag>(modifier: modifier) => writeDuplicateModifierMessage<modifier>;
type writeDuplicateModifierMessage<modifier extends ModifiableFlag> = `Modifier ${modifier} cannot appear multiple times in a single group`;
declare const multipleModifierDashesMessage = "Modifiers can include at most one '-' to negate subsequent flags";
type multipleModifierDashesMessage = typeof multipleModifierDashesMessage;
declare const missingNegatedModifierMessage = "- must be followed by the modifier flag to negate ('i', 'm' or 's')";
type missingNegatedModifierMessage = typeof missingNegatedModifierMessage;
declare const writeInvalidModifierMessage: <char extends string>(char: char) => writeInvalidModifierMessage<char>;
type writeInvalidModifierMessage<char extends string> = `Modifier flag ${char} must be 'i', 'm' or 's'`;
type parseNamedGroupOrLookbehind<s extends State, unscanned extends string> = unscanned extends Scanner.shift<LookaroundChar, infer next> ? s.pushGroup<s, State.UnnamedCaptureKind.lookaround, next, undefined> : shiftNamedGroup<unscanned> extends (Scanner.shiftResult<infer name, infer following>) ? s.pushGroup<s, name, following, undefined> : s.error<writeUnclosedGroupMessage<")">>;
type shiftNamedGroup<unscanned extends string> = unscanned extends `${infer name}>${infer next}` ? name extends "" ? Scanner.shiftResult<"", ErrorMessage<unnamedCaptureGroupMessage>> : Scanner.shiftResult<name, next> : Scanner.shiftResult<"", ErrorMessage<writeUnclosedGroupMessage<">">>>;
declare const unnamedCaptureGroupMessage = "Capture group <> requires a name";
type unnamedCaptureGroupMessage = typeof unnamedCaptureGroupMessage;
declare const unescapedLiteralQuestionMarkMessage = "literal ? must be escaped at the start of a group";
type unescapedLiteralQuestionMarkMessage = typeof unescapedLiteralQuestionMarkMessage;
type parseState<s extends State> = s["unscanned"] extends ErrorMessage ? s["unscanned"] : s["unscanned"] extends "" ? s.finalize<s> : parseState<next$1<s>>;
type next$1<s extends State> = s["unscanned"] extends Scanner.shift<infer lookahead, infer unscanned> ? lookahead extends "." ? s.shiftQuantifiable<s, string, unscanned> : lookahead extends Backslash ? parseEscape<s, unscanned> : lookahead extends "|" ? s.finalizeBranch<s, unscanned> : lookahead extends Anchor ? s.anchor<s, AnchorMarker<lookahead>, unscanned> : lookahead extends "(" ? parseGroup<s, unscanned> : lookahead extends ")" ? s.popGroup<s, unscanned> : lookahead extends QuantifyingChar ? parseBuiltinQuantifier<s, lookahead, unscanned> : lookahead extends "{" ? parsePossibleRange<s, unscanned> : lookahead extends "[" ? parseCharset<s, unscanned> : s.shiftQuantifiable<s, maybeSplitCasing<s["caseInsensitive"], lookahead>, unscanned> : never;
type maybeSplitCasing<caseInsensitive extends boolean, char extends string> = caseInsensitive extends false ? char : Lowercase<char> extends Uppercase<char> ? char : UnionTree<[
    Lowercase<char>,
    Capitalize<char>
]>;
type IndexedCaptures = Array<string | undefined>;
type NamedCaptures = Record<string, string | undefined>;
type UnicodeFlag = "v" | "u";
type Flags = `${"d" | ""}${"g" | ""}${"i" | ""}${"m" | ""}${"s" | ""}${UnicodeFlag | ""}${"y" | ""}`;
type RegexContext = {
    flags?: Flags;
    captures?: IndexedCaptures;
    names?: NamedCaptures;
};
interface Regex<out pattern extends string = string, out ctx extends RegexContext = RegexContext> extends RegExp {
    [inferred]: pattern;
    infer: pattern;
    inferCaptures: ctx["captures"] extends IndexedCaptures ? ctx["captures"] : [
    ];
    inferNamedCaptures: ctx["names"] extends NamedCaptures ? ctx["names"] : {};
    inferExecArray: RegexExecArray<[
        pattern,
        ...this["inferCaptures"]
    ], this["inferNamedCaptures"], this["flags"]>;
    flags: ctx["flags"] extends Flags ? ctx["flags"] : "";
    test(s: string): s is pattern;
    exec(s: string): this["inferExecArray"] | null;
    exec(s: string): never;
}
interface RegexParser {
    <src extends string, flags extends Flags = "">(src: regex.validate<src, flags>, flags?: flags): regex.parse<src, flags>;
    as: <pattern extends string = string, ctx extends RegexContext = {}>(src: string, flags?: Flags) => Regex<pattern, ctx>;
}
declare const regex: RegexParser;
type regex<pattern extends string = string, ctx extends RegexContext = RegexContext> = Regex<pattern, ctx>;
declare namespace regex {
    type infer<src extends string, flags extends Flags = ""> = parse<src, flags> extends Regex<infer pattern> ? pattern : never;
    type validate<src extends string, flags extends Flags = ""> = parse<src, flags> extends infer e extends ErrorMessage ? e : src;
    type parse<src extends string, flags extends Flags = ""> = parseState<State.initialize<src, flags>>;
}
type StringifiablePrefixOperator = "keyof";
declare const minComparators: {
    readonly ">": true;
    readonly ">=": true;
};
type MinComparator = keyof typeof minComparators;
declare const maxComparators: {
    readonly "<": true;
    readonly "<=": true;
};
type MaxComparator = keyof typeof maxComparators;
declare const comparators: {
    ">": boolean;
    ">=": boolean;
    "<": boolean;
    "<=": boolean;
    "==": boolean;
};
type Comparator = keyof typeof comparators;
type InvertedComparators = {
    "<": ">";
    ">": "<";
    "<=": ">=";
    ">=": "<=";
    "==": "==";
};
type BranchOperator = "&" | "|" | "|>";
type OpenLeftBound = {
    limit: LimitLiteral;
    comparator: MinComparator;
};
declare const writeOpenRangeMessage: <min extends LimitLiteral, comparator extends MinComparator>(min: min, comparator: comparator) => writeOpenRangeMessage<min, comparator>;
type writeOpenRangeMessage<min extends LimitLiteral, comparator extends MinComparator> = `Left bounds are only valid when paired with right bounds (try ...${comparator}${min})`;
type writeUnpairableComparatorMessage<comparator extends Comparator> = `Left-bounded expressions must specify their limits using < or <= (was ${comparator})`;
declare const writeUnpairableComparatorMessage: <comparator extends Comparator>(comparator: comparator) => writeUnpairableComparatorMessage<comparator>;
declare const writeMultipleLeftBoundsMessage: <openLimit extends LimitLiteral, openComparator extends MinComparator, limit extends LimitLiteral, comparator extends MinComparator>(openLimit: openLimit, openComparator: openComparator, limit: limit, comparator: comparator) => writeMultipleLeftBoundsMessage<openLimit, openComparator, limit, comparator>;
type writeMultipleLeftBoundsMessage<openLimit extends LimitLiteral, openComparator extends MinComparator, limit extends LimitLiteral, comparator extends MinComparator> = `An expression may have at most one left bound (parsed ${openLimit}${InvertedComparators[openComparator]}, ${limit}${InvertedComparators[comparator]})`;
declare const terminatingChars: {
    readonly " ": 1;
    readonly "\n": 1;
    readonly "\t": 1;
    readonly "<": 1;
    readonly ">": 1;
    readonly "=": 1;
    readonly "|": 1;
    readonly "&": 1;
    readonly ")": 1;
    readonly "[": 1;
    readonly "%": 1;
    readonly ",": 1;
    readonly ":": 1;
    readonly "?": 1;
    readonly "#": 1;
};
type TerminatingChar = keyof typeof terminatingChars;
declare const finalizingLookaheads: {
    readonly ">": 1;
    readonly ",": 1;
    readonly "": 1;
    readonly "=": 1;
    readonly "?": 1;
};
type FinalizingLookahead = keyof typeof finalizingLookaheads;
declare const lookaheadIsFinalizing: (lookahead: string, unscanned: string) => lookahead is ">" | "," | "=" | "?";
type lookaheadIsFinalizing<lookahead extends string, unscanned extends string> = lookahead extends ">" ? unscanned extends `=${infer nextUnscanned}` ? nextUnscanned extends `=${string}` ? true : false : Scanner.skipWhitespace<unscanned> extends ("" | `${TerminatingChar}${string}`) ? true : false : lookahead extends "=" ? unscanned extends `=${string}` ? false : true : lookahead extends "," | "?" ? true : false;
type InfixToken = Comparator | "|" | "&" | "%" | ":" | "=>" | "|>" | "#" | "@" | "=";
type PostfixToken = "[]" | "?";
type BranchState$1 = {
    prefixes: StringifiablePrefixOperator[];
    leftBound: OpenLeftBound | null;
    intersection: BaseRoot | null;
    union: BaseRoot | null;
    pipe: BaseRoot | null;
};
type RootedRuntimeState = requireKeys<RuntimeState, "root">;
declare class RuntimeState {
    root: BaseRoot | undefined;
    branches: BranchState$1;
    finalizer: FinalizingLookahead | undefined;
    groups: BranchState$1[];
    scanner: Scanner;
    ctx: BaseParseContext;
    constructor(scanner: Scanner, ctx: BaseParseContext);
    error(message: string): never;
    hasRoot(): this is RootedRuntimeState;
    setRoot(root: BaseRoot): void;
    unsetRoot(): this["root"];
    constrainRoot(...args: Parameters<BaseRoot<any>["constrain"]>): void;
    finalize(finalizer: FinalizingLookahead): void;
    reduceLeftBound(limit: LimitLiteral, comparator: Comparator): void;
    finalizeBranches(): void;
    finalizeGroup(): void;
    addPrefix(prefix: StringifiablePrefixOperator): void;
    applyPrefixes(): void;
    pushRootToBranch(token: BranchOperator): void;
    parseUntilFinalizer(): RootedRuntimeState;
    parseOperator(this: RootedRuntimeState): void;
    parseOperand(): void;
    private assertRangeUnset;
    reduceGroupOpen(): void;
    previousOperator(): MinComparator | StringifiablePrefixOperator | InfixToken | undefined;
    shiftedBy(count: number): this;
}
type StaticState = {
    root: unknown;
    branches: BranchState;
    groups: BranchState[];
    finalizer: FinalizingLookahead | ErrorMessage | undefined;
    scanned: string;
    unscanned: string;
};
type BranchState = {
    prefixes: StringifiablePrefixOperator[];
    leftBound: OpenLeftBound | undefined;
    intersection: unknown;
    pipe: unknown;
    union: unknown;
};
declare namespace s {
    type initialize<def extends string> = from<{
        root: undefined;
        branches: initialBranches;
        groups: [
        ];
        finalizer: undefined;
        scanned: "";
        unscanned: def;
    }>;
    type error<message extends string> = from<{
        root: ErrorMessage<message>;
        branches: initialBranches;
        groups: [
        ];
        finalizer: ErrorMessage<message>;
        scanned: "";
        unscanned: "";
    }>;
    type completion<text extends string> = from<{
        root: Completion<text>;
        branches: initialBranches;
        groups: [
        ];
        finalizer: Completion<text>;
        scanned: "";
        unscanned: "";
    }>;
    type initialBranches = branchesFrom<{
        prefixes: [
        ];
        leftBound: undefined;
        intersection: undefined;
        pipe: undefined;
        union: undefined;
    }>;
    type updateScanned<previousScanned extends string, previousUnscanned extends string, updatedUnscanned extends string> = previousUnscanned extends `${infer justScanned}${updatedUnscanned}` ? `${previousScanned}${justScanned}` : previousScanned;
    type setRoot<s extends StaticState, root, unscanned extends string = s["unscanned"]> = from<{
        root: root;
        branches: s["branches"];
        groups: s["groups"];
        finalizer: s["finalizer"];
        scanned: updateScanned<s["scanned"], s["unscanned"], unscanned>;
        unscanned: unscanned;
    }>;
    type addPrefix<s extends StaticState, prefix extends StringifiablePrefixOperator, unscanned extends string = s["unscanned"]> = from<{
        root: s["root"];
        branches: {
            prefixes: [
                ...s["branches"]["prefixes"],
                prefix
            ];
            leftBound: s["branches"]["leftBound"];
            intersection: s["branches"]["intersection"];
            pipe: s["branches"]["pipe"];
            union: s["branches"]["union"];
        };
        groups: s["groups"];
        finalizer: s["finalizer"];
        scanned: updateScanned<s["scanned"], s["unscanned"], unscanned>;
        unscanned: unscanned;
    }>;
    type reduceBranch<s extends StaticState, token extends BranchOperator, unscanned extends string> = s["branches"]["leftBound"] extends {} ? openRangeError<s["branches"]["leftBound"]> : from<{
        root: undefined;
        branches: {
            prefixes: [
            ];
            leftBound: undefined;
            intersection: token extends "&" ? mergeToIntersection<s> : undefined;
            union: token extends "|" ? mergeToUnion<s> : token extends "|>" ? undefined : s["branches"]["union"];
            pipe: token extends "|>" ? mergeToPipe<s> : s["branches"]["pipe"];
        };
        groups: s["groups"];
        finalizer: s["finalizer"];
        scanned: updateScanned<s["scanned"], s["unscanned"], unscanned>;
        unscanned: unscanned;
    }>;
    type reduceLeftBound<s extends StaticState, limit extends LimitLiteral, comparator extends Comparator, unscanned extends string> = comparator extends "<" | "<=" ? s["branches"]["leftBound"] extends {} ? s.error<writeMultipleLeftBoundsMessage<s["branches"]["leftBound"]["limit"], s["branches"]["leftBound"]["comparator"], limit, InvertedComparators[comparator]>> : from<{
        root: undefined;
        branches: {
            prefixes: s["branches"]["prefixes"];
            leftBound: {
                limit: limit;
                comparator: InvertedComparators[comparator];
            };
            intersection: s["branches"]["intersection"];
            pipe: s["branches"]["pipe"];
            union: s["branches"]["union"];
        };
        groups: s["groups"];
        finalizer: s["finalizer"];
        scanned: updateScanned<s["scanned"], s["unscanned"], unscanned>;
        unscanned: unscanned;
    }> : s.error<writeUnpairableComparatorMessage<comparator>>;
    type reduceRange<s extends StaticState, minLimit extends LimitLiteral, minComparator extends MinComparator, maxComparator extends MaxComparator, maxLimit extends LimitLiteral, unscanned extends string> = s.from<{
        root: [
            minLimit,
            minComparator,
            [
                s["root"],
                maxComparator,
                maxLimit
            ]
        ];
        branches: {
            prefixes: s["branches"]["prefixes"];
            leftBound: undefined;
            intersection: s["branches"]["intersection"];
            pipe: s["branches"]["pipe"];
            union: s["branches"]["union"];
        };
        groups: s["groups"];
        finalizer: s["finalizer"];
        scanned: updateScanned<s["scanned"], s["unscanned"], unscanned>;
        unscanned: unscanned;
    }>;
    type reduceSingleBound<s extends StaticState, comparator extends Comparator, limit extends number | string, unscanned extends string> = s.from<{
        root: [
            s["root"],
            comparator,
            limit
        ];
        branches: {
            prefixes: s["branches"]["prefixes"];
            leftBound: undefined;
            intersection: s["branches"]["intersection"];
            pipe: s["branches"]["pipe"];
            union: s["branches"]["union"];
        };
        groups: s["groups"];
        finalizer: s["finalizer"];
        scanned: updateScanned<s["scanned"], s["unscanned"], unscanned>;
        unscanned: unscanned;
    }>;
    type mergeToIntersection<s extends StaticState> = s["branches"]["intersection"] extends undefined ? mergePrefixes<s> : [
        s["branches"]["intersection"],
        "&",
        mergePrefixes<s>
    ];
    type mergeToUnion<s extends StaticState> = s["branches"]["union"] extends undefined ? mergeToIntersection<s> : [
        s["branches"]["union"],
        "|",
        mergeToIntersection<s>
    ];
    type mergeToPipe<s extends StaticState> = s["branches"]["pipe"] extends undefined ? mergeToUnion<s> : [
        s["branches"]["pipe"],
        "|>",
        mergeToUnion<s>
    ];
    type mergePrefixes<s extends StaticState, remaining extends unknown[] = s["branches"]["prefixes"]> = remaining extends [
        infer head,
        ...infer tail
    ] ? [
        head,
        mergePrefixes<s, tail>
    ] : s["root"];
    type popGroup<stack extends BranchState[], top extends BranchState> = [
        ...stack,
        top
    ];
    type finalizeGroup<s extends StaticState, unscanned extends string> = s["branches"]["leftBound"] extends {} ? openRangeError<s["branches"]["leftBound"]> : s["groups"] extends popGroup<infer stack, infer top> ? from<{
        groups: stack;
        branches: top;
        root: mergeToPipe<s>;
        finalizer: s["finalizer"];
        scanned: updateScanned<s["scanned"], s["unscanned"], unscanned>;
        unscanned: unscanned;
    }> : s.error<writeUnmatchedGroupCloseMessage<")", unscanned>>;
    type reduceGroupOpen<s extends StaticState, unscanned extends string> = from<{
        groups: [
            ...s["groups"],
            s["branches"]
        ];
        branches: initialBranches;
        root: undefined;
        finalizer: s["finalizer"];
        scanned: updateScanned<s["scanned"], s["unscanned"], unscanned>;
        unscanned: unscanned;
    }>;
    type finalize<s extends StaticState, finalizer extends FinalizingLookahead> = s["groups"] extends [
    ] ? s["branches"]["leftBound"] extends {} ? openRangeError<s["branches"]["leftBound"]> : from<{
        root: mergeToPipe<s>;
        groups: s["groups"];
        branches: initialBranches;
        finalizer: finalizer;
        scanned: s["scanned"];
        unscanned: s["unscanned"];
    }> : s.error<writeUnclosedGroupMessage<")">>;
    type openRangeError<range extends defined<BranchState["leftBound"]>> = s.error<writeOpenRangeMessage<range["limit"], range["comparator"]>>;
    type previousOperator<s extends StaticState> = s["branches"]["leftBound"] extends {} ? s["branches"]["leftBound"]["comparator"] : s["branches"]["prefixes"] extends ([
        ...unknown[],
        infer tail extends string
    ]) ? tail : s["branches"]["intersection"] extends {} ? "&" : s["branches"]["union"] extends {} ? "|" : undefined;
    type scanTo<s extends StaticState, unscanned extends string> = from<{
        root: s["root"];
        branches: s["branches"];
        groups: s["groups"];
        finalizer: s["finalizer"];
        scanned: updateScanned<s["scanned"], s["unscanned"], unscanned>;
        unscanned: unscanned;
    }>;
    type from<s extends StaticState> = s;
    type branchesFrom<b extends BranchState> = b;
}
declare global {
    export interface ArkEnv {
        $(): Ark;
    }
}
declare namespace ArkAmbient {
    type $ = ReturnType<ArkEnv["$"]>;
    type meta = ArkEnv.meta;
    type prototypes = ArkEnv.prototypes;
}
type StringLiteral<contents extends string = string> = DoubleQuotedStringLiteral<contents> | SingleQuotedStringLiteral<contents>;
type DoubleQuotedStringLiteral<contents extends string = string> = `"${contents}"`;
type SingleQuotedStringLiteral<contents extends string = string> = `'${contents}'`;
declare const parseEnclosed: (s: RuntimeState, enclosing: EnclosingStartToken) => void;
type parseEnclosed<s extends StaticState, enclosingStart extends EnclosingStartToken, unscanned extends string> = Scanner.shiftUntilEscapable<unscanned, EnclosingTokens[enclosingStart], enclosingStart extends EnclosingRegexToken ? Backslash : ""> extends Scanner.shiftResult<infer scanned, infer nextUnscanned> ? _parseEnclosed<s, enclosingStart, scanned, nextUnscanned> : never;
type _parseEnclosed<s extends StaticState, enclosingStart extends EnclosingStartToken, scanned extends string, nextUnscanned extends string, def extends string = `${enclosingStart}${scanned}${EnclosingTokens[enclosingStart]}`> = nextUnscanned extends "" ? s.error<writeUnterminatedEnclosedMessage<scanned, enclosingStart>> : enclosingStart extends EnclosingQuote ? s.setRoot<s, InferredAst<scanned, def>, nextUnscanned extends Scanner.shift<string, infer unscanned> ? unscanned : ""> : enclosingStart extends EnclosingRegexToken ? regex.parse<scanned> extends infer r ? r extends Regex ? s.setRoot<s, InferredAst<enclosingStart extends "/" ? r["infer"] : (In: r["infer"]) => Out<r["inferExecArray"]>, def>, nextUnscanned extends Scanner.shift<string, infer unscanned> ? unscanned : ""> : r extends ErrorMessage<infer e> ? s.error<e> : never : never : s.setRoot<s, InferredAst<Date, def>, nextUnscanned extends Scanner.shift<string, infer unscanned> ? unscanned : "">;
declare const enclosingQuote: {
    readonly "'": 1;
    readonly '"': 1;
};
type EnclosingQuote = keyof typeof enclosingQuote;
declare const enclosingLiteralTokens: {
    readonly "d'": "'";
    readonly 'd"': "\"";
    readonly "'": "'";
    readonly '"': "\"";
};
type EnclosingLiteralTokens = typeof enclosingLiteralTokens;
type EnclosingLiteralStartToken = keyof EnclosingLiteralTokens;
declare const enclosingRegexTokens: {
    readonly "/": "/";
    readonly "x/": "/";
};
type EnclosingRegexTokens = typeof enclosingRegexTokens;
type EnclosingRegexToken = keyof EnclosingRegexTokens;
declare const enclosingTokens: {
    readonly "/": "/";
    readonly "x/": "/";
    readonly "d'": "'";
    readonly 'd"': "\"";
    readonly "'": "'";
    readonly '"': "\"";
};
type EnclosingTokens = typeof enclosingTokens;
type EnclosingStartToken = keyof EnclosingTokens;
declare const enclosingCharDescriptions: {
    readonly '"': "double-quote";
    readonly "'": "single-quote";
    readonly "/": "forward slash";
};
type enclosingCharDescriptions = typeof enclosingCharDescriptions;
declare const writeUnterminatedEnclosedMessage: <fragment extends string, enclosingStart extends EnclosingStartToken>(fragment: fragment, enclosingStart: enclosingStart) => writeUnterminatedEnclosedMessage<fragment, enclosingStart>;
type writeUnterminatedEnclosedMessage<fragment extends string, enclosingStart extends EnclosingStartToken> = `${enclosingStart}${fragment} requires a closing ${enclosingCharDescriptions[EnclosingTokens[enclosingStart]]}`;
type UnitLiteralKeyword = "null" | "undefined" | "true" | "false";
type UnitLiteral = UnenclosedUnitLiteral | EnclosedUnitLiteral;
type UnenclosedUnitLiteral = BigintLiteral | NumberLiteral | UnitLiteralKeyword;
type EnclosedUnitLiteral = StringLiteral | DateLiteral;
type EmptyCollectionLiteral = "[]" | "{}";
type DefaultLiteral = UnitLiteral | EmptyCollectionLiteral;
type inferDefaultLiteral<literal> = literal extends "[]" ? [
] : literal extends "{}" ? {} : type.infer<literal>;
type ParsedDefaultableProperty = readonly [
    BaseRoot,
    "=",
    unknown
];
declare const parseDefault: (s: RootedRuntimeState) => ParsedDefaultableProperty;
type parseDefault<root, unscanned extends string> = trim$1<unscanned> extends infer defaultExpression extends string ? defaultExpression extends UnenclosedUnitLiteral | EmptyCollectionLiteral ? [
    root,
    "=",
    defaultExpression
] : defaultExpression extends (`${infer start extends EnclosingLiteralStartToken}${string}`) ? defaultExpression extends `${start}${infer nextUnscanned}` ? isValidEnclosedLiteral<start, nextUnscanned> extends true ? [
    root,
    "=",
    defaultExpression
] : ErrorMessage<writeNonLiteralDefaultMessage<defaultExpression>> : never : ErrorMessage<writeNonLiteralDefaultMessage<defaultExpression>> : never;
type isValidEnclosedLiteral<start extends EnclosingLiteralStartToken, unscanned extends string> = Scanner.shiftUntilEscapable<unscanned, EnclosingLiteralTokens[start], ""> extends Scanner.shiftResult<string, infer nextUnscanned> ? nextUnscanned extends EnclosingLiteralTokens[start] ? true : false : false;
declare const writeNonLiteralDefaultMessage: <defaultDef extends string>(defaultDef: defaultDef) => writeNonLiteralDefaultMessage<defaultDef>;
type writeNonLiteralDefaultMessage<defaultDef extends string> = `Default value '${defaultDef}' must be a literal value`;
type astToString<ast> = ast extends InferredAst | DefAst ? ast[2] : ast extends PostfixExpression<infer operator, infer operand> ? operator extends "[]" ? `${astToString<operand>}[]` : never : ast extends InfixExpression<infer operator, infer l, infer r> ? operator extends "&" | "|" | "%" | Comparator ? `${astToString<l>} ${operator} ${astToString<r>}` : never : ast extends Stringifiable ? `${ast extends bigint ? `${ast}n` : ast}` : "...";
type writeConstrainedMorphMessage<constrainedAst> = `To constrain the output of ${astToString<constrainedAst>}, pipe like myMorph.to('number > 0').
To constrain the input, intersect like myMorph.and('number > 0').`;
type GenericInstantiationAst<generic extends GenericAst = GenericAst, argAsts extends unknown[] = unknown[]> = [
    generic,
    "<>",
    argAsts
];
type inferGenericInstantiation<g extends GenericAst, argAsts extends unknown[], $, args> = g["bodyDef"] extends Hkt ? Hkt.apply<g["bodyDef"], {
    [i in keyof argAsts]: inferExpression<argAsts[i], $, args>;
}> : inferDefinition<g["bodyDef"], resolveScope<g["$"], $>, {
    [i in keyof g["names"] & `${number}` as g["names"][i]]: inferExpression<argAsts[i & keyof argAsts], resolveScope<g["arg$"], $>, args>;
}>;
type validateGenericInstantiation<g extends GenericAst, argAsts extends unknown[], $, args> = validateGenericArgs<g["paramsAst"], argAsts, $, args, [
]>;
type validateGenericArgs<params extends array<GenericParamAst>, argAsts extends array, $, args, indices extends 1[]> = argAsts extends readonly [
    infer arg,
    ...infer argsTail
] ? validateAst<arg, $, args> extends infer e extends ErrorMessage ? e : inferAstRoot<arg, $, args> extends params[indices["length"]][1] ? validateGenericArgs<params, argsTail, $, args, [
    ...indices,
    1
]> : ErrorMessage<writeUnsatisfiedParameterConstraintMessage<params[indices["length"]][0], typeToString<params[indices["length"]][1]>, astToString<arg>>> : undefined;
type resolveScope<g$, $> = g$ extends UnparsedScope ? $ : g$;
type inferAstRoot<ast, $, args> = ast extends array ? inferExpression<ast, $, args> : never;
type inferAstIn<ast, $, args> = distill.In<inferAstRoot<ast, $, args>>;
type DefAst<def = unknown, alias extends string = string> = [
    def,
    "def",
    alias
];
type InferredAst<t = unknown, def extends string = string> = [
    t,
    "inferred",
    def
];
type inferExpression<ast, $, args> = ast extends array ? ast extends InferredAst<infer resolution> ? resolution : ast extends DefAst<infer def> ? inferDefinition<def, $, args> : ast extends GenericInstantiationAst<infer g, infer argAsts> ? inferGenericInstantiation<g, argAsts, $, args> : ast[1] extends "[]" ? inferExpression<ast[0], $, args>[] : ast[1] extends "|" ? inferExpression<ast[0], $, args> | inferExpression<ast[2], $, args> : ast[1] extends "&" ? inferIntersection<inferExpression<ast[0], $, args>, inferExpression<ast[2], $, args>> : ast[1] extends "|>" ? inferPipe<inferExpression<ast[0], $, args>, inferExpression<ast[2], $, args>> : ast[1] extends "=" ? inferDefaultLiteral<ast[2]> extends infer defaultValue ? withDefault<inferExpression<ast[0], $, args>, defaultValue> : never : ast[1] extends "#" ? type.brand<inferExpression<ast[0], $, args>, ast[2]> : ast[1] extends Comparator ? ast[0] extends LimitLiteral ? inferExpression<ast[2], $, args> : inferExpression<ast[0], $, args> : ast[1] extends "%" ? inferExpression<ast[0], $, args> : ast[1] extends "?" ? inferExpression<ast[0], $, args> : ast[0] extends "keyof" ? arkKeyOf<inferExpression<ast[1], $, args>> : never : never;
type PostfixExpression<operator extends PostfixToken = PostfixToken, operand = unknown> = readonly [
    operand,
    operator
];
type InfixExpression<operator extends InfixToken = InfixToken, l = unknown, r = unknown> = [
    l,
    operator,
    r
];
declare const parseUnenclosed: (s: RuntimeState) => void;
type parseUnenclosed<s extends StaticState, $, args> = Scanner.shiftUntil<s["unscanned"], TerminatingChar> extends (Scanner.shiftResult<infer token, infer unscanned>) ? tryResolve<s, unscanned, token, $, args> extends s.from<infer s> ? s : never : never;
type parseResolution<s extends StaticState, unscanned extends string, alias extends string, resolution, $, args> = resolutionToAst<alias, resolution> extends infer ast ? ast extends GenericAst ? parseGenericInstantiation<alias, ast, s.scanTo<s, unscanned>, $, args> : s.setRoot<s, ast, unscanned> : never;
declare const parseGenericInstantiation: (name: string, g: GenericRoot, s: RuntimeState) => BaseRoot;
type parseGenericInstantiation<name extends string, g extends GenericAst, s extends StaticState, $, args> = Scanner.skipWhitespace<s["unscanned"]> extends `<${infer unscanned}` ? parseGenericArgs<name, g, unscanned, $, args> extends infer result ? result extends ParsedArgs<infer argAsts, infer nextUnscanned> ? s.setRoot<s, GenericInstantiationAst<g, argAsts>, nextUnscanned> : result : never : s.error<writeInvalidGenericArgCountMessage<name, genericParamNames<g["paramsAst"]>, [
]>>;
type tryResolve<s extends StaticState, unscanned extends string, token extends string, $, args> = token extends keyof args ? parseResolution<s, unscanned, token, args[token], $, args> : token extends keyof $ ? parseResolution<s, unscanned, token, $[token], $, args> : token extends keyof ArkAmbient.$ ? parseResolution<s, unscanned, token, ArkAmbient.$[token], $, args> : `#${token}` extends keyof $ ? parseResolution<s, unscanned, token, $[`#${token}`], $, args> : token extends NumberLiteral<infer n> ? s.setRoot<s, InferredAst<n, token>, unscanned> : token extends (`${infer submodule extends keyof $ & string}.${infer reference}`) ? tryResolveSubmodule<token, $[submodule], reference, s, unscanned, $, args, [
    submodule
]> : token extends (`${infer submodule extends keyof ArkAmbient.$ & string}.${infer reference}`) ? tryResolveSubmodule<token, ArkAmbient.$[submodule], reference, s, unscanned, $, args, [
    submodule
]> : token extends BigintLiteral<infer b> ? s.setRoot<s, InferredAst<b, token>, unscanned> : token extends "keyof" ? s.addPrefix<s, "keyof", unscanned> : unresolvableState<s, token, $, args, [
]>;
type tryResolveSubmodule<token extends string, resolution, reference extends string, s extends StaticState, unscanned extends string, $, args, submodulePath extends string[]> = resolution extends {
    [arkKind]: "module";
} ? reference extends keyof resolution ? parseResolution<s, unscanned, token, resolution[reference], $, args> : reference extends (`${infer nestedSubmodule extends keyof resolution & string}.${infer nestedReference}`) ? tryResolveSubmodule<token, resolution[nestedSubmodule], nestedReference, s, unscanned, $, args, [
    ...submodulePath,
    nestedSubmodule
]> : unresolvableState<s, reference, resolution, {}, submodulePath> : s.error<writeNonSubmoduleDotMessage<lastOf<submodulePath>>>;
type unresolvableState<s extends StaticState, token extends string, resolutions, args, submodulePath extends string[]> = [
    token,
    s["unscanned"]
] extends [
    "",
    Scanner.shift<"#", infer unscanned>
] ? Scanner.shiftUntil<unscanned, TerminatingChar> extends (Scanner.shiftResult<infer name, string>) ? s.error<writePrefixedPrivateReferenceMessage<name>> : never : validReferenceFromToken<token, resolutions, args, submodulePath> extends (never) ? s.error<writeUnresolvableMessage<qualifiedReference<token, submodulePath>>> : s.completion<`${s["scanned"]}${qualifiedReference<validReferenceFromToken<token, resolutions, args, submodulePath>, submodulePath>}`>;
type qualifiedReference<reference extends string, submodulePath extends string[]> = join<[
    ...submodulePath,
    reference
], ".">;
type validReferenceFromToken<token extends string, $, args, submodulePath extends string[]> = Extract<submodulePath["length"] extends 0 ? BaseCompletions<$, args> : resolvableReferenceIn<$>, `${token}${string}`>;
type writeMissingRightOperandMessage<token extends string, unscanned extends string = ""> = `Token '${token}' requires a right operand${unscanned extends "" ? "" : ` before '${unscanned}'`}`;
declare const writeMissingRightOperandMessage: <token extends string, unscanned extends string>(token: token, unscanned?: unscanned) => writeMissingRightOperandMessage<token, unscanned>;
declare const parseOperand: (s: RuntimeState) => void;
type parseOperand<s extends StaticState, $, args> = s["unscanned"] extends Scanner.shift<infer lookahead, infer unscanned> ? lookahead extends "(" ? s.reduceGroupOpen<s, unscanned> : lookahead extends EnclosingStartToken ? parseEnclosed<s, lookahead, unscanned> : lookahead extends WhitespaceChar ? parseOperand<s.scanTo<s, unscanned>, $, args> : lookahead extends "d" ? unscanned extends (Scanner.shift<infer enclosing extends EnclosingQuote, infer nextUnscanned>) ? parseEnclosed<s, `d${enclosing}`, nextUnscanned> : parseUnenclosed<s, $, args> : lookahead extends "x" ? unscanned extends Scanner.shift<"/", infer nextUnscanned> ? parseEnclosed<s, "x/", nextUnscanned> : parseUnenclosed<s, $, args> : parseUnenclosed<s, $, args> : s.completion<`${s["scanned"]}${BaseCompletions<$, args>}`>;
declare const parseBound: (s: RootedRuntimeState, start: ComparatorStartChar) => void;
type parseBound<s extends StaticState, start extends ComparatorStartChar, unscanned extends string, $, args> = shiftComparator<start, unscanned> extends infer shiftResultOrError ? shiftResultOrError extends (Scanner.shiftResult<infer comparator extends Comparator, infer nextUnscanned>) ? s["root"] extends (InferredAst<Date | number, `${infer limit extends number | DateLiteral}`>) ? s.reduceLeftBound<s, limit, comparator, nextUnscanned> : parseRightBound<s.scanTo<s, nextUnscanned>, comparator, $, args> : shiftResultOrError : never;
type OneCharComparator = ">" | "<";
type ComparatorStartChar = Comparator extends `${infer char}${string}` ? char : never;
declare const shiftComparator: (s: RuntimeState, start: ComparatorStartChar) => Comparator;
type shiftComparator<start extends ComparatorStartChar, unscanned extends string> = unscanned extends `=${infer nextUnscanned}` ? [
    `${start}=`,
    nextUnscanned
] : [
    start & OneCharComparator,
    unscanned
];
declare const parseRightBound: (s: RootedRuntimeState, comparator: Comparator) => void;
type parseRightBound<s extends StaticState, comparator extends Comparator, $, args> = parseOperand<s, $, args> extends infer nextState extends StaticState ? nextState["root"] extends (InferredAst<unknown, `${infer limit extends number | DateLiteral}`>) ? s["branches"]["leftBound"] extends {} ? comparator extends MaxComparator ? s.reduceRange<s, s["branches"]["leftBound"]["limit"], s["branches"]["leftBound"]["comparator"], comparator, limit, nextState["unscanned"]> : s.error<writeUnpairableComparatorMessage<comparator>> : s.reduceSingleBound<s, comparator, limit, nextState["unscanned"]> : s.error<writeInvalidLimitMessage<comparator, astToString<nextState["root"]>, "right">> : never;
declare const writeInvalidLimitMessage: <comparator extends Comparator, limit extends string | number, boundKind extends BoundExpressionKind>(comparator: comparator, limit: limit, boundKind: boundKind) => writeInvalidLimitMessage<comparator, limit, boundKind>;
type writeInvalidLimitMessage<comparator extends Comparator, limit extends string | number, boundKind extends BoundExpressionKind> = `Comparator ${boundKind extends "left" ? InvertedComparators[comparator] : comparator} must be ${boundKind extends "left" ? "preceded" : "followed"} by a corresponding literal (was ${limit})`;
type BoundExpressionKind = "left" | "right";
declare const parseBrand: (s: RootedRuntimeState) => void;
type parseBrand<s extends StaticState, unscanned extends string> = Scanner.shiftUntil<Scanner.skipWhitespace<unscanned>, TerminatingChar> extends Scanner.shiftResult<`${infer brandName}`, infer nextUnscanned> ? brandName extends "" ? s.error<emptyBrandNameMessage> : s.setRoot<s, [
    s["root"],
    "#",
    brandName
], nextUnscanned> : never;
declare const parseDivisor: (s: RootedRuntimeState) => void;
type parseDivisor<s extends StaticState, unscanned extends string> = Scanner.shiftUntil<Scanner.skipWhitespace<unscanned>, TerminatingChar> extends Scanner.shiftResult<infer scanned, infer nextUnscanned> ? scanned extends `${infer divisor extends number}` ? divisor extends 0 ? s.error<writeInvalidDivisorMessage<0>> : s.setRoot<s, [
    s["root"],
    "%",
    divisor
], nextUnscanned> : s.error<writeInvalidDivisorMessage<scanned>> : never;
declare const writeInvalidDivisorMessage: <divisor extends string | number>(divisor: divisor) => writeInvalidDivisorMessage<divisor>;
type writeInvalidDivisorMessage<divisor extends string | number> = `% operator must be followed by a non-zero integer literal (was ${divisor})`;
declare const parseOperator: (s: RootedRuntimeState) => void;
type parseOperator<s extends StaticState, $, args> = s["unscanned"] extends Scanner.shift<infer lookahead, infer unscanned> ? lookahead extends "[" ? unscanned extends Scanner.shift<"]", infer nextUnscanned> ? s.setRoot<s, [
    s["root"],
    "[]"
], nextUnscanned> : s.error<incompleteArrayTokenMessage> : lookahead extends "|" ? unscanned extends Scanner.shift<">", infer nextUnscanned> ? s.reduceBranch<s, "|>", nextUnscanned> : s.reduceBranch<s, lookahead, unscanned> : lookahead extends "&" ? s.reduceBranch<s, lookahead, unscanned> : lookahead extends ")" ? s.finalizeGroup<s, unscanned> : lookaheadIsFinalizing<lookahead, unscanned> extends true ? s.finalize<s.scanTo<s, unscanned>, lookahead & FinalizingLookahead> : lookahead extends ComparatorStartChar ? parseBound<s, lookahead, unscanned, $, args> : lookahead extends "%" ? parseDivisor<s, unscanned> : lookahead extends "#" ? parseBrand<s, unscanned> : lookahead extends WhitespaceChar ? parseOperator<s.scanTo<s, unscanned>, $, args> : s.error<writeUnexpectedCharacterMessage<lookahead>> : s.finalize<s, "">;
declare const writeUnexpectedCharacterMessage: <char extends string, shouldBe extends string>(char: char, shouldBe?: shouldBe) => writeUnexpectedCharacterMessage<char, shouldBe>;
type writeUnexpectedCharacterMessage<char extends string, shouldBe extends string = ""> = `'${char}' is not allowed here${shouldBe extends "" ? "" : ` (should be ${shouldBe})`}`;
declare const incompleteArrayTokenMessage = "Missing expected ']'";
type incompleteArrayTokenMessage = typeof incompleteArrayTokenMessage;
declare const parseString: (def: string, ctx: BaseParseContext) => InnerParseResult;
type parseString<def extends string, $, args> = def extends keyof $ ? resolutionToAst<def, $[def]> : def extends `${infer child}[]` ? child extends keyof $ ? [
    resolutionToAst<child, $[child]>,
    "[]"
] : fullStringParse<s.initialize<def>, $, args> : fullStringParse<s.initialize<def>, $, args>;
type inferString<def extends string, $, args> = inferAstRoot<parseString<def, $, args>, $, args>;
type BaseCompletions<$, args, otherSuggestions extends string = never> = resolvableReferenceIn<$> | resolvableReferenceIn<ArkAmbient.$> | (keyof args & string) | StringifiablePrefixOperator | otherSuggestions;
declare const fullStringParse: (s: RuntimeState) => InnerParseResult;
type fullStringParse<s extends StaticState, $, args> = extractFinalizedResult<parseUntilFinalizer<s, $, args>>;
declare const parseUntilFinalizer: (s: RuntimeState) => RootedRuntimeState;
type parseUntilFinalizer<s extends StaticState, $, args> = s["finalizer"] extends undefined ? parseUntilFinalizer<next<s, $, args>, $, args> : s;
declare const next: (s: RuntimeState) => void;
type next<s extends StaticState, $, args> = s["root"] extends undefined ? parseOperand<s, $, args> : parseOperator<s, $, args>;
type extractFinalizedResult<s extends StaticState> = s["finalizer"] extends "" ? s["root"] : s["finalizer"] extends ErrorMessage ? s["finalizer"] : s["finalizer"] extends "?" ? [
    s["root"],
    "?"
] : s["finalizer"] extends "=" ? parseDefault<s["root"], s["unscanned"]> : ErrorMessage<writeUnexpectedCharacterMessage<s["finalizer"] & string>>;
declare const parseGenericArgs: (name: string, g: GenericRoot, s: RuntimeState) => BaseRoot[];
type parseGenericArgs<name extends string, g extends GenericAst, unscanned extends string, $, args> = _parseGenericArgs<name, g, unscanned, $, args, [
], [
]>;
type ParsedArgs<result extends unknown[] = unknown[], unscanned extends string = string> = {
    result: result;
    unscanned: unscanned;
};
declare const _parseGenericArgs: (name: string, g: GenericRoot, s: RuntimeState, argNodes: BaseRoot[]) => BaseRoot[];
type _parseGenericArgs<name extends string, g extends GenericAst, unscanned extends string, $, args, argDefs extends string[], argAsts extends unknown[]> = parseUntilFinalizer<s.initialize<unscanned>, $, args> extends (infer finalArgState extends StaticState) ? {
    defs: [
        ...argDefs,
        finalArgState["scanned"] extends `${infer def}${"," | ">"}` ? def : finalArgState["scanned"]
    ];
    asts: [
        ...argAsts,
        finalArgState["root"]
    ];
    unscanned: finalArgState["unscanned"];
} extends ({
    defs: infer nextDefs extends string[];
    asts: infer nextAsts extends unknown[];
    unscanned: infer nextUnscanned extends string;
}) ? finalArgState["finalizer"] extends ">" ? nextAsts["length"] extends g["paramsAst"]["length"] ? ParsedArgs<nextAsts, nextUnscanned> : s.error<writeInvalidGenericArgCountMessage<name, genericParamNames<g["paramsAst"]>, nextDefs>> : finalArgState["finalizer"] extends "," ? _parseGenericArgs<name, g, nextUnscanned, $, args, nextDefs, nextAsts> : finalArgState["finalizer"] extends ErrorMessage ? finalArgState : s.error<writeUnclosedGroupMessage<">">> : never : never;
declare const writeInvalidGenericArgCountMessage: <name extends string, params extends array<string>, argDefs extends array<string>>(name: name, params: params, argDefs: argDefs) => writeInvalidGenericArgCountMessage<name, params, argDefs>;
type writeInvalidGenericArgCountMessage<name extends string, params extends array<string>, argDefs extends array<string>> = `${name}<${join<params, ", ">}> requires exactly ${params["length"]} args (got ${argDefs["length"]}${argDefs["length"] extends (0) ? "" : `: ${join<argDefs, ",">}`})`;
type validateRange<l, comparator extends Comparator, r, $, args> = [
    l
] extends [
    LimitLiteral
] ? validateBound<r, comparator, l, "left", $, args> : [
    l
] extends [
    [
        infer leftAst,
        Comparator,
        unknown
    ]
] ? ErrorMessage<writeDoubleRightBoundMessage<astToString<leftAst>>> : validateBound<l, comparator, r & LimitLiteral, "right", $, args>;
type validateBound<boundedAst, comparator extends Comparator, limit extends LimitLiteral, boundKind extends BoundExpressionKind, $, args> = inferAstRoot<boundedAst, $, args> extends infer bounded ? isNumericallyBoundable<bounded> extends true ? limit extends number ? validateAst<boundedAst, $, args> : ErrorMessage<writeInvalidLimitMessage<comparator, limit, boundKind>> : [
    bounded
] extends [
    Date
] ? validateAst<boundedAst, $, args> : [
    bounded
] extends [
    InferredMorph
] ? ErrorMessage<writeConstrainedMorphMessage<boundedAst>> : ErrorMessage<writeUnboundableMessage<typeToString<bounded>>> : never;
type isNumericallyBoundable<bounded> = [
    bounded
] extends [
    number
] ? true : [
    bounded
] extends [
    string
] ? true : [
    bounded
] extends [
    array
] ? true : false;
declare const writeDoubleRightBoundMessage: <root extends string>(root: root) => writeDoubleRightBoundMessage<root>;
type writeDoubleRightBoundMessage<root extends string> = `Expression ${root} must have at most one right bound`;
type validateDefault<baseAst, defaultLiteral extends DefaultLiteral, $, args> = validateAst<baseAst, $, args> extends infer e extends ErrorMessage ? e : inferDefaultLiteral<defaultLiteral> extends inferAstIn<baseAst, $, args> ? undefined : defaultLiteral extends "{}" ? EmptyObject extends inferAstIn<baseAst, $, args> ? undefined : ErrorMessage<writeUnassignableDefaultValueMessage<astToString<baseAst>, defaultLiteral>> : ErrorMessage<writeUnassignableDefaultValueMessage<astToString<baseAst>, defaultLiteral>>;
type validateDivisor<l, $, args> = inferAstRoot<l, $, args> extends infer data ? [
    data
] extends [
    number
] ? validateAst<l, $, args> : [
    data
] extends [
    InferredMorph
] ? ErrorMessage<writeConstrainedMorphMessage<l>> : ErrorMessage<writeIndivisibleMessage<data>> : never;
type validateKeyof<operandAst, $, args> = inferAstRoot<operandAst, $, args> extends infer data ? [
    data
] extends [
    object
] ? validateAst<operandAst, $, args> : ErrorMessage<writeNonStructuralOperandMessage<"keyof", typeToString<data>>> : never;
type validateAst<ast, $, args> = ast extends ErrorMessage ? ast : ast extends InferredAst ? validateInferredAst<ast[0], ast[2]> : ast extends DefAst ? ast[2] extends PrivateDeclaration<infer name> ? ErrorMessage<writePrefixedPrivateReferenceMessage<name>> : undefined : ast extends PostfixExpression<"[]" | "?", infer operand> ? validateAst<operand, $, args> : ast extends InfixExpression<infer operator, infer l, infer r> ? operator extends BranchOperator ? validateInfix<ast, $, args> : operator extends Comparator ? validateRange<l, operator, r, $, args> : operator extends "%" ? validateDivisor<l, $, args> : operator extends "=" ? validateDefault<l, r & DefaultLiteral, $, args> : operator extends "#" ? validateAst<l, $, args> : ErrorMessage<writeUnexpectedExpressionMessage<astToString<ast>>> : ast extends [
    "keyof",
    infer operand
] ? validateKeyof<operand, $, args> : ast extends GenericInstantiationAst<infer g, infer argAsts> ? validateGenericInstantiation<g, argAsts, $, args> : ErrorMessage<writeUnexpectedExpressionMessage<astToString<ast>>> & {
    ast: ast;
};
type writeUnexpectedExpressionMessage<expression extends string> = `Failed to parse the expression resulting from ${expression}`;
declare const writePrefixedPrivateReferenceMessage: <name extends string>(name: name) => writePrefixedPrivateReferenceMessage<name>;
type writePrefixedPrivateReferenceMessage<name extends string> = `Private type references should not include '#'. Use '${name}' instead.`;
type validateInferredAst<inferred, def extends string> = def extends NumberLiteral ? number extends inferred ? ErrorMessage<writeMalformedNumericLiteralMessage<def, "number">> : undefined : def extends BigintLiteral ? bigint extends inferred ? ErrorMessage<writeMalformedNumericLiteralMessage<def, "bigint">> : undefined : [
    inferred
] extends [
    anyOrNever
] ? undefined : def extends PrivateDeclaration<infer name> ? ErrorMessage<writePrefixedPrivateReferenceMessage<name>> : inferred extends Generic ? ErrorMessage<writeInvalidGenericArgCountMessage<def, inferred["names"], [
]>> : inferred extends {
    [arkKind]: "module";
} ? "root" extends keyof inferred ? undefined : ErrorMessage<writeMissingSubmoduleAccessMessage<def>> : def extends ErrorMessage ? def : undefined;
type validateString<def extends string, $, args> = parseString<def, $, args> extends infer ast ? validateAst<ast, $, args> extends infer result extends ErrorMessage ? result extends Completion<infer text> ? text : result : def : never;
type validateInfix<ast extends InfixExpression, $, args> = validateAst<ast[0], $, args> extends infer e extends ErrorMessage ? e : validateAst<ast[2], $, args> extends infer e extends ErrorMessage ? e : undefined;
declare const shallowOptionalMessage = "Optional definitions like 'string?' are only valid as properties in an object or tuple";
type shallowOptionalMessage = typeof shallowOptionalMessage;
declare const shallowDefaultableMessage = "Defaultable definitions like 'number = 0' are only valid as properties in an object or tuple";
type shallowDefaultableMessage = typeof shallowDefaultableMessage;
type inferObjectLiteral<def extends object, $, args> = show<"..." extends keyof def ? merge<inferDefinition<def["..."], $, args>, _inferObjectLiteral<def, $, args>> : _inferObjectLiteral<def, $, args>>;
type _inferObjectLiteral<def extends object, $, args> = {
    -readonly [k in keyof def as nonOptionalKeyFromEntry<k, def[k], $, args>]: inferDefinition<def[k], $, args>;
} & {
    -readonly [k in keyof def as optionalKeyFromEntry<k, def[k]>]?: def[k] extends OptionalPropertyDefinition<infer baseDef> ? inferDefinition<baseDef, $, args> : inferDefinition<def[k], $, args>;
};
type validateObjectLiteral<def, $, args> = {
    [k in keyof def]: preparseKey<k> extends (infer parsedKey extends PreparsedKey) ? parsedKey extends PreparsedEntryKey<"index"> ? validateString<parsedKey["normalized"], $, args> extends (ErrorMessage<infer message>) ? ErrorType<message> : inferDefinition<parsedKey["normalized"], $, args> extends Key ? validateProperty<def[k], parsedKey["kind"], $, args> : ErrorMessage<writeInvalidPropertyKeyMessage<parsedKey["normalized"]>> : validateProperty<def[k], parsedKey["kind"], $, args> : never;
};
type nonOptionalKeyFromEntry<k extends PropertyKey, v, $, args> = preparseKey<k> extends infer parsedKey ? parsedKey extends PreparsedEntryKey<"required"> ? [
    v
] extends [
    OptionalPropertyDefinition
] ? [
    v
] extends [
    anyOrNever
] ? parsedKey["normalized"] : never : parsedKey["normalized"] : parsedKey extends PreparsedEntryKey<"index"> ? inferDefinition<parsedKey["normalized"], $, args> & Key : never : never;
type optionalKeyFromEntry<k extends PropertyKey, v> = preparseKey<k> extends infer parsedKey ? parsedKey extends PreparsedEntryKey<"optional"> ? parsedKey["normalized"] : v extends OptionalPropertyDefinition ? k : never : never;
type normalizedKeyKind<kind extends EntryKeyKind> = kind extends "index" ? string : Key;
type PreparsedEntryKey<kind extends EntryKeyKind = EntryKeyKind, normalized extends normalizedKeyKind<kind> = normalizedKeyKind<kind>> = {
    kind: kind;
    normalized: normalized;
};
type PreparsedSpecialKey<kind extends SpecialKeyKind = SpecialKeyKind> = {
    kind: kind;
};
type PreparsedKey = PreparsedEntryKey | PreparsedSpecialKey;
declare namespace PreparsedKey {
    type from<t extends PreparsedKey> = t;
}
type ParsedKeyKind = EntryKeyKind | SpecialKeyKind;
type EntryKeyKind = "required" | "optional" | "index";
type SpecialKeyKind = "spread" | "undeclared";
type MetaKey = "..." | "+";
type IndexKey<def extends string = string> = `[${def}]`;
declare const preparseKey: (key: Key) => PreparsedKey;
type preparseKey<k> = k extends symbol ? PreparsedKey.from<{
    kind: "required";
    normalized: k;
}> : k extends `${infer inner}?` ? inner extends `${infer baseName}${Backslash}` ? PreparsedKey.from<{
    kind: "required";
    normalized: `${baseName}?`;
}> : PreparsedKey.from<{
    kind: "optional";
    normalized: inner;
}> : k extends "+" ? {
    kind: "undeclared";
} : k extends "..." ? {
    kind: "spread";
} : k extends `${Backslash}${infer escapedMeta extends MetaKey}` ? PreparsedKey.from<{
    kind: "required";
    normalized: escapedMeta;
}> : k extends IndexKey<infer def> ? PreparsedKey.from<{
    kind: "index";
    normalized: def;
}> : PreparsedKey.from<{
    kind: "required";
    normalized: k extends (`${Backslash}${infer escapedIndexKey extends IndexKey}`) ? escapedIndexKey : k extends Key ? k : `${k & number}`;
}>;
declare const writeInvalidSpreadTypeMessage: <def extends string>(def: def) => writeInvalidSpreadTypeMessage<def>;
type writeInvalidSpreadTypeMessage<def extends string> = `Spread operand must resolve to an object literal type (was ${def})`;
type ParsedOptionalProperty = readonly [
    BaseRoot,
    "?"
];
type validateProperty<def, keyKind extends ParsedKeyKind, $, args> = [
    def
] extends [
    anyOrNever
] ? def : keyKind extends "spread" ? def extends validateInnerDefinition<def, $, args> ? inferDefinition<def, $, args> extends object ? def : ErrorType<writeInvalidSpreadTypeMessage<typeToString<inferDefinition<def, $, args>>>> : validateInnerDefinition<def, $, args> : keyKind extends "undeclared" ? UndeclaredKeyBehavior : keyKind extends "required" ? validateInnerDefinition<def, $, args> : def extends OptionalPropertyDefinition ? ErrorMessage<invalidOptionalKeyKindMessage> : isDefaultable<def, $, args> extends true ? ErrorMessage<invalidDefaultableKeyKindMessage> : validateInnerDefinition<def, $, args>;
type isDefaultable<def, $, args> = def extends DefaultablePropertyTuple ? true : def extends PossibleDefaultableStringDefinition ? parseString<def, $, args> extends DefaultablePropertyTuple ? true : false : false;
type OptionalPropertyDefinition<baseDef = unknown> = OptionalPropertyTuple<baseDef> | OptionalPropertyString<baseDef & string>;
type OptionalPropertyString<baseDef extends string = string> = `${baseDef}?`;
type OptionalPropertyTuple<baseDef = unknown> = readonly [
    baseDef,
    "?"
];
type PossibleDefaultableStringDefinition = `${string}=${string}`;
type DefaultablePropertyTuple<baseDef = unknown, thunkableProperty = unknown> = readonly [
    baseDef,
    "=",
    thunkableProperty
];
declare const invalidOptionalKeyKindMessage = "Only required keys may make their values optional, e.g. { [mySymbol]: ['number', '?'] }";
type invalidOptionalKeyKindMessage = typeof invalidOptionalKeyKindMessage;
declare const invalidDefaultableKeyKindMessage = "Only required keys may specify default values, e.g. { value: 'number = 0' }";
type invalidDefaultableKeyKindMessage = typeof invalidDefaultableKeyKindMessage;
type validateTupleLiteral<def extends array, $, args> = parseSequence<def, $, args> extends infer s extends SequenceParseState ? Readonly<s["validated"]> : never;
type inferTupleLiteral<def extends array, $, args> = parseSequence<def, $, args> extends infer s extends SequenceParseState ? s["inferred"] : never;
type SequencePhase = satisfy<keyof Sequence.Inner, SequencePhase.prefix | SequencePhase.optionals | SequencePhase.defaultables | SequencePhase.postfix>;
declare namespace SequencePhase {
    type prefix = "prefix";
    type optionals = "optionals";
    type defaultables = "defaultables";
    type postfix = "postfix";
}
type SequenceParseState = {
    unscanned: array;
    inferred: array;
    validated: array;
    phase: SequencePhase;
};
type parseSequence<def extends array, $, args> = parseNextElement<{
    unscanned: def;
    inferred: [
    ];
    validated: [
    ];
    phase: SequencePhase.prefix;
}, $, args>;
type PreparsedElementKind = "required" | SequencePhase.optionals | SequencePhase.defaultables;
type PreparsedElement = {
    head: unknown;
    tail: array;
    inferred: unknown;
    validated: unknown;
    kind: PreparsedElementKind;
    spread: boolean;
};
declare namespace PreparsedElement {
    type from<result extends PreparsedElement> = result;
    type required = "required";
    type optionals = "optionals";
    type defaultables = "defaultables";
}
type preparseNextState<s extends SequenceParseState, $, args> = s["unscanned"] extends readonly [
    "...",
    infer head,
    ...infer tail
] ? preparseNextElement<head, tail, true, $, args> : s["unscanned"] extends readonly [
    infer head,
    ...infer tail
] ? preparseNextElement<head, tail, false, $, args> : null;
type preparseNextElement<head, tail extends array, spread extends boolean, $, args> = PreparsedElement.from<{
    head: head;
    tail: tail;
    inferred: inferDefinition<head, $, args>;
    validated: validateInnerDefinition<head, $, args>;
    kind: head extends OptionalPropertyDefinition ? PreparsedElement.optionals : head extends DefaultablePropertyTuple ? PreparsedElement.defaultables : isDefaultable<head, $, args> extends true ? PreparsedElement.defaultables : PreparsedElement.required;
    spread: spread;
}>;
type parseNextElement<s extends SequenceParseState, $, args> = preparseNextState<s, $, args> extends infer next extends PreparsedElement ? parseNextElement<{
    unscanned: next["tail"];
    inferred: nextInferred<s, next>;
    validated: nextValidated<s, next>;
    phase: next["kind"] extends (SequencePhase.optionals | SequencePhase.defaultables) ? next["kind"] : number extends nextInferred<s, next>["length"] ? s["phase"] : SequencePhase.prefix;
}, $, args> : s;
type nextInferred<s extends SequenceParseState, next extends PreparsedElement> = next["spread"] extends true ? [
    ...s["inferred"],
    ...conform<next["inferred"], array>
] : next["kind"] extends SequencePhase.optionals ? [
    ...s["inferred"],
    next["inferred"]?
] : [
    ...s["inferred"],
    next["inferred"]
];
type nextValidated<s extends SequenceParseState, next extends PreparsedElement> = [
    ...s["validated"],
    ...nextValidatedSpreadOperatorIfPresent<s, next>,
    nextValidatedElement<s, next>
];
type nextValidatedSpreadOperatorIfPresent<s extends SequenceParseState, next extends PreparsedElement> = next["spread"] extends true ? [
    next["inferred"] extends infer spreadOperand extends array ? [
        number,
        number
    ] extends ([
        s["inferred"]["length"],
        spreadOperand["length"]
    ]) ? ErrorMessage<multipleVariadicMessage> : "..." : ErrorMessage<writeNonArraySpreadMessage<next["head"]>>
] : [
];
type nextValidatedElement<s extends SequenceParseState, next extends PreparsedElement> = next["kind"] extends SequencePhase.optionals ? next["spread"] extends true ? ErrorMessage<spreadOptionalMessage> : s["phase"] extends SequencePhase.postfix ? ErrorMessage<optionalOrDefaultableAfterVariadicMessage> : next["validated"] : next["kind"] extends SequencePhase.defaultables ? next["spread"] extends true ? ErrorMessage<spreadDefaultableMessage> : s["phase"] extends SequencePhase.optionals ? ErrorMessage<defaultablePostOptionalMessage> : s["phase"] extends SequencePhase.postfix ? ErrorMessage<optionalOrDefaultableAfterVariadicMessage> : next["validated"] : [
    s["phase"],
    next["spread"]
] extends ([
    SequencePhase.optionals | SequencePhase.defaultables,
    false
]) ? ErrorMessage<postfixAfterOptionalOrDefaultableMessage> : next["validated"];
declare const writeNonArraySpreadMessage: <operand extends string>(operand: operand) => writeNonArraySpreadMessage<operand>;
type writeNonArraySpreadMessage<operand> = `Spread element must be an array${operand extends string ? ` (was ${operand})` : ""}`;
declare const multipleVariadicMesage = "A tuple may have at most one variadic element";
type multipleVariadicMessage = typeof multipleVariadicMesage;
declare const optionalOrDefaultableAfterVariadicMessage = "An optional element may not follow a variadic element";
type optionalOrDefaultableAfterVariadicMessage = typeof optionalOrDefaultableAfterVariadicMessage;
declare const spreadOptionalMessage = "A spread element cannot be optional";
type spreadOptionalMessage = typeof spreadOptionalMessage;
declare const spreadDefaultableMessage = "A spread element cannot have a default";
type spreadDefaultableMessage = typeof spreadDefaultableMessage;
declare const defaultablePostOptionalMessage = "A defaultable element may not follow an optional element without a default";
type defaultablePostOptionalMessage = typeof defaultablePostOptionalMessage;
interface Type$6<out t extends object = object, $ = {}> extends Type$1<t, $> {
    readonly(): t extends array ? Type$5<{
        readonly [i in keyof t]: t[i];
    }, $> : Type$6<{
        readonly [k in keyof t]: t[k];
    }, $>;
    keyof(): instantiateType<arkKeyOf<t>, $>;
    get<const k1 extends arkIndexableOf<t>, r = instantiateType<arkGet<t, k1>, $>>(k1: k1 | type.cast<k1>): r extends infer _ ? _ : never;
    get<const k1 extends arkIndexableOf<t>, const k2 extends arkIndexableOf<arkGet<t, k1>>, r = instantiateType<arkGet<arkGet<t, k1>, k2>, $>>(k1: k1 | type.cast<k1>, k2: k2 | type.cast<k2>): r extends infer _ ? _ : never;
    get<const k1 extends arkIndexableOf<t>, const k2 extends arkIndexableOf<arkGet<t, k1>>, const k3 extends arkIndexableOf<arkGet<arkGet<t, k1>, k2>>, r = instantiateType<arkGet<arkGet<arkGet<t, k1>, k2>, k3>, $>>(k1: k1 | type.cast<k1>, k2: k2 | type.cast<k2>, k3: k3 | type.cast<k3>): r extends infer _ ? _ : never;
    pick<const key extends arkKeyOf<t> = never>(...keys: (key | type.cast<key>)[]): Type$6<{
        [k in keyof t as Extract<toArkKey<t, k>, key>]: t[k];
    }, $>;
    omit<const key extends arkKeyOf<t> = never>(...keys: (key | type.cast<key>)[]): Type$6<{
        [k in keyof t as Exclude<toArkKey<t, k>, key>]: t[k];
    }, $>;
    merge<const def, inferredDef = type.infer<def, $>, r = Type$6<merge<t, inferredDef>, $>>(def: type.validate<def, $> & (inferredDef extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredDef
    ]>)): r extends infer _ ? _ : never;
    required(): Type$6<{
        [k in keyof t]-?: t[k];
    }, $>;
    partial(): Type$6<{
        [k in keyof t]?: t[k];
    }, $>;
    map<transformed extends listable<MappedTypeProp>, r = Type$6<constructMapped<t, transformed>, $>>(flatMapEntry: (entry: typePropOf<t, $>) => transformed): r extends infer _ ? _ : never;
    props: array<typePropOf<t, $>>;
}
type typePropOf<o, $> = keyof o extends infer k ? k extends keyof o ? typeProp<o, k, $> : never : never;
type typeProp<o, k extends keyof o, $, t = o[k] & ({} | null)> = t extends Default<infer t, infer defaultValue> ? DefaultedTypeProp<k & Key, t, defaultValue, $> : BaseTypeProp<k extends optionalKeyOf<o> ? "optional" : "required", k & Key, t, $>;
interface BaseTypeProp<kind extends Prop.Kind = Prop.Kind, k extends Key = Key, out v = unknown, $ = {}> {
    kind: kind;
    key: k;
    value: instantiateType<v, $>;
    meta: ArkEnv.meta;
    toJSON: () => JsonStructure;
}
interface DefaultedTypeProp<k extends Key = Key, v = unknown, defaultValue = v, $ = {}> extends BaseTypeProp<"optional", k, v, $> {
    default: defaultValue;
}
type MappedTypeProp<k extends Key = Key, v = unknown> = BaseMappedTypeProp<k, v> | OptionalMappedTypeProp<k, v>;
type BaseMappedTypeProp<k extends Key, v> = merge<BaseMappedPropInner, {
    key: k;
    value: type.cast<v>;
}>;
type OptionalMappedTypeProp<k extends Key, v> = merge<OptionalMappedPropInner, {
    key: k;
    value: type.cast<v>;
    default?: v;
}>;
type constructMapped<t, transformed extends listable<MappedTypeProp>> = show<intersectUnion<fromTypeProps<t, transformed extends array ? transformed : [
    transformed
]>>>;
type fromTypeProps<t, props extends array<MappedTypeProp>> = show<{
    [prop in props[number] as Extract<applyHomomorphicOptionality<t, prop>, {
        kind: "required";
    }>["key"]]: prop["value"][inferred];
} & {
    [prop in props[number] as Extract<applyHomomorphicOptionality<t, prop>, {
        kind: "optional";
        default?: never;
    }>["key"]]?: prop["value"][inferred];
} & {
    [prop in props[number] as Extract<applyHomomorphicOptionality<t, prop>, {
        kind: "optional";
        default: unknown;
    }>["key"]]: withDefault<prop["value"][inferred], prop["default" & keyof prop]>;
}>;
type NonObjectMergeErrorMessage = "Merged type must be an object";
type applyHomomorphicOptionality<t, prop extends MappedTypeProp> = prop["kind"] extends string ? prop : prop & {
    kind: prop["key"] extends optionalKeyOf<t> ? "optional" : "required";
};
interface Type$5<out t extends readonly unknown[] = readonly unknown[], $ = {}> extends Type$6<t, $> {
    atLeastLength(schema: InclusiveNumericRangeSchema): this;
    atMostLength(schema: InclusiveNumericRangeSchema): this;
    moreThanLength(schema: ExclusiveNumericRangeSchema): this;
    lessThanLength(schema: ExclusiveNumericRangeSchema): this;
    exactlyLength(schema: ExactLength.Schema): this;
}
interface Type$4<out t extends globalThis.Date = globalThis.Date, $ = {}> extends Type$6<t, $> {
    atOrAfter(schema: InclusiveDateRangeSchema): this;
    atOrBefore(schema: InclusiveDateRangeSchema): this;
    laterThan(schema: ExclusiveDateRangeSchema): this;
    earlierThan(schema: ExclusiveDateRangeSchema): this;
}
interface Type$3<out t extends number = number, $ = {}> extends Type$1<t, $> {
    divisibleBy(schema: Divisor.Schema): this;
    atLeast(schema: InclusiveNumericRangeSchema): this;
    atMost(schema: InclusiveNumericRangeSchema): this;
    moreThan(schema: ExclusiveNumericRangeSchema): this;
    lessThan(schema: ExclusiveNumericRangeSchema): this;
}
interface Type$2<out t extends string = string, $ = {}> extends Type$1<t, $> {
    matching<const schema extends Pattern.Schema>(schema: schema): schema extends string ? Type$2<regex.infer<schema>, $> : schema extends {
        rule: infer pattern extends string;
    } ? Type$2<regex.infer<pattern>, $> : this;
    atLeastLength(schema: InclusiveNumericRangeSchema): this;
    atMostLength(schema: InclusiveNumericRangeSchema): this;
    moreThanLength(schema: ExclusiveNumericRangeSchema): this;
    lessThanLength(schema: ExclusiveNumericRangeSchema): this;
    exactlyLength(schema: ExactLength.Schema): this;
}
type instantiateType<t, $> = [
    t
] extends [
    anyOrNever
] ? Type$1<t, $> : [
    t
] extends [
    object
] ? [
    t
] extends [
    array
] ? Type$5<t, $> : [
    t
] extends [
    Date
] ? Type$4<t, $> : Type$6<t, $> : [
    t
] extends [
    string
] ? Type$2<t, $> : [
    t
] extends [
    number
] ? Type$3<t, $> : Type$1<t, $>;
type NaryUnionParser<$> = {
    (): Type<never, $>;
    <const a, r = Type<type.infer<a, $>, $>>(a: type.validate<a, $>): r extends infer _ ? _ : never;
    <const a, const b, r = Type<type.infer<a, $> | type.infer<b, $>, $>>(a: type.validate<a, $>, b: type.validate<b, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, r = Type<type.infer<a, $> | type.infer<b, $> | type.infer<c, $>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, r = Type<type.infer<a, $> | type.infer<b, $> | type.infer<c, $> | type.infer<d, $>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, r = Type<type.infer<a, $> | type.infer<b, $> | type.infer<c, $> | type.infer<d, $> | type.infer<e, $>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>, e: type.validate<e, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, r = Type<type.infer<a, $> | type.infer<b, $> | type.infer<c, $> | type.infer<d, $> | type.infer<e, $> | type.infer<f, $>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>, e: type.validate<e, $>, f: type.validate<f, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, r = Type<type.infer<a, $> | type.infer<b, $> | type.infer<c, $> | type.infer<d, $> | type.infer<e, $> | type.infer<f, $> | type.infer<g, $>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>, e: type.validate<e, $>, f: type.validate<f, $>, g: type.validate<g, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, r = Type<type.infer<a, $> | type.infer<b, $> | type.infer<c, $> | type.infer<d, $> | type.infer<e, $> | type.infer<f, $> | type.infer<g, $> | type.infer<h, $>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>, e: type.validate<e, $>, f: type.validate<f, $>, g: type.validate<g, $>, h: type.validate<h, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, r = Type<type.infer<a, $> | type.infer<b, $> | type.infer<c, $> | type.infer<d, $> | type.infer<e, $> | type.infer<f, $> | type.infer<g, $> | type.infer<h, $> | type.infer<i, $>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>, e: type.validate<e, $>, f: type.validate<f, $>, g: type.validate<g, $>, h: type.validate<h, $>, i: type.validate<i, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, const j, r = Type<type.infer<a, $> | type.infer<b, $> | type.infer<c, $> | type.infer<d, $> | type.infer<e, $> | type.infer<f, $> | type.infer<g, $> | type.infer<h, $> | type.infer<i, $> | type.infer<j, $>>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>, e: type.validate<e, $>, f: type.validate<f, $>, g: type.validate<g, $>, h: type.validate<h, $>, i: type.validate<i, $>, j: type.validate<j, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, const j, const k, r = Type<type.infer<a, $> | type.infer<b, $> | type.infer<c, $> | type.infer<d, $> | type.infer<e, $> | type.infer<f, $> | type.infer<g, $> | type.infer<h, $> | type.infer<i, $> | type.infer<j, $> | type.infer<k, $>>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>, e: type.validate<e, $>, f: type.validate<f, $>, g: type.validate<g, $>, h: type.validate<h, $>, i: type.validate<i, $>, j: type.validate<j, $>, k: type.validate<k, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, const j, const k, const l, r = Type<type.infer<a, $> | type.infer<b, $> | type.infer<c, $> | type.infer<d, $> | type.infer<e, $> | type.infer<f, $> | type.infer<g, $> | type.infer<h, $> | type.infer<i, $> | type.infer<j, $> | type.infer<k, $> | type.infer<l, $>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>, e: type.validate<e, $>, f: type.validate<f, $>, g: type.validate<g, $>, h: type.validate<h, $>, i: type.validate<i, $>, j: type.validate<j, $>, k: type.validate<k, $>, l: type.validate<l, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, const j, const k, const l, const m, r = Type<type.infer<a, $> | type.infer<b, $> | type.infer<c, $> | type.infer<d, $> | type.infer<e, $> | type.infer<f, $> | type.infer<g, $> | type.infer<h, $> | type.infer<i, $> | type.infer<j, $> | type.infer<k, $> | type.infer<l, $> | type.infer<m, $>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>, e: type.validate<e, $>, f: type.validate<f, $>, g: type.validate<g, $>, h: type.validate<h, $>, i: type.validate<i, $>, j: type.validate<j, $>, k: type.validate<k, $>, l: type.validate<l, $>, m: type.validate<m, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, const j, const k, const l, const m, const n, r = Type<type.infer<a, $> | type.infer<b, $> | type.infer<c, $> | type.infer<d, $> | type.infer<e, $> | type.infer<f, $> | type.infer<g, $> | type.infer<h, $> | type.infer<i, $> | type.infer<j, $> | type.infer<k, $> | type.infer<l, $> | type.infer<m, $> | type.infer<n, $>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>, e: type.validate<e, $>, f: type.validate<f, $>, g: type.validate<g, $>, h: type.validate<h, $>, i: type.validate<i, $>, j: type.validate<j, $>, k: type.validate<k, $>, l: type.validate<l, $>, m: type.validate<m, $>, n: type.validate<n, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, const j, const k, const l, const m, const n, const o, r = Type<type.infer<a, $> | type.infer<b, $> | type.infer<c, $> | type.infer<d, $> | type.infer<e, $> | type.infer<f, $> | type.infer<g, $> | type.infer<h, $> | type.infer<i, $> | type.infer<j, $> | type.infer<k, $> | type.infer<l, $> | type.infer<m, $> | type.infer<n, $> | type.infer<o, $>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>, e: type.validate<e, $>, f: type.validate<f, $>, g: type.validate<g, $>, h: type.validate<h, $>, i: type.validate<i, $>, j: type.validate<j, $>, k: type.validate<k, $>, l: type.validate<l, $>, m: type.validate<m, $>, n: type.validate<n, $>, o: type.validate<o, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, const j, const k, const l, const m, const n, const o, const p, r = Type<type.infer<a, $> | type.infer<b, $> | type.infer<c, $> | type.infer<d, $> | type.infer<e, $> | type.infer<f, $> | type.infer<g, $> | type.infer<h, $> | type.infer<i, $> | type.infer<j, $> | type.infer<k, $> | type.infer<l, $> | type.infer<m, $> | type.infer<n, $> | type.infer<o, $> | type.infer<p, $>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>, e: type.validate<e, $>, f: type.validate<f, $>, g: type.validate<g, $>, h: type.validate<h, $>, i: type.validate<i, $>, j: type.validate<j, $>, k: type.validate<k, $>, l: type.validate<l, $>, m: type.validate<m, $>, n: type.validate<n, $>, o: type.validate<o, $>, p: type.validate<p, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, const j, const k, const l, const m, const n, const o, const p, const q, r = Type<type.infer<a, $> | type.infer<b, $> | type.infer<c, $> | type.infer<d, $> | type.infer<e, $> | type.infer<f, $> | type.infer<g, $> | type.infer<h, $> | type.infer<i, $> | type.infer<j, $> | type.infer<k, $> | type.infer<l, $> | type.infer<m, $> | type.infer<n, $> | type.infer<o, $> | type.infer<p, $> | type.infer<q, $>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>, e: type.validate<e, $>, f: type.validate<f, $>, g: type.validate<g, $>, h: type.validate<h, $>, i: type.validate<i, $>, j: type.validate<j, $>, k: type.validate<k, $>, l: type.validate<l, $>, m: type.validate<m, $>, n: type.validate<n, $>, o: type.validate<o, $>, p: type.validate<p, $>, q: type.validate<q, $>): r extends infer _ ? _ : never;
    <const defs extends readonly unknown[], r = Type<type.infer<defs[number], $>, $>>(...defs: {
        [i in keyof defs]: type.validate<defs[i], $>;
    }): r extends infer _ ? _ : never;
};
type NaryIntersectionParser<$> = {
    (): Type<unknown, $>;
    <const a, r = Type<type.infer<a, $>, $>>(a: type.validate<a, $>): r extends infer _ ? _ : never;
    <const a, const b, r = Type<inferIntersection<type.infer<a, $>, type.infer<b, $>>, $>>(a: type.validate<a, $>, b: type.validate<b, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, r = Type<inferNaryIntersection<[
        type.infer<a, $>,
        type.infer<b, $>,
        type.infer<c, $>
    ]>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, r = Type<inferNaryIntersection<[
        type.infer<a, $>,
        type.infer<b, $>,
        type.infer<c, $>,
        type.infer<d, $>
    ]>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, r = Type<inferNaryIntersection<[
        type.infer<a, $>,
        type.infer<b, $>,
        type.infer<c, $>,
        type.infer<d, $>,
        type.infer<e, $>
    ]>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>, e: type.validate<e, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, r = Type<inferNaryIntersection<[
        type.infer<a, $>,
        type.infer<b, $>,
        type.infer<c, $>,
        type.infer<d, $>,
        type.infer<e, $>,
        type.infer<f, $>
    ]>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>, e: type.validate<e, $>, f: type.validate<f, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, r = Type<inferNaryIntersection<[
        type.infer<a, $>,
        type.infer<b, $>,
        type.infer<c, $>,
        type.infer<d, $>,
        type.infer<e, $>,
        type.infer<f, $>,
        type.infer<g, $>
    ]>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>, e: type.validate<e, $>, f: type.validate<f, $>, g: type.validate<g, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, r = Type<inferNaryIntersection<[
        type.infer<a, $>,
        type.infer<b, $>,
        type.infer<c, $>,
        type.infer<d, $>,
        type.infer<e, $>,
        type.infer<f, $>,
        type.infer<g, $>,
        type.infer<h, $>
    ]>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>, e: type.validate<e, $>, f: type.validate<f, $>, g: type.validate<g, $>, h: type.validate<h, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, r = Type<inferNaryIntersection<[
        type.infer<a, $>,
        type.infer<b, $>,
        type.infer<c, $>,
        type.infer<d, $>,
        type.infer<e, $>,
        type.infer<f, $>,
        type.infer<g, $>,
        type.infer<h, $>,
        type.infer<i, $>
    ]>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>, e: type.validate<e, $>, f: type.validate<f, $>, g: type.validate<g, $>, h: type.validate<h, $>, i: type.validate<i, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, const j, r = Type<inferNaryIntersection<[
        type.infer<a, $>,
        type.infer<b, $>,
        type.infer<c, $>,
        type.infer<d, $>,
        type.infer<e, $>,
        type.infer<f, $>,
        type.infer<g, $>,
        type.infer<h, $>,
        type.infer<i, $>,
        type.infer<j, $>
    ]>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>, e: type.validate<e, $>, f: type.validate<f, $>, g: type.validate<g, $>, h: type.validate<h, $>, i: type.validate<i, $>, j: type.validate<j, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, const j, const k, r = Type<inferNaryIntersection<[
        type.infer<a, $>,
        type.infer<b, $>,
        type.infer<c, $>,
        type.infer<d, $>,
        type.infer<e, $>,
        type.infer<f, $>,
        type.infer<g, $>,
        type.infer<h, $>,
        type.infer<i, $>,
        type.infer<j, $>,
        type.infer<k, $>
    ]>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>, e: type.validate<e, $>, f: type.validate<f, $>, g: type.validate<g, $>, h: type.validate<h, $>, i: type.validate<i, $>, j: type.validate<j, $>, k: type.validate<k, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, const j, const k, const l, r = Type<inferNaryIntersection<[
        type.infer<a, $>,
        type.infer<b, $>,
        type.infer<c, $>,
        type.infer<d, $>,
        type.infer<e, $>,
        type.infer<f, $>,
        type.infer<g, $>,
        type.infer<h, $>,
        type.infer<i, $>,
        type.infer<j, $>,
        type.infer<k, $>,
        type.infer<l, $>
    ]>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>, e: type.validate<e, $>, f: type.validate<f, $>, g: type.validate<g, $>, h: type.validate<h, $>, i: type.validate<i, $>, j: type.validate<j, $>, k: type.validate<k, $>, l: type.validate<l, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, const j, const k, const l, const m, r = Type<inferNaryIntersection<[
        type.infer<a, $>,
        type.infer<b, $>,
        type.infer<c, $>,
        type.infer<d, $>,
        type.infer<e, $>,
        type.infer<f, $>,
        type.infer<g, $>,
        type.infer<h, $>,
        type.infer<i, $>,
        type.infer<j, $>,
        type.infer<k, $>,
        type.infer<l, $>,
        type.infer<m, $>
    ]>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>, e: type.validate<e, $>, f: type.validate<f, $>, g: type.validate<g, $>, h: type.validate<h, $>, i: type.validate<i, $>, j: type.validate<j, $>, k: type.validate<k, $>, l: type.validate<l, $>, m: type.validate<m, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, const j, const k, const l, const m, const n, r = Type<inferNaryIntersection<[
        type.infer<a, $>,
        type.infer<b, $>,
        type.infer<c, $>,
        type.infer<d, $>,
        type.infer<e, $>,
        type.infer<f, $>,
        type.infer<g, $>,
        type.infer<h, $>,
        type.infer<i, $>,
        type.infer<j, $>,
        type.infer<k, $>,
        type.infer<l, $>,
        type.infer<m, $>,
        type.infer<n, $>
    ]>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>, e: type.validate<e, $>, f: type.validate<f, $>, g: type.validate<g, $>, h: type.validate<h, $>, i: type.validate<i, $>, j: type.validate<j, $>, k: type.validate<k, $>, l: type.validate<l, $>, m: type.validate<m, $>, n: type.validate<n, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, const j, const k, const l, const m, const n, const o, r = Type<inferNaryIntersection<[
        type.infer<a, $>,
        type.infer<b, $>,
        type.infer<c, $>,
        type.infer<d, $>,
        type.infer<e, $>,
        type.infer<f, $>,
        type.infer<g, $>,
        type.infer<h, $>,
        type.infer<i, $>,
        type.infer<j, $>,
        type.infer<k, $>,
        type.infer<l, $>,
        type.infer<m, $>,
        type.infer<n, $>,
        type.infer<o, $>
    ]>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>, e: type.validate<e, $>, f: type.validate<f, $>, g: type.validate<g, $>, h: type.validate<h, $>, i: type.validate<i, $>, j: type.validate<j, $>, k: type.validate<k, $>, l: type.validate<l, $>, m: type.validate<m, $>, n: type.validate<n, $>, o: type.validate<o, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, const j, const k, const l, const m, const n, const o, const p, r = Type<inferNaryIntersection<[
        type.infer<a, $>,
        type.infer<b, $>,
        type.infer<c, $>,
        type.infer<d, $>,
        type.infer<e, $>,
        type.infer<f, $>,
        type.infer<g, $>,
        type.infer<h, $>,
        type.infer<i, $>,
        type.infer<j, $>,
        type.infer<k, $>,
        type.infer<l, $>,
        type.infer<m, $>,
        type.infer<n, $>,
        type.infer<o, $>,
        type.infer<p, $>
    ]>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>, e: type.validate<e, $>, f: type.validate<f, $>, g: type.validate<g, $>, h: type.validate<h, $>, i: type.validate<i, $>, j: type.validate<j, $>, k: type.validate<k, $>, l: type.validate<l, $>, m: type.validate<m, $>, n: type.validate<n, $>, o: type.validate<o, $>, p: type.validate<p, $>): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, const j, const k, const l, const m, const n, const o, const p, const q, r = Type<inferNaryIntersection<[
        type.infer<a, $>,
        type.infer<b, $>,
        type.infer<c, $>,
        type.infer<d, $>,
        type.infer<e, $>,
        type.infer<f, $>,
        type.infer<g, $>,
        type.infer<h, $>,
        type.infer<i, $>,
        type.infer<j, $>,
        type.infer<k, $>,
        type.infer<l, $>,
        type.infer<m, $>,
        type.infer<n, $>,
        type.infer<o, $>,
        type.infer<p, $>,
        type.infer<q, $>
    ]>, $>>(a: type.validate<a, $>, b: type.validate<b, $>, c: type.validate<c, $>, d: type.validate<d, $>, e: type.validate<e, $>, f: type.validate<f, $>, g: type.validate<g, $>, h: type.validate<h, $>, i: type.validate<i, $>, j: type.validate<j, $>, k: type.validate<k, $>, l: type.validate<l, $>, m: type.validate<m, $>, n: type.validate<n, $>, o: type.validate<o, $>, p: type.validate<p, $>, q: type.validate<q, $>): r extends infer _ ? _ : never;
    <const defs extends readonly unknown[], r = Type<inferNaryIntersection<{
        [i in keyof defs]: type.infer<defs[i], $>;
    }>, $>>(...defs: {
        [i in keyof defs]: type.validate<defs[i], $>;
    }): r extends infer _ ? _ : never;
};
type NaryMergeParser<$> = {
    (): Type<object, $>;
    <const a, inferredA = type.infer<a, $>, r = Type<inferredA, $>>(a: type.validate<a, $> & (inferredA extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredA
    ]>)): r extends infer _ ? _ : never;
    <const a, const b, inferredA = type.infer<a, $>, inferredB = type.infer<b, $>, r = Type<merge<inferredA, inferredB>, $>>(a: type.validate<a, $> & (inferredA extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredA
    ]>), b: type.validate<b, $> & (inferredB extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredB
    ]>)): r extends infer _ ? _ : never;
    <const a, const b, const c, inferredA = type.infer<a, $>, inferredB = type.infer<b, $>, inferredC = type.infer<c, $>, r = Type<inferNaryMerge<[
        inferredA,
        inferredB,
        inferredC
    ]>, $>>(a: type.validate<a, $> & (inferredA extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredA
    ]>), b: type.validate<b, $> & (inferredB extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredB
    ]>), c: type.validate<c, $> & (inferredC extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredC
    ]>)): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, inferredA = type.infer<a, $>, inferredB = type.infer<b, $>, inferredC = type.infer<c, $>, inferredD = type.infer<d, $>, r = Type<inferNaryMerge<[
        inferredA,
        inferredB,
        inferredC,
        inferredD
    ]>, $>>(a: type.validate<a, $> & (inferredA extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredA
    ]>), b: type.validate<b, $> & (inferredB extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredB
    ]>), c: type.validate<c, $> & (inferredC extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredC
    ]>), d: type.validate<d, $> & (inferredD extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredD
    ]>)): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, inferredA = type.infer<a, $>, inferredB = type.infer<b, $>, inferredC = type.infer<c, $>, inferredD = type.infer<d, $>, inferredE = type.infer<e, $>, r = Type<inferNaryMerge<[
        inferredA,
        inferredB,
        inferredC,
        inferredD,
        inferredE
    ]>, $>>(a: type.validate<a, $> & (inferredA extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredA
    ]>), b: type.validate<b, $> & (inferredB extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredB
    ]>), c: type.validate<c, $> & (inferredC extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredC
    ]>), d: type.validate<d, $> & (inferredD extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredD
    ]>), e: type.validate<e, $> & (inferredE extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredE
    ]>)): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, inferredA = type.infer<a, $>, inferredB = type.infer<b, $>, inferredC = type.infer<c, $>, inferredD = type.infer<d, $>, inferredE = type.infer<e, $>, inferredF = type.infer<f, $>, r = Type<inferNaryMerge<[
        inferredA,
        inferredB,
        inferredC,
        inferredD,
        inferredE,
        inferredF
    ]>, $>>(a: type.validate<a, $> & (inferredA extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredA
    ]>), b: type.validate<b, $> & (inferredB extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredB
    ]>), c: type.validate<c, $> & (inferredC extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredC
    ]>), d: type.validate<d, $> & (inferredD extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredD
    ]>), e: type.validate<e, $> & (inferredE extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredE
    ]>), f: type.validate<f, $> & (inferredF extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredF
    ]>)): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, inferredA = type.infer<a, $>, inferredB = type.infer<b, $>, inferredC = type.infer<c, $>, inferredD = type.infer<d, $>, inferredE = type.infer<e, $>, inferredF = type.infer<f, $>, inferredG = type.infer<g, $>, r = Type<inferNaryMerge<[
        inferredA,
        inferredB,
        inferredC,
        inferredD,
        inferredE,
        inferredF,
        inferredG
    ]>, $>>(a: type.validate<a, $> & (inferredA extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredA
    ]>), b: type.validate<b, $> & (inferredB extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredB
    ]>), c: type.validate<c, $> & (inferredC extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredC
    ]>), d: type.validate<d, $> & (inferredD extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredD
    ]>), e: type.validate<e, $> & (inferredE extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredE
    ]>), f: type.validate<f, $> & (inferredF extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredF
    ]>), g: type.validate<g, $> & (inferredG extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredG
    ]>)): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, inferredA = type.infer<a, $>, inferredB = type.infer<b, $>, inferredC = type.infer<c, $>, inferredD = type.infer<d, $>, inferredE = type.infer<e, $>, inferredF = type.infer<f, $>, inferredG = type.infer<g, $>, inferredH = type.infer<h, $>, r = Type<inferNaryMerge<[
        inferredA,
        inferredB,
        inferredC,
        inferredD,
        inferredE,
        inferredF,
        inferredG,
        inferredH
    ]>, $>>(a: type.validate<a, $> & (inferredA extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredA
    ]>), b: type.validate<b, $> & (inferredB extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredB
    ]>), c: type.validate<c, $> & (inferredC extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredC
    ]>), d: type.validate<d, $> & (inferredD extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredD
    ]>), e: type.validate<e, $> & (inferredE extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredE
    ]>), f: type.validate<f, $> & (inferredF extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredF
    ]>), g: type.validate<g, $> & (inferredG extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredG
    ]>), h: type.validate<h, $> & (inferredH extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredH
    ]>)): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, inferredA = type.infer<a, $>, inferredB = type.infer<b, $>, inferredC = type.infer<c, $>, inferredD = type.infer<d, $>, inferredE = type.infer<e, $>, inferredF = type.infer<f, $>, inferredG = type.infer<g, $>, inferredH = type.infer<h, $>, inferredI = type.infer<i, $>, r = Type<inferNaryMerge<[
        inferredA,
        inferredB,
        inferredC,
        inferredD,
        inferredE,
        inferredF,
        inferredG,
        inferredH,
        inferredI
    ]>, $>>(a: type.validate<a, $> & (inferredA extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredA
    ]>), b: type.validate<b, $> & (inferredB extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredB
    ]>), c: type.validate<c, $> & (inferredC extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredC
    ]>), d: type.validate<d, $> & (inferredD extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredD
    ]>), e: type.validate<e, $> & (inferredE extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredE
    ]>), f: type.validate<f, $> & (inferredF extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredF
    ]>), g: type.validate<g, $> & (inferredG extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredG
    ]>), h: type.validate<h, $> & (inferredH extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredH
    ]>), i: type.validate<i, $> & (inferredI extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredI
    ]>)): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, const j, inferredA = type.infer<a, $>, inferredB = type.infer<b, $>, inferredC = type.infer<c, $>, inferredD = type.infer<d, $>, inferredE = type.infer<e, $>, inferredF = type.infer<f, $>, inferredG = type.infer<g, $>, inferredH = type.infer<h, $>, inferredI = type.infer<i, $>, inferredJ = type.infer<j, $>, r = Type<inferNaryMerge<[
        inferredA,
        inferredB,
        inferredC,
        inferredD,
        inferredE,
        inferredF,
        inferredG,
        inferredH,
        inferredI,
        inferredJ
    ]>, $>>(a: type.validate<a, $> & (inferredA extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredA
    ]>), b: type.validate<b, $> & (inferredB extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredB
    ]>), c: type.validate<c, $> & (inferredC extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredC
    ]>), d: type.validate<d, $> & (inferredD extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredD
    ]>), e: type.validate<e, $> & (inferredE extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredE
    ]>), f: type.validate<f, $> & (inferredF extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredF
    ]>), g: type.validate<g, $> & (inferredG extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredG
    ]>), h: type.validate<h, $> & (inferredH extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredH
    ]>), i: type.validate<i, $> & (inferredI extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredI
    ]>), j: type.validate<j, $> & (inferredJ extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredJ
    ]>)): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, const j, const k, inferredA = type.infer<a, $>, inferredB = type.infer<b, $>, inferredC = type.infer<c, $>, inferredD = type.infer<d, $>, inferredE = type.infer<e, $>, inferredF = type.infer<f, $>, inferredG = type.infer<g, $>, inferredH = type.infer<h, $>, inferredI = type.infer<i, $>, inferredJ = type.infer<j, $>, inferredK = type.infer<k, $>, r = Type<inferNaryMerge<[
        inferredA,
        inferredB,
        inferredC,
        inferredD,
        inferredE,
        inferredF,
        inferredG,
        inferredH,
        inferredI,
        inferredJ,
        inferredK
    ]>, $>>(a: type.validate<a, $> & (inferredA extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredA
    ]>), b: type.validate<b, $> & (inferredB extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredB
    ]>), c: type.validate<c, $> & (inferredC extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredC
    ]>), d: type.validate<d, $> & (inferredD extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredD
    ]>), e: type.validate<e, $> & (inferredE extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredE
    ]>), f: type.validate<f, $> & (inferredF extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredF
    ]>), g: type.validate<g, $> & (inferredG extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredG
    ]>), h: type.validate<h, $> & (inferredH extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredH
    ]>), i: type.validate<i, $> & (inferredI extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredI
    ]>), j: type.validate<j, $> & (inferredJ extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredJ
    ]>), k: type.validate<k, $> & (inferredK extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredK
    ]>)): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, const j, const k, const l, inferredA = type.infer<a, $>, inferredB = type.infer<b, $>, inferredC = type.infer<c, $>, inferredD = type.infer<d, $>, inferredE = type.infer<e, $>, inferredF = type.infer<f, $>, inferredG = type.infer<g, $>, inferredH = type.infer<h, $>, inferredI = type.infer<i, $>, inferredJ = type.infer<j, $>, inferredK = type.infer<k, $>, inferredL = type.infer<l, $>, r = Type<inferNaryMerge<[
        inferredA,
        inferredB,
        inferredC,
        inferredD,
        inferredE,
        inferredF,
        inferredG,
        inferredH,
        inferredI,
        inferredJ,
        inferredK,
        inferredL
    ]>, $>>(a: type.validate<a, $> & (inferredA extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredA
    ]>), b: type.validate<b, $> & (inferredB extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredB
    ]>), c: type.validate<c, $> & (inferredC extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredC
    ]>), d: type.validate<d, $> & (inferredD extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredD
    ]>), e: type.validate<e, $> & (inferredE extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredE
    ]>), f: type.validate<f, $> & (inferredF extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredF
    ]>), g: type.validate<g, $> & (inferredG extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredG
    ]>), h: type.validate<h, $> & (inferredH extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredH
    ]>), i: type.validate<i, $> & (inferredI extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredI
    ]>), j: type.validate<j, $> & (inferredJ extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredJ
    ]>), k: type.validate<k, $> & (inferredK extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredK
    ]>), l: type.validate<l, $> & (inferredL extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredL
    ]>)): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, const j, const k, const l, const m, inferredA = type.infer<a, $>, inferredB = type.infer<b, $>, inferredC = type.infer<c, $>, inferredD = type.infer<d, $>, inferredE = type.infer<e, $>, inferredF = type.infer<f, $>, inferredG = type.infer<g, $>, inferredH = type.infer<h, $>, inferredI = type.infer<i, $>, inferredJ = type.infer<j, $>, inferredK = type.infer<k, $>, inferredL = type.infer<l, $>, inferredM = type.infer<m, $>, r = Type<inferNaryMerge<[
        inferredA,
        inferredB,
        inferredC,
        inferredD,
        inferredE,
        inferredF,
        inferredG,
        inferredH,
        inferredI,
        inferredJ,
        inferredK,
        inferredL,
        inferredM
    ]>, $>>(a: type.validate<a, $> & (inferredA extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredA
    ]>), b: type.validate<b, $> & (inferredB extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredB
    ]>), c: type.validate<c, $> & (inferredC extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredC
    ]>), d: type.validate<d, $> & (inferredD extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredD
    ]>), e: type.validate<e, $> & (inferredE extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredE
    ]>), f: type.validate<f, $> & (inferredF extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredF
    ]>), g: type.validate<g, $> & (inferredG extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredG
    ]>), h: type.validate<h, $> & (inferredH extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredH
    ]>), i: type.validate<i, $> & (inferredI extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredI
    ]>), j: type.validate<j, $> & (inferredJ extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredJ
    ]>), k: type.validate<k, $> & (inferredK extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredK
    ]>), l: type.validate<l, $> & (inferredL extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredL
    ]>), m: type.validate<m, $> & (inferredM extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredM
    ]>)): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, const j, const k, const l, const m, const n, inferredA = type.infer<a, $>, inferredB = type.infer<b, $>, inferredC = type.infer<c, $>, inferredD = type.infer<d, $>, inferredE = type.infer<e, $>, inferredF = type.infer<f, $>, inferredG = type.infer<g, $>, inferredH = type.infer<h, $>, inferredI = type.infer<i, $>, inferredJ = type.infer<j, $>, inferredK = type.infer<k, $>, inferredL = type.infer<l, $>, inferredM = type.infer<m, $>, inferredN = type.infer<n, $>, r = Type<inferNaryMerge<[
        inferredA,
        inferredB,
        inferredC,
        inferredD,
        inferredE,
        inferredF,
        inferredG,
        inferredH,
        inferredI,
        inferredJ,
        inferredK,
        inferredL,
        inferredM,
        inferredN
    ]>, $>>(a: type.validate<a, $> & (inferredA extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredA
    ]>), b: type.validate<b, $> & (inferredB extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredB
    ]>), c: type.validate<c, $> & (inferredC extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredC
    ]>), d: type.validate<d, $> & (inferredD extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredD
    ]>), e: type.validate<e, $> & (inferredE extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredE
    ]>), f: type.validate<f, $> & (inferredF extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredF
    ]>), g: type.validate<g, $> & (inferredG extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredG
    ]>), h: type.validate<h, $> & (inferredH extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredH
    ]>), i: type.validate<i, $> & (inferredI extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredI
    ]>), j: type.validate<j, $> & (inferredJ extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredJ
    ]>), k: type.validate<k, $> & (inferredK extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredK
    ]>), l: type.validate<l, $> & (inferredL extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredL
    ]>), m: type.validate<m, $> & (inferredM extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredM
    ]>), n: type.validate<n, $> & (inferredN extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredN
    ]>)): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, const j, const k, const l, const m, const n, const o, inferredA = type.infer<a, $>, inferredB = type.infer<b, $>, inferredC = type.infer<c, $>, inferredD = type.infer<d, $>, inferredE = type.infer<e, $>, inferredF = type.infer<f, $>, inferredG = type.infer<g, $>, inferredH = type.infer<h, $>, inferredI = type.infer<i, $>, inferredJ = type.infer<j, $>, inferredK = type.infer<k, $>, inferredL = type.infer<l, $>, inferredM = type.infer<m, $>, inferredN = type.infer<n, $>, inferredO = type.infer<o, $>, r = Type<inferNaryMerge<[
        inferredA,
        inferredB,
        inferredC,
        inferredD,
        inferredE,
        inferredF,
        inferredG,
        inferredH,
        inferredI,
        inferredJ,
        inferredK,
        inferredL,
        inferredM,
        inferredN,
        inferredO
    ]>, $>>(a: type.validate<a, $> & (inferredA extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredA
    ]>), b: type.validate<b, $> & (inferredB extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredB
    ]>), c: type.validate<c, $> & (inferredC extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredC
    ]>), d: type.validate<d, $> & (inferredD extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredD
    ]>), e: type.validate<e, $> & (inferredE extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredE
    ]>), f: type.validate<f, $> & (inferredF extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredF
    ]>), g: type.validate<g, $> & (inferredG extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredG
    ]>), h: type.validate<h, $> & (inferredH extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredH
    ]>), i: type.validate<i, $> & (inferredI extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredI
    ]>), j: type.validate<j, $> & (inferredJ extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredJ
    ]>), k: type.validate<k, $> & (inferredK extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredK
    ]>), l: type.validate<l, $> & (inferredL extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredL
    ]>), m: type.validate<m, $> & (inferredM extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredM
    ]>), n: type.validate<n, $> & (inferredN extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredN
    ]>), o: type.validate<o, $> & (inferredO extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredO
    ]>)): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, const j, const k, const l, const m, const n, const o, const p, inferredA = type.infer<a, $>, inferredB = type.infer<b, $>, inferredC = type.infer<c, $>, inferredD = type.infer<d, $>, inferredE = type.infer<e, $>, inferredF = type.infer<f, $>, inferredG = type.infer<g, $>, inferredH = type.infer<h, $>, inferredI = type.infer<i, $>, inferredJ = type.infer<j, $>, inferredK = type.infer<k, $>, inferredL = type.infer<l, $>, inferredM = type.infer<m, $>, inferredN = type.infer<n, $>, inferredO = type.infer<o, $>, inferredP = type.infer<p, $>, r = Type<inferNaryMerge<[
        inferredA,
        inferredB,
        inferredC,
        inferredD,
        inferredE,
        inferredF,
        inferredG,
        inferredH,
        inferredI,
        inferredJ,
        inferredK,
        inferredL,
        inferredM,
        inferredN,
        inferredO,
        inferredP
    ]>, $>>(a: type.validate<a, $> & (inferredA extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredA
    ]>), b: type.validate<b, $> & (inferredB extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredB
    ]>), c: type.validate<c, $> & (inferredC extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredC
    ]>), d: type.validate<d, $> & (inferredD extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredD
    ]>), e: type.validate<e, $> & (inferredE extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredE
    ]>), f: type.validate<f, $> & (inferredF extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredF
    ]>), g: type.validate<g, $> & (inferredG extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredG
    ]>), h: type.validate<h, $> & (inferredH extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredH
    ]>), i: type.validate<i, $> & (inferredI extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredI
    ]>), j: type.validate<j, $> & (inferredJ extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredJ
    ]>), k: type.validate<k, $> & (inferredK extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredK
    ]>), l: type.validate<l, $> & (inferredL extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredL
    ]>), m: type.validate<m, $> & (inferredM extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredM
    ]>), n: type.validate<n, $> & (inferredN extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredN
    ]>), o: type.validate<o, $> & (inferredO extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredO
    ]>), p: type.validate<p, $> & (inferredP extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredP
    ]>)): r extends infer _ ? _ : never;
    <const a, const b, const c, const d, const e, const f, const g, const h, const i, const j, const k, const l, const m, const n, const o, const p, const q, inferredA = type.infer<a, $>, inferredB = type.infer<b, $>, inferredC = type.infer<c, $>, inferredD = type.infer<d, $>, inferredE = type.infer<e, $>, inferredF = type.infer<f, $>, inferredG = type.infer<g, $>, inferredH = type.infer<h, $>, inferredI = type.infer<i, $>, inferredJ = type.infer<j, $>, inferredK = type.infer<k, $>, inferredL = type.infer<l, $>, inferredM = type.infer<m, $>, inferredN = type.infer<n, $>, inferredO = type.infer<o, $>, inferredP = type.infer<p, $>, inferredQ = type.infer<q, $>, r = Type<inferNaryMerge<[
        inferredA,
        inferredB,
        inferredC,
        inferredD,
        inferredE,
        inferredF,
        inferredG,
        inferredH,
        inferredI,
        inferredJ,
        inferredK,
        inferredL,
        inferredM,
        inferredN,
        inferredO,
        inferredP,
        inferredQ
    ]>, $>>(a: type.validate<a, $> & (inferredA extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredA
    ]>), b: type.validate<b, $> & (inferredB extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredB
    ]>), c: type.validate<c, $> & (inferredC extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredC
    ]>), d: type.validate<d, $> & (inferredD extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredD
    ]>), e: type.validate<e, $> & (inferredE extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredE
    ]>), f: type.validate<f, $> & (inferredF extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredF
    ]>), g: type.validate<g, $> & (inferredG extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredG
    ]>), h: type.validate<h, $> & (inferredH extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredH
    ]>), i: type.validate<i, $> & (inferredI extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredI
    ]>), j: type.validate<j, $> & (inferredJ extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredJ
    ]>), k: type.validate<k, $> & (inferredK extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredK
    ]>), l: type.validate<l, $> & (inferredL extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredL
    ]>), m: type.validate<m, $> & (inferredM extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredM
    ]>), n: type.validate<n, $> & (inferredN extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredN
    ]>), o: type.validate<o, $> & (inferredO extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredO
    ]>), p: type.validate<p, $> & (inferredP extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredP
    ]>), q: type.validate<q, $> & (inferredQ extends object ? unknown : ErrorType<[
        NonObjectMergeErrorMessage,
        actual: inferredQ
    ]>)): r extends infer _ ? _ : never;
    <const defs extends readonly unknown[], r = Type<inferNaryMerge<{
        [i in keyof defs]: type.infer<defs[i], $>;
    }>, $>>(...defs: {
        [i in keyof defs]: type.validate<defs[i], $> & (type.infer<defs[i], $> extends object ? unknown : ErrorType<[
            NonObjectMergeErrorMessage,
            actual: type.infer<defs[i], $>
        ]>);
    }): r extends infer _ ? _ : never;
};
type NaryPipeParser<$, initial = unknown> = {
    (): Type<initial, $>;
    <a extends Morph<distill.Out<initial>>, r = instantiateType<inferMorph<initial, a>, $>>(a: a): r extends infer _ ? _ : never;
    <a extends Morph<distill.Out<initial>>, b extends Morph<inferMorphOut<a>>, r = instantiateType<inferNaryPipe<[
        Type<initial>,
        a,
        b
    ]>, $>>(a: a, b: b): r extends infer _ ? _ : never;
    <a extends Morph<distill.Out<initial>>, b extends Morph<inferMorphOut<a>>, c extends Morph<inferMorphOut<b>>, r = instantiateType<inferNaryPipe<[
        Type<initial>,
        a,
        b,
        c
    ]>, $>>(a: a, b: b, c: c): r extends infer _ ? _ : never;
    <a extends Morph<distill.Out<initial>>, b extends Morph<inferMorphOut<a>>, c extends Morph<inferMorphOut<b>>, d extends Morph<inferMorphOut<c>>, r = instantiateType<inferNaryPipe<[
        Type<initial>,
        a,
        b,
        c,
        d
    ]>, $>>(a: a, b: b, c: c, d: d): r extends infer _ ? _ : never;
    <a extends Morph<distill.Out<initial>>, b extends Morph<inferMorphOut<a>>, c extends Morph<inferMorphOut<b>>, d extends Morph<inferMorphOut<c>>, e extends Morph<inferMorphOut<d>>, r = instantiateType<inferNaryPipe<[
        Type<initial>,
        a,
        b,
        c,
        d,
        e
    ]>, $>>(a: a, b: b, c: c, d: d, e: e): r extends infer _ ? _ : never;
    <a extends Morph<distill.Out<initial>>, b extends Morph<inferMorphOut<a>>, c extends Morph<inferMorphOut<b>>, d extends Morph<inferMorphOut<c>>, e extends Morph<inferMorphOut<d>>, f extends Morph<inferMorphOut<e>>, r = instantiateType<inferNaryPipe<[
        Type<initial>,
        a,
        b,
        c,
        d,
        e,
        f
    ]>, $>>(a: a, b: b, c: c, d: d, e: e, f: f): r extends infer _ ? _ : never;
    <a extends Morph<distill.Out<initial>>, b extends Morph<inferMorphOut<a>>, c extends Morph<inferMorphOut<b>>, d extends Morph<inferMorphOut<c>>, e extends Morph<inferMorphOut<d>>, f extends Morph<inferMorphOut<e>>, g extends Morph<inferMorphOut<f>>, r = instantiateType<inferNaryPipe<[
        Type<initial>,
        a,
        b,
        c,
        d,
        e,
        f,
        g
    ]>, $>>(a: a, b: b, c: c, d: d, e: e, f: f, g: g): r extends infer _ ? _ : never;
    <a extends Morph<distill.Out<initial>>, b extends Morph<inferMorphOut<a>>, c extends Morph<inferMorphOut<b>>, d extends Morph<inferMorphOut<c>>, e extends Morph<inferMorphOut<d>>, f extends Morph<inferMorphOut<e>>, g extends Morph<inferMorphOut<f>>, h extends Morph<inferMorphOut<g>>, r = instantiateType<inferNaryPipe<[
        Type<initial>,
        a,
        b,
        c,
        d,
        e,
        f,
        g,
        h
    ]>, $>>(a: a, b: b, c: c, d: d, e: e, f: f, g: g, h: h): r extends infer _ ? _ : never;
    <a extends Morph<distill.Out<initial>>, b extends Morph<inferMorphOut<a>>, c extends Morph<inferMorphOut<b>>, d extends Morph<inferMorphOut<c>>, e extends Morph<inferMorphOut<d>>, f extends Morph<inferMorphOut<e>>, g extends Morph<inferMorphOut<f>>, h extends Morph<inferMorphOut<g>>, i extends Morph<inferMorphOut<h>>, r = instantiateType<inferNaryPipe<[
        Type<initial>,
        a,
        b,
        c,
        d,
        e,
        f,
        g,
        h,
        i
    ]>, $>>(a: a, b: b, c: c, d: d, e: e, f: f, g: g, h: h, i: i): r extends infer _ ? _ : never;
    <a extends Morph<distill.Out<initial>>, b extends Morph<inferMorphOut<a>>, c extends Morph<inferMorphOut<b>>, d extends Morph<inferMorphOut<c>>, e extends Morph<inferMorphOut<d>>, f extends Morph<inferMorphOut<e>>, g extends Morph<inferMorphOut<f>>, h extends Morph<inferMorphOut<g>>, i extends Morph<inferMorphOut<h>>, j extends Morph<inferMorphOut<i>>, r = instantiateType<inferNaryPipe<[
        Type<initial>,
        a,
        b,
        c,
        d,
        e,
        f,
        g,
        h,
        i,
        j
    ]>, $>>(a: a, b: b, c: c, d: d, e: e, f: f, g: g, h: h, i: i, j: j): r extends infer _ ? _ : never;
    <a extends Morph<distill.Out<initial>>, b extends Morph<inferMorphOut<a>>, c extends Morph<inferMorphOut<b>>, d extends Morph<inferMorphOut<c>>, e extends Morph<inferMorphOut<d>>, f extends Morph<inferMorphOut<e>>, g extends Morph<inferMorphOut<f>>, h extends Morph<inferMorphOut<g>>, i extends Morph<inferMorphOut<h>>, j extends Morph<inferMorphOut<i>>, k extends Morph<inferMorphOut<j>>, r = instantiateType<inferNaryPipe<[
        Type<initial>,
        a,
        b,
        c,
        d,
        e,
        f,
        g,
        h,
        i,
        j,
        k
    ]>, $>>(a: a, b: b, c: c, d: d, e: e, f: f, g: g, h: h, i: i, j: j, k: k): r extends infer _ ? _ : never;
    <a extends Morph<distill.Out<initial>>, b extends Morph<inferMorphOut<a>>, c extends Morph<inferMorphOut<b>>, d extends Morph<inferMorphOut<c>>, e extends Morph<inferMorphOut<d>>, f extends Morph<inferMorphOut<e>>, g extends Morph<inferMorphOut<f>>, h extends Morph<inferMorphOut<g>>, i extends Morph<inferMorphOut<h>>, j extends Morph<inferMorphOut<i>>, k extends Morph<inferMorphOut<j>>, l extends Morph<inferMorphOut<k>>, r = instantiateType<inferNaryPipe<[
        Type<initial>,
        a,
        b,
        c,
        d,
        e,
        f,
        g,
        h,
        i,
        j,
        k,
        l
    ]>, $>>(a: a, b: b, c: c, d: d, e: e, f: f, g: g, h: h, i: i, j: j, k: k, l: l): r extends infer _ ? _ : never;
    <a extends Morph<distill.Out<initial>>, b extends Morph<inferMorphOut<a>>, c extends Morph<inferMorphOut<b>>, d extends Morph<inferMorphOut<c>>, e extends Morph<inferMorphOut<d>>, f extends Morph<inferMorphOut<e>>, g extends Morph<inferMorphOut<f>>, h extends Morph<inferMorphOut<g>>, i extends Morph<inferMorphOut<h>>, j extends Morph<inferMorphOut<i>>, k extends Morph<inferMorphOut<j>>, l extends Morph<inferMorphOut<k>>, m extends Morph<inferMorphOut<l>>, r = instantiateType<inferNaryPipe<[
        Type<initial>,
        a,
        b,
        c,
        d,
        e,
        f,
        g,
        h,
        i,
        j,
        k,
        l,
        m
    ]>, $>>(a: a, b: b, c: c, d: d, e: e, f: f, g: g, h: h, i: i, j: j, k: k, l: l, m: m): r extends infer _ ? _ : never;
    <a extends Morph<distill.Out<initial>>, b extends Morph<inferMorphOut<a>>, c extends Morph<inferMorphOut<b>>, d extends Morph<inferMorphOut<c>>, e extends Morph<inferMorphOut<d>>, f extends Morph<inferMorphOut<e>>, g extends Morph<inferMorphOut<f>>, h extends Morph<inferMorphOut<g>>, i extends Morph<inferMorphOut<h>>, j extends Morph<inferMorphOut<i>>, k extends Morph<inferMorphOut<j>>, l extends Morph<inferMorphOut<k>>, m extends Morph<inferMorphOut<l>>, n extends Morph<inferMorphOut<m>>, r = instantiateType<inferNaryPipe<[
        Type<initial>,
        a,
        b,
        c,
        d,
        e,
        f,
        g,
        h,
        i,
        j,
        k,
        l,
        m,
        n
    ]>, $>>(a: a, b: b, c: c, d: d, e: e, f: f, g: g, h: h, i: i, j: j, k: k, l: l, m: m, n: n): r extends infer _ ? _ : never;
    <a extends Morph<distill.Out<initial>>, b extends Morph<inferMorphOut<a>>, c extends Morph<inferMorphOut<b>>, d extends Morph<inferMorphOut<c>>, e extends Morph<inferMorphOut<d>>, f extends Morph<inferMorphOut<e>>, g extends Morph<inferMorphOut<f>>, h extends Morph<inferMorphOut<g>>, i extends Morph<inferMorphOut<h>>, j extends Morph<inferMorphOut<i>>, k extends Morph<inferMorphOut<j>>, l extends Morph<inferMorphOut<k>>, m extends Morph<inferMorphOut<l>>, n extends Morph<inferMorphOut<m>>, o extends Morph<inferMorphOut<n>>, r = instantiateType<inferNaryPipe<[
        Type<initial>,
        a,
        b,
        c,
        d,
        e,
        f,
        g,
        h,
        i,
        j,
        k,
        l,
        m,
        n,
        o
    ]>, $>>(a: a, b: b, c: c, d: d, e: e, f: f, g: g, h: h, i: i, j: j, k: k, l: l, m: m, n: n, o: o): r extends infer _ ? _ : never;
    <a extends Morph<distill.Out<initial>>, b extends Morph<inferMorphOut<a>>, c extends Morph<inferMorphOut<b>>, d extends Morph<inferMorphOut<c>>, e extends Morph<inferMorphOut<d>>, f extends Morph<inferMorphOut<e>>, g extends Morph<inferMorphOut<f>>, h extends Morph<inferMorphOut<g>>, i extends Morph<inferMorphOut<h>>, j extends Morph<inferMorphOut<i>>, k extends Morph<inferMorphOut<j>>, l extends Morph<inferMorphOut<k>>, m extends Morph<inferMorphOut<l>>, n extends Morph<inferMorphOut<m>>, o extends Morph<inferMorphOut<n>>, p extends Morph<inferMorphOut<o>>, r = instantiateType<inferNaryPipe<[
        Type<initial>,
        a,
        b,
        c,
        d,
        e,
        f,
        g,
        h,
        i,
        j,
        k,
        l,
        m,
        n,
        o,
        p
    ]>, $>>(a: a, b: b, c: c, d: d, e: e, f: f, g: g, h: h, i: i, j: j, k: k, l: l, m: m, n: n, o: o, p: p): r extends infer _ ? _ : never;
    <a extends Morph<distill.Out<initial>>, b extends Morph<inferMorphOut<a>>, c extends Morph<inferMorphOut<b>>, d extends Morph<inferMorphOut<c>>, e extends Morph<inferMorphOut<d>>, f extends Morph<inferMorphOut<e>>, g extends Morph<inferMorphOut<f>>, h extends Morph<inferMorphOut<g>>, i extends Morph<inferMorphOut<h>>, j extends Morph<inferMorphOut<i>>, k extends Morph<inferMorphOut<j>>, l extends Morph<inferMorphOut<k>>, m extends Morph<inferMorphOut<l>>, n extends Morph<inferMorphOut<m>>, o extends Morph<inferMorphOut<n>>, p extends Morph<inferMorphOut<o>>, q extends Morph<inferMorphOut<p>>, r = instantiateType<inferNaryPipe<[
        Type<initial>,
        a,
        b,
        c,
        d,
        e,
        f,
        g,
        h,
        i,
        j,
        k,
        l,
        m,
        n,
        o,
        p,
        q
    ]>, $>>(a: a, b: b, c: c, d: d, e: e, f: f, g: g, h: h, i: i, j: j, k: k, l: l, m: m, n: n, o: o, p: p, q: q): r extends infer _ ? _ : never;
    <const morphs extends readonly Morph[], r = Type<inferNaryPipe<morphs>, $>>(...defs: morphs): r extends infer _ ? _ : never;
};
interface Inferred<out t = unknown, $ = {}> {
    internal: BaseRoot;
    [inferred]: t;
    precompilation: string | undefined;
    t: t;
    $: Scope<$>;
    infer: this["inferOut"];
    inferOut: distill.Out<t>;
    inferIntrospectableOut: distill.introspectable.Out<t>;
    inferIn: distill.In<t>;
    json: JsonStructure;
    toJSON(): JsonStructure;
    toJsonSchema(options?: ToJsonSchema.Options): JsonSchema;
    meta: ArkAmbient.meta;
    description: string;
    expression: string;
    assert: (data: unknown) => this["infer"];
    allows: (data: unknown) => data is this["inferIn"];
    configure: NodeSelector.SelectableFn<TypeMeta.MappableInput, this>;
    describe: NodeSelector.SelectableFn<string, this>;
    onUndeclaredKey(behavior: UndeclaredKeyBehavior): this;
    onDeepUndeclaredKey(behavior: UndeclaredKeyBehavior): this;
    from(literal: this["inferIn"]): this["infer"];
    get in(): instantiateType<this["inferIn"], $>;
    get out(): instantiateType<this["inferIntrospectableOut"], $>;
    brand<const name extends string, r = instantiateType<type.brand<t, name>, $>>(name: name): r extends infer _ ? _ : never;
    array(): Type$5<t[], $>;
    optional(): [
        this,
        "?"
    ];
    default<const value extends defaultFor<this["inferIn"]>>(value: value): [
        this,
        "=",
        value
    ];
    filter<narrowed extends this["inferIn"] = never, r = instantiateType<[
        narrowed
    ] extends [
        never
    ] ? t : t extends InferredMorph<never, infer o> ? (In: narrowed) => o : narrowed, $>>(predicate: Predicate.Castable<this["inferIn"], narrowed>): r extends infer _ ? _ : never;
    narrow<narrowed extends this["infer"] = never, r = instantiateType<[
        narrowed
    ] extends [
        never
    ] ? t : t extends InferredMorph<infer i, infer o> ? o extends To ? (In: i) => To<narrowed> : (In: i) => Out<narrowed> : narrowed, $>>(predicate: Predicate.Castable<this["infer"], narrowed>): r extends infer _ ? _ : never;
    pipe: ChainedPipeParser<$, t>;
    to<const def, r = instantiateType<inferPipe<t, type.infer<def, $>>, $>>(def: type.validate<def, $>): r extends infer _ ? _ : never;
    select: BaseNode["select"];
}
interface Type$1<out t = unknown, $ = {}> extends Callable<(data: unknown) => distill.Out<t> | ArkEnv.onFail>, Inferred<t, $> {
    as<castTo = unset>(...args: validateChainedAsArgs<castTo>): instantiateType<castTo, $>;
    and<const def, r = instantiateType<inferIntersection<t, type.infer<def, $>>, $>>(def: type.validate<def, $>): r extends infer _ ? _ : never;
    or<const def, r = instantiateType<t | type.infer<def, $>, $>>(def: type.validate<def, $>): r extends infer _ ? _ : never;
    intersect<const def, r = instantiateType<inferIntersection<t, type.infer<def, $>>, $>>(def: type.validate<def, $>): r extends infer _ ? _ | Disjoint : never;
    equals<const def>(def: type.validate<def, $>): boolean;
    ifEquals<const def, r = type.instantiate<def, $>>(def: type.validate<def, $>): r extends infer _ ? _ | undefined : never;
    extends<const def>(other: type.validate<def, $>): boolean;
    ifExtends<const def, r = type.instantiate<def, $>>(other: type.validate<def, $>): r extends infer _ ? _ | undefined : never;
    overlaps<const def>(r: type.validate<def, $>): boolean;
    extract<const def, r = instantiateType<t extends type.infer<def, $> ? t : never, $>>(r: type.validate<def, $>): r extends infer _ extends r ? _ : never;
    exclude<const def, r = instantiateType<t extends type.infer<def, $> ? never : t, $>>(r: type.validate<def, $>): r extends infer _ ? _ : never;
    distribute<mapOut, reduceOut = mapOut[]>(mapBranch: (branch: Type$1, i: number, branches: array<Type$1>) => mapOut, reduceMapped?: (mappedBranches: mapOut[]) => reduceOut): reduceOut;
    "~standard": StandardSchemaV1.ArkTypeProps<this["inferIn"] extends infer _ ? _ : never, this["inferOut"] extends infer _ ? _ : never>;
    apply: Function["apply"];
    bind: Function["bind"];
    call: Function["call"];
    caller: Function;
    length: number;
    name: string;
    prototype: Function["prototype"];
    arguments: Function["arguments"];
    Symbol: never;
}
interface ChainedPipeParser<$, t> extends NaryPipeParser<$, t> {
    try: NaryPipeParser<$, t>;
}
type validateChainedAsArgs<t> = [
    t
] extends [
    unset
] ? [
    t
] extends [
    anyOrNever
] ? [
] : [
    ErrorMessage<"as requires an explicit type parameter like myType.as<t>()">
] : [
];
type MatchParserContext<input = unknown> = {
    cases: Morph[];
    $: unknown;
    input: input;
    checked: boolean;
    key: PropertyKey | null;
};
declare namespace ctx {
    type from<ctx extends MatchParserContext> = ctx;
    type init<$, input = unknown, checked extends boolean = false> = from<{
        cases: [
        ];
        $: $;
        input: input;
        checked: checked;
        key: null;
    }>;
    type atKey<ctx extends MatchParserContext, key extends string> = from<{
        cases: ctx["cases"];
        $: ctx["$"];
        input: ctx["input"];
        checked: ctx["checked"];
        key: key;
    }>;
}
interface MatchParser<$> extends CaseMatchParser<ctx.init<$>> {
    in<const def>(def: type.validate<def, $>): ChainableMatchParser<ctx.init<$, type.infer<def, $>, true>>;
    in<const typedInput = never>(...args: [
        typedInput
    ] extends [
        never
    ] ? [
        ErrorMessage<"in requires a definition or type argument (in('string') or in<string>())">
    ] : [
    ]): ChainableMatchParser<ctx.init<$, typedInput>>;
    in<const def>(def: type.validate<def, $>): ChainableMatchParser<ctx.init<$, type.infer<def, $>, true>>;
    case: CaseParser<ctx.init<$>>;
    at: AtParser<ctx.init<$>>;
}
type addCasesToContext<ctx extends MatchParserContext, cases extends unknown[]> = cases extends Morph[] ? ctx.from<{
    $: ctx["$"];
    input: ctx["input"];
    cases: [
        ...ctx["cases"],
        ...cases
    ];
    checked: ctx["checked"];
    key: ctx["key"];
}> : never;
type addDefaultToContext<ctx extends MatchParserContext, defaultCase extends DefaultCase<ctx>> = ctx.from<{
    $: ctx["$"];
    input: defaultCase extends "never" ? Morph.In<ctx["cases"][number]> : ctx["input"];
    cases: defaultCase extends "never" | "assert" ? ctx["cases"] : defaultCase extends Morph ? ctx["checked"] extends true ? [
        (In: unknown) => ArkErrors,
        ...ctx["cases"],
        defaultCase
    ] : [
        ...ctx["cases"],
        defaultCase
    ] : [
        ...ctx["cases"],
        (In: ctx["input"]) => ArkErrors
    ];
    checked: ctx["checked"];
    key: ctx["key"];
}>;
type CaseKeyKind = "def" | "string";
type casesToMorphTuple<cases, ctx extends MatchParserContext, kind extends CaseKeyKind> = unionToTuple<propValueOf<{
    [def in Exclude<keyof cases, "default">]: cases[def] extends (Morph<never, infer o>) ? kind extends "def" ? (In: inferCaseArg<def extends number ? `${number}` : def, ctx, "in">) => o : (In: maybeLiftToKey<def, ctx>) => o : never;
}>>;
type addCasesToParser<cases, ctx extends MatchParserContext, kind extends CaseKeyKind> = cases extends {
    default: infer defaultDef extends DefaultCase<ctx>;
} ? finalizeMatchParser<addCasesToContext<ctx, casesToMorphTuple<cases, ctx, kind>>, defaultDef> : ChainableMatchParser<addCasesToContext<ctx, casesToMorphTuple<cases, ctx, kind>>>;
type inferCaseArg<def, ctx extends MatchParserContext, endpoint extends "in" | "out"> = _finalizeCaseArg<maybeLiftToKey<type.infer<def, ctx["$"]>, ctx>, ctx, endpoint>;
type maybeLiftToKey<t, ctx extends MatchParserContext> = ctx["key"] extends PropertyKey ? {
    [k in ctx["key"]]: t;
} : t;
type _finalizeCaseArg<t, ctx extends MatchParserContext, endpoint extends "in" | "out", ctxInput = ctx["input"]> = ctxInput extends unknown ? t extends unknown ? distill<t, endpoint> extends infer result ? ctxInput extends result ? ctxInput : show<ctxInput & result> : never : never : never;
type CaseParser<ctx extends MatchParserContext> = <const def, ret>(def: type.validate<def, ctx["$"]>, resolve: (In: inferCaseArg<def, ctx, "out">) => ret) => ChainableMatchParser<addCasesToContext<ctx, [
    (In: inferCaseArg<def, ctx, "in">) => ret
]>>;
type validateKey<key extends Key, ctx extends MatchParserContext> = ctx["key"] extends Key ? ErrorMessage<doubleAtMessage> : ctx["cases"]["length"] extends 0 ? keyof ctx["input"] extends never ? key : conform<key, keyof ctx["input"]> : ErrorMessage<chainedAtMessage>;
interface StringsParser<ctx extends MatchParserContext> {
    <const cases>(def: cases extends validateStringCases<cases, ctx> ? cases : validateStringCases<cases, ctx>): addCasesToParser<cases, ctx, "string">;
}
type validateStringCases<cases, ctx extends MatchParserContext> = unknown extends ctx["input"] ? {
    [k in keyof cases]?: k extends "default" ? DefaultCase<ctx> : (In: _finalizeCaseArg<maybeLiftToKey<k, ctx>, ctx, "out">) => unknown;
} & {
    default?: DefaultCase<ctx>;
} : {
    [k in keyof cases]?: k extends "default" ? DefaultCase<ctx> : k extends stringValue<ctx> ? (In: _finalizeCaseArg<maybeLiftToKey<k, ctx>, ctx, "out">) => unknown : ErrorType<`${k & string} must be a possible string value`>;
} & {
    [k in stringValue<ctx>]?: unknown;
} & {
    default?: DefaultCase<ctx>;
};
type stringValue<ctx extends MatchParserContext> = ctx["input"] extends string ? ctx["input"] : ctx["key"] extends keyof ctx["input"] ? ctx["input"][ctx["key"]] extends infer s extends string ? s : never : never;
interface AtParser<ctx extends MatchParserContext> {
    <const key extends string>(key: validateKey<key, ctx>): ChainableMatchParser<ctx.atKey<ctx, key>>;
    <const key extends string, const cases, ctxAtKey extends MatchParserContext = ctx.atKey<ctx, key>>(key: validateKey<key, ctx>, cases: cases extends validateCases<cases, ctxAtKey> ? cases : errorCases<cases, ctxAtKey>): addCasesToParser<cases, ctxAtKey, "def">;
}
interface ChainableMatchParser<ctx extends MatchParserContext> {
    case: CaseParser<ctx>;
    match: CaseMatchParser<ctx>;
    default: DefaultMethod<ctx>;
    at: AtParser<ctx>;
    strings: StringsParser<ctx>;
}
type DefaultCaseKeyword = "never" | "assert" | "reject";
type DefaultCase<ctx extends MatchParserContext = MatchParserContext<any>> = DefaultCaseKeyword | Morph<ctx["input"]>;
type DefaultMethod<ctx extends MatchParserContext> = <const def extends DefaultCase<ctx>>(def: def) => finalizeMatchParser<ctx, def>;
type validateCases<cases, ctx extends MatchParserContext> = {
    [def in keyof cases | BaseCompletions<ctx["$"], {}, "default">]?: def extends "default" ? DefaultCase<ctx> : def extends number ? (In: inferCaseArg<`${def}`, ctx, "out">) => unknown : def extends type.validate<def, ctx["$"]> ? (In: inferCaseArg<def, ctx, "out">) => unknown : type.validate<def, ctx["$"]>;
};
type errorCases<cases, ctx extends MatchParserContext> = {
    [def in keyof cases]?: def extends "default" ? DefaultCase<ctx> : def extends number ? (In: inferCaseArg<`${def}`, ctx, "out">) => unknown : def extends type.validate<def, ctx["$"]> ? (In: inferCaseArg<def, ctx, "out">) => unknown : ErrorType<type.validate<def, ctx["$"]>>;
} & {
    [k in BaseCompletions<ctx["$"], {}>]?: (In: inferCaseArg<k, ctx, "out">) => unknown;
} & {
    default?: DefaultCase<ctx>;
};
type CaseMatchParser<ctx extends MatchParserContext> = <const cases>(def: cases extends validateCases<cases, ctx> ? cases : errorCases<cases, ctx>) => addCasesToParser<cases, ctx, "def">;
type finalizeMatchParser<ctx extends MatchParserContext, defaultCase extends DefaultCase<ctx>> = addDefaultToContext<ctx, defaultCase> extends (infer ctx extends MatchParserContext) ? Match<ctx["input"], ctx["cases"]> : never;
interface Match<In = any, cases extends Morph[] = Morph[]> extends Inferred<(In: Morph.In<cases[number]>) => Out<ReturnType<cases[number]>>> {
    <const data extends In>(data: data): {
        [i in numericStringKeyOf<cases>]: isDisjoint<data, Morph.In<cases[i]>> extends true ? never : Morph.Out<cases[i]>;
    }[numericStringKeyOf<cases>];
}
declare class InternalMatchParser extends Callable<InternalCaseParserFn> {
    $: InternalScope;
    constructor($: InternalScope);
    in(def?: unknown): InternalChainedMatchParser;
    at(key: Key, cases?: InternalCases): InternalChainedMatchParser | Match;
    case(when: unknown, then: Morph): InternalChainedMatchParser;
}
type InternalCases = Record<string, Morph | DefaultCase>;
type InternalCaseParserFn = (cases: InternalCases) => InternalChainedMatchParser | Match;
type CaseEntry = [
    BaseRoot,
    Morph
] | [
    "default",
    DefaultCase
];
declare class InternalChainedMatchParser extends Callable<InternalCaseParserFn> {
    $: InternalScope;
    in: BaseRoot | undefined;
    protected key: Key | undefined;
    protected branches: BaseRoot[];
    constructor($: InternalScope, In?: BaseRoot);
    at(key: Key, cases?: InternalCases): InternalChainedMatchParser | Match;
    case(def: unknown, resolver: Morph): InternalChainedMatchParser;
    protected caseEntry(node: BaseRoot, resolver: Morph): InternalChainedMatchParser;
    match(cases: InternalCases): InternalChainedMatchParser | Match;
    strings(cases: InternalCases): InternalChainedMatchParser | Match;
    protected caseEntries(entries: CaseEntry[]): InternalChainedMatchParser | Match;
    default(defaultCase: DefaultCase): Match;
}
declare const chainedAtMessage = "A key matcher must be specified before the first case i.e. match.at('foo') or match.in<object>().at('bar')";
type chainedAtMessage = typeof chainedAtMessage;
declare const doubleAtMessage = "At most one key matcher may be specified per expression";
type doubleAtMessage = typeof doubleAtMessage;
type maybeValidateTupleExpression<def extends array, $, args> = def extends IndexZeroExpression ? validatePrefixExpression<def, $, args> : def extends IndexOneExpression ? validateIndexOneExpression<def, $, args> : def extends (readonly [
    "",
    ...unknown[]
] | readonly [
    unknown,
    "",
    ...unknown[]
]) ? readonly [
    def[0] extends "" ? BaseCompletions<$, args, IndexZeroOperator | "..."> : def[0],
    def[1] extends "" ? BaseCompletions<$, args, IndexOneOperator | "..."> : def[1]
] : null;
type inferTupleExpression<def extends TupleExpression, $, args> = def[1] extends "[]" ? inferDefinition<def[0], $, args>[] : def[1] extends "?" ? inferDefinition<def[0], $, args> : def[1] extends "&" ? inferIntersection<inferDefinition<def[0], $, args>, inferDefinition<def[2], $, args>> : def[1] extends "|" ? inferDefinition<def[0], $, args> | inferDefinition<def[2], $, args> : def[1] extends ":" ? inferPredicate<inferDefinition<def[0], $, args>, def[2]> : def[1] extends "=>" ? parseMorph<def[0], def[2], $, args> : def[1] extends "|>" ? parseTo<def[0], def[2], $, args> : def[1] extends "=" ? withDefault<inferDefinition<def[0], $, args>, unwrapDefault<def[2]>> : def[1] extends "@" ? inferDefinition<def[0], $, args> : def extends readonly [
    "===",
    ...infer values
] ? values[number] : def extends (readonly [
    "instanceof",
    ...infer constructors extends Constructor[]
]) ? InstanceType<constructors[number]> : def[0] extends "keyof" ? inferKeyOfExpression<def[1], $, args> : never;
type validatePrefixExpression<def extends IndexZeroExpression, $, args> = def["length"] extends 1 ? readonly [
    writeMissingRightOperandMessage<def[0]>
] : def[0] extends "keyof" ? readonly [
    def[0],
    validateDefinition<def[1], $, args>
] : def[0] extends "===" ? readonly [
    def[0],
    ...unknown[]
] : def[0] extends "instanceof" ? readonly [
    def[0],
    ...Constructor[]
] : never;
type validateIndexOneExpression<def extends IndexOneExpression, $, args> = def[1] extends TuplePostfixOperator ? readonly [
    validateDefinition<def[0], $, args>,
    def[1]
] : readonly [
    validateDefinition<def[0], $, args>,
    def["length"] extends 2 ? writeMissingRightOperandMessage<def[1]> : def[1],
    def[1] extends "|" ? validateDefinition<def[2], $, args> : def[1] extends "&" ? validateDefinition<def[2], $, args> : def[1] extends ":" ? Predicate<type.infer.Out<def[0], $, args>> : def[1] extends "=>" ? Morph<type.infer.Out<def[0], $, args>> : def[1] extends "|>" ? validateDefinition<def[2], $, args> : def[1] extends "=" ? defaultFor<type.infer.In<def[0], $, args>> : def[1] extends "@" ? TypeMeta.MappableInput : validateDefinition<def[2], $, args>,
    ...(def[1] extends "@" ? [
        NodeSelector?
    ] : [
    ])
];
type inferKeyOfExpression<operandDef, $, args> = show<keyof inferDefinition<operandDef, $, args>>;
type TupleExpression = IndexZeroExpression | IndexOneExpression;
type ArgTwoOperator = Exclude<IndexOneOperator, "?" | "=">;
type parseTo<inDef, outDef, $, args> = inferPipe<inferDefinition<inDef, $, args>, inferDefinition<outDef, $, args>>;
type parseMorph<inDef, morph, $, args> = morph extends Morph ? inferMorphOut<morph> extends infer out ? (In: distill.In<inferDefinition<inDef, $, args>>) => Out<out> : never : never;
type IndexOneExpression<token extends string = IndexOneOperator> = readonly [
    unknown,
    token,
    ...unknown[]
];
type IndexOneParser<token extends string> = (def: IndexOneExpression<token>, ctx: BaseParseContext) => BaseRoot;
declare const postfixParsers: {
    "?": IndexOneParser<"?">;
    "[]": IndexOneParser<"[]">;
};
type TuplePostfixOperator = keyof typeof postfixParsers;
declare const infixParsers: {
    "&": IndexOneParser<"&">;
    "|": IndexOneParser<"|">;
    "|>": IndexOneParser<"|>">;
    "=": IndexOneParser<"=">;
    ":": IndexOneParser<":">;
    "=>": IndexOneParser<"=>">;
    "@": IndexOneParser<"@">;
};
type TupleInfixOperator = keyof typeof infixParsers;
declare const indexOneParsers: {
    "&": IndexOneParser<"&">;
    "|": IndexOneParser<"|">;
    "|>": IndexOneParser<"|>">;
    "=": IndexOneParser<"=">;
    ":": IndexOneParser<":">;
    "=>": IndexOneParser<"=>">;
    "@": IndexOneParser<"@">;
    "?": IndexOneParser<"?">;
    "[]": IndexOneParser<"[]">;
};
type IndexOneOperator = keyof typeof indexOneParsers;
type IndexZeroParser<token extends string> = (def: IndexZeroExpression<token>, ctx: BaseParseContext) => BaseRoot;
type IndexZeroExpression<token extends string = IndexZeroOperator> = readonly [
    token,
    ...unknown[]
];
declare const indexZeroParsers: {
    keyof: IndexZeroParser<"keyof">;
    instanceof: IndexZeroParser<"instanceof">;
    "===": IndexZeroParser<"===">;
};
type IndexZeroOperator = keyof typeof indexZeroParsers;
type TypeParserAttachments = Omit<TypeParser, never>;
interface TypeParser<$ = {}> extends Ark.boundTypeAttachments<$> {
    <const def, r = type.instantiate<def, $>>(def: type.validate<def, $>): r extends infer _ ? _ : never;
    <const params extends ParameterString, const def, r = Generic<parseValidGenericParams<params, $>, def, $>>(params: validateParameterString<params, $>, def: type.validate<def, $, baseGenericConstraints<parseValidGenericParams<params, $>>>): r extends infer _ ? _ : never;
    <const zero, const one, const rest extends array, r = type.instantiate<[
        zero,
        one,
        ...rest
    ], $>>(_0: zero extends IndexZeroOperator ? zero : type.validate<zero, $>, _1: zero extends "keyof" ? type.validate<one, $> : zero extends "instanceof" ? conform<one, Constructor> : zero extends "===" ? conform<one, unknown> : conform<one, ArgTwoOperator>, ..._2: zero extends "===" ? rest : zero extends "instanceof" ? conform<rest, readonly Constructor[]> : one extends TupleInfixOperator ? one extends ":" ? [
        Predicate<distill.In<type.infer<zero, $>>>
    ] : one extends "=>" ? [
        Morph<distill.Out<type.infer<zero, $>>, unknown>
    ] : one extends "|>" ? [
        type.validate<rest[0], $>
    ] : one extends "@" ? [
        TypeMeta.MappableInput,
        NodeSelector?
    ] : [
        type.validate<rest[0], $>
    ] : [
    ]): r extends infer _ ? _ : never;
    errors: typeof ArkErrors;
    hkt: typeof Hkt;
    keywords: typeof keywords;
    $: Scope<$>;
    raw(def: unknown): Type$1<any, $>;
    module: ModuleParser;
    scope: ScopeParser;
    define: DefinitionParser<$>;
    declare: DeclarationParser<$>;
    generic: GenericParser<$>;
    match: MatchParser<$>;
    schema: SchemaParser<$>;
    unit: UnitTypeParser<$>;
    enumerated: EnumeratedTypeParser<$>;
    valueOf: ValueOfTypeParser<$>;
    instanceOf: InstanceOfTypeParser<$>;
    or: NaryUnionParser<$>;
    and: NaryIntersectionParser<$>;
    merge: NaryMergeParser<$>;
    pipe: NaryPipeParser<$>;
    fn: FnParser<$>;
}
declare class InternalTypeParser extends Callable<(...args: unknown[]) => BaseRoot | Generic, TypeParserAttachments> {
    constructor($: InternalScope);
}
type UnitTypeParser<$> = <const t>(value: t) => Type<t, $>;
type InstanceOfTypeParser<$> = <const t extends object>(ctor: Constructor<t>) => Type<t, $>;
type EnumeratedTypeParser<$> = <const values extends readonly unknown[]>(...values: values) => Type<values[number], $>;
type ValueOfTypeParser<$> = <const o extends object>(o: o) => Type<o[keyof o], $>;
type DefinitionParser<$> = <const def>(def: type.validate<def, $>) => def;
type SchemaParser<$> = (schema: RootSchema, opts?: BaseParseOptions) => Type<unknown, $>;
type TypeConstructor<t = unknown, $ = {}> = new (def: unknown, $: Scope<$>) => Type<t, $>;
type Type<t = unknown, $ = {}> = instantiateType<t, $>;
declare const Type: TypeConstructor;
type BaseFnParser<$ = {}> = <const args extends readonly unknown[], paramsT extends readonly unknown[] = inferTupleLiteral<args extends readonly [
    ...infer params,
    ":",
    unknown
] ? params : args, $, {}>, returnT = args extends readonly [
    ...unknown[],
    ":",
    infer returnDef
] ? type.infer<returnDef, $> : unknown>(...args: {
    [i in keyof args]: conform<args[i], get<validateFnArgs<args, $>, i>>;
}) => <internalSignature extends (...args: distill.Out<paramsT>) => distill.In<returnT>, externalSignature extends Fn = (...args: applyElementLabels<distill.In<paramsT>, Required<Parameters<internalSignature>>>) => args extends readonly [
    ...unknown[],
    ":",
    unknown
] ? distill.Out<returnT> : ReturnType<internalSignature>>(implementation: internalSignature) => TypedFn<externalSignature, $, args extends readonly [
    ...unknown[],
    ":",
    unknown
] ? Return.introspectable : {}>;
interface FnParser<$ = {}> extends BaseFnParser<$> {
    $: Scope<$>;
    raw: RawFnParser;
}
type RawFnParser = (...args: unknown[]) => (...args: unknown[]) => unknown;
declare class InternalFnParser extends Callable<(...args: unknown[]) => Fn> {
    constructor($: InternalScope);
}
declare namespace TypedFn {
    type meta = {
        introspectableReturn?: true;
    };
}
interface TypedFn<signature extends Fn = Fn, $ = {}, meta extends TypedFn.meta = {}> extends Callable<signature> {
    expression: string;
    params: signature extends Fn<infer params> ? Type<params, $> : never;
    returns: Type<meta extends Return.introspectable ? ReturnType<signature> : unknown, $>;
}
declare namespace Return {
    interface introspectable {
        introspectableReturn: true;
    }
}
type validateFnArgs<args, $> = args extends readonly unknown[] ? args extends readonly [
    ...infer paramDefs,
    ":",
    infer returnDef
] ? readonly [
    ...validateFnParamDefs<paramDefs, $>,
    ":",
    type.validate<returnDef, $>
] : validateFnParamDefs<args, $> : never;
type validateFnParamDefs<paramDefs extends readonly unknown[], $> = paramDefs extends validateTupleLiteral<paramDefs, $, {}> ? paramDefs : paramDefs extends {
    [i in keyof paramDefs]: paramDefs[i] extends "..." ? paramDefs[i] : validateInnerDefinition<paramDefs[i], $, {}>;
} ? validateTupleLiteral<paramDefs, $, {}> : {
    [i in keyof paramDefs]: validateInnerDefinition<paramDefs[i], $, {}>;
};
interface ArkScopeConfig extends ArkSchemaScopeConfig {
}
interface ScopeParser {
    <const def>(def: scope.validate<def>, config?: ArkScopeConfig): Scope<scope.infer<def>>;
    define: <const def>(def: scope.validate<def>) => def;
}
type ModuleParser = <const def>(def: scope.validate<def>, config?: ArkScopeConfig) => scope.infer<def> extends infer $ ? Module<{
    [k in exportedNameOf<$>]: $[k];
}> : never;
type bindThis<def> = {
    this: Def<def>;
};
type Def<def = {}> = Brand<def, "unparsed">;
type UnparsedScope = "$";
type PreparsedResolution = PreparsedNodeResolution;
type bootstrapAliases<def> = {
    [k in Exclude<keyof def, GenericDeclaration>]: def[k] extends (PreparsedResolution) ? def[k] extends {
        t: infer g extends GenericAst;
    } ? g : def[k] extends Module<infer $> | BoundModule<infer $, any> ? Submodule<$> : def[k] : def[k] extends (() => infer thunkReturn extends PreparsedResolution) ? thunkReturn extends {
        t: infer g extends GenericAst;
    } ? g : thunkReturn extends Module<infer $> | BoundModule<infer $, any> ? Submodule<$> : thunkReturn : Def<def[k]>;
} & {
    [k in keyof def & GenericDeclaration as extractGenericName<k>]: GenericAst<parseValidGenericParams<extractGenericParameters<k>, bootstrapAliases<def>>, def[k], UnparsedScope>;
};
type inferBootstrapped<$> = {
    [name in keyof $]: $[name] extends Def<infer def> ? inferDefinition<def, $, {}> : $[name] extends {
        t: infer g extends GenericAst;
    } ? bindGenericToScope<g, $> : $[name];
} & unknown;
type bindGenericToScope<g extends GenericAst, $> = GenericAst<g["paramsAst"], g["bodyDef"], g["$"] extends UnparsedScope ? $ : g["$"], $>;
type extractGenericName<k> = k extends GenericDeclaration<infer name> ? name : never;
type extractGenericParameters<k> = k extends `${string}<${infer params}>` ? ParameterString<params> : never;
type resolutionToAst<alias extends string, resolution> = [
    resolution
] extends [
    anyOrNever
] ? InferredAst<resolution, alias> : resolution extends Def<infer def> ? DefAst<def, alias> : resolution extends {
    [arkKind]: "module";
    root: infer root;
} ? InferredAst<root, alias> : resolution extends GenericAst ? resolution : InferredAst<resolution, alias>;
interface InternalScope {
    constructor: typeof InternalScope;
}
declare class InternalScope<$ extends {} = {}> extends BaseScope<$> {
    get ambientAttachments(): Ark.boundTypeAttachments<$> | undefined;
    protected preparseOwnAliasEntry(alias: string, def: unknown): AliasDefEntry;
    parseGenericParams(def: string, opts: BaseParseOptions): array<GenericParamDef>;
    protected normalizeRootScopeValue(resolution: unknown): unknown;
    protected preparseOwnDefinitionFormat(def: unknown, opts: BaseParseOptions): BaseRoot | BaseParseContextInput;
    parseOwnDefinitionFormat(def: unknown, ctx: BaseParseContext): BaseRoot;
    unit: UnitTypeParser<$>;
    valueOf: ValueOfTypeParser<$>;
    enumerated: EnumeratedTypeParser<$>;
    instanceOf: InstanceOfTypeParser<$>;
    or: NaryUnionParser<$>;
    and: NaryIntersectionParser<$>;
    merge: NaryMergeParser<$>;
    pipe: NaryPipeParser<$>;
    fn: InternalFnParser;
    match: InternalMatchParser;
    declare: () => {
        type: InternalTypeParser;
    };
    define<def>(def: def): def;
    type: InternalTypeParser;
    static scope: ScopeParser;
    static module: ModuleParser;
}
declare const scope: ScopeParser;
declare namespace scope {
    type validate<def> = {
        [k in keyof def]: k extends noSuggest ? unknown : parseScopeKey<k, def>["params"] extends infer params ? params extends array<GenericParamAst> ? params["length"] extends 0 ? def[k] extends type.Any | PreparsedResolution ? def[k] : k extends (PrivateDeclaration<infer name extends keyof def & string>) ? ErrorType<writeDuplicateAliasError<name>> : type.validate<def[k], bootstrapAliases<def>, {}> : type.validate<def[k], bootstrapAliases<def>, baseGenericConstraints<params>> : params : never;
    };
    type infer<def> = inferBootstrapped<bootstrapAliases<def>>;
}
interface ScopeConstructor {
    new <$ = {}>(...args: ConstructorParameters<typeof InternalScope>): Scope<$>;
    scope: ScopeParser;
    module: ModuleParser;
}
interface Scope<$ = {}> {
    t: $;
    [arkKind]: "scope";
    config: ArkScopeConfig;
    references: readonly BaseNode[];
    json: JsonStructure;
    exportedNames: array<exportedNameOf<$>>;
    aliases: Record<string, unknown>;
    internal: toInternalScope<$>;
    defineSchema<const def extends RootSchema>(schema: def): def;
    node<kinds extends NodeKind | array<RootKind>>(kinds: kinds, schema: NodeSchema<flattenListable<kinds>>, opts?: BaseParseOptions): nodeOfKind<reducibleKindOf<flattenListable<kinds>>>;
    unit: UnitTypeParser<$>;
    enumerated: EnumeratedTypeParser<$>;
    valueOf: ValueOfTypeParser<$>;
    instanceOf: InstanceOfTypeParser<$>;
    type: TypeParser<$>;
    match: MatchParser<$>;
    fn: FnParser<$>;
    declare: DeclarationParser<$>;
    define: DefinitionParser<$>;
    generic: GenericParser<$>;
    schema: SchemaParser<$>;
    import(): Module<{
        [k in exportedNameOf<$> as PrivateDeclaration<k>]: $[k];
    }>;
    import<names extends exportedNameOf<$>[]>(...names: names): BoundModule<{
        [k in names[number] as PrivateDeclaration<k>]: $[k];
    } & unknown, $>;
    export(): Module<{
        [k in exportedNameOf<$>]: $[k];
    }>;
    export<names extends exportedNameOf<$>[]>(...names: names): BoundModule<{
        [k in names[number]]: $[k];
    } & unknown, $>;
    resolve<name extends exportedNameOf<$>>(name: name): instantiateExport<$[name], $>;
}
declare const Scope: ScopeConstructor;
type parseScopeKey<k, def> = k extends `${infer name}<${infer params}>` ? parseGenericScopeKey<name, params, def> : {
    name: k;
    params: [
    ];
};
type parseGenericScopeKey<name extends string, params extends string, def> = {
    name: name;
    params: parseGenericParams<params, bootstrapAliases<def>>;
};
type InnerParseResult = BaseRoot | ParsedOptionalProperty | ParsedDefaultableProperty;
type inferDefinition<def, $, args> = [
    def
] extends [
    anyOrNever
] ? def : def extends type.cast<infer t> ? ifEmptyObjectLiteral<def, object, t> : def extends ThunkCast<infer t> ? t : def extends string ? inferString<def, $, args> : def extends array ? inferTuple<def, $, args> : def extends RegExp ? string : def extends StandardSchemaV1 ? inferStandardSchema<def> : def extends object ? inferObjectLiteral<def, $, args> : never;
type inferStandardSchema<schema extends StandardSchemaV1, i = StandardSchemaV1.InferInput<schema>, o = StandardSchemaV1.InferOutput<schema>> = [
    i,
    o
] extends [
    o,
    i
] ? i : (In: i) => Out<o>;
type validateDefinition<def, $, args> = null extends undefined ? ErrorMessage<`'strict' or 'strictNullChecks' must be set to true in your tsconfig's 'compilerOptions'`> : [
    def
] extends [
    anyOrNever
] ? def : def extends OptionalPropertyDefinition ? ErrorMessage<shallowOptionalMessage> : isDefaultable<def, $, args> extends true ? ErrorMessage<shallowDefaultableMessage> : validateInnerDefinition<def, $, args>;
type validateInnerDefinition<def, $, args> = [
    def
] extends [
    TerminalObjectDefinition
] ? def : def extends string ? validateString<def, $, args> : unknown extends def ? BaseCompletions<$, args> | {} : def extends readonly unknown[] ? validateTuple<def, $, args> : def extends BadDefinitionType ? ErrorMessage<writeBadDefinitionTypeMessage<objectKindOrDomainOf<def>>> : validateObjectLiteral<def, $, args>;
type validateTuple<def extends array, $, args> = maybeValidateTupleExpression<def, $, args> extends infer result ? result extends null ? validateTupleLiteral<def, $, args> : result : never;
type inferTuple<def extends array, $, args> = def extends TupleExpression ? inferTupleExpression<def, $, args> : inferTupleLiteral<def, $, args>;
type TerminalObjectDefinition = type.cast<unknown> | Fn | RegExp | StandardSchemaV1;
type ThunkCast<t = unknown> = () => type.cast<t>;
type BadDefinitionType = Exclude<Primitive, string>;
declare const writeBadDefinitionTypeMessage: <actual extends string>(actual: actual) => writeBadDefinitionTypeMessage<actual>;
type writeBadDefinitionTypeMessage<actual extends string> = `Type definitions must be strings or objects (was ${actual})`;
type DeclarationParser<$> = <preinferred = unset, ctx extends DeclareContext = {}>() => {
    type: <const def>(def: [
        preinferred
    ] extends [
        unset
    ] ? [
        preinferred
    ] extends [
        anyOrNever
    ] ? validateDeclared<preinferred, def, $, ctx> : ErrorMessage<`declare<ExternalType>() requires a generic argument`> : validateDeclared<preinferred, def, $, ctx>) => Type<finalizePreinferred<preinferred, def, $, ctx>, $>;
};
type finalizePreinferred<preinferred, def, $, ctx extends DeclareContext> = ctx["side"] extends distill.Side ? ctx["side"] extends "in" ? (In: preinferred) => type.infer.Out<def, $> : (In: type.infer.In<def, $>) => Out<preinferred> : preinferred;
type DeclareContext = {
    side?: "in" | "out";
};
type validateDeclared<declared, def, $, ctx extends DeclareContext> = def extends type.validate<def, $> ? validateInference<def, declared, $, bindThis<def>, ctx> : type.validate<def, $>;
type validateInference<def, declared, $, args, ctx extends DeclareContext> = def extends TerminalObjectDefinition | ThunkCast | TupleExpression ? keyof def extends never ? validateObjectInference<def, declared, $, args, ctx> : validateShallowInference<inferDefinition<def, $, args>, declared, ctx> : def extends array ? validateArrayInference<def, declared, $, args, ctx> : def extends object ? validateObjectInference<def, declared, $, args, ctx> : validateShallowInference<inferDefinition<def, $, args>, declared, ctx>;
type validateArrayInference<def extends array, declared, $, args, ctx extends DeclareContext> = declared extends array ? {
    [i in keyof declared]: i extends keyof def ? validateInference<def[i], declared[i], $, args, ctx> : declared[i];
} : show<declarationMismatch<inferDefinition<def, $, args>, declared>>;
type validateObjectInference<def extends object, declared, $, args, ctx extends DeclareContext> = show<{
    [k in requiredKeyOf<declared>]: k extends keyof def ? validateInference<def[k], declared[k], $, args, ctx> : declared[k];
} & {
    [k in optionalKeyOf<declared> & string as declaredOptionalKeySuggestion<k, def>]: declaredOptionalValueSuggestion<def, k, declared, $, args, ctx>;
}>;
type declaredOptionalKeySuggestion<k extends string, def> = k extends keyof def ? def[k] extends OptionalPropertyDefinition ? k : `${k}?` : `${k}?`;
type declaredOptionalValueSuggestion<def, k extends keyof declared & string, declared, $, args, ctx extends DeclareContext> = k extends keyof def ? def[k] extends OptionalPropertyDefinition ? validateInference<def[k], Required<declared>[k], $, args, ctx> : declared[k] : `${k}?` extends keyof def ? validateInference<def[`${k}?`], Required<declared>[k], $, args, ctx> : declared[k];
type validateShallowInference<t, declared, ctx extends DeclareContext, inferred = ctx["side"] extends distill.Side ? distill<t, ctx["side"]> : t> = equals<inferred, declared> extends true ? unknown : show<declarationMismatch<inferred, declared>>;
type declarationMismatch<inferred, declared> = ErrorType<{
    declared: declared;
    inferred: inferred;
}>;
declare class MergeHkt extends Hkt<[
    base: object,
    props: object
]> {
    body: merge<this[0], this[1]>;
    description: string;
}
declare const Merge: GenericRoot<readonly [
    [
        "base",
        object
    ],
    [
        "props",
        object
    ]
], MergeHkt>;
declare const arkBuiltins: arkBuiltins;
type arkBuiltins = Module<arkBuiltins.$>;
declare namespace arkBuiltins {
    type submodule = Submodule<$>;
    type $ = {
        Key: Key;
        Merge: typeof Merge.t;
    };
}
declare const number: number.module;
declare namespace number {
    type module = Module<submodule>;
    type submodule = Submodule<$>;
    type $ = {
        root: number;
        epoch: number;
        integer: number;
        safe: number;
        NaN: number;
        Infinity: number;
        NegativeInfinity: number;
    };
}
declare const stringInteger: stringInteger.module;
declare namespace stringInteger {
    type module = Module<submodule>;
    type submodule = Submodule<$>;
    type $ = {
        root: string;
        parse: (In: string) => To<number>;
    };
}
declare const base64: Module<{
    root: unknown;
    url: unknown;
}>;
declare namespace base64 {
    type module = Module<submodule>;
    type submodule = Submodule<$>;
    type $ = {
        root: string;
        url: string;
    };
}
declare const capitalize: capitalize.module;
declare namespace capitalize {
    type module = Module<submodule>;
    type submodule = Submodule<$>;
    type $ = {
        root: (In: string) => To<string>;
        preformatted: string;
    };
}
declare const stringDate: stringDate.module;
declare namespace stringDate {
    type module = Module<stringDate.submodule>;
    type submodule = Submodule<$>;
    type $ = {
        root: string;
        parse: (In: string) => To<Date>;
        iso: iso.submodule;
        epoch: epoch.submodule;
    };
    namespace iso {
        type submodule = Submodule<$>;
        type $ = {
            root: string;
            parse: (In: string) => To<Date>;
        };
    }
    namespace epoch {
        type submodule = Submodule<$>;
        type $ = {
            root: string;
            parse: (In: string) => To<Date>;
        };
    }
}
declare const ip: ip.module;
declare namespace ip {
    type module = Module<submodule>;
    type submodule = Submodule<$>;
    type $ = {
        root: string;
        v4: string;
        v6: string;
    };
}
declare namespace stringJson {
    type module = Module<submodule>;
    type submodule = Submodule<$>;
    type $ = {
        root: string;
        parse: (In: string) => To<Json>;
    };
}
declare namespace lower {
    type module = Module<submodule>;
    type submodule = Submodule<$>;
    type $ = {
        root: (In: string) => To<string>;
        preformatted: string;
    };
}
declare const normalize: Module<{
    root: unknown;
    NFC: Submodule<{
        root: unknown;
        preformatted: unknown;
    }>;
    NFD: Submodule<{
        root: unknown;
        preformatted: unknown;
    }>;
    NFKC: Submodule<{
        root: unknown;
        preformatted: unknown;
    }>;
    NFKD: Submodule<{
        root: unknown;
        preformatted: unknown;
    }>;
}>;
declare namespace normalize {
    type module = Module<submodule>;
    type submodule = Submodule<$>;
    type $ = {
        root: (In: string) => To<string>;
        NFC: NFC.submodule;
        NFD: NFD.submodule;
        NFKC: NFKC.submodule;
        NFKD: NFKD.submodule;
    };
    namespace NFC {
        type submodule = Submodule<$>;
        type $ = {
            root: (In: string) => To<string>;
            preformatted: string;
        };
    }
    namespace NFD {
        type submodule = Submodule<$>;
        type $ = {
            root: (In: string) => To<string>;
            preformatted: string;
        };
    }
    namespace NFKC {
        type submodule = Submodule<$>;
        type $ = {
            root: (In: string) => To<string>;
            preformatted: string;
        };
    }
    namespace NFKD {
        type submodule = Submodule<$>;
        type $ = {
            root: (In: string) => To<string>;
            preformatted: string;
        };
    }
}
declare const stringNumeric: stringNumeric.module;
declare namespace stringNumeric {
    type module = Module<submodule>;
    type submodule = Submodule<$>;
    type $ = {
        root: string;
        parse: (In: string) => To<number>;
    };
}
declare namespace trim {
    type module = Module<submodule>;
    type submodule = Submodule<$>;
    type $ = {
        root: (In: string) => To<string>;
        preformatted: string;
    };
}
declare const upper: upper.module;
declare namespace upper {
    type module = Module<submodule>;
    type submodule = Submodule<$>;
    type $ = {
        root: (In: string) => To<string>;
        preformatted: string;
    };
}
declare const url: url.module;
declare namespace url {
    type module = Module<submodule>;
    type submodule = Submodule<$>;
    type $ = {
        root: string;
        parse: (In: string) => To<URL>;
    };
}
declare const uuid: Module<{
    root: string;
    v4: unknown;
    v6: unknown;
    v1: unknown;
    v2: unknown;
    v3: unknown;
    v5: unknown;
    v7: unknown;
    v8: unknown;
}>;
declare namespace uuid {
    type module = Module<submodule>;
    type submodule = Submodule<$>;
    type $ = {
        root: string;
        v1: string;
        v2: string;
        v3: string;
        v4: string;
        v5: string;
        v6: string;
        v7: string;
        v8: string;
    };
    namespace $ {
        type flat = {};
    }
}
declare const string: Module<{
    root: unknown;
    alpha: unknown;
    alphanumeric: unknown;
    hex: unknown;
    base64: Submodule<{
        root: unknown;
        url: unknown;
    }>;
    capitalize: Submodule<capitalize.submodule>;
    creditCard: unknown;
    date: Submodule<stringDate.submodule>;
    digits: unknown;
    email: unknown;
    integer: Submodule<stringInteger.submodule>;
    ip: Submodule<ip.submodule>;
    json: Submodule<stringJson.submodule>;
    lower: Submodule<lower.submodule>;
    normalize: Submodule<{
        root: unknown;
        NFC: Submodule<{
            root: unknown;
            preformatted: unknown;
        }>;
        NFD: Submodule<{
            root: unknown;
            preformatted: unknown;
        }>;
        NFKC: Submodule<{
            root: unknown;
            preformatted: unknown;
        }>;
        NFKD: Submodule<{
            root: unknown;
            preformatted: unknown;
        }>;
    }>;
    numeric: Submodule<stringNumeric.submodule>;
    regex: unknown;
    semver: unknown;
    trim: Submodule<trim.submodule>;
    upper: Submodule<upper.submodule>;
    url: Submodule<url.submodule>;
    uuid: Submodule<{
        root: string;
        v4: unknown;
        v6: unknown;
        v1: unknown;
        v2: unknown;
        v3: unknown;
        v5: unknown;
        v7: unknown;
        v8: unknown;
    }>;
}>;
declare namespace string {
    type module = Module<string.submodule>;
    type submodule = Submodule<$>;
    type $ = {
        root: string;
        alpha: string;
        alphanumeric: string;
        hex: string;
        base64: base64.submodule;
        capitalize: capitalize.submodule;
        creditCard: string;
        date: stringDate.submodule;
        digits: string;
        email: string;
        integer: stringInteger.submodule;
        ip: ip.submodule;
        json: stringJson.submodule;
        lower: lower.submodule;
        normalize: normalize.submodule;
        numeric: stringNumeric.submodule;
        regex: string;
        semver: string;
        trim: trim.submodule;
        upper: upper.submodule;
        url: url.submodule;
        uuid: uuid.submodule;
    };
}
declare const arkTsKeywords: arkTsKeywords;
type arkTsKeywords = Module<arkTsKeywords.$>;
declare namespace arkTsKeywords {
    type submodule = Submodule<$>;
    type $ = {
        bigint: bigint;
        boolean: boolean;
        false: false;
        never: never;
        null: null;
        number: number;
        object: object;
        string: string;
        symbol: symbol;
        true: true;
        unknown: unknown;
        undefined: undefined;
    };
}
declare const unknown: Module<{
    root: unknown;
    any: unknown;
}>;
declare namespace unknown {
    type submodule = Submodule<$>;
    type $ = {
        root: unknown;
        any: any;
    };
}
declare const json: Module<{
    root: unknown;
    stringify: unknown;
}>;
declare namespace json {
    type submodule = Submodule<$>;
    type $ = {
        root: Json;
        stringify: (In: Json) => To<string>;
    };
}
declare const object: Module<{
    root: unknown;
    json: Submodule<{
        root: unknown;
        stringify: unknown;
    }>;
}>;
declare namespace object {
    type submodule = Submodule<$>;
    type $ = {
        root: object;
        json: json.submodule;
    };
}
declare class RecordHkt extends Hkt<[
    Key,
    unknown
]> {
    body: Record$1<this[0], this[1]>;
    description: string;
}
declare const Record$1: GenericRoot<readonly [
    [
        "K",
        Key
    ],
    [
        "V",
        unknown
    ]
], RecordHkt>;
declare class PickHkt extends Hkt<[
    object,
    Key
]> {
    body: pick<this[0], this[1] & keyof this[0]>;
    description: string;
}
declare const Pick: GenericRoot<readonly [
    [
        "T",
        object
    ],
    [
        "K",
        Key
    ]
], PickHkt>;
declare class OmitHkt extends Hkt<[
    object,
    Key
]> {
    body: omit<this[0], this[1] & keyof this[0]>;
    description: string;
}
declare const Omit$1: GenericRoot<readonly [
    [
        "T",
        object
    ],
    [
        "K",
        Key
    ]
], OmitHkt>;
declare class PartialHkt extends Hkt<[
    object
]> {
    body: show<Partial$1<this[0]>>;
    description: string;
}
declare const Partial$1: GenericRoot<readonly [
    [
        "T",
        object
    ]
], PartialHkt>;
declare class RequiredHkt extends Hkt<[
    object
]> {
    body: show<Required$1<this[0]>>;
    description: string;
}
declare const Required$1: GenericRoot<readonly [
    [
        "T",
        object
    ]
], RequiredHkt>;
declare class ExcludeHkt extends Hkt<[
    unknown,
    unknown
]> {
    body: Exclude$1<this[0], this[1]>;
    description: string;
}
declare const Exclude$1: GenericRoot<readonly [
    [
        "T",
        unknown
    ],
    [
        "U",
        unknown
    ]
], ExcludeHkt>;
declare class ExtractHkt extends Hkt<[
    unknown,
    unknown
]> {
    body: Extract$1<this[0], this[1]>;
    description: string;
}
declare const Extract$1: GenericRoot<readonly [
    [
        "T",
        unknown
    ],
    [
        "U",
        unknown
    ]
], ExtractHkt>;
declare const arkTsGenerics: arkTsGenerics.module;
declare namespace arkTsGenerics {
    type module = Module<arkTsGenerics.$>;
    type submodule = Submodule<$>;
    type $ = {
        Exclude: typeof Exclude$1.t;
        Extract: typeof Extract$1.t;
        Omit: typeof Omit$1.t;
        Partial: typeof Partial$1.t;
        Pick: typeof Pick.t;
        Record: typeof Record$1.t;
        Required: typeof Required$1.t;
    };
}
interface Ark extends Omit<Ark.keywords, keyof Ark.wrapped>, Ark.wrapped {
}
declare namespace Ark {
    interface keywords extends arkTsKeywords.$, arkTsGenerics.$, arkPrototypes.keywords, arkBuiltins.$ {
    }
    interface wrapped extends arkPrototypes.wrapped {
        string: string.submodule;
        number: number.submodule;
        object: object.submodule;
        unknown: unknown.submodule;
    }
    type flat = flatResolutionsOf<Ark>;
    interface typeAttachments extends arkTsKeywords.$ {
        arrayIndex: arkPrototypes.$["Array"]["index"];
        Key: arkBuiltins.$["Key"];
        Record: arkTsGenerics.$["Record"];
        Date: arkPrototypes.$["Date"];
        Array: arkPrototypes.$["Array"]["root"];
    }
    interface boundTypeAttachments<$> extends Omit<BoundModule<typeAttachments, $>, arkKind> {
    }
}
declare const keywords: Module<Ark>;
declare const type: TypeParser<{}>;
declare namespace type {
    interface cast<to> {
        [inferred]?: to;
    }
    type errors = ArkErrors;
    type validate<def, $ = {}, args = bindThis<def>> = validateDefinition<def, $, args>;
    type instantiate<def, $ = {}, args = bindThis<def>> = instantiateType<inferDefinition<def, $, args>, $>;
    type infer<def, $ = {}, args = bindThis<def>> = inferDefinition<def, $, args>;
    namespace infer {
        type In<def, $ = {}, args = {}> = distill.In<inferDefinition<def, $, args>>;
        type Out<def, $ = {}, args = {}> = distill.Out<inferDefinition<def, $, args>>;
        namespace introspectable {
            type Out<def, $ = {}, args = {}> = distill.introspectable.Out<inferDefinition<def, $, args>>;
        }
    }
    type brand<t, id> = t extends InferredMorph<infer i, infer o> ? o["introspectable"] extends true ? (In: i) => To<Brand<o["t"], id>> : (In: i) => Out<Brand<o["t"], id>> : Brand<t, id>;
    interface Any<out t = any, $ = any> extends Type$1<t, $> {
    }
}
type type<t = unknown, $ = {}> = Type<t, $>;
declare const match: MatchParser<{}>;
type ParameterString<params extends string = string> = `<${params}>`;
type extractParams<s extends ParameterString> = s extends ParameterString<infer params> ? params : never;
type validateParameterString<s extends ParameterString, $> = parseGenericParams<extractParams<s>, $> extends infer e extends ErrorMessage ? e : s;
type validateGenericArg<arg, param extends GenericParamAst, $> = type.infer<arg, $> extends param[1] ? unknown : ErrorType<[
    `Invalid argument for ${param[0]}`,
    expected: param[1]
]>;
type GenericInstantiator<params extends array<GenericParamAst>, def, $, args$> = params["length"] extends 1 ? {
    <const a, r = instantiateGeneric<def, params, [
        a
    ], $, args$>>(a: type.validate<a, args$> & validateGenericArg<a, params[0], args$>): r extends infer _ ? _ : never;
} : params["length"] extends 2 ? {
    <const a, const b, r = instantiateGeneric<def, params, [
        a,
        b
    ], $, args$>>(...args: [
        type.validate<a, args$> & validateGenericArg<a, params[0], args$>,
        type.validate<b, args$> & validateGenericArg<b, params[1], args$>
    ]): r extends infer _ ? _ : never;
} : params["length"] extends 3 ? {
    <const a, const b, const c, r = instantiateGeneric<def, params, [
        a,
        b,
        c
    ], $, args$>>(...args: [
        type.validate<a, args$> & validateGenericArg<a, params[0], args$>,
        type.validate<b, args$> & validateGenericArg<b, params[1], args$>,
        type.validate<c, args$> & validateGenericArg<c, params[2], args$>
    ]): r extends infer _ ? _ : never;
} : params["length"] extends 4 ? {
    <const a, const b, const c, const d, r = instantiateGeneric<def, params, [
        a,
        b,
        c,
        d
    ], $, args$>>(...args: [
        type.validate<a, args$> & validateGenericArg<a, params[0], args$>,
        type.validate<b, args$> & validateGenericArg<b, params[1], args$>,
        type.validate<c, args$> & validateGenericArg<c, params[2], args$>,
        type.validate<d, args$> & validateGenericArg<d, params[3], args$>
    ]): r extends infer _ ? _ : never;
} : params["length"] extends 5 ? {
    <const a, const b, const c, const d, const e, r = instantiateGeneric<def, params, [
        a,
        b,
        c,
        d,
        e
    ], $, args$>>(...args: [
        type.validate<a, args$> & validateGenericArg<a, params[0], args$>,
        type.validate<b, args$> & validateGenericArg<b, params[1], args$>,
        type.validate<c, args$> & validateGenericArg<c, params[2], args$>,
        type.validate<d, args$> & validateGenericArg<d, params[3], args$>,
        type.validate<e, args$> & validateGenericArg<e, params[4], args$>
    ]): r extends infer _ ? _ : never;
} : params["length"] extends 6 ? {
    <const a, const b, const c, const d, const e, const f, r = instantiateGeneric<def, params, [
        a,
        b,
        c,
        d,
        e,
        f
    ], $, args$>>(...args: [
        type.validate<a, args$> & validateGenericArg<a, params[0], args$>,
        type.validate<b, args$> & validateGenericArg<b, params[1], args$>,
        type.validate<c, args$> & validateGenericArg<c, params[2], args$>,
        type.validate<d, args$> & validateGenericArg<d, params[3], args$>,
        type.validate<e, args$> & validateGenericArg<e, params[4], args$>,
        type.validate<f, args$> & validateGenericArg<f, params[5], args$>
    ]): r extends infer _ ? _ : never;
} : (error: ErrorMessage<`You may not define more than 6 positional generic parameters`>) => never;
type instantiateGeneric<def, params extends array<GenericParamAst>, args, $, args$> = Type<[
    def
] extends [
    Hkt
] ? Hkt.apply<def, {
    [i in keyof args]: type.infer<args[i], args$>;
}> : inferDefinition<def, $, bindGenericArgs<params, args$, args>>, args$>;
type bindGenericArgs<params extends array<GenericParamAst>, $, args> = {
    [i in keyof params & `${number}` as params[i][0]]: type.infer<args[i & keyof args], $>;
};
type baseGenericResolutions<params extends array<GenericParamAst>, $> = baseGenericConstraints<params> extends infer baseConstraints ? {
    [k in keyof baseConstraints]: Type<baseConstraints[k], $>;
} : never;
type baseGenericConstraints<params extends array<GenericParamAst>> = {
    [i in keyof params & `${number}` as params[i][0]]: params[i][1];
};
type GenericConstructor<params extends array<GenericParamAst> = array<GenericParamAst>, bodyDef = unknown, $ = {}, arg$ = {}> = new () => Generic<params, bodyDef, $, arg$>;
interface Generic<params extends array<GenericParamAst> = array<GenericParamAst>, bodyDef = unknown, $ = {}, arg$ = $> extends Callable<GenericInstantiator<params, bodyDef, $, arg$>> {
    [arkKind]: "generic";
    t: GenericAst<params, bodyDef, $, arg$>;
    bodyDef: bodyDef;
    params: {
        [i in keyof params]: [
            params[i][0],
            Type<params[i][1], $>
        ];
    };
    names: genericParamNames<params>;
    constraints: {
        [i in keyof params]: Type<params[i][1], $>;
    };
    $: Scope<$>;
    arg$: Scope<arg$>;
    internal: GenericRoot;
    json: JsonStructure;
}
declare const Generic: GenericConstructor;
type GenericDeclaration<name extends string = string, params extends ParameterString = ParameterString> = `${name}${params}`;
type parseValidGenericParams<def extends ParameterString, $> = conform<parseGenericParams<extractParams<def>, $>, array<GenericParamAst>>;
declare const emptyGenericParameterMessage = "An empty string is not a valid generic parameter name";
type emptyGenericParameterMessage = typeof emptyGenericParameterMessage;
type parseGenericParams<def extends string, $> = parseNextNameChar<Scanner.skipWhitespace<def>, "", [
], $>;
type ParamsTerminator = WhitespaceChar | ",";
type parseName<unscanned extends string, result extends array<GenericParamAst>, $> = parseNextNameChar<Scanner.skipWhitespace<unscanned>, "", result, $>;
type parseNextNameChar<unscanned extends string, name extends string, result extends array<GenericParamAst>, $> = unscanned extends `${infer lookahead}${infer nextUnscanned}` ? lookahead extends ParamsTerminator ? name extends "" ? ErrorMessage<emptyGenericParameterMessage> : lookahead extends "," ? parseName<nextUnscanned, [
    ...result,
    [
        name,
        unknown
    ]
], $> : lookahead extends WhitespaceChar ? _parseOptionalConstraint<nextUnscanned, name, result, $> : never : parseNextNameChar<nextUnscanned, `${name}${lookahead}`, result, $> : name extends "" ? result : [
    ...result,
    [
        name,
        unknown
    ]
];
declare const extendsToken = "extends ";
type extendsToken = typeof extendsToken;
declare const _parseOptionalConstraint: (scanner: Scanner, name: string, result: GenericParamDef[], ctx: BaseParseContext) => GenericParamDef[];
type _parseOptionalConstraint<unscanned extends string, name extends string, result extends array<GenericParamAst>, $> = Scanner.skipWhitespace<unscanned> extends (`${extendsToken}${infer nextUnscanned}`) ? parseUntilFinalizer<s.initialize<nextUnscanned>, $, {}> extends (infer finalArgState extends StaticState) ? validateAst<finalArgState["root"], $, {}> extends (infer e extends ErrorMessage) ? e : parseName<finalArgState["unscanned"], [
    ...result,
    [
        name,
        inferAstRoot<finalArgState["root"], $, {}>
    ]
], $> : never : parseName<Scanner.skipWhitespace<unscanned> extends `,${infer nextUnscanned}` ? nextUnscanned : unscanned, [
    ...result,
    [
        name,
        unknown
    ]
], $>;
type genericParamDefToAst<schema extends GenericParamDef, $> = schema extends string ? [
    schema,
    unknown
] : schema extends readonly [
    infer name,
    infer def
] ? [
    name,
    type.infer<def, $>
] : never;
type genericParamDefsToAst<defs extends array<GenericParamDef>, $> = [
    ...{
        [i in keyof defs]: genericParamDefToAst<defs[i], $>;
    }
];
type GenericParser<$ = {}> = <const paramsDef extends array<GenericParamDef>>(...params: {
    [i in keyof paramsDef]: paramsDef[i] extends (readonly [
        infer name,
        infer def
    ]) ? readonly [
        name,
        type.validate<def, $>
    ] : paramsDef[i];
}) => GenericBodyParser<genericParamDefsToAst<paramsDef, $>, $>;
interface GenericBodyParser<params extends array<GenericParamAst>, $> {
    <const body>(body: type.validate<body, $, baseGenericConstraints<params>>): Generic<params, body, $, $>;
    <hkt extends Hkt.constructor>(instantiateDef: LazyGenericBody<baseGenericResolutions<params, $>>, hkt: hkt): Generic<params, InstanceType<hkt>, $, $>;
}
declare const Module: new <$ extends {}>(exports: exportScope<$>) => Module<$>;
interface Module<$ extends {} = {}> extends RootModule<exportScope<$>> {
}
type exportScope<$> = bindExportsToScope<$, $>;
declare const BoundModule: new <exports extends {}, $ extends {}>(exports: bindExportsToScope<exports, $>, $: $) => BoundModule<exports, $>;
interface BoundModule<exports extends {}, $> extends RootModule<bindExportsToScope<exports, $>> {
}
type bindExportsToScope<exports, $> = {
    [k in keyof exports]: instantiateExport<exports[k], $>;
} & unknown;
type Submodule<exports extends {}> = RootModule<exports & ("root" extends keyof exports ? {
    [inferred]: exports["root"];
} : {})>;
type instantiateExport<t, $> = [
    t
] extends [
    PreparsedNodeResolution
] ? [
    t
] extends [
    anyOrNever
] ? Type<t, $> : t extends GenericAst<infer params, infer body, infer body$> ? Generic<params, body, body$, $> : t extends Submodule<infer exports> ? BoundModule<exports, $> : never : Type<t, $>;
declare class liftFromHkt extends Hkt<[
    element: unknown
]> {
    body: liftArray<this[0]> extends infer lifted ? (In: this[0] | lifted) => To<lifted> : never;
}
declare const liftFrom: GenericRoot<readonly [
    [
        "element",
        unknown
    ]
], liftFromHkt>;
declare const arkArray: arkArray.module;
declare namespace arkArray {
    type module = Module<submodule>;
    type submodule = Submodule<$>;
    type $ = {
        root: unknown[];
        readonly: readonly unknown[];
        index: NonNegativeIntegerString;
        liftFrom: typeof liftFrom.t;
    };
}
type NonNegativeIntegerString = `${Digit}` | (`${Exclude<Digit, 0>}${string}` & `${bigint}`);
type FormDataValue = string | File;
type ParsedFormData = Record<string, FormDataValue | FormDataValue[]>;
declare const arkFormData: arkFormData.module;
declare namespace arkFormData {
    type module = Module<submodule>;
    type submodule = Submodule<$>;
    type $ = {
        root: FormData;
        value: FormDataValue;
        parse: (In: FormData) => To<ParsedFormData>;
        parsed: ParsedFormData;
    };
}
declare const TypedArray: TypedArray.module;
declare namespace TypedArray {
    type module = Module<TypedArray.$>;
    type submodule = Submodule<$>;
    type $ = {
        Int8: Int8Array;
        Uint8: Uint8Array;
        Uint8Clamped: Uint8ClampedArray;
        Int16: Int16Array;
        Uint16: Uint16Array;
        Int32: Int32Array;
        Uint32: Uint32Array;
        Float32: Float32Array;
        Float64: Float64Array;
        BigInt64: BigInt64Array;
        BigUint64: BigUint64Array;
    };
}
declare const omittedPrototypes: {
    Boolean: 1;
    Number: 1;
    String: 1;
};
declare const arkPrototypes: arkPrototypes.module;
declare namespace arkPrototypes {
    type module = Module<submodule>;
    type submodule = Submodule<$>;
    interface keywords extends ecmascript, platform {
    }
    interface $ extends Omit<keywords, keyof wrapped>, wrapped {
    }
    interface wrapped {
        Array: arkArray.submodule;
        TypedArray: TypedArray.submodule;
        FormData: arkFormData.submodule;
    }
    type ecmascript = Omit<EcmascriptObjects, keyof typeof omittedPrototypes>;
    type platform = PlatformObjects;
    interface instances extends ecmascript, platform {
    }
    type NonDegenerateName = keyof instances extends infer k ? k extends keyof instances ? {} extends instances[k] ? never : k : never : never;
    type instanceOf<name extends NonDegenerateName = NonDegenerateName> = instances[name];
}
type DateLiteral<source extends string = string> = `d"${source}"` | `d'${source}'`;
type LimitLiteral = number | DateLiteral;
type distill<t, side extends distill.Side> = finalizeDistillation<t, _distill<t, side>>;
declare namespace distill {
    type Side = "in" | "out" | "introspectableOut";
    type In<t> = distill<t, "in">;
    type Out<t> = distill<t, "out">;
    namespace introspectable {
        type Out<t> = distill<t, "introspectableOut">;
    }
}
type finalizeDistillation<t, distilled> = equals<t, distilled> extends true ? t : distilled;
type _distill<t, side extends distill.Side> = t extends undefined ? t : [
    t
] extends [
    anyOrNever
] ? t : unknown extends t ? unknown : t extends Brand<infer base> ? side extends "in" ? base : t : t extends TerminallyInferredObject | Primitive ? t : t extends Function ? t extends (...args: never) => anyOrNever ? t : t extends InferredMorph<infer i, infer o> ? distillIo<i, o, side> : t : t extends Default<infer constraint> ? _distill<constraint, side> : t extends array ? distillArray<t, side> : isSafelyMappable<t> extends true ? distillMappable<t, side> : t;
type distillMappable<o, side extends distill.Side> = side extends "in" ? show<{
    [k in keyof o as k extends inferredDefaultKeyOf<o> ? never : k]: _distill<o[k], side>;
} & {
    [k in inferredDefaultKeyOf<o>]?: _distill<o[k], side>;
}> : {
    [k in keyof o]: _distill<o[k], side>;
};
type distillIo<i, o extends Out, side extends distill.Side> = side extends "out" ? _distill<o["t"], side> : side extends "in" ? _distill<i, side> : o extends To<infer validatedOut> ? _distill<validatedOut, side> : unknown;
type unwrapInput<t> = t extends InferredMorph<infer i> ? t extends anyOrNever ? t : i : t;
type inferredDefaultKeyOf<o> = keyof o extends infer k ? k extends keyof o ? unwrapInput<o[k]> extends Default<infer t> ? [
    t
] extends [
    anyOrNever
] ? never : k : never : never : never;
type distillArray<t extends array, side extends distill.Side> = t[number][] extends t ? alignReadonly<_distill<t[number], side>[], t> : distillNonArraykeys<t, alignReadonly<distillArrayFromPrefix<[
    ...t
], side, [
]>, t>, side>;
type alignReadonly<result extends unknown[], original extends array> = original extends unknown[] ? result : Readonly<result>;
type distillNonArraykeys<originalArray extends array, distilledArray, side extends distill.Side> = keyof originalArray extends keyof distilledArray ? distilledArray : distilledArray & _distill<{
    [k in keyof originalArray as k extends keyof distilledArray ? never : k]: originalArray[k];
}, side>;
type distillArrayFromPrefix<t extends array, side extends distill.Side, prefix extends array> = t extends readonly [
    infer head,
    ...infer tail
] ? distillArrayFromPrefix<tail, side, [
    side,
    head
] extends [
    "in",
    Default
] ? [
    ...prefix,
    _distill<head, side>?
] : [
    ...prefix,
    _distill<head, side>
]> : [
    ...prefix,
    ...distillArrayFromPostfix<t, side, [
    ]>
];
type distillArrayFromPostfix<t extends array, side extends distill.Side, postfix extends array> = t extends readonly [
    ...infer init,
    infer last
] ? distillArrayFromPostfix<init, side, [
    _distill<last, side>,
    ...postfix
]> : [
    ...{
        [i in keyof t]: _distill<t[i], side>;
    },
    ...postfix
];
type BuiltinTerminalObjectKind = Exclude<arkPrototypes.NonDegenerateName, "Array" | "Function">;
type TerminallyInferredObject = arkPrototypes.instanceOf<BuiltinTerminalObjectKind> | ArkEnv.prototypes;
type inferPredicate<t, predicate> = predicate extends (data: any, ...args: any[]) => data is infer narrowed ? narrowed : t;
type inferNaryPipe<morphs extends readonly Morph[]> = _inferNaryPipe<morphs, unknown>;
type _inferNaryPipe<remaining extends readonly unknown[], result> = remaining extends (readonly [
    infer head extends Morph,
    ...infer tail extends Morph[]
]) ? _inferNaryPipe<tail, inferMorph<result, head>> : result;
type inferNaryIntersection<types extends readonly unknown[]> = number extends types["length"] ? _inferNaryIntersection<unionToTuple<types[number]>, unknown> : _inferNaryIntersection<types, unknown>;
type _inferNaryIntersection<remaining extends readonly unknown[], result> = remaining extends readonly [
    infer head,
    ...infer tail
] ? _inferNaryIntersection<tail, inferIntersection<result, head>> : result;
type inferNaryMerge<types extends readonly unknown[]> = number extends types["length"] ? _inferUnorderedMerge<types> : _inferNaryMerge<types, {}>;
type _inferUnorderedMerge<types extends readonly unknown[], optionalKey extends PropertyKey = optionalAtLeastOnceUnionKeyOf<types[number]>, requiredKey extends PropertyKey = Exclude<unionKeyOf<types[number]>, optionalKey>> = show<{
    [k in requiredKey]: types[number] extends infer v ? v extends unknown ? k extends keyof v ? v[k] : never : never : never;
} & {
    [k in optionalKey]?: types[number] extends infer v ? v extends unknown ? k extends keyof v ? v[k] : never : never : never;
}>;
type optionalAtLeastOnceUnionKeyOf<t> = t extends unknown ? optionalKeyOf<t> : never;
type _inferNaryMerge<remaining extends readonly unknown[], result> = remaining extends (readonly [
    infer head,
    ...infer tail extends readonly unknown[]
]) ? _inferNaryMerge<tail, merge<result, head>> : result;
type inferMorphOut<morph extends Morph> = Exclude<ReturnType<morph>, ArkError | ArkErrors>;
declare const isMorphOutKey: " isMorphOut";
interface Out<o = any> {
    [isMorphOutKey]: true;
    t: o;
    introspectable: boolean;
}
interface To<o = any> extends Out<o> {
    introspectable: true;
}
type InferredMorph<i = never, o extends Out = Out> = (In: i) => o;
declare const defaultsToKey: " defaultsTo";
type Default<t = unknown, v = unknown> = {
    [defaultsToKey]: [
        t,
        v
    ];
};
type withDefault<t, v, undistributed = t> = t extends InferredMorph ? addDefaultToMorph<t, v> : Default<Exclude<undistributed, InferredMorph>, v>;
type addDefaultToMorph<t extends InferredMorph, v> = [
    normalizeMorphDistribution<t>
] extends [
    InferredMorph<infer i, infer o>
] ? (In: Default<i, v>) => o : never;
type normalizeMorphDistribution<t, undistributedIn = t extends InferredMorph<infer i> ? i : never, undistributedOut extends Out = t extends InferredMorph<never, infer o> ? [
    o
] extends [
    To<infer unwrappedOut>
] ? To<unwrappedOut> : o : never> = (Extract<t, InferredMorph> extends anyOrNever ? never : Extract<t, InferredMorph> extends InferredMorph<infer i, infer o> ? [
    undistributedOut
] extends [
    o
] ? (In: undistributedIn) => undistributedOut : [
    undistributedIn
] extends [
    i
] ? (In: undistributedIn) => undistributedOut : t : never) | Exclude<t, InferredMorph> extends infer _ ? _ : never;
type defaultFor<t = unknown> = (Primitive extends t ? Primitive : t extends Primitive ? t : never) | (() => t);
type inferIntersection<l, r> = normalizeMorphDistribution<_inferIntersection<l, r, false>>;
type inferMorph<t, morph extends Morph> = morph extends type.cast<infer tMorph> ? inferPipe<t, tMorph> : inferMorphOut<morph> extends infer out ? (In: distill.In<t>) => Out<out> : never;
type inferPipe<l, r> = normalizeMorphDistribution<_inferIntersection<l, r, true>>;
type _inferIntersection<l, r, piped extends boolean> = [
    l & r
] extends [
    infer t extends anyOrNever
] ? t : l extends InferredMorph<infer lIn, infer lOut> ? r extends InferredMorph<never, infer rOut> ? piped extends true ? (In: lIn) => rOut : never : piped extends true ? (In: lIn) => To<r> : (In: _inferIntersection<lIn, r, false>) => lOut : r extends InferredMorph<infer rIn, infer rOut> ? (In: _inferIntersection<rIn, l, false>) => rOut : [
    l,
    r
] extends [
    object,
    object
] ? intersectObjects<l, r, piped> extends infer result ? result : never : l & r;
interface MorphableIntersection<piped extends boolean> extends Hkt<[
    unknown,
    unknown
]> {
    body: _inferIntersection<this[0], this[1], piped>;
}
type intersectObjects<l, r, piped extends boolean> = l extends array ? r extends array ? intersectArrays<l, r, MorphableIntersection<piped>> : l & r : r extends array ? l & r : keyof l & keyof r extends never ? show<l & r> : show<{
    [k in keyof l]: k extends keyof r ? _inferIntersection<l[k], r[k], piped> : l[k];
} & {
    [k in keyof r]: k extends keyof l ? _inferIntersection<l[k], r[k], piped> : r[k];
}>;
export { ArkErrors, Type, match, scope, type };
