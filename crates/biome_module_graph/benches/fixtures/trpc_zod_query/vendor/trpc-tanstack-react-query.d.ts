import { TRPCUntypedClient, TRPCClient, TRPCRequestOptions, TRPCClientErrorLike } from "./trpc-client";
import { AnyTRPCRouter, inferRouterContext, AnyTRPCRootTypes, TRPCRouterRecord, AnyTRPCProcedure, TRPCProcedureType, inferProcedureInput, inferTransformedProcedureOutput } from "./trpc-server";
import { QueryClient, QueryFilters, SkipToken, DefinedInitialDataOptions, DataTag, UnusedSkipTokenOptions, UndefinedInitialDataOptions, DefinedInitialDataInfiniteOptions, InfiniteData, UnusedSkipTokenInfiniteOptions, UndefinedInitialDataInfiniteOptions, UseMutationOptions } from "./tanstack-query";
import { MaybePromise, DistributiveOmit, coerceAsyncIterableToArray, inferAsyncIterableYield } from "./trpc-server";
import { TRPCConnectionState } from "./trpc-client";
import { Unsubscribable } from "./trpc-server";
type WithRequired<TObj, TKey extends keyof TObj> = TObj & {
    [P in TKey]-?: TObj[P];
};
type ResolverDef = {
    input: any;
    output: any;
    transformer: boolean;
    errorShape: any;
    featureFlags: FeatureFlags;
};
type CursorInput = {
    cursor?: any;
};
type OptionalCursorInput = CursorInput | void;
type ExtractCursorType<TInput> = TInput extends CursorInput ? TInput['cursor'] : unknown;
type TRPCInfiniteData<TInput, TOutput> = InfiniteData<TOutput, NonNullable<ExtractCursorType<TInput>> | null>;
interface TRPCReactRequestOptions extends Omit<TRPCRequestOptions, 'signal'> {
    ssr?: boolean;
    abortOnUnmount?: boolean;
}
interface TRPCQueryBaseOptions {
    trpc?: TRPCReactRequestOptions;
}
interface TRPCQueryOptionsResult {
    trpc: {
        path: string;
    };
}
type QueryType = 'any' | 'infinite' | 'query';
type TRPCQueryKeyWithoutPrefix = [
    path: string[],
    opts?: {
        input?: unknown;
        type?: Exclude<QueryType, 'any'>;
    }
];
type TRPCQueryKeyWithPrefix = [
    prefix: string[],
    ...TRPCQueryKeyWithoutPrefix
];
type TRPCQueryKey<TPrefixEnabled extends boolean = false> = TPrefixEnabled extends true ? TRPCQueryKeyWithPrefix : TRPCQueryKeyWithoutPrefix;
type TRPCMutationKeyWithPrefix = [
    prefix: string[],
    ...TRPCMutationKeyWithoutPrefix
];
type TRPCMutationKeyWithoutPrefix = [
    path: string[]
];
type TRPCMutationKey<TPrefixEnabled extends boolean = false> = TPrefixEnabled extends true ? TRPCMutationKeyWithPrefix : TRPCMutationKeyWithoutPrefix;
type FeatureFlags = {
    keyPrefix: boolean;
};
type ofFeatureFlags<T extends FeatureFlags> = T;
type KeyPrefixOptions<TFeatureFlags extends FeatureFlags> = TFeatureFlags['keyPrefix'] extends true ? {
    keyPrefix: string;
} : {
    keyPrefix?: never;
};
type DefaultFeatureFlags = ofFeatureFlags<{
    keyPrefix: false;
}>;
type ReservedOptions$2 = 'queryKey' | 'queryFn' | 'queryHashFn' | 'queryHash' | 'initialPageParam';
interface UndefinedTRPCInfiniteQueryOptionsIn<TInput, TQueryFnData, TData, TError, TFeatureFlags extends FeatureFlags> extends DistributiveOmit<UndefinedInitialDataInfiniteOptions<TQueryFnData, TError, TRPCInfiniteData<TInput, TData>, TRPCQueryKey<TFeatureFlags['keyPrefix']>, NonNullable<ExtractCursorType<TInput>> | null>, ReservedOptions$2>, TRPCQueryBaseOptions {
    initialCursor?: NonNullable<ExtractCursorType<TInput>> | null;
}
interface UndefinedTRPCInfiniteQueryOptionsOut<TInput, TQueryFnData, TData, TError, TFeatureFlags extends FeatureFlags> extends DistributiveOmit<UndefinedInitialDataInfiniteOptions<TQueryFnData, TError, TRPCInfiniteData<TInput, TData>, TRPCQueryKey<TFeatureFlags['keyPrefix']>, NonNullable<ExtractCursorType<TInput>> | null>, 'initialPageParam'>, TRPCQueryOptionsResult {
    queryKey: DataTag<TRPCQueryKey<TFeatureFlags['keyPrefix']>, TRPCInfiniteData<TInput, TData>, TError>;
    initialPageParam: NonNullable<ExtractCursorType<TInput>> | null;
}
interface DefinedTRPCInfiniteQueryOptionsIn<TInput, TQueryFnData, TData, TError, TFeatureFlags extends FeatureFlags> extends DistributiveOmit<DefinedInitialDataInfiniteOptions<TQueryFnData, TError, TRPCInfiniteData<TInput, TData>, TRPCQueryKey<TFeatureFlags['keyPrefix']>, NonNullable<ExtractCursorType<TInput>> | null>, ReservedOptions$2>, TRPCQueryBaseOptions {
    initialCursor?: NonNullable<ExtractCursorType<TInput>> | null;
}
interface DefinedTRPCInfiniteQueryOptionsOut<TInput, TQueryFnData, TData, TError, TFeatureFlags extends FeatureFlags> extends DistributiveOmit<DefinedInitialDataInfiniteOptions<TQueryFnData, TError, TRPCInfiniteData<TInput, TData>, TRPCQueryKey<TFeatureFlags['keyPrefix']>, NonNullable<ExtractCursorType<TInput>> | null>, 'initialPageParam'>, TRPCQueryOptionsResult {
    queryKey: DataTag<TRPCQueryKey<TFeatureFlags['keyPrefix']>, TRPCInfiniteData<TInput, TData>, TError>;
    initialPageParam: NonNullable<ExtractCursorType<TInput>> | null;
}
interface UnusedSkipTokenTRPCInfiniteQueryOptionsIn<TInput, TQueryFnData, TData, TError, TFeatureFlags extends FeatureFlags> extends DistributiveOmit<UnusedSkipTokenInfiniteOptions<TQueryFnData, TError, TRPCInfiniteData<TInput, TData>, TRPCQueryKey<TFeatureFlags['keyPrefix']>, NonNullable<ExtractCursorType<TInput>> | null>, ReservedOptions$2>, TRPCQueryBaseOptions {
    initialCursor?: NonNullable<ExtractCursorType<TInput>> | null;
}
interface UnusedSkipTokenTRPCInfiniteQueryOptionsOut<TInput, TQueryFnData, TData, TError, TFeatureFlags extends FeatureFlags> extends DistributiveOmit<UnusedSkipTokenInfiniteOptions<TQueryFnData, TError, TRPCInfiniteData<TInput, TData>, TRPCQueryKey<TFeatureFlags['keyPrefix']>, NonNullable<ExtractCursorType<TInput>> | null>, 'initialPageParam'>, TRPCQueryOptionsResult {
    queryKey: DataTag<TRPCQueryKey<TFeatureFlags['keyPrefix']>, TRPCInfiniteData<TInput, TData>, TError>;
    initialPageParam: NonNullable<ExtractCursorType<TInput>> | null;
}
interface TRPCInfiniteQueryOptions<TDef extends ResolverDef> {
    <TQueryFnData extends TDef['output'], TData = TQueryFnData>(input: TDef['input'] | SkipToken, opts: DefinedTRPCInfiniteQueryOptionsIn<TDef['input'], TQueryFnData, TData, TRPCClientErrorLike<{
        transformer: TDef['transformer'];
        errorShape: TDef['errorShape'];
    }>, TDef['featureFlags']>): DefinedTRPCInfiniteQueryOptionsOut<TDef['input'], TQueryFnData, TData, TRPCClientErrorLike<{
        transformer: TDef['transformer'];
        errorShape: TDef['errorShape'];
    }>, TDef['featureFlags']>;
    <TQueryFnData extends TDef['output'], TData = TQueryFnData>(input: TDef['input'], opts: UnusedSkipTokenTRPCInfiniteQueryOptionsIn<TDef['input'], TQueryFnData, TData, TRPCClientErrorLike<{
        transformer: TDef['transformer'];
        errorShape: TDef['errorShape'];
    }>, TDef['featureFlags']>): UnusedSkipTokenTRPCInfiniteQueryOptionsOut<TDef['input'], TQueryFnData, TData, TRPCClientErrorLike<{
        transformer: TDef['transformer'];
        errorShape: TDef['errorShape'];
    }>, TDef['featureFlags']>;
    <TQueryFnData extends TDef['output'], TData = TQueryFnData>(input: TDef['input'] | SkipToken, opts: UndefinedTRPCInfiniteQueryOptionsIn<TDef['input'], TQueryFnData, TData, TRPCClientErrorLike<{
        transformer: TDef['transformer'];
        errorShape: TDef['errorShape'];
    }>, TDef['featureFlags']>): UndefinedTRPCInfiniteQueryOptionsOut<TDef['input'], TQueryFnData, TData, TRPCClientErrorLike<{
        transformer: TDef['transformer'];
        errorShape: TDef['errorShape'];
    }>, TDef['featureFlags']>;
}
type ReservedOptions$1 = 'mutationKey' | 'mutationFn';
type TRPCMutationOptionsIn<TInput, TError, TOutput, TContext, TFeatureFlags extends FeatureFlags> = DistributiveOmit<UseMutationOptions<TOutput, TError, TInput, TContext>, ReservedOptions$1> & TRPCQueryBaseOptions & KeyPrefixOptions<TFeatureFlags>;
interface TRPCMutationOptionsOut<TInput, TError, TOutput, TContext, TFeatureFlags extends FeatureFlags> extends UseMutationOptions<TOutput, TError, TInput, TContext>, TRPCQueryOptionsResult {
    mutationKey: TRPCMutationKey<TFeatureFlags['keyPrefix']>;
}
interface TRPCMutationOptions<TDef extends ResolverDef, TFeatureFlags extends FeatureFlags = DefaultFeatureFlags> {
    <TContext = unknown>(opts?: TRPCMutationOptionsIn<TDef['input'], TRPCClientErrorLike<TDef>, TDef['output'], TContext, TFeatureFlags>): TRPCMutationOptionsOut<TDef['input'], TRPCClientErrorLike<TDef>, TDef['output'], TContext, TFeatureFlags>;
}
interface MutationOptionsOverride {
    onSuccess: (opts: {
        originalFn: () => MaybePromise<void>;
        queryClient: QueryClient;
        meta: Record<string, unknown>;
    }) => MaybePromise<void>;
}
type ReservedOptions = 'queryKey' | 'queryFn' | 'queryHashFn' | 'queryHash';
interface UndefinedTRPCQueryOptionsIn<TQueryFnData, TData, TError, TFeatureFlags extends FeatureFlags> extends DistributiveOmit<UndefinedInitialDataOptions<coerceAsyncIterableToArray<TQueryFnData>, TError, coerceAsyncIterableToArray<TData>, TRPCQueryKey<TFeatureFlags['keyPrefix']>>, ReservedOptions>, TRPCQueryBaseOptions {
}
interface UndefinedTRPCQueryOptionsOut<TQueryFnData, TOutput, TError, TFeatureFlags extends FeatureFlags> extends UndefinedInitialDataOptions<coerceAsyncIterableToArray<TQueryFnData>, TError, coerceAsyncIterableToArray<TOutput>, TRPCQueryKey<TFeatureFlags['keyPrefix']>>, TRPCQueryOptionsResult {
    queryKey: DataTag<TRPCQueryKey<TFeatureFlags['keyPrefix']>, coerceAsyncIterableToArray<TOutput>, TError>;
}
interface DefinedTRPCQueryOptionsIn<TQueryFnData, TData, TError, TFeatureFlags extends FeatureFlags> extends DistributiveOmit<DefinedInitialDataOptions<coerceAsyncIterableToArray<NoInfer<TQueryFnData>>, TError, coerceAsyncIterableToArray<TData>, TRPCQueryKey<TFeatureFlags['keyPrefix']>>, ReservedOptions>, TRPCQueryBaseOptions {
}
interface DefinedTRPCQueryOptionsOut<TQueryFnData, TData, TError, TFeatureFlags extends FeatureFlags> extends DefinedInitialDataOptions<coerceAsyncIterableToArray<TQueryFnData>, TError, coerceAsyncIterableToArray<TData>, TRPCQueryKey<TFeatureFlags['keyPrefix']>>, TRPCQueryOptionsResult {
    queryKey: DataTag<TRPCQueryKey<TFeatureFlags['keyPrefix']>, coerceAsyncIterableToArray<TData>, TError>;
}
interface UnusedSkipTokenTRPCQueryOptionsIn<TQueryFnData, TData, TError, TFeatureFlags extends FeatureFlags> extends DistributiveOmit<UnusedSkipTokenOptions<coerceAsyncIterableToArray<TQueryFnData>, TError, coerceAsyncIterableToArray<TData>, TRPCQueryKey<TFeatureFlags['keyPrefix']>>, ReservedOptions>, TRPCQueryBaseOptions {
}
interface UnusedSkipTokenTRPCQueryOptionsOut<TQueryFnData, TOutput, TError, TFeatureFlags extends FeatureFlags> extends UnusedSkipTokenOptions<coerceAsyncIterableToArray<TQueryFnData>, TError, coerceAsyncIterableToArray<TOutput>, TRPCQueryKey<TFeatureFlags['keyPrefix']>>, TRPCQueryOptionsResult {
    queryKey: DataTag<TRPCQueryKey<TFeatureFlags['keyPrefix']>, coerceAsyncIterableToArray<TOutput>, TError>;
}
interface TRPCQueryOptions<TDef extends ResolverDef, TFeatureFlags extends FeatureFlags = DefaultFeatureFlags> {
    <TQueryFnData extends TDef['output'], TData = TQueryFnData>(input: TDef['input'] | SkipToken, opts: DefinedTRPCQueryOptionsIn<TQueryFnData, TData, TRPCClientErrorLike<{
        transformer: TDef['transformer'];
        errorShape: TDef['errorShape'];
    }>, TFeatureFlags>): DefinedTRPCQueryOptionsOut<TQueryFnData, TData, TRPCClientErrorLike<{
        transformer: TDef['transformer'];
        errorShape: TDef['errorShape'];
    }>, TFeatureFlags>;
    <TQueryFnData extends TDef['output'], TData = TQueryFnData>(input: TDef['input'], opts?: UnusedSkipTokenTRPCQueryOptionsIn<TQueryFnData, TData, TRPCClientErrorLike<{
        transformer: TDef['transformer'];
        errorShape: TDef['errorShape'];
    }>, TFeatureFlags>): UnusedSkipTokenTRPCQueryOptionsOut<TQueryFnData, TData, TRPCClientErrorLike<{
        transformer: TDef['transformer'];
        errorShape: TDef['errorShape'];
    }>, TFeatureFlags>;
    <TQueryFnData extends TDef['output'], TData = TQueryFnData>(input: TDef['input'] | SkipToken, opts?: UndefinedTRPCQueryOptionsIn<TQueryFnData, TData, TRPCClientErrorLike<{
        transformer: TDef['transformer'];
        errorShape: TDef['errorShape'];
    }>, TFeatureFlags>): UndefinedTRPCQueryOptionsOut<TQueryFnData, TData, TRPCClientErrorLike<{
        transformer: TDef['transformer'];
        errorShape: TDef['errorShape'];
    }>, TFeatureFlags>;
}
interface BaseTRPCSubscriptionOptionsIn<TOutput, TError> {
    enabled?: boolean;
    onStarted?: () => void;
    onData?: (data: inferAsyncIterableYield<TOutput>) => void;
    onError?: (err: TError) => void;
    onConnectionStateChange?: (state: TRPCConnectionState<TError>) => void;
}
interface UnusedSkipTokenTRPCSubscriptionOptionsIn<TOutput, TError> {
    onStarted?: () => void;
    onData?: (data: inferAsyncIterableYield<TOutput>) => void;
    onError?: (err: TError) => void;
    onConnectionStateChange?: (state: TRPCConnectionState<TError>) => void;
}
interface TRPCSubscriptionOptionsOut<TOutput, TError, TFeatureFlags extends FeatureFlags> extends UnusedSkipTokenTRPCSubscriptionOptionsIn<TOutput, TError>, TRPCQueryOptionsResult {
    enabled: boolean;
    queryKey: TRPCQueryKey<TFeatureFlags['keyPrefix']>;
    subscribe: (innerOpts: UnusedSkipTokenTRPCSubscriptionOptionsIn<TOutput, TError>) => Unsubscribable;
}
interface TRPCSubscriptionOptions<TDef extends ResolverDef, TFeatureFlags extends FeatureFlags = DefaultFeatureFlags> {
    (input: TDef['input'], opts?: UnusedSkipTokenTRPCSubscriptionOptionsIn<inferAsyncIterableYield<TDef['output']>, TRPCClientErrorLike<TDef>>): TRPCSubscriptionOptionsOut<inferAsyncIterableYield<TDef['output']>, TRPCClientErrorLike<TDef>, TFeatureFlags>;
    (input: TDef['input'] | SkipToken, opts?: BaseTRPCSubscriptionOptionsIn<inferAsyncIterableYield<TDef['output']>, TRPCClientErrorLike<TDef>>): TRPCSubscriptionOptionsOut<inferAsyncIterableYield<TDef['output']>, TRPCClientErrorLike<TDef>, TFeatureFlags>;
}
interface DecorateRouterKeyable<TFeatureFlags extends FeatureFlags> {
    pathKey: () => TRPCQueryKey<TFeatureFlags['keyPrefix']>;
    pathFilter: (filters?: QueryFilters<TRPCQueryKey<TFeatureFlags['keyPrefix']>>) => WithRequired<QueryFilters<TRPCQueryKey<TFeatureFlags['keyPrefix']>>, 'queryKey'>;
}
interface TypeHelper<TDef extends ResolverDef> {
    '~types': {
        input: TDef['input'];
        output: TDef['output'];
        errorShape: TDef['errorShape'];
    };
}
type inferInput<TProcedure extends DecorateInfiniteQueryProcedure<any> | DecorateQueryProcedure<any> | DecorateMutationProcedure<any>> = TProcedure['~types']['input'];
type inferOutput<TProcedure extends DecorateInfiniteQueryProcedure<any> | DecorateQueryProcedure<any> | DecorateMutationProcedure<any>> = TProcedure['~types']['output'];
interface DecorateInfiniteQueryProcedure<TDef extends ResolverDef> extends TypeHelper<TDef> {
    infiniteQueryOptions: TRPCInfiniteQueryOptions<TDef>;
    infiniteQueryKey: (input?: Partial<TDef['input']>) => DataTag<TRPCQueryKey<TDef['featureFlags']['keyPrefix']>, TRPCInfiniteData<TDef['input'], TDef['output']>, TRPCClientErrorLike<{
        transformer: TDef['transformer'];
        errorShape: TDef['errorShape'];
    }>>;
    infiniteQueryFilter: (input?: Partial<TDef['input']>, filters?: QueryFilters<DataTag<TRPCQueryKey<TDef['featureFlags']['keyPrefix']>, TRPCInfiniteData<TDef['input'], TDef['output']>, TRPCClientErrorLike<{
        transformer: TDef['transformer'];
        errorShape: TDef['errorShape'];
    }>>>) => WithRequired<QueryFilters<DataTag<TRPCQueryKey<TDef['featureFlags']['keyPrefix']>, TRPCInfiniteData<TDef['input'], TDef['output']>, TRPCClientErrorLike<{
        transformer: TDef['transformer'];
        errorShape: TDef['errorShape'];
    }>>>, 'queryKey'>;
}
interface DecorateQueryProcedure<TDef extends ResolverDef> extends TypeHelper<TDef>, DecorateRouterKeyable<TDef['featureFlags']> {
    queryOptions: TRPCQueryOptions<TDef>;
    queryKey: (input?: Partial<TDef['input']>) => DataTag<TRPCQueryKey<TDef['featureFlags']['keyPrefix']>, TDef['output'], TRPCClientErrorLike<{
        transformer: TDef['transformer'];
        errorShape: TDef['errorShape'];
    }>>;
    queryFilter: (input?: Partial<TDef['input']>, filters?: QueryFilters<DataTag<TRPCQueryKey<TDef['featureFlags']['keyPrefix']>, TDef['output'], TRPCClientErrorLike<{
        transformer: TDef['transformer'];
        errorShape: TDef['errorShape'];
    }>>>) => WithRequired<QueryFilters<DataTag<TRPCQueryKey<TDef['featureFlags']['keyPrefix']>, TDef['output'], TRPCClientErrorLike<{
        transformer: TDef['transformer'];
        errorShape: TDef['errorShape'];
    }>>>, 'queryKey'>;
}
interface DecorateMutationProcedure<TDef extends ResolverDef> extends TypeHelper<TDef> {
    mutationOptions: TRPCMutationOptions<TDef>;
    mutationKey: () => TRPCMutationKey<TDef['featureFlags']['keyPrefix']>;
}
interface DecorateSubscriptionProcedure<TDef extends ResolverDef> extends TypeHelper<TDef> {
    subscriptionOptions: TRPCSubscriptionOptions<TDef>;
}
type DecorateProcedure<TType extends TRPCProcedureType, TDef extends ResolverDef> = TType extends 'query' ? DecorateQueryProcedure<TDef> & (TDef['input'] extends OptionalCursorInput ? DecorateInfiniteQueryProcedure<TDef> : Record<string, never>) : TType extends 'mutation' ? DecorateMutationProcedure<TDef> : TType extends 'subscription' ? DecorateSubscriptionProcedure<TDef> : never;
type DecoratedRouterRecord<TRoot extends AnyTRPCRootTypes, TRecord extends TRPCRouterRecord, TFeatureFlags extends FeatureFlags = DefaultFeatureFlags> = {
    [TKey in keyof TRecord]: TRecord[TKey] extends (infer $Value) ? $Value extends TRPCRouterRecord ? DecoratedRouterRecord<TRoot, $Value, TFeatureFlags> & DecorateRouterKeyable<TFeatureFlags> : $Value extends AnyTRPCProcedure ? DecorateProcedure<$Value['_def']['type'], {
        input: inferProcedureInput<$Value>;
        output: inferTransformedProcedureOutput<TRoot, $Value>;
        transformer: TRoot['transformer'];
        errorShape: TRoot['errorShape'];
        featureFlags: TFeatureFlags;
    }> : never : never;
};
type TRPCOptionsProxy<TRouter extends AnyTRPCRouter, TFeatureFlags extends FeatureFlags = DefaultFeatureFlags> = DecoratedRouterRecord<TRouter['_def']['_config']['$types'], TRouter['_def']['record'], TFeatureFlags> & DecorateRouterKeyable<TFeatureFlags>;
type TRPCOptionsProxyOptionsBase<TFeatureFlags extends FeatureFlags = DefaultFeatureFlags> = {
    queryClient: QueryClient | (() => QueryClient);
    overrides?: {
        mutations?: MutationOptionsOverride;
    };
} & KeyPrefixOptions<TFeatureFlags>;
interface TRPCOptionsProxyOptionsInternal<TRouter extends AnyTRPCRouter> {
    router: TRouter;
    ctx: inferRouterContext<TRouter> | (() => MaybePromise<inferRouterContext<TRouter>>);
}
interface TRPCOptionsProxyOptionsExternal<TRouter extends AnyTRPCRouter> {
    client: TRPCUntypedClient<TRouter> | TRPCClient<TRouter>;
}
type TRPCOptionsProxyOptions<TRouter extends AnyTRPCRouter, TFeatureFlags extends FeatureFlags = DefaultFeatureFlags> = TRPCOptionsProxyOptionsBase<TFeatureFlags> & (TRPCOptionsProxyOptionsInternal<TRouter> | TRPCOptionsProxyOptionsExternal<TRouter>);
declare function createTRPCOptionsProxy<TRouter extends AnyTRPCRouter, TFeatureFlags extends FeatureFlags = DefaultFeatureFlags>(opts: TRPCOptionsProxyOptions<TRouter, TFeatureFlags>): TRPCOptionsProxy<TRouter, TFeatureFlags>;
export { createTRPCOptionsProxy };
export type { TRPCOptionsProxy, inferInput, inferOutput };
