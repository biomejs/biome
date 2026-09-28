import { Observable, Unsubscribable } from "./trpc-server";
import { InferrableClientTypes, Maybe, TRPCResultMessage, TRPCSuccessResponse, DefaultErrorShape, inferClientTypes, TRPCErrorResponse, AnyClientTypes, DataTransformerOptions, TypeError, AnyRouter as AnyRouter$1, RouterRecord, AnyProcedure, ProcedureType, inferAsyncIterableYield, inferProcedureInput, inferTransformedProcedureOutput } from "./trpc-server";
import { AnyRouter } from "./trpc-server";
interface ConnectionStateBase<TError> {
    type: 'state';
    data?: never;
    error: TError | null;
}
interface ConnectionIdleState extends ConnectionStateBase<null> {
    state: 'idle';
}
interface ConnectionConnectingState<TError> extends ConnectionStateBase<TError | null> {
    state: 'connecting';
}
interface ConnectionPendingState extends ConnectionStateBase<null> {
    state: 'pending';
}
type TRPCConnectionState<TError> = ConnectionIdleState | ConnectionConnectingState<TError> | ConnectionPendingState;
type FetchEsque = (input: RequestInfo | URL | string, init?: RequestInit | RequestInitEsque) => Promise<ResponseEsque>;
interface RequestInitEsque {
    body?: FormData | string | null | Uint8Array<ArrayBuffer> | Blob | File;
    headers?: [
        string,
        string
    ][] | Record<string, string>;
    method?: string;
    signal?: AbortSignal | undefined;
}
type WebReadableStreamEsque = {
    getReader: () => ReadableStreamDefaultReader<Uint8Array>;
};
type NodeJSReadableStreamEsque = {
    on(eventName: string | symbol, listener: (...args: any[]) => void): NodeJSReadableStreamEsque;
};
interface ResponseEsque {
    readonly ok: boolean;
    readonly body?: NodeJSReadableStreamEsque | WebReadableStreamEsque | null;
    json(): Promise<unknown>;
}
type NonEmptyArray<TItem> = [
    TItem,
    ...TItem[]
];
type ClientContext = Record<string, unknown>;
interface TRPCProcedureOptions {
    context?: ClientContext;
    signal?: AbortSignal;
}
type inferErrorShape<TInferrable extends InferrableClientTypes> = inferClientTypes<TInferrable>['errorShape'];
interface TRPCClientErrorBase<TShape extends DefaultErrorShape> {
    readonly message: string;
    readonly shape: Maybe<TShape>;
    readonly data: Maybe<TShape['data']>;
}
type TRPCClientErrorLike<TInferrable extends InferrableClientTypes> = TRPCClientErrorBase<inferErrorShape<TInferrable>>;
declare function isTRPCClientError<TInferrable extends InferrableClientTypes>(cause: unknown): cause is TRPCClientError<TInferrable>;
declare class TRPCClientError<TRouterOrProcedure extends InferrableClientTypes> extends Error implements TRPCClientErrorBase<inferErrorShape<TRouterOrProcedure>> {
    readonly cause: Error | undefined;
    readonly shape: Maybe<inferErrorShape<TRouterOrProcedure>>;
    readonly data: Maybe<inferErrorShape<TRouterOrProcedure>['data']>;
    meta: Record<string, unknown> | undefined;
    constructor(message: string, opts?: {
        result?: Maybe<TRPCErrorResponse<inferErrorShape<TRouterOrProcedure>>>;
        cause?: Error;
        meta?: Record<string, unknown>;
    });
    static from<TRouterOrProcedure extends InferrableClientTypes>(_cause: Error | TRPCErrorResponse<any> | object, opts?: {
        meta?: Record<string, unknown>;
        cause?: Error;
    }): TRPCClientError<TRouterOrProcedure>;
}
interface OperationContext extends Record<string, unknown> {
}
type Operation<TInput = unknown> = {
    id: number;
    type: 'mutation' | 'query' | 'subscription';
    input: TInput;
    path: string;
    context: OperationContext;
    signal: Maybe<AbortSignal>;
};
interface HeadersInitEsque {
    [Symbol.iterator](): IterableIterator<[
        string,
        string
    ]>;
}
type HTTPHeaders = HeadersInitEsque | Record<string, string[] | string | undefined>;
interface TRPCClientRuntime {
}
interface OperationResultEnvelope<TOutput, TError> {
    result: TRPCResultMessage<TOutput>['result'] | TRPCSuccessResponse<TOutput>['result'] | TRPCConnectionState<TError>;
    context?: OperationContext;
}
type OperationResultObservable<TInferrable extends InferrableClientTypes, TOutput> = Observable<OperationResultEnvelope<TOutput, TRPCClientError<TInferrable>>, TRPCClientError<TInferrable>>;
type OperationLink<TInferrable extends InferrableClientTypes, TInput = unknown, TOutput = unknown> = (opts: {
    op: Operation<TInput>;
    next: (op: Operation<TInput>) => OperationResultObservable<TInferrable, TOutput>;
}) => OperationResultObservable<TInferrable, TOutput>;
type TRPCLink<TInferrable extends InferrableClientTypes> = (opts: TRPCClientRuntime) => OperationLink<TInferrable>;
type TransformerOptionYes = {
    transformer: DataTransformerOptions;
};
type TransformerOptionNo = {
    transformer?: TypeError<'You must define a transformer on your your `initTRPC`-object first'>;
};
type TransformerOptions<TRoot extends Pick<AnyClientTypes, 'transformer'>> = TRoot['transformer'] extends true ? TransformerOptionYes : TransformerOptionNo;
type HTTPLinkBaseOptions<TRoot extends Pick<AnyClientTypes, 'transformer'>> = {
    url: string | URL;
    fetch?: FetchEsque;
    methodOverride?: 'POST';
} & TransformerOptions<TRoot>;
type HTTPBatchLinkOptions<TRoot extends AnyClientTypes> = HTTPLinkBaseOptions<TRoot> & {
    maxURLLength?: number;
    headers?: HTTPHeaders | ((opts: {
        opList: NonEmptyArray<Operation>;
    }) => HTTPHeaders | Promise<HTTPHeaders>);
    maxItems?: number;
};
declare function httpBatchLink<TRouter extends AnyRouter>(opts: HTTPBatchLinkOptions<TRouter['_def']['_config']['$types']>): TRPCLink<TRouter>;
type HTTPLinkOptions<TRoot extends AnyClientTypes> = HTTPLinkBaseOptions<TRoot> & {
    headers?: HTTPHeaders | ((opts: {
        op: Operation;
    }) => HTTPHeaders | Promise<HTTPHeaders>);
};
declare function httpLink<TRouter extends AnyRouter$1 = AnyRouter$1>(opts: HTTPLinkOptions<TRouter['_def']['_config']['$types']>): TRPCLink<TRouter>;
type ConsoleEsque = {
    log: (...args: any[]) => void;
    error: (...args: any[]) => void;
};
type EnableFnOptions<TRouter extends InferrableClientTypes> = {
    direction: 'down';
    result: OperationResultEnvelope<unknown, TRPCClientError<TRouter>> | TRPCClientError<TRouter>;
} | (Operation & {
    direction: 'up';
});
type EnabledFn<TRouter extends AnyRouter$1> = (opts: EnableFnOptions<TRouter>) => boolean;
type LoggerLinkFnOptions<TRouter extends AnyRouter$1> = Operation & ({
    direction: 'down';
    result: OperationResultEnvelope<unknown, TRPCClientError<TRouter>> | TRPCClientError<TRouter>;
    elapsedMs: number;
} | {
    direction: 'up';
});
type LoggerLinkFn<TRouter extends AnyRouter$1> = (opts: LoggerLinkFnOptions<TRouter>) => void;
type ColorMode = 'ansi' | 'css' | 'none';
interface LoggerLinkOptions<TRouter extends AnyRouter$1> {
    logger?: LoggerLinkFn<TRouter>;
    enabled?: EnabledFn<TRouter>;
    console?: ConsoleEsque;
    colorMode?: ColorMode;
    withContext?: boolean;
}
declare function loggerLink<TRouter extends AnyRouter$1 = AnyRouter$1>(opts?: LoggerLinkOptions<TRouter>): TRPCLink<TRouter>;
declare function splitLink<TRouter extends AnyRouter$1 = AnyRouter$1>(opts: {
    condition: (op: Operation) => boolean;
    true: TRPCLink<TRouter> | TRPCLink<TRouter>[];
    false: TRPCLink<TRouter> | TRPCLink<TRouter>[];
}): TRPCLink<TRouter>;
interface TRPCRequestOptions {
    context?: OperationContext;
    signal?: AbortSignal;
}
interface TRPCSubscriptionObserver<TValue, TError> {
    onStarted: (opts: {
        context: OperationContext | undefined;
    }) => void;
    onData: (value: inferAsyncIterableYield<TValue>) => void;
    onError: (err: TError) => void;
    onStopped: () => void;
    onComplete: () => void;
    onConnectionStateChange: (state: TRPCConnectionState<TError>) => void;
}
type CreateTRPCClientOptions<TRouter extends InferrableClientTypes> = {
    links: TRPCLink<TRouter>[];
    transformer?: TypeError<'The transformer property has moved to httpLink/httpBatchLink/wsLink'>;
};
declare class TRPCUntypedClient<TInferrable extends InferrableClientTypes> {
    private readonly links;
    readonly runtime: TRPCClientRuntime;
    private requestId;
    constructor(opts: CreateTRPCClientOptions<TInferrable>);
    private $request;
    private requestAsPromise;
    query(path: string, input?: unknown, opts?: TRPCRequestOptions): Promise<unknown>;
    mutation(path: string, input?: unknown, opts?: TRPCRequestOptions): Promise<unknown>;
    subscription(path: string, input: unknown, opts: Partial<TRPCSubscriptionObserver<unknown, TRPCClientError<AnyRouter$1>>> & TRPCRequestOptions): Unsubscribable;
}
declare const untypedClientSymbol: unique symbol;
type TRPCClient<TRouter extends AnyRouter$1> = DecoratedProcedureRecord<{
    transformer: TRouter['_def']['_config']['$types']['transformer'];
    errorShape: TRouter['_def']['_config']['$types']['errorShape'];
}, TRouter['_def']['record']> & {
    [untypedClientSymbol]: TRPCUntypedClient<TRouter>;
};
type TRPCResolverDef = {
    input: any;
    output: any;
    transformer: boolean;
    errorShape: any;
};
type coerceAsyncGeneratorToIterable<T> = T extends AsyncGenerator<infer $T, infer $Return, infer $Next> ? AsyncIterable<$T, $Return, $Next> : T;
type Resolver<TDef extends TRPCResolverDef> = (input: TDef['input'], opts?: TRPCProcedureOptions) => Promise<coerceAsyncGeneratorToIterable<TDef['output']>>;
type SubscriptionResolver<TDef extends TRPCResolverDef> = (input: TDef['input'], opts: Partial<TRPCSubscriptionObserver<TDef['output'], TRPCClientError<TDef>>> & TRPCProcedureOptions) => Unsubscribable;
type DecorateProcedure<TType extends ProcedureType, TDef extends TRPCResolverDef> = TType extends 'query' ? {
    query: Resolver<TDef>;
} : TType extends 'mutation' ? {
    mutate: Resolver<TDef>;
} : TType extends 'subscription' ? {
    subscribe: SubscriptionResolver<TDef>;
} : never;
type DecoratedProcedureRecord<TRoot extends InferrableClientTypes, TRecord extends RouterRecord> = {
    [TKey in keyof TRecord]: TRecord[TKey] extends (infer $Value) ? $Value extends AnyProcedure ? DecorateProcedure<$Value['_def']['type'], {
        input: inferProcedureInput<$Value>;
        output: inferTransformedProcedureOutput<inferClientTypes<TRoot>, $Value>;
        errorShape: inferClientTypes<TRoot>['errorShape'];
        transformer: inferClientTypes<TRoot>['transformer'];
    }> : $Value extends RouterRecord ? DecoratedProcedureRecord<TRoot, $Value> : never : never;
};
declare function createTRPCClient<TRouter extends AnyRouter$1>(opts: CreateTRPCClientOptions<TRouter>): TRPCClient<TRouter>;
export { TRPCClientError, TRPCUntypedClient, createTRPCClient, httpBatchLink, httpLink, isTRPCClientError, loggerLink, splitLink };
export type { TRPCClient, TRPCClientErrorLike, TRPCConnectionState, TRPCLink, TRPCRequestOptions };
