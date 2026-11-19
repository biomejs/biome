interface Unsubscribable {
    unsubscribe(): void;
}
type UnsubscribeFn = () => void;
interface Subscribable<TValue, TError> {
    subscribe(observer: Partial<Observer<TValue, TError>>): Unsubscribable;
}
interface Observable<TValue, TError> extends Subscribable<TValue, TError> {
    pipe(): Observable<TValue, TError>;
    pipe<TValue1, TError1>(op1: OperatorFunction<TValue, TError, TValue1, TError1>): Observable<TValue1, TError1>;
    pipe<TValue1, TError1, TValue2, TError2>(op1: OperatorFunction<TValue, TError, TValue1, TError1>, op2: OperatorFunction<TValue1, TError1, TValue2, TError2>): Observable<TValue2, TError2>;
    pipe<TValue1, TError1, TValue2, TError2, TValue3, TError3>(op1: OperatorFunction<TValue, TError, TValue1, TError1>, op2: OperatorFunction<TValue1, TError1, TValue2, TError2>, op3: OperatorFunction<TValue2, TError2, TValue3, TError3>): Observable<TValue2, TError2>;
    pipe<TValue1, TError1, TValue2, TError2, TValue3, TError3, TValue4, TError4>(op1: OperatorFunction<TValue, TError, TValue1, TError1>, op2: OperatorFunction<TValue1, TError1, TValue2, TError2>, op3: OperatorFunction<TValue2, TError2, TValue3, TError3>, op4: OperatorFunction<TValue3, TError3, TValue4, TError4>): Observable<TValue2, TError2>;
    pipe<TValue1, TError1, TValue2, TError2, TValue3, TError3, TValue4, TError4, TValue5, TError5>(op1: OperatorFunction<TValue, TError, TValue1, TError1>, op2: OperatorFunction<TValue1, TError1, TValue2, TError2>, op3: OperatorFunction<TValue2, TError2, TValue3, TError3>, op4: OperatorFunction<TValue3, TError3, TValue4, TError4>, op5: OperatorFunction<TValue4, TError4, TValue5, TError5>): Observable<TValue2, TError2>;
}
interface Observer<TValue, TError> {
    next: (value: TValue) => void;
    error: (err: TError) => void;
    complete: () => void;
}
type TeardownLogic = Unsubscribable | UnsubscribeFn | void;
type UnaryFunction<TSource, TReturn> = (source: TSource) => TReturn;
type OperatorFunction<TValueBefore, TErrorBefore, TValueAfter, TErrorAfter> = UnaryFunction<Subscribable<TValueBefore, TErrorBefore>, Subscribable<TValueAfter, TErrorAfter>>;
type inferObservableValue<TObservable> = TObservable extends Observable<infer TValue, unknown> ? TValue : never;
declare function observable<TValue, TError = unknown>(subscribe: (observer: Observer<TValue, TError>) => TeardownLogic): Observable<TValue, TError>;
type Maybe<TType> = TType | null | undefined;
type Simplify<TType> = TType extends any[] | Date ? TType : {
    [K in keyof TType]: TType[K];
};
type MaybePromise<TType> = Promise<TType> | TType;
type Unwrap<TType> = TType extends ((...args: any[]) => infer R) ? Awaited<R> : TType;
type DistributiveOmit<TObj, TKey extends keyof any> = TObj extends any ? Omit<TObj, TKey> : never;
type WithoutIndexSignature<TObj> = {
    [K in keyof TObj as string extends K ? never : number extends K ? never : K]: TObj[K];
};
type Overwrite<TType, TWith> = TWith extends any ? TType extends object ? {
    [K in keyof WithoutIndexSignature<TType> | keyof WithoutIndexSignature<TWith>]: K extends keyof TWith ? TWith[K] : K extends keyof TType ? TType[K] : never;
} & (string extends keyof TWith ? {
    [key: string]: TWith[string];
} : number extends keyof TWith ? {
    [key: number]: TWith[number];
} : {}) : TWith : never;
type ValidateShape<TActualShape, TExpectedShape> = TActualShape extends TExpectedShape ? Exclude<keyof TActualShape, keyof TExpectedShape> extends never ? TActualShape : TExpectedShape : never;
type GetRawInputFn = () => Promise<unknown>;
declare const _errorSymbol: unique symbol;
type TypeError<TMessage extends string> = TMessage & {
    _: typeof _errorSymbol;
};
type ValueOf<TObj> = TObj[keyof TObj];
type coerceAsyncIterableToArray<TValue> = TValue extends AsyncIterable<infer $Inferred> ? $Inferred[] : TValue;
type inferAsyncIterableYield<T> = T extends AsyncIterable<infer U> ? U : T;
declare const TRPC_ERROR_CODES_BY_KEY: {
    readonly PARSE_ERROR: -32700;
    readonly BAD_REQUEST: -32600;
    readonly INTERNAL_SERVER_ERROR: -32603;
    readonly NOT_IMPLEMENTED: -32603;
    readonly BAD_GATEWAY: -32603;
    readonly SERVICE_UNAVAILABLE: -32603;
    readonly GATEWAY_TIMEOUT: -32603;
    readonly UNAUTHORIZED: -32001;
    readonly PAYMENT_REQUIRED: -32002;
    readonly FORBIDDEN: -32003;
    readonly NOT_FOUND: -32004;
    readonly METHOD_NOT_SUPPORTED: -32005;
    readonly TIMEOUT: -32008;
    readonly CONFLICT: -32009;
    readonly PRECONDITION_FAILED: -32012;
    readonly PAYLOAD_TOO_LARGE: -32013;
    readonly UNSUPPORTED_MEDIA_TYPE: -32015;
    readonly UNPROCESSABLE_CONTENT: -32022;
    readonly PRECONDITION_REQUIRED: -32028;
    readonly TOO_MANY_REQUESTS: -32029;
    readonly CLIENT_CLOSED_REQUEST: -32099;
};
type TRPC_ERROR_CODE_NUMBER = ValueOf<typeof TRPC_ERROR_CODES_BY_KEY>;
type TRPC_ERROR_CODE_KEY = keyof typeof TRPC_ERROR_CODES_BY_KEY;
declare class TRPCError extends Error {
    readonly cause?: Error;
    readonly code: "BAD_GATEWAY" | "BAD_REQUEST" | "CLIENT_CLOSED_REQUEST" | "CONFLICT" | "FORBIDDEN" | "GATEWAY_TIMEOUT" | "INTERNAL_SERVER_ERROR" | "METHOD_NOT_SUPPORTED" | "NOT_FOUND" | "NOT_IMPLEMENTED" | "PARSE_ERROR" | "PAYLOAD_TOO_LARGE" | "PAYMENT_REQUIRED" | "PRECONDITION_FAILED" | "PRECONDITION_REQUIRED" | "SERVICE_UNAVAILABLE" | "TIMEOUT" | "TOO_MANY_REQUESTS" | "UNAUTHORIZED" | "UNPROCESSABLE_CONTENT" | "UNSUPPORTED_MEDIA_TYPE";
    constructor(opts: {
        message?: string;
        code: TRPC_ERROR_CODE_KEY;
        cause?: unknown;
    });
}
interface StandardSchemaV1<Input = unknown, Output = Input> {
    readonly '~standard': StandardSchemaV1.Props<Input, Output>;
}
declare namespace StandardSchemaV1 {
    interface Props<Input = unknown, Output = Input> {
        readonly version: 1;
        readonly vendor: string;
        readonly validate: (value: unknown) => Result<Output> | Promise<Result<Output>>;
        readonly types?: Types<Input, Output> | undefined;
    }
    type Result<Output> = SuccessResult<Output> | FailureResult;
    interface SuccessResult<Output> {
        readonly value: Output;
        readonly issues?: undefined;
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
    interface Types<Input = unknown, Output = Input> {
        readonly input: Input;
        readonly output: Output;
    }
    type InferInput<Schema extends StandardSchemaV1> = NonNullable<Schema['~standard']['types']>['input'];
    type InferOutput<Schema extends StandardSchemaV1> = NonNullable<Schema['~standard']['types']>['output'];
}
type ParserZodEsque<TInput, TParsedInput> = {
    _input: TInput;
    _output: TParsedInput;
};
type ParserValibotEsque<TInput, TParsedInput> = {
    schema: {
        _types?: {
            input: TInput;
            output: TParsedInput;
        };
    };
};
type ParserArkTypeEsque<TInput, TParsedInput> = {
    inferIn: TInput;
    infer: TParsedInput;
};
type ParserStandardSchemaEsque<TInput, TParsedInput> = StandardSchemaV1<TInput, TParsedInput>;
type ParserMyZodEsque<TInput> = {
    parse: (input: any) => TInput;
};
type ParserSuperstructEsque<TInput> = {
    create: (input: unknown) => TInput;
};
type ParserCustomValidatorEsque<TInput> = (input: unknown) => Promise<TInput> | TInput;
type ParserYupEsque<TInput> = {
    validateSync: (input: unknown) => TInput;
};
type ParserScaleEsque<TInput> = {
    assert(value: unknown): asserts value is TInput;
};
type ParserWithoutInput<TInput> = ParserCustomValidatorEsque<TInput> | ParserMyZodEsque<TInput> | ParserScaleEsque<TInput> | ParserSuperstructEsque<TInput> | ParserYupEsque<TInput>;
type ParserWithInputOutput<TInput, TParsedInput> = ParserZodEsque<TInput, TParsedInput> | ParserValibotEsque<TInput, TParsedInput> | ParserArkTypeEsque<TInput, TParsedInput> | ParserStandardSchemaEsque<TInput, TParsedInput>;
type Parser = ParserWithInputOutput<any, any> | ParserWithoutInput<any>;
type inferParser<TParser extends Parser> = TParser extends ParserStandardSchemaEsque<infer $TIn, infer $TOut> ? {
    in: $TIn;
    out: $TOut;
} : TParser extends ParserWithInputOutput<infer $TIn, infer $TOut> ? {
    in: $TIn;
    out: $TOut;
} : TParser extends ParserWithoutInput<infer $InOut> ? {
    in: $InOut;
    out: $InOut;
} : never;
declare const middlewareMarker: 'middlewareMarker' & {
    __brand: 'middlewareMarker';
};
type MiddlewareMarker = typeof middlewareMarker;
interface MiddlewareResultBase {
    readonly marker: MiddlewareMarker;
}
interface MiddlewareOKResult<_TContextOverride> extends MiddlewareResultBase {
    ok: true;
    data: unknown;
}
interface MiddlewareErrorResult<_TContextOverride> extends MiddlewareResultBase {
    ok: false;
    error: TRPCError;
}
type MiddlewareResult<_TContextOverride> = MiddlewareErrorResult<_TContextOverride> | MiddlewareOKResult<_TContextOverride>;
interface MiddlewareBuilder<TContext, TMeta, TContextOverrides, TInputOut> {
    unstable_pipe<$ContextOverridesOut>(fn: MiddlewareFunction<TContext, TMeta, TContextOverrides, $ContextOverridesOut, TInputOut> | MiddlewareBuilder<Overwrite<TContext, TContextOverrides>, TMeta, $ContextOverridesOut, TInputOut>): MiddlewareBuilder<TContext, TMeta, Overwrite<TContextOverrides, $ContextOverridesOut>, TInputOut>;
    _middlewares: MiddlewareFunction<TContext, TMeta, TContextOverrides, object, TInputOut>[];
}
type MiddlewareFunction<TContext, TMeta, TContextOverridesIn, $ContextOverridesOut, TInputOut> = {
    (opts: {
        ctx: Simplify<Overwrite<TContext, TContextOverridesIn>>;
        type: ProcedureType;
        path: string;
        input: TInputOut;
        getRawInput: GetRawInputFn;
        meta: TMeta | undefined;
        signal: AbortSignal | undefined;
        batchIndex: number;
        next: {
            (): Promise<MiddlewareResult<TContextOverridesIn>>;
            <$ContextOverride>(opts: {
                ctx?: $ContextOverride;
                input?: unknown;
            }): Promise<MiddlewareResult<$ContextOverride>>;
            (opts: {
                getRawInput: GetRawInputFn;
            }): Promise<MiddlewareResult<TContextOverridesIn>>;
        };
    }): Promise<MiddlewareResult<$ContextOverridesOut>>;
    _type?: string | undefined;
};
type AnyMiddlewareFunction = MiddlewareFunction<any, any, any, any, any>;
declare const trackedSymbol: unique symbol;
type TrackedId = string & {
    __brand: 'TrackedId';
};
type TrackedEnvelope<TData> = [
    TrackedId,
    TData,
    typeof trackedSymbol
];
interface TrackedData<TData> {
    id: string;
    data: TData;
}
declare function tracked<TData>(id: string, data: TData): TrackedEnvelope<TData>;
type inferTrackedOutput<TData> = TData extends TrackedEnvelope<infer $Data> ? TrackedData<$Data> : TData;
type UnsetMarker = 'unsetMarker' & {
    __brand: 'unsetMarker';
};
type IntersectIfDefined<TType, TWith> = TType extends UnsetMarker ? TWith : TWith extends UnsetMarker ? TType : Simplify<TType & TWith>;
type DefaultValue<TValue, TFallback> = TValue extends UnsetMarker ? TFallback : TValue;
type inferAsyncIterable<TOutput> = TOutput extends AsyncIterable<infer $Yield, infer $Return, infer $Next> ? {
    yield: $Yield;
    return: $Return;
    next: $Next;
} : never;
type inferSubscriptionOutput$1<TOutput> = TOutput extends AsyncIterable<any> ? AsyncIterable<inferTrackedOutput<inferAsyncIterable<TOutput>['yield']>, inferAsyncIterable<TOutput>['return'], inferAsyncIterable<TOutput>['next']> : TypeError<'Subscription output could not be inferred'>;
type CallerOverride<TContext> = (opts: {
    args: unknown[];
    invoke: (opts: ProcedureCallOptions<TContext>) => Promise<unknown>;
    _def: AnyProcedure['_def'];
}) => Promise<unknown>;
type ProcedureBuilderDef<TMeta> = {
    procedure: true;
    inputs: Parser[];
    output?: Parser;
    meta?: TMeta;
    resolver?: ProcedureBuilderResolver;
    middlewares: AnyMiddlewareFunction[];
    mutation?: boolean;
    query?: boolean;
    subscription?: boolean;
    type?: ProcedureType;
    caller?: CallerOverride<unknown>;
};
interface ProcedureResolverOptions<TContext, _TMeta, TContextOverridesIn, TInputOut> {
    ctx: Simplify<Overwrite<TContext, TContextOverridesIn>>;
    input: TInputOut extends UnsetMarker ? undefined : TInputOut;
    signal: AbortSignal | undefined;
    path: string;
    batchIndex?: number;
}
type ProcedureResolver<TContext, TMeta, TContextOverrides, TInputOut, TOutputParserIn, $Output> = (opts: ProcedureResolverOptions<TContext, TMeta, TContextOverrides, TInputOut>) => MaybePromise<DefaultValue<TOutputParserIn, $Output>>;
type AnyProcedureBuilder = ProcedureBuilder<any, any, any, any, any, any, any, any>;
type inferProcedureBuilderResolverOptions<TProcedureBuilder extends AnyProcedureBuilder> = TProcedureBuilder extends ProcedureBuilder<infer TContext, infer TMeta, infer TContextOverrides, infer _TInputIn, infer TInputOut, infer _TOutputIn, infer _TOutputOut, infer _TCaller> ? ProcedureResolverOptions<TContext, TMeta, TContextOverrides, TInputOut extends UnsetMarker ? unknown : TInputOut extends object ? Simplify<TInputOut & {
    [keyAddedByInputCallFurtherDown: string]: unknown;
}> : TInputOut> : never;
interface ProcedureBuilder<TContext, TMeta, TContextOverrides, TInputIn, TInputOut, TOutputIn, TOutputOut, TCaller extends boolean> {
    input<$Parser extends Parser>(schema: TInputOut extends UnsetMarker ? $Parser : inferParser<$Parser>['out'] extends Record<string, unknown> | undefined ? TInputOut extends Record<string, unknown> | undefined ? undefined extends inferParser<$Parser>['out'] ? undefined extends TInputOut ? $Parser : TypeError<'Cannot chain an optional parser to a required parser'> : $Parser : TypeError<'All input parsers did not resolve to an object'> : TypeError<'All input parsers did not resolve to an object'>): ProcedureBuilder<TContext, TMeta, TContextOverrides, IntersectIfDefined<TInputIn, inferParser<$Parser>['in']>, IntersectIfDefined<TInputOut, inferParser<$Parser>['out']>, TOutputIn, TOutputOut, TCaller>;
    output<$Parser extends Parser>(schema: $Parser): ProcedureBuilder<TContext, TMeta, TContextOverrides, TInputIn, TInputOut, IntersectIfDefined<TOutputIn, inferParser<$Parser>['in']>, IntersectIfDefined<TOutputOut, inferParser<$Parser>['out']>, TCaller>;
    meta(meta: TMeta): ProcedureBuilder<TContext, TMeta, TContextOverrides, TInputIn, TInputOut, TOutputIn, TOutputOut, TCaller>;
    use<$ContextOverridesOut>(fn: MiddlewareBuilder<Overwrite<TContext, TContextOverrides>, TMeta, $ContextOverridesOut, TInputOut> | MiddlewareFunction<TContext, TMeta, TContextOverrides, $ContextOverridesOut, TInputOut>): ProcedureBuilder<TContext, TMeta, Overwrite<TContextOverrides, $ContextOverridesOut>, TInputIn, TInputOut, TOutputIn, TOutputOut, TCaller>;
    unstable_concat<$Context, $Meta, $ContextOverrides, $InputIn, $InputOut, $OutputIn, $OutputOut>(builder: Overwrite<TContext, TContextOverrides> extends $Context ? TMeta extends $Meta ? ProcedureBuilder<$Context, $Meta, $ContextOverrides, $InputIn, $InputOut, $OutputIn, $OutputOut, TCaller> : TypeError<'Meta mismatch'> : TypeError<'Context mismatch'>): ProcedureBuilder<TContext, TMeta, Overwrite<TContextOverrides, $ContextOverrides>, IntersectIfDefined<TInputIn, $InputIn>, IntersectIfDefined<TInputOut, $InputOut>, IntersectIfDefined<TOutputIn, $OutputIn>, IntersectIfDefined<TOutputOut, $OutputOut>, TCaller>;
    concat<$Context, $Meta, $ContextOverrides, $InputIn, $InputOut, $OutputIn, $OutputOut>(builder: Overwrite<TContext, TContextOverrides> extends $Context ? TMeta extends $Meta ? ProcedureBuilder<$Context, $Meta, $ContextOverrides, $InputIn, $InputOut, $OutputIn, $OutputOut, TCaller> : TypeError<'Meta mismatch'> : TypeError<'Context mismatch'>): ProcedureBuilder<TContext, TMeta, Overwrite<TContextOverrides, $ContextOverrides>, IntersectIfDefined<TInputIn, $InputIn>, IntersectIfDefined<TInputOut, $InputOut>, IntersectIfDefined<TOutputIn, $OutputIn>, IntersectIfDefined<TOutputOut, $OutputOut>, TCaller>;
    query<$Output>(resolver: ProcedureResolver<TContext, TMeta, TContextOverrides, TInputOut, TOutputIn, $Output>): TCaller extends true ? (input: DefaultValue<TInputIn, void>) => Promise<DefaultValue<TOutputOut, $Output>> : QueryProcedure<{
        input: DefaultValue<TInputIn, void>;
        output: DefaultValue<TOutputOut, $Output>;
        meta: TMeta;
    }>;
    mutation<$Output>(resolver: ProcedureResolver<TContext, TMeta, TContextOverrides, TInputOut, TOutputIn, $Output>): TCaller extends true ? (input: DefaultValue<TInputIn, void>) => Promise<DefaultValue<TOutputOut, $Output>> : MutationProcedure<{
        input: DefaultValue<TInputIn, void>;
        output: DefaultValue<TOutputOut, $Output>;
        meta: TMeta;
    }>;
    subscription<$Output extends AsyncIterable<any, void, any>>(resolver: ProcedureResolver<TContext, TMeta, TContextOverrides, TInputOut, TOutputIn, $Output>): TCaller extends true ? TypeError<'Not implemented'> : SubscriptionProcedure<{
        input: DefaultValue<TInputIn, void>;
        output: inferSubscriptionOutput$1<DefaultValue<TOutputOut, $Output>>;
        meta: TMeta;
    }>;
    subscription<$Output extends Observable<any, any>>(resolver: ProcedureResolver<TContext, TMeta, TContextOverrides, TInputOut, TOutputIn, $Output>): TCaller extends true ? TypeError<'Not implemented'> : LegacyObservableSubscriptionProcedure<{
        input: DefaultValue<TInputIn, void>;
        output: inferObservableValue<DefaultValue<TOutputOut, $Output>>;
        meta: TMeta;
    }>;
    experimental_caller(caller: CallerOverride<TContext>): ProcedureBuilder<TContext, TMeta, TContextOverrides, TInputIn, TInputOut, TOutputIn, TOutputOut, true>;
    _def: ProcedureBuilderDef<TMeta>;
}
type ProcedureBuilderResolver = (opts: ProcedureResolverOptions<any, any, any, any>) => Promise<unknown>;
interface ProcedureCallOptions<TContext> {
    ctx: TContext;
    getRawInput: GetRawInputFn;
    input?: unknown;
    path: string;
    type: ProcedureType;
    signal: AbortSignal | undefined;
    batchIndex: number;
}
declare const procedureTypes: readonly [
    'query',
    'mutation',
    'subscription'
];
type ProcedureType = (typeof procedureTypes)[number];
interface BuiltProcedureDef {
    meta: unknown;
    input: unknown;
    output: unknown;
}
interface Procedure<TType extends ProcedureType, TDef extends BuiltProcedureDef> {
    _def: {
        $types: {
            input: TDef['input'];
            output: TDef['output'];
        };
        procedure: true;
        type: TType;
        meta: unknown;
        experimental_caller: boolean;
        inputs: Parser[];
    };
    meta: TDef['meta'];
    (opts: ProcedureCallOptions<unknown>): Promise<TDef['output']>;
}
interface QueryProcedure<TDef extends BuiltProcedureDef> extends Procedure<'query', TDef> {
}
interface MutationProcedure<TDef extends BuiltProcedureDef> extends Procedure<'mutation', TDef> {
}
interface SubscriptionProcedure<TDef extends BuiltProcedureDef> extends Procedure<'subscription', TDef> {
}
interface LegacyObservableSubscriptionProcedure<TDef extends BuiltProcedureDef> extends SubscriptionProcedure<TDef> {
    _observable: true;
}
type AnyQueryProcedure = QueryProcedure<any>;
type AnyMutationProcedure = MutationProcedure<any>;
type AnySubscriptionProcedure = SubscriptionProcedure<any> | LegacyObservableSubscriptionProcedure<any>;
type AnyProcedure = AnyQueryProcedure | AnyMutationProcedure | AnySubscriptionProcedure;
type inferProcedureInput<TProcedure extends AnyProcedure> = undefined extends inferProcedureParams<TProcedure>['$types']['input'] ? void | inferProcedureParams<TProcedure>['$types']['input'] : inferProcedureParams<TProcedure>['$types']['input'];
type inferProcedureParams<TProcedure> = TProcedure extends AnyProcedure ? TProcedure['_def'] : never;
type inferProcedureOutput<TProcedure> = inferProcedureParams<TProcedure>['$types']['output'];
interface ErrorHandlerOptions<TContext> {
    error: TRPCError;
    type: ProcedureType | 'unknown';
    path: string | undefined;
    input: unknown;
    ctx: TContext | undefined;
}
interface TRPCErrorShape<TData extends object = object> {
    code: TRPC_ERROR_CODE_NUMBER;
    message: string;
    data: TData;
}
declare namespace JSONRPC2 {
    type RequestId = number | string | null;
    interface BaseEnvelope {
        id?: RequestId;
        jsonrpc?: '2.0';
    }
    interface BaseRequest<TMethod extends string = string> extends BaseEnvelope {
        method: TMethod;
    }
    interface Request<TMethod extends string = string, TParams = unknown> extends BaseRequest<TMethod> {
        params: TParams;
    }
    interface ResultResponse<TResult = unknown> extends BaseEnvelope {
        result: TResult;
    }
    interface ErrorResponse<TError extends TRPCErrorShape = TRPCErrorShape> extends BaseEnvelope {
        error: TError;
    }
}
interface TRPCResult<TData = unknown> {
    data: TData;
    type?: 'data';
    id?: string;
}
interface TRPCSuccessResponse<TData> extends JSONRPC2.ResultResponse<TRPCResult<TData>> {
}
interface TRPCErrorResponse<TError extends TRPCErrorShape = TRPCErrorShape> extends JSONRPC2.ErrorResponse<TError> {
}
interface TRPCResultMessage<TData> extends JSONRPC2.ResultResponse<{
    type: 'started';
    data?: never;
} | {
    type: 'stopped';
    data?: never;
} | TRPCResult<TData>> {
}
interface DataTransformer {
    serialize(object: any): any;
    deserialize(object: any): any;
}
interface InputDataTransformer extends DataTransformer {
    serialize(object: any): any;
    deserialize(object: any): any;
}
interface OutputDataTransformer extends DataTransformer {
    serialize(object: any): any;
    deserialize(object: any): any;
}
interface CombinedDataTransformer {
    input: InputDataTransformer;
    output: OutputDataTransformer;
}
type DataTransformerOptions = CombinedDataTransformer | DataTransformer;
type ErrorFormatter<TContext, TShape extends TRPCErrorShape> = (opts: {
    error: TRPCError;
    type: ProcedureType | 'unknown';
    path: string | undefined;
    input: unknown;
    ctx: TContext | undefined;
    shape: DefaultErrorShape;
}) => TShape;
type DefaultErrorData = {
    code: TRPC_ERROR_CODE_KEY;
    httpStatus: number;
    path?: string;
    stack?: string;
};
interface DefaultErrorShape extends TRPCErrorShape<DefaultErrorData> {
    message: string;
    code: TRPC_ERROR_CODE_NUMBER;
}
type Serialize$2 = (value: any) => any;
type PathArray = readonly (string | number)[];
type ProducerOnError = (opts: {
    error: unknown;
    path: PathArray;
}) => void;
interface JSONLProducerOptions {
    serialize?: Serialize$2;
    data: Record<string, unknown> | unknown[];
    onError?: ProducerOnError;
    formatError?: (opts: {
        error: unknown;
        path: PathArray;
    }) => unknown;
    maxDepth?: number;
    pingMs?: number;
}
type Serialize$1 = (value: any) => any;
interface SSEPingOptions {
    enabled: boolean;
    intervalMs?: number;
}
interface SSEClientOptions {
    reconnectAfterInactivityMs?: number;
}
interface SSEStreamProducerOptions<TValue = unknown> {
    serialize?: Serialize$1;
    data: AsyncIterable<TValue>;
    maxDepth?: number;
    ping?: SSEPingOptions;
    maxDurationMs?: number;
    emitAndEndImmediately?: boolean;
    formatError?: (opts: {
        error: unknown;
    }) => unknown;
    client?: SSEClientOptions;
}
interface RootTypes {
    ctx: object;
    meta: object;
    errorShape: DefaultErrorShape;
    transformer: boolean;
}
interface RootConfig<TTypes extends RootTypes> {
    $types: TTypes;
    transformer: CombinedDataTransformer;
    errorFormatter: ErrorFormatter<TTypes['ctx'], TTypes['errorShape']>;
    allowOutsideOfServer: boolean;
    isServer: boolean;
    isDev: boolean;
    defaultMeta?: TTypes['meta'] extends object ? TTypes['meta'] : never;
    sse?: {
        enabled?: boolean;
    } & Pick<SSEStreamProducerOptions, 'ping' | 'emitAndEndImmediately' | 'maxDurationMs' | 'client'>;
    jsonl?: Pick<JSONLProducerOptions, 'pingMs'>;
    experimental?: {};
}
type CreateRootTypes<TGenerics extends RootTypes> = TGenerics;
type AnyRootTypes = CreateRootTypes<{
    ctx: any;
    meta: any;
    errorShape: any;
    transformer: any;
}>;
interface RouterRecord {
    [key: string]: AnyProcedure | RouterRecord;
}
type DecorateProcedure<TProcedure extends AnyProcedure> = (input: inferProcedureInput<TProcedure>) => Promise<TProcedure['_def']['type'] extends 'subscription' ? TProcedure extends LegacyObservableSubscriptionProcedure<any> ? Observable<inferProcedureOutput<TProcedure>, TRPCError> : inferProcedureOutput<TProcedure> : inferProcedureOutput<TProcedure>>;
type DecorateRouterRecord<TRecord extends RouterRecord> = {
    [TKey in keyof TRecord]: TRecord[TKey] extends (infer $Value) ? $Value extends AnyProcedure ? DecorateProcedure<$Value> : $Value extends RouterRecord ? DecorateRouterRecord<$Value> : never : never;
};
type RouterCallerErrorHandler<TContext> = (opts: ErrorHandlerOptions<TContext>) => void;
type RouterCaller<TRoot extends AnyRootTypes, TRecord extends RouterRecord> = (ctx: TRoot['ctx'] | (() => MaybePromise<TRoot['ctx']>), options?: {
    onError?: RouterCallerErrorHandler<TRoot['ctx']>;
    signal?: AbortSignal;
}) => DecorateRouterRecord<TRecord>;
type Lazy<TAny> = (() => Promise<TAny>) & {};
type LazyLoader<TAny> = {
    load: () => Promise<void>;
    ref: Lazy<TAny>;
};
interface RouterDef<TRoot extends AnyRootTypes, TRecord extends RouterRecord> {
    _config: RootConfig<TRoot>;
    router: true;
    procedure?: never;
    procedures: TRecord;
    record: TRecord;
    lazy: Record<string, LazyLoader<AnyRouter>>;
}
interface Router<TRoot extends AnyRootTypes, TRecord extends RouterRecord> {
    _def: RouterDef<TRoot, TRecord>;
    createCaller: RouterCaller<TRoot, TRecord>;
}
type BuiltRouter<TRoot extends AnyRootTypes, TRecord extends RouterRecord> = Router<TRoot, TRecord> & TRecord;
interface RouterBuilder<TRoot extends AnyRootTypes> {
    <TIn extends CreateRouterOptions>(_: TIn): BuiltRouter<TRoot, DecorateCreateRouterOptions<TIn>>;
}
type AnyRouter = Router<any, any>;
type inferRouterRootTypes<TRouter extends AnyRouter> = TRouter['_def']['_config']['$types'];
type inferRouterContext<TRouter extends AnyRouter> = inferRouterRootTypes<TRouter>['ctx'];
type CreateRouterOptions = {
    [key: string]: AnyProcedure | AnyRouter | CreateRouterOptions | Lazy<AnyRouter>;
};
type DecorateCreateRouterOptions<TRouterOptions extends CreateRouterOptions> = {
    [K in keyof TRouterOptions]: TRouterOptions[K] extends (infer $Value) ? $Value extends AnyProcedure ? $Value : $Value extends Router<any, infer TRecord> ? TRecord : $Value extends Lazy<Router<any, infer TRecord>> ? TRecord : $Value extends CreateRouterOptions ? DecorateCreateRouterOptions<$Value> : never : never;
};
interface RouterCallerFactory<TRoot extends AnyRootTypes> {
    <TRecord extends RouterRecord>(router: Pick<Router<TRoot, TRecord>, '_def'>): RouterCaller<TRoot, TRecord>;
}
type MergeRouters<TRouters extends AnyRouter[], TRoot extends AnyRootTypes = TRouters[0]['_def']['_config']['$types'], TRecord extends RouterRecord = {}> = TRouters extends [
    infer Head extends AnyRouter,
    ...infer Tail extends AnyRouter[]
] ? MergeRouters<Tail, TRoot, Head['_def']['record'] & TRecord> : BuiltRouter<TRoot, TRecord>;
type AnyClientTypes = Pick<AnyRootTypes, 'errorShape' | 'transformer'>;
type InitLike = {
    _config: {
        $types: AnyClientTypes;
    };
};
type RouterLike = {
    _def: InitLike;
};
type RootConfigLike = {
    $types: AnyClientTypes;
};
type InferrableClientTypes = RouterLike | InitLike | RootConfigLike | AnyClientTypes;
type PickTypes<T extends AnyClientTypes> = {
    transformer: T['transformer'];
    errorShape: T['errorShape'];
};
type inferClientTypes<TInferrable extends InferrableClientTypes> = TInferrable extends AnyClientTypes ? PickTypes<TInferrable> : TInferrable extends RootConfigLike ? PickTypes<TInferrable['$types']> : TInferrable extends InitLike ? PickTypes<TInferrable['_config']['$types']> : TInferrable extends RouterLike ? PickTypes<TInferrable['_def']['_config']['$types']> : never;
type JsonPrimitive = boolean | number | string | null;
type JsonArray = JsonValue[] | readonly JsonValue[];
type JsonObject = {
    readonly [key: string | number]: JsonValue;
    [key: symbol]: never;
};
type JsonValue = JsonPrimitive | JsonObject | JsonArray;
type IsJson<T> = T extends JsonValue ? true : false;
type NonJsonPrimitive = Function | symbol | undefined;
type IsAny<T> = 0 extends T & 1 ? true : false;
type JsonReturnable = JsonPrimitive | undefined;
type IsRecord<T extends object> = keyof WithoutIndexSignature<T> extends never ? true : false;
type Serialize<T> = IsAny<T> extends true ? any : unknown extends T ? unknown : IsJson<T> extends true ? T : T extends AsyncIterable<infer $T, infer $Return, infer $Next> ? AsyncIterable<Serialize<$T>, Serialize<$Return>, Serialize<$Next>> : T extends PromiseLike<infer $T> ? Promise<Serialize<$T>> : T extends JsonReturnable ? T : T extends Map<any, any> | Set<any> ? object : T extends NonJsonPrimitive ? never : T extends {
    toJSON(): infer U;
} ? U : T extends [
] ? [
] : T extends [
    unknown,
    ...unknown[]
] ? SerializeTuple<T> : T extends readonly (infer U)[] ? (U extends NonJsonPrimitive ? null : Serialize<U>)[] : T extends object ? IsRecord<T> extends true ? Record<keyof T, Serialize<T[keyof T]>> : Simplify<SerializeObject<UndefinedToOptional<T>>> : never;
type SerializeTuple<T extends [
    unknown,
    ...unknown[]
]> = {
    [K in keyof T]: T[K] extends NonJsonPrimitive ? null : Serialize<T[K]>;
};
type SerializeObjectKey<T extends Record<any, any>, K> = K extends symbol ? never : IsAny<T[K]> extends true ? K : unknown extends T[K] ? K : T[K] extends NonJsonPrimitive ? never : K;
type SerializeObject<T extends object> = {
    [K in keyof T as SerializeObjectKey<T, K>]: Serialize<T[K]>;
};
type FilterDefinedKeys<T extends object> = Exclude<{
    [K in keyof T]: undefined extends T[K] ? never : K;
}[keyof T], undefined>;
type ExactOptionalPropertyTypes = {
    a?: 0 | undefined;
} extends {
    a?: 0;
} ? false : true;
type HasIndexSignature<T extends object> = string extends keyof T ? true : false;
type HandleIndexSignature<T extends object> = {
    [K in keyof Omit<T, keyof WithoutIndexSignature<T>>]: Exclude<T[K], undefined>;
};
type HandleUndefined<T extends object> = {
    [K in keyof Omit<T, FilterDefinedKeys<T>>]?: Exclude<T[K], undefined>;
};
type UndefinedToOptional<T extends object> = Pick<WithoutIndexSignature<T>, FilterDefinedKeys<WithoutIndexSignature<T>>> & (ExactOptionalPropertyTypes extends true ? HandleIndexSignature<T> & HandleUndefined<WithoutIndexSignature<T>> : HasIndexSignature<T> extends true ? HandleIndexSignature<T> : HandleUndefined<T>);
type inferTransformedProcedureOutput<TInferrable extends InferrableClientTypes, TProcedure extends AnyProcedure> = inferClientTypes<TInferrable>['transformer'] extends false ? Serialize<inferProcedureOutput<TProcedure>> : inferProcedureOutput<TProcedure>;
type GetInferenceHelpers<TType extends 'input' | 'output', TRoot extends AnyClientTypes, TRecord extends RouterRecord> = {
    [TKey in keyof TRecord]: TRecord[TKey] extends (infer $Value) ? $Value extends AnyProcedure ? TType extends 'input' ? inferProcedureInput<$Value> : inferTransformedProcedureOutput<TRoot, $Value> : $Value extends RouterRecord ? GetInferenceHelpers<TType, TRoot, $Value> : never : never;
};
type inferRouterInputs<TRouter extends AnyRouter> = GetInferenceHelpers<'input', TRouter['_def']['_config']['$types'], TRouter['_def']['record']>;
type inferRouterOutputs<TRouter extends AnyRouter> = GetInferenceHelpers<'output', TRouter['_def']['_config']['$types'], TRouter['_def']['record']>;
type inferErrorFormatterShape<TType> = TType extends ErrorFormatter<any, infer TShape> ? TShape : DefaultErrorShape;
interface RuntimeConfigOptions<TContext extends object, TMeta extends object> extends Partial<Omit<RootConfig<{
    ctx: TContext;
    meta: TMeta;
    errorShape: any;
    transformer: any;
}>, '$types' | 'transformer'>> {
    transformer?: DataTransformerOptions;
}
type ContextCallback = (...args: any[]) => object | Promise<object>;
interface TRPCRootObject<TContext extends object, TMeta extends object, TOptions extends RuntimeConfigOptions<TContext, TMeta>, $Root extends AnyRootTypes = {
    ctx: TContext;
    meta: TMeta;
    errorShape: undefined extends TOptions['errorFormatter'] ? DefaultErrorShape : inferErrorFormatterShape<TOptions['errorFormatter']>;
    transformer: undefined extends TOptions['transformer'] ? false : true;
}> {
    _config: RootConfig<$Root>;
    procedure: ProcedureBuilder<TContext, TMeta, object, UnsetMarker, UnsetMarker, UnsetMarker, UnsetMarker, false>;
    middleware: <$ContextOverrides>(fn: MiddlewareFunction<TContext, TMeta, object, $ContextOverrides, unknown>) => MiddlewareBuilder<TContext, TMeta, $ContextOverrides, unknown>;
    router: RouterBuilder<$Root>;
    mergeRouters: <TRouters extends AnyRouter[]>(...routerList: [
        ...TRouters
    ]) => MergeRouters<TRouters>;
    createCallerFactory: RouterCallerFactory<$Root>;
}
declare class TRPCBuilder<TContext extends object, TMeta extends object> {
    context<TNewContext extends object | ContextCallback>(): TRPCBuilder<TNewContext extends ContextCallback ? Unwrap<TNewContext> : TNewContext, TMeta>;
    meta<TNewMeta extends object>(): TRPCBuilder<TContext, TNewMeta>;
    create<TOptions extends RuntimeConfigOptions<TContext, TMeta>>(opts?: ValidateShape<TOptions, RuntimeConfigOptions<TContext, TMeta>>): TRPCRootObject<TContext, TMeta, TOptions>;
}
declare const initTRPC: TRPCBuilder<object, object>;
export { TRPCError, initTRPC, observable, tracked };
export type { AnyClientTypes, AnyProcedure, AnyRouter, AnyProcedure as AnyTRPCProcedure, AnyRootTypes as AnyTRPCRootTypes, AnyRouter as AnyTRPCRouter, DataTransformerOptions, DefaultErrorShape, DistributiveOmit, InferrableClientTypes, Maybe, MaybePromise, Observable, ProcedureType, RouterRecord, TRPCErrorResponse, ProcedureType as TRPCProcedureType, TRPCResultMessage, RouterRecord as TRPCRouterRecord, TRPCSuccessResponse, TypeError, Unsubscribable, coerceAsyncIterableToArray, inferAsyncIterableYield, inferClientTypes, inferProcedureBuilderResolverOptions, inferProcedureInput, inferRouterContext, inferRouterInputs, inferRouterOutputs, inferTransformedProcedureOutput };
