declare class Subscribable<TListener extends Function> {
    protected listeners: Set<TListener>;
    constructor();
    subscribe(listener: TListener): () => void;
    hasListeners(): boolean;
    protected onSubscribe(): void;
    protected onUnsubscribe(): void;
}
declare abstract class Removable {
    #private;
    gcTime: number;
    destroy(): void;
    protected scheduleGc(): void;
    protected updateGcTime(newGcTime: number | undefined): void;
    protected clearGcTimeout(): void;
    protected abstract optionalRemove(): void;
}
type QueryObserverListener<TData, TError> = (result: QueryObserverResult<TData, TError>) => void;
interface ObserverFetchOptions extends FetchOptions {
    throwOnError?: boolean;
}
declare class QueryObserver<TQueryFnData = unknown, TError = DefaultError, TData = TQueryFnData, TQueryData = TQueryFnData, TQueryKey extends QueryKey = QueryKey> extends Subscribable<QueryObserverListener<TData, TError>> {
    #private;
    options: QueryObserverOptions<TQueryFnData, TError, TData, TQueryData, TQueryKey>;
    constructor(client: QueryClient, options: QueryObserverOptions<TQueryFnData, TError, TData, TQueryData, TQueryKey>);
    protected bindMethods(): void;
    protected onSubscribe(): void;
    protected onUnsubscribe(): void;
    shouldFetchOnReconnect(): boolean;
    shouldFetchOnWindowFocus(): boolean;
    destroy(): void;
    setOptions(options: QueryObserverOptions<TQueryFnData, TError, TData, TQueryData, TQueryKey>): void;
    getOptimisticResult(options: DefaultedQueryObserverOptions<TQueryFnData, TError, TData, TQueryData, TQueryKey>): QueryObserverResult<TData, TError>;
    getCurrentResult(): QueryObserverResult<TData, TError>;
    trackResult(result: QueryObserverResult<TData, TError>, onPropTracked?: (key: keyof QueryObserverResult) => void): QueryObserverResult<TData, TError>;
    trackProp(key: keyof QueryObserverResult): void;
    getCurrentQuery(): Query<TQueryFnData, TError, TQueryData, TQueryKey>;
    refetch({ ...options }?: RefetchOptions): Promise<QueryObserverResult<TData, TError>>;
    fetchOptimistic(options: QueryObserverOptions<TQueryFnData, TError, TData, TQueryData, TQueryKey>): Promise<QueryObserverResult<TData, TError>>;
    protected fetch(fetchOptions: ObserverFetchOptions): Promise<QueryObserverResult<TData, TError>>;
    protected createResult(query: Query<TQueryFnData, TError, TQueryData, TQueryKey>, options: QueryObserverOptions<TQueryFnData, TError, TData, TQueryData, TQueryKey>): QueryObserverResult<TData, TError>;
    updateResult(): void;
    onQueryUpdate(): void;
}
interface QueryConfig<TQueryFnData, TError, TData, TQueryKey extends QueryKey = QueryKey> {
    client: QueryClient;
    queryKey: TQueryKey;
    queryHash: string;
    options?: QueryOptions<TQueryFnData, TError, TData, TQueryKey>;
    defaultOptions?: QueryOptions<TQueryFnData, TError, TData, TQueryKey>;
    state?: QueryState<TData, TError>;
}
interface QueryState<TData = unknown, TError = DefaultError> {
    data: TData | undefined;
    dataUpdateCount: number;
    dataUpdatedAt: number;
    error: TError | null;
    errorUpdateCount: number;
    errorUpdatedAt: number;
    fetchFailureCount: number;
    fetchFailureReason: TError | null;
    fetchMeta: FetchMeta | null;
    isInvalidated: boolean;
    status: QueryStatus;
    fetchStatus: FetchStatus;
}
interface FetchContext<TQueryFnData, TError, TData, TQueryKey extends QueryKey = QueryKey> {
    fetchFn: () => unknown | Promise<unknown>;
    fetchOptions?: FetchOptions;
    signal: AbortSignal;
    options: QueryOptions<TQueryFnData, TError, TData, any>;
    client: QueryClient;
    queryKey: TQueryKey;
    state: QueryState<TData, TError>;
}
interface QueryBehavior<TQueryFnData = unknown, TError = DefaultError, TData = TQueryFnData, TQueryKey extends QueryKey = QueryKey> {
    onFetch: (context: FetchContext<TQueryFnData, TError, TData, TQueryKey>, query: Query) => void;
}
type FetchDirection = 'forward' | 'backward';
interface FetchMeta {
    fetchMore?: {
        direction: FetchDirection;
    };
}
interface FetchOptions<TData = unknown> {
    cancelRefetch?: boolean;
    meta?: FetchMeta;
    initialPromise?: Promise<TData>;
}
interface FailedAction$1<TError> {
    type: 'failed';
    failureCount: number;
    error: TError;
}
interface FetchAction {
    type: 'fetch';
    meta?: FetchMeta;
}
interface SuccessAction$1<TData> {
    data: TData | undefined;
    type: 'success';
    dataUpdatedAt?: number;
    manual?: boolean;
}
interface ErrorAction$1<TError> {
    type: 'error';
    error: TError;
}
interface InvalidateAction {
    type: 'invalidate';
}
interface PauseAction$1 {
    type: 'pause';
}
interface ContinueAction$1 {
    type: 'continue';
}
interface SetStateAction<TData, TError> {
    type: 'setState';
    state: Partial<QueryState<TData, TError>>;
}
type Action$1<TData, TError> = ContinueAction$1 | ErrorAction$1<TError> | FailedAction$1<TError> | FetchAction | InvalidateAction | PauseAction$1 | SetStateAction<TData, TError> | SuccessAction$1<TData>;
declare class Query<TQueryFnData = unknown, TError = DefaultError, TData = TQueryFnData, TQueryKey extends QueryKey = QueryKey> extends Removable {
    #private;
    queryKey: TQueryKey;
    queryHash: string;
    options: QueryOptions<TQueryFnData, TError, TData, TQueryKey>;
    state: QueryState<TData, TError>;
    observers: Array<QueryObserver<any, any, any, any, any>>;
    constructor(config: QueryConfig<TQueryFnData, TError, TData, TQueryKey>);
    get meta(): QueryMeta | undefined;
    get queryType(): "infinite" | undefined;
    get promise(): Promise<TData> | undefined;
    setOptions(options?: QueryOptions<TQueryFnData, TError, TData, TQueryKey>): void;
    protected optionalRemove(): void;
    setData(newData: TData, options?: SetDataOptions & {
        manual: boolean;
    }): TData;
    setState(state: Partial<QueryState<TData, TError>>): void;
    cancel(options?: CancelOptions): Promise<void>;
    destroy(): void;
    get resetState(): QueryState<TData, TError>;
    reset(): void;
    isActive(): boolean;
    isDisabled(): boolean;
    isFetched(): boolean;
    isStatic(): boolean;
    isStale(): boolean;
    isStaleByTime(staleTime?: StaleTime): boolean;
    onFocus(): void;
    onOnline(): void;
    addObserver(observer: QueryObserver<any, any, any, any, any>): void;
    removeObserver(observer: QueryObserver<any, any, any, any, any>): void;
    getObserversCount(): number;
    invalidate(): void;
    fetch(options?: QueryOptions<TQueryFnData, TError, TData, TQueryKey>, fetchOptions?: FetchOptions<TQueryFnData>): Promise<TData>;
}
type MutationObserverListener<TData, TError, TVariables, TOnMutateResult> = (result: MutationObserverResult<TData, TError, TVariables, TOnMutateResult>) => void;
declare class MutationObserver<TData = unknown, TError = DefaultError, TVariables = void, TOnMutateResult = unknown> extends Subscribable<MutationObserverListener<TData, TError, TVariables, TOnMutateResult>> {
    #private;
    options: MutationObserverOptions<TData, TError, TVariables, TOnMutateResult>;
    constructor(client: QueryClient, options: MutationObserverOptions<TData, TError, TVariables, TOnMutateResult>);
    protected bindMethods(): void;
    setOptions(options: MutationObserverOptions<TData, TError, TVariables, TOnMutateResult>): void;
    protected onSubscribe(): void;
    protected onUnsubscribe(): void;
    onMutationUpdate(action: Action<TData, TError, TVariables, TOnMutateResult>): void;
    getCurrentResult(): MutationObserverResult<TData, TError, TVariables, TOnMutateResult>;
    reset(): void;
    mutate(variables: TVariables, options?: MutateOptions<TData, TError, TVariables, TOnMutateResult>): Promise<TData>;
}
interface MutationCacheConfig {
    onError?: (error: DefaultError, variables: unknown, onMutateResult: unknown, mutation: Mutation<unknown, unknown, unknown>, context: MutationFunctionContext) => Promise<unknown> | unknown;
    onSuccess?: (data: unknown, variables: unknown, onMutateResult: unknown, mutation: Mutation<unknown, unknown, unknown>, context: MutationFunctionContext) => Promise<unknown> | unknown;
    onMutate?: (variables: unknown, mutation: Mutation<unknown, unknown, unknown>, context: MutationFunctionContext) => Promise<unknown> | unknown;
    onSettled?: (data: unknown | undefined, error: DefaultError | null, variables: unknown, onMutateResult: unknown, mutation: Mutation<unknown, unknown, unknown>, context: MutationFunctionContext) => Promise<unknown> | unknown;
}
interface NotifyEventMutationAdded extends NotifyEvent {
    type: 'added';
    mutation: Mutation<any, any, any, any>;
}
interface NotifyEventMutationRemoved extends NotifyEvent {
    type: 'removed';
    mutation: Mutation<any, any, any, any>;
}
interface NotifyEventMutationObserverAdded extends NotifyEvent {
    type: 'observerAdded';
    mutation: Mutation<any, any, any, any>;
    observer: MutationObserver<any, any, any>;
}
interface NotifyEventMutationObserverRemoved extends NotifyEvent {
    type: 'observerRemoved';
    mutation: Mutation<any, any, any, any>;
    observer: MutationObserver<any, any, any>;
}
interface NotifyEventMutationObserverOptionsUpdated extends NotifyEvent {
    type: 'observerOptionsUpdated';
    mutation?: Mutation<any, any, any, any>;
    observer: MutationObserver<any, any, any, any>;
}
interface NotifyEventMutationUpdated extends NotifyEvent {
    type: 'updated';
    mutation: Mutation<any, any, any, any>;
    action: Action<any, any, any, any>;
}
type MutationCacheNotifyEvent = NotifyEventMutationAdded | NotifyEventMutationRemoved | NotifyEventMutationObserverAdded | NotifyEventMutationObserverRemoved | NotifyEventMutationObserverOptionsUpdated | NotifyEventMutationUpdated;
type MutationCacheListener = (event: MutationCacheNotifyEvent) => void;
declare class MutationCache extends Subscribable<MutationCacheListener> {
    #private;
    config: MutationCacheConfig;
    constructor(config?: MutationCacheConfig);
    build<TData, TError, TVariables, TOnMutateResult>(client: QueryClient, options: MutationOptions<TData, TError, TVariables, TOnMutateResult>, state?: MutationState<TData, TError, TVariables, TOnMutateResult>): Mutation<TData, TError, TVariables, TOnMutateResult>;
    add(mutation: Mutation<any, any, any, any>): void;
    remove(mutation: Mutation<any, any, any, any>): void;
    canRun(mutation: Mutation<any, any, any, any>): boolean;
    runNext(mutation: Mutation<any, any, any, any>): Promise<unknown>;
    clear(): void;
    getAll(): Array<Mutation>;
    find<TData = unknown, TError = DefaultError, TVariables = any, TOnMutateResult = unknown>(filters: MutationFilters): Mutation<TData, TError, TVariables, TOnMutateResult> | undefined;
    findAll(filters?: MutationFilters): Array<Mutation>;
    notify(event: MutationCacheNotifyEvent): void;
    resumePausedMutations(): Promise<unknown>;
}
interface MutationConfig<TData, TError, TVariables, TOnMutateResult> {
    client: QueryClient;
    mutationId: number;
    mutationCache: MutationCache;
    options: MutationOptions<TData, TError, TVariables, TOnMutateResult>;
    state?: MutationState<TData, TError, TVariables, TOnMutateResult>;
}
interface MutationState<TData = unknown, TError = DefaultError, TVariables = unknown, TOnMutateResult = unknown> {
    context: TOnMutateResult | undefined;
    data: TData | undefined;
    error: TError | null;
    failureCount: number;
    failureReason: TError | null;
    isPaused: boolean;
    status: MutationStatus;
    variables: TVariables | undefined;
    submittedAt: number;
}
interface FailedAction<TError> {
    type: 'failed';
    failureCount: number;
    error: TError | null;
}
interface PendingAction<TVariables, TOnMutateResult> {
    type: 'pending';
    isPaused: boolean;
    variables?: TVariables;
    context?: TOnMutateResult;
}
interface SuccessAction<TData> {
    type: 'success';
    data: TData;
}
interface ErrorAction<TError> {
    type: 'error';
    error: TError;
}
interface PauseAction {
    type: 'pause';
}
interface ContinueAction {
    type: 'continue';
}
type Action<TData, TError, TVariables, TOnMutateResult> = ContinueAction | ErrorAction<TError> | FailedAction<TError> | PendingAction<TVariables, TOnMutateResult> | PauseAction | SuccessAction<TData>;
declare class Mutation<TData = unknown, TError = DefaultError, TVariables = unknown, TOnMutateResult = unknown> extends Removable {
    #private;
    state: MutationState<TData, TError, TVariables, TOnMutateResult>;
    options: MutationOptions<TData, TError, TVariables, TOnMutateResult>;
    readonly mutationId: number;
    constructor(config: MutationConfig<TData, TError, TVariables, TOnMutateResult>);
    setOptions(options: MutationOptions<TData, TError, TVariables, TOnMutateResult>): void;
    get meta(): MutationMeta | undefined;
    addObserver(observer: MutationObserver<any, any, any, any>): void;
    removeObserver(observer: MutationObserver<any, any, any, any>): void;
    protected optionalRemove(): void;
    continue(): Promise<unknown>;
    execute(variables: TVariables): Promise<TData>;
}
type DropLast<T extends ReadonlyArray<unknown>> = T extends readonly [
    ...infer R,
    unknown
] ? readonly [
    ...R
] : never;
type TuplePrefixes<T extends ReadonlyArray<unknown>> = T extends readonly [
] ? readonly [
] : TuplePrefixes<DropLast<T>> | T;
interface QueryFilters<TQueryKey extends QueryKey = QueryKey> {
    type?: QueryTypeFilter;
    exact?: boolean;
    predicate?: (query: Query) => boolean;
    queryKey?: TQueryKey | TuplePrefixes<TQueryKey>;
    stale?: boolean;
    fetchStatus?: FetchStatus;
}
interface MutationFilters<TData = unknown, TError = DefaultError, TVariables = unknown, TOnMutateResult = unknown> {
    exact?: boolean;
    predicate?: (mutation: Mutation<TData, TError, TVariables, TOnMutateResult>) => boolean;
    mutationKey?: TuplePrefixes<MutationKey>;
    status?: MutationStatus;
}
type Updater<TInput, TOutput> = TOutput | ((input: TInput) => TOutput);
type QueryTypeFilter = 'all' | 'active' | 'inactive';
declare function hashKey(queryKey: QueryKey | MutationKey): string;
declare function keepPreviousData<T>(previousData: T | undefined): T | undefined;
declare const skipToken: unique symbol;
type SkipToken = typeof skipToken;
interface QueryCacheConfig {
    onError?: (error: DefaultError, query: Query<unknown, unknown, unknown>) => void;
    onSuccess?: (data: unknown, query: Query<unknown, unknown, unknown>) => void;
    onSettled?: (data: unknown | undefined, error: DefaultError | null, query: Query<unknown, unknown, unknown>) => void;
}
interface NotifyEventQueryAdded extends NotifyEvent {
    type: 'added';
    query: Query<any, any, any, any>;
}
interface NotifyEventQueryRemoved extends NotifyEvent {
    type: 'removed';
    query: Query<any, any, any, any>;
}
interface NotifyEventQueryUpdated extends NotifyEvent {
    type: 'updated';
    query: Query<any, any, any, any>;
    action: Action$1<any, any>;
}
interface NotifyEventQueryObserverAdded extends NotifyEvent {
    type: 'observerAdded';
    query: Query<any, any, any, any>;
    observer: QueryObserver<any, any, any, any, any>;
}
interface NotifyEventQueryObserverRemoved extends NotifyEvent {
    type: 'observerRemoved';
    query: Query<any, any, any, any>;
    observer: QueryObserver<any, any, any, any, any>;
}
interface NotifyEventQueryObserverResultsUpdated extends NotifyEvent {
    type: 'observerResultsUpdated';
    query: Query<any, any, any, any>;
}
interface NotifyEventQueryObserverOptionsUpdated extends NotifyEvent {
    type: 'observerOptionsUpdated';
    query: Query<any, any, any, any>;
    observer: QueryObserver<any, any, any, any, any>;
}
type QueryCacheNotifyEvent = NotifyEventQueryAdded | NotifyEventQueryRemoved | NotifyEventQueryUpdated | NotifyEventQueryObserverAdded | NotifyEventQueryObserverRemoved | NotifyEventQueryObserverResultsUpdated | NotifyEventQueryObserverOptionsUpdated;
type QueryCacheListener = (event: QueryCacheNotifyEvent) => void;
declare class QueryCache extends Subscribable<QueryCacheListener> {
    #private;
    config: QueryCacheConfig;
    constructor(config?: QueryCacheConfig);
    build<TQueryFnData = unknown, TError = DefaultError, TData = TQueryFnData, TQueryKey extends QueryKey = QueryKey>(client: QueryClient, options: WithRequired<QueryOptions<TQueryFnData, TError, TData, TQueryKey>, 'queryKey'>, state?: QueryState<TData, TError>): Query<TQueryFnData, TError, TData, TQueryKey>;
    add(query: Query<any, any, any, any>): void;
    remove(query: Query<any, any, any, any>): void;
    clear(): void;
    get<TQueryFnData = unknown, TError = DefaultError, TData = TQueryFnData, TQueryKey extends QueryKey = QueryKey>(queryHash: string): Query<TQueryFnData, TError, TData, TQueryKey> | undefined;
    getAll(): Array<Query>;
    find<TQueryFnData = unknown, TError = DefaultError, TData = TQueryFnData>(filters: WithRequired<QueryFilters, 'queryKey'>): Query<TQueryFnData, TError, TData> | undefined;
    findAll(filters?: QueryFilters<any>): Array<Query>;
    notify(event: QueryCacheNotifyEvent): void;
    onFocus(): void;
    onOnline(): void;
}
declare class QueryClient {
    #private;
    constructor(config?: QueryClientConfig);
    mount(): void;
    unmount(): void;
    isFetching<TQueryFilters extends QueryFilters<any> = QueryFilters>(filters?: TQueryFilters): number;
    isMutating<TMutationFilters extends MutationFilters<any, any> = MutationFilters>(filters?: TMutationFilters): number;
    getQueryData<TQueryFnData = unknown, TTaggedQueryKey extends QueryKey = QueryKey, TInferredQueryFnData = InferDataFromTag<TQueryFnData, TTaggedQueryKey>>(queryKey: TTaggedQueryKey): TInferredQueryFnData | undefined;
    ensureQueryData<TQueryFnData, TError = DefaultError, TData = TQueryFnData, TQueryKey extends QueryKey = QueryKey>(options: EnsureQueryDataOptions<TQueryFnData, TError, TData, TQueryKey>): Promise<TData>;
    getQueriesData<TQueryFnData = unknown, TQueryFilters extends QueryFilters<any> = QueryFilters>(filters: TQueryFilters): Array<[
        QueryKey,
        TQueryFnData | undefined
    ]>;
    setQueryData<TQueryFnData = unknown, TTaggedQueryKey extends QueryKey = QueryKey, TInferredQueryFnData = InferDataFromTag<TQueryFnData, TTaggedQueryKey>>(queryKey: TTaggedQueryKey, updater: Updater<NoInfer<TInferredQueryFnData> | undefined, NoInfer<TInferredQueryFnData> | undefined>, options?: SetDataOptions): NoInfer<TInferredQueryFnData> | undefined;
    setQueriesData<TQueryFnData, TQueryFilters extends QueryFilters<any> = QueryFilters>(filters: TQueryFilters, updater: Updater<NoInfer<TQueryFnData> | undefined, NoInfer<TQueryFnData> | undefined>, options?: SetDataOptions): Array<[
        QueryKey,
        TQueryFnData | undefined
    ]>;
    getQueryState<TQueryFnData = unknown, TError = DefaultError, TTaggedQueryKey extends QueryKey = QueryKey, TInferredQueryFnData = InferDataFromTag<TQueryFnData, TTaggedQueryKey>, TInferredError = InferErrorFromTag<TError, TTaggedQueryKey>>(queryKey: TTaggedQueryKey): QueryState<TInferredQueryFnData, TInferredError> | undefined;
    removeQueries<TTaggedQueryKey extends QueryKey = QueryKey>(filters?: QueryFilters<TTaggedQueryKey>): void;
    resetQueries<TTaggedQueryKey extends QueryKey = QueryKey>(filters?: QueryFilters<TTaggedQueryKey>, options?: ResetOptions): Promise<void>;
    cancelQueries<TTaggedQueryKey extends QueryKey = QueryKey>(filters?: QueryFilters<TTaggedQueryKey>, cancelOptions?: CancelOptions): Promise<void>;
    invalidateQueries<TTaggedQueryKey extends QueryKey = QueryKey>(filters?: InvalidateQueryFilters<TTaggedQueryKey>, options?: InvalidateOptions): Promise<void>;
    refetchQueries<TTaggedQueryKey extends QueryKey = QueryKey>(filters?: RefetchQueryFilters<TTaggedQueryKey>, options?: RefetchOptions): Promise<void>;
    query<TQueryFnData, TError = DefaultError, TData = TQueryFnData, TQueryData = TQueryFnData, TQueryKey extends QueryKey = QueryKey, TPageParam = never>(options: QueryExecuteOptions<TQueryFnData, TError, TData, TQueryData, TQueryKey, TPageParam>): Promise<TData>;
    fetchQuery<TQueryFnData, TError = DefaultError, TData = TQueryFnData, TQueryKey extends QueryKey = QueryKey, TPageParam = never>(options: FetchQueryOptions<TQueryFnData, TError, TData, TQueryKey, TPageParam>): Promise<TData>;
    prefetchQuery<TQueryFnData = unknown, TError = DefaultError, TData = TQueryFnData, TQueryKey extends QueryKey = QueryKey>(options: FetchQueryOptions<TQueryFnData, TError, TData, TQueryKey>): Promise<void>;
    infiniteQuery<TQueryFnData, TError = DefaultError, TData = InfiniteData<TQueryFnData>, TQueryKey extends QueryKey = QueryKey, TPageParam = unknown>(options: InfiniteQueryExecuteOptions<TQueryFnData, TError, TData, TQueryKey, TPageParam>): Promise<Array<TData> extends Array<InfiniteData<TQueryFnData>> ? InfiniteData<TQueryFnData, TPageParam> : TData>;
    fetchInfiniteQuery<TQueryFnData, TError = DefaultError, TData = TQueryFnData, TQueryKey extends QueryKey = QueryKey, TPageParam = unknown>(options: FetchInfiniteQueryOptions<TQueryFnData, TError, TData, TQueryKey, TPageParam>): Promise<InfiniteData<TData, TPageParam>>;
    prefetchInfiniteQuery<TQueryFnData, TError = DefaultError, TData = TQueryFnData, TQueryKey extends QueryKey = QueryKey, TPageParam = unknown>(options: FetchInfiniteQueryOptions<TQueryFnData, TError, TData, TQueryKey, TPageParam>): Promise<void>;
    ensureInfiniteQueryData<TQueryFnData, TError = DefaultError, TData = TQueryFnData, TQueryKey extends QueryKey = QueryKey, TPageParam = unknown>(options: EnsureInfiniteQueryDataOptions<TQueryFnData, TError, TData, TQueryKey, TPageParam>): Promise<InfiniteData<TData, TPageParam>>;
    resumePausedMutations(): Promise<unknown>;
    getQueryCache(): QueryCache;
    getMutationCache(): MutationCache;
    getDefaultOptions(): DefaultOptions;
    setDefaultOptions(options: DefaultOptions): void;
    setQueryDefaults<TQueryFnData = unknown, TError = DefaultError, TData = TQueryFnData, TQueryData = TQueryFnData>(queryKey: QueryKey, options: Partial<OmitKeyof<QueryObserverOptions<TQueryFnData, TError, TData, TQueryData>, 'queryKey'>>): void;
    getQueryDefaults(queryKey: QueryKey): OmitKeyof<QueryObserverOptions<any, any, any, any, any>, 'queryKey'>;
    setMutationDefaults<TData = unknown, TError = DefaultError, TVariables = void, TOnMutateResult = unknown>(mutationKey: MutationKey, options: OmitKeyof<MutationObserverOptions<TData, TError, TVariables, TOnMutateResult>, 'mutationKey'>): void;
    getMutationDefaults(mutationKey: MutationKey): OmitKeyof<MutationObserverOptions<any, any, any, any>, 'mutationKey'>;
    defaultQueryOptions<TQueryFnData = unknown, TError = DefaultError, TData = TQueryFnData, TQueryData = TQueryFnData, TQueryKey extends QueryKey = QueryKey, TPageParam = never>(options: QueryObserverOptions<TQueryFnData, TError, TData, TQueryData, TQueryKey, TPageParam> | DefaultedQueryObserverOptions<TQueryFnData, TError, TData, TQueryData, TQueryKey>): DefaultedQueryObserverOptions<TQueryFnData, TError, TData, TQueryData, TQueryKey>;
    defaultMutationOptions<T extends MutationOptions<any, any, any, any>>(options?: T): T;
    clear(): void;
}
type RetryValue<TError> = boolean | number | ShouldRetryFunction<TError>;
type ShouldRetryFunction<TError = DefaultError> = (failureCount: number, error: TError) => boolean;
type RetryDelayValue<TError> = number | RetryDelayFunction<TError>;
type RetryDelayFunction<TError = DefaultError> = (failureCount: number, error: TError) => number;
type NonUndefinedGuard<T> = T extends undefined ? never : T;
type OmitKeyof<TObject, TKey extends (TStrictly extends 'safely' ? keyof TObject | (string & Record<never, never>) | (number & Record<never, never>) | (symbol & Record<never, never>) : keyof TObject), TStrictly extends 'strictly' | 'safely' = 'strictly'> = Omit<TObject, TKey>;
interface Register {
}
type DefaultError = Register extends {
    defaultError: infer TError;
} ? TError : Error;
type QueryKey = Register extends {
    queryKey: infer TQueryKey;
} ? TQueryKey extends ReadonlyArray<unknown> ? TQueryKey : TQueryKey extends Array<unknown> ? TQueryKey : ReadonlyArray<unknown> : ReadonlyArray<unknown>;
declare const dataTagSymbol: unique symbol;
type dataTagSymbol = typeof dataTagSymbol;
declare const dataTagErrorSymbol: unique symbol;
type dataTagErrorSymbol = typeof dataTagErrorSymbol;
declare const unsetMarker: unique symbol;
type UnsetMarker = typeof unsetMarker;
type AnyDataTag = {
    [dataTagSymbol]: any;
    [dataTagErrorSymbol]: any;
};
type DataTag<TType, TValue, TError = UnsetMarker> = TType extends AnyDataTag ? TType : TType & {
    [dataTagSymbol]: TValue;
    [dataTagErrorSymbol]: TError;
};
type InferDataFromTag<TQueryFnData, TTaggedQueryKey extends QueryKey> = TTaggedQueryKey extends DataTag<unknown, infer TaggedValue, unknown> ? TaggedValue : TQueryFnData;
type InferErrorFromTag<TError, TTaggedQueryKey extends QueryKey> = TTaggedQueryKey extends DataTag<unknown, unknown, infer TaggedError> ? TaggedError extends UnsetMarker ? TError : TaggedError : TError;
type QueryFunction<T = unknown, TQueryKey extends QueryKey = QueryKey, TPageParam = never> = (context: QueryFunctionContext<TQueryKey, TPageParam>) => T | Promise<T>;
type StaleTime = number | 'static';
type StaleTimeFunction<TQueryFnData = unknown, TError = DefaultError, TData = TQueryFnData, TQueryKey extends QueryKey = QueryKey> = StaleTime | ((query: Query<TQueryFnData, TError, TData, TQueryKey>) => StaleTime);
type QueryBooleanOption<TQueryFnData = unknown, TError = DefaultError, TData = TQueryFnData, TQueryKey extends QueryKey = QueryKey> = boolean | ((query: Query<TQueryFnData, TError, TData, TQueryKey>) => boolean);
type QueryPersister<T = unknown, TQueryKey extends QueryKey = QueryKey, TPageParam = never> = [
    TPageParam
] extends [
    never
] ? (queryFn: QueryFunction<T, TQueryKey, never>, context: QueryFunctionContext<TQueryKey>, query: Query) => T | Promise<T> : (queryFn: QueryFunction<T, TQueryKey, TPageParam>, context: QueryFunctionContext<TQueryKey>, query: Query) => T | Promise<T>;
type QueryFunctionContext<TQueryKey extends QueryKey = QueryKey, TPageParam = never> = [
    TPageParam
] extends [
    never
] ? {
    client: QueryClient;
    queryKey: TQueryKey;
    signal: AbortSignal;
    meta: QueryMeta | undefined;
    pageParam?: unknown;
    direction?: unknown;
} : {
    client: QueryClient;
    queryKey: TQueryKey;
    signal: AbortSignal;
    pageParam: TPageParam;
    direction: FetchDirection;
    meta: QueryMeta | undefined;
};
type InitialDataFunction<T> = () => T | undefined;
type NonFunctionGuard<T> = T extends Function ? never : T;
type PlaceholderDataFunction<TQueryFnData = unknown, TError = DefaultError, TQueryData = TQueryFnData, TQueryKey extends QueryKey = QueryKey> = (previousData: TQueryData | undefined, previousQuery: Query<TQueryFnData, TError, TQueryData, TQueryKey> | undefined) => TQueryData | undefined;
type QueryKeyHashFunction<TQueryKey extends QueryKey> = (queryKey: TQueryKey) => string;
type GetPreviousPageParamFunction<TPageParam, TQueryFnData = unknown> = (firstPage: TQueryFnData, allPages: Array<TQueryFnData>, firstPageParam: TPageParam, allPageParams: Array<TPageParam>) => TPageParam | undefined | null;
type GetNextPageParamFunction<TPageParam, TQueryFnData = unknown> = (lastPage: TQueryFnData, allPages: Array<TQueryFnData>, lastPageParam: TPageParam, allPageParams: Array<TPageParam>) => TPageParam | undefined | null;
interface InfiniteData<TData, TPageParam = unknown> {
    pages: Array<TData>;
    pageParams: Array<TPageParam>;
}
type QueryMeta = Register extends {
    queryMeta: infer TQueryMeta;
} ? TQueryMeta extends Record<string, unknown> ? TQueryMeta : Record<string, unknown> : Record<string, unknown>;
type NetworkMode = 'online' | 'always' | 'offlineFirst';
type NotifyOnChangeProps = Array<keyof InfiniteQueryObserverResult> | 'all' | undefined | (() => Array<keyof InfiniteQueryObserverResult> | 'all' | undefined);
interface QueryOptions<TQueryFnData = unknown, TError = DefaultError, TData = TQueryFnData, TQueryKey extends QueryKey = QueryKey, TPageParam = never> {
    retry?: RetryValue<TError>;
    retryDelay?: RetryDelayValue<TError>;
    networkMode?: NetworkMode;
    gcTime?: number;
    queryFn?: QueryFunction<TQueryFnData, TQueryKey, TPageParam> | SkipToken;
    persister?: QueryPersister<TQueryFnData, NoInfer<TQueryKey>, TPageParam>;
    queryHash?: string;
    queryKey?: TQueryKey;
    queryKeyHashFn?: QueryKeyHashFunction<TQueryKey>;
    initialData?: TData | InitialDataFunction<TData>;
    initialDataUpdatedAt?: number | (() => number | undefined);
    behavior?: QueryBehavior<TQueryFnData, TError, TData, TQueryKey>;
    structuralSharing?: boolean | ((oldData: unknown | undefined, newData: unknown) => unknown);
    _defaulted?: boolean;
    _type?: 'infinite';
    meta?: QueryMeta;
    maxPages?: number;
}
interface InitialPageParam<TPageParam = unknown> {
    initialPageParam: TPageParam;
}
interface InfiniteQueryPageParamsOptions<TQueryFnData = unknown, TPageParam = unknown> extends InitialPageParam<TPageParam> {
    getPreviousPageParam?: GetPreviousPageParamFunction<TPageParam, TQueryFnData>;
    getNextPageParam: GetNextPageParamFunction<TPageParam, TQueryFnData>;
}
type ThrowOnError<TQueryFnData, TError, TQueryData, TQueryKey extends QueryKey> = boolean | ((error: TError, query: Query<TQueryFnData, TError, TQueryData, TQueryKey>) => boolean);
interface QueryObserverOptions<TQueryFnData = unknown, TError = DefaultError, TData = TQueryFnData, TQueryData = TQueryFnData, TQueryKey extends QueryKey = QueryKey, TPageParam = never> extends WithRequired<QueryOptions<TQueryFnData, TError, TQueryData, TQueryKey, TPageParam>, 'queryKey'> {
    enabled?: QueryBooleanOption<TQueryFnData, TError, TQueryData, TQueryKey>;
    staleTime?: StaleTimeFunction<TQueryFnData, TError, TQueryData, TQueryKey>;
    refetchInterval?: number | false | ((query: Query<TQueryFnData, TError, TQueryData, TQueryKey>) => number | false | undefined);
    refetchIntervalInBackground?: boolean;
    refetchOnWindowFocus?: boolean | 'always' | ((query: Query<TQueryFnData, TError, TQueryData, TQueryKey>) => boolean | 'always');
    refetchOnReconnect?: boolean | 'always' | ((query: Query<TQueryFnData, TError, TQueryData, TQueryKey>) => boolean | 'always');
    refetchOnMount?: boolean | 'always' | ((query: Query<TQueryFnData, TError, TQueryData, TQueryKey>) => boolean | 'always');
    retryOnMount?: QueryBooleanOption<TQueryFnData, TError, TQueryData, TQueryKey>;
    notifyOnChangeProps?: NotifyOnChangeProps;
    throwOnError?: ThrowOnError<TQueryFnData, TError, TQueryData, TQueryKey>;
    select?: (data: TQueryData) => TData;
    suspense?: boolean;
    placeholderData?: NonFunctionGuard<TQueryData> | PlaceholderDataFunction<NonFunctionGuard<TQueryData>, TError, NonFunctionGuard<TQueryData>, TQueryKey>;
    _optimisticResults?: 'optimistic' | 'isRestoring';
}
type WithRequired<TTarget, TKey extends keyof TTarget> = TTarget & {
    [_ in TKey]: {};
};
type DefaultedQueryObserverOptions<TQueryFnData = unknown, TError = DefaultError, TData = TQueryFnData, TQueryData = TQueryFnData, TQueryKey extends QueryKey = QueryKey> = WithRequired<QueryObserverOptions<TQueryFnData, TError, TData, TQueryData, TQueryKey>, 'throwOnError' | 'refetchOnReconnect' | 'queryHash'>;
interface InfiniteQueryObserverOptions<TQueryFnData = unknown, TError = DefaultError, TData = TQueryFnData, TQueryKey extends QueryKey = QueryKey, TPageParam = unknown> extends QueryObserverOptions<TQueryFnData, TError, TData, InfiniteData<TQueryFnData, TPageParam>, TQueryKey, TPageParam>, InfiniteQueryPageParamsOptions<TQueryFnData, TPageParam> {
}
type DefaultedInfiniteQueryObserverOptions<TQueryFnData = unknown, TError = DefaultError, TData = TQueryFnData, TQueryKey extends QueryKey = QueryKey, TPageParam = unknown> = WithRequired<InfiniteQueryObserverOptions<TQueryFnData, TError, TData, TQueryKey, TPageParam>, 'throwOnError' | 'refetchOnReconnect' | 'queryHash'>;
interface QueryExecuteOptions<TQueryFnData = unknown, TError = DefaultError, TData = TQueryFnData, TQueryData = TQueryFnData, TQueryKey extends QueryKey = QueryKey, TPageParam = never> extends WithRequired<QueryOptions<TQueryFnData, TError, TQueryData, TQueryKey, TPageParam>, 'queryKey'> {
    initialPageParam?: never;
    select?: (data: TQueryData) => TData;
    staleTime?: StaleTimeFunction<TQueryFnData, TError, TQueryData, TQueryKey>;
}
interface FetchQueryOptions<TQueryFnData = unknown, TError = DefaultError, TData = TQueryFnData, TQueryKey extends QueryKey = QueryKey, TPageParam = never> extends WithRequired<QueryOptions<TQueryFnData, TError, TData, TQueryKey, TPageParam>, 'queryKey'> {
    initialPageParam?: never;
    staleTime?: StaleTimeFunction<TQueryFnData, TError, TData, TQueryKey>;
}
interface EnsureQueryDataOptions<TQueryFnData = unknown, TError = DefaultError, TData = TQueryFnData, TQueryKey extends QueryKey = QueryKey, TPageParam = never> extends FetchQueryOptions<TQueryFnData, TError, TData, TQueryKey, TPageParam> {
    revalidateIfStale?: boolean;
}
type EnsureInfiniteQueryDataOptions<TQueryFnData = unknown, TError = DefaultError, TData = TQueryFnData, TQueryKey extends QueryKey = QueryKey, TPageParam = unknown> = FetchInfiniteQueryOptions<TQueryFnData, TError, TData, TQueryKey, TPageParam> & {
    revalidateIfStale?: boolean;
};
type InfiniteQueryPages<TQueryFnData = unknown, TPageParam = unknown> = {
    pages?: never;
} | {
    pages: number;
    getNextPageParam: GetNextPageParamFunction<TPageParam, TQueryFnData>;
};
type InfiniteQueryExecuteOptions<TQueryFnData = unknown, TError = DefaultError, TData = InfiniteData<TQueryFnData>, TQueryKey extends QueryKey = QueryKey, TPageParam = unknown> = Omit<QueryExecuteOptions<TQueryFnData, TError, TData, InfiniteData<TQueryFnData, TPageParam>, TQueryKey, TPageParam>, 'initialPageParam'> & InitialPageParam<TPageParam> & InfiniteQueryPages<TQueryFnData, TPageParam>;
type FetchInfiniteQueryOptions<TQueryFnData = unknown, TError = DefaultError, TData = TQueryFnData, TQueryKey extends QueryKey = QueryKey, TPageParam = unknown> = Omit<FetchQueryOptions<TQueryFnData, TError, InfiniteData<TData, TPageParam>, TQueryKey, TPageParam>, 'initialPageParam'> & InitialPageParam<TPageParam> & InfiniteQueryPages<TQueryFnData, TPageParam>;
interface ResultOptions {
    throwOnError?: boolean;
}
interface RefetchOptions extends ResultOptions {
    cancelRefetch?: boolean;
}
interface InvalidateQueryFilters<TQueryKey extends QueryKey = QueryKey> extends QueryFilters<TQueryKey> {
    refetchType?: QueryTypeFilter | 'none';
}
interface RefetchQueryFilters<TQueryKey extends QueryKey = QueryKey> extends QueryFilters<TQueryKey> {
}
interface InvalidateOptions extends RefetchOptions {
}
interface ResetOptions extends RefetchOptions {
}
interface FetchNextPageOptions extends ResultOptions {
    cancelRefetch?: boolean;
}
interface FetchPreviousPageOptions extends ResultOptions {
    cancelRefetch?: boolean;
}
type QueryStatus = 'pending' | 'error' | 'success';
type FetchStatus = 'fetching' | 'paused' | 'idle';
interface QueryObserverBaseResult<TData = unknown, TError = DefaultError> {
    data: TData | undefined;
    dataUpdatedAt: number;
    error: TError | null;
    errorUpdatedAt: number;
    failureCount: number;
    failureReason: TError | null;
    errorUpdateCount: number;
    isError: boolean;
    isFetched: boolean;
    isFetchedAfterMount: boolean;
    isFetching: boolean;
    isLoading: boolean;
    isPending: boolean;
    isLoadingError: boolean;
    isInitialLoading: boolean;
    isPaused: boolean;
    isPlaceholderData: boolean;
    isRefetchError: boolean;
    isRefetching: boolean;
    isStale: boolean;
    isSuccess: boolean;
    isEnabled: boolean;
    refetch: (options?: RefetchOptions) => Promise<QueryObserverResult<TData, TError>>;
    status: QueryStatus;
    fetchStatus: FetchStatus;
}
interface QueryObserverPendingResult<TData = unknown, TError = DefaultError> extends QueryObserverBaseResult<TData, TError> {
    data: undefined;
    error: null;
    isError: false;
    isPending: true;
    isLoadingError: false;
    isRefetchError: false;
    isSuccess: false;
    isPlaceholderData: false;
    status: 'pending';
}
interface QueryObserverLoadingResult<TData = unknown, TError = DefaultError> extends QueryObserverBaseResult<TData, TError> {
    data: undefined;
    error: null;
    isError: false;
    isPending: true;
    isLoading: true;
    isLoadingError: false;
    isRefetchError: false;
    isSuccess: false;
    isPlaceholderData: false;
    status: 'pending';
}
interface QueryObserverLoadingErrorResult<TData = unknown, TError = DefaultError> extends QueryObserverBaseResult<TData, TError> {
    data: undefined;
    error: TError;
    isError: true;
    isPending: false;
    isLoading: false;
    isLoadingError: true;
    isRefetchError: false;
    isSuccess: false;
    isPlaceholderData: false;
    status: 'error';
}
interface QueryObserverRefetchErrorResult<TData = unknown, TError = DefaultError> extends QueryObserverBaseResult<TData, TError> {
    data: TData;
    error: TError;
    isError: true;
    isPending: false;
    isLoading: false;
    isLoadingError: false;
    isRefetchError: true;
    isSuccess: false;
    isPlaceholderData: false;
    status: 'error';
}
interface QueryObserverSuccessResult<TData = unknown, TError = DefaultError> extends QueryObserverBaseResult<TData, TError> {
    data: TData;
    error: null;
    isError: false;
    isPending: false;
    isLoading: false;
    isLoadingError: false;
    isRefetchError: false;
    isSuccess: true;
    isPlaceholderData: false;
    status: 'success';
}
interface QueryObserverPlaceholderResult<TData = unknown, TError = DefaultError> extends QueryObserverBaseResult<TData, TError> {
    data: TData;
    isError: false;
    error: null;
    isPending: false;
    isLoading: false;
    isLoadingError: false;
    isRefetchError: false;
    isSuccess: true;
    isPlaceholderData: true;
    status: 'success';
}
type DefinedQueryObserverResult<TData = unknown, TError = DefaultError> = QueryObserverRefetchErrorResult<TData, TError> | QueryObserverSuccessResult<TData, TError>;
type QueryObserverResult<TData = unknown, TError = DefaultError> = DefinedQueryObserverResult<TData, TError> | QueryObserverLoadingErrorResult<TData, TError> | QueryObserverLoadingResult<TData, TError> | QueryObserverPendingResult<TData, TError> | QueryObserverPlaceholderResult<TData, TError>;
interface InfiniteQueryObserverBaseResult<TData = unknown, TError = DefaultError> extends QueryObserverBaseResult<TData, TError> {
    fetchNextPage: (options?: FetchNextPageOptions) => Promise<InfiniteQueryObserverResult<TData, TError>>;
    fetchPreviousPage: (options?: FetchPreviousPageOptions) => Promise<InfiniteQueryObserverResult<TData, TError>>;
    hasNextPage: boolean;
    hasPreviousPage: boolean;
    isFetchNextPageError: boolean;
    isFetchingNextPage: boolean;
    isFetchPreviousPageError: boolean;
    isFetchingPreviousPage: boolean;
}
interface InfiniteQueryObserverPendingResult<TData = unknown, TError = DefaultError> extends InfiniteQueryObserverBaseResult<TData, TError> {
    data: undefined;
    error: null;
    isError: false;
    isPending: true;
    isLoadingError: false;
    isRefetchError: false;
    isFetchNextPageError: false;
    isFetchPreviousPageError: false;
    isSuccess: false;
    isPlaceholderData: false;
    status: 'pending';
}
interface InfiniteQueryObserverLoadingResult<TData = unknown, TError = DefaultError> extends InfiniteQueryObserverBaseResult<TData, TError> {
    data: undefined;
    error: null;
    isError: false;
    isPending: true;
    isLoading: true;
    isLoadingError: false;
    isRefetchError: false;
    isFetchNextPageError: false;
    isFetchPreviousPageError: false;
    isSuccess: false;
    isPlaceholderData: false;
    status: 'pending';
}
interface InfiniteQueryObserverLoadingErrorResult<TData = unknown, TError = DefaultError> extends InfiniteQueryObserverBaseResult<TData, TError> {
    data: undefined;
    error: TError;
    isError: true;
    isPending: false;
    isLoading: false;
    isLoadingError: true;
    isRefetchError: false;
    isFetchNextPageError: false;
    isFetchPreviousPageError: false;
    isSuccess: false;
    isPlaceholderData: false;
    status: 'error';
}
interface InfiniteQueryObserverRefetchErrorResult<TData = unknown, TError = DefaultError> extends InfiniteQueryObserverBaseResult<TData, TError> {
    data: TData;
    error: TError;
    isError: true;
    isPending: false;
    isLoading: false;
    isLoadingError: false;
    isRefetchError: true;
    isSuccess: false;
    isPlaceholderData: false;
    status: 'error';
}
interface InfiniteQueryObserverSuccessResult<TData = unknown, TError = DefaultError> extends InfiniteQueryObserverBaseResult<TData, TError> {
    data: TData;
    error: null;
    isError: false;
    isPending: false;
    isLoading: false;
    isLoadingError: false;
    isRefetchError: false;
    isFetchNextPageError: false;
    isFetchPreviousPageError: false;
    isSuccess: true;
    isPlaceholderData: false;
    status: 'success';
}
interface InfiniteQueryObserverPlaceholderResult<TData = unknown, TError = DefaultError> extends InfiniteQueryObserverBaseResult<TData, TError> {
    data: TData;
    isError: false;
    error: null;
    isPending: false;
    isLoading: false;
    isLoadingError: false;
    isRefetchError: false;
    isSuccess: true;
    isPlaceholderData: true;
    isFetchNextPageError: false;
    isFetchPreviousPageError: false;
    status: 'success';
}
type DefinedInfiniteQueryObserverResult<TData = unknown, TError = DefaultError> = InfiniteQueryObserverRefetchErrorResult<TData, TError> | InfiniteQueryObserverSuccessResult<TData, TError>;
type InfiniteQueryObserverResult<TData = unknown, TError = DefaultError> = DefinedInfiniteQueryObserverResult<TData, TError> | InfiniteQueryObserverLoadingErrorResult<TData, TError> | InfiniteQueryObserverLoadingResult<TData, TError> | InfiniteQueryObserverPendingResult<TData, TError> | InfiniteQueryObserverPlaceholderResult<TData, TError>;
type MutationKey = Register extends {
    mutationKey: infer TMutationKey;
} ? TMutationKey extends ReadonlyArray<unknown> ? TMutationKey : TMutationKey extends Array<unknown> ? TMutationKey : ReadonlyArray<unknown> : ReadonlyArray<unknown>;
type MutationStatus = 'idle' | 'pending' | 'success' | 'error';
type MutationScope = {
    id: string;
};
type MutationMeta = Register extends {
    mutationMeta: infer TMutationMeta;
} ? TMutationMeta extends Record<string, unknown> ? TMutationMeta : Record<string, unknown> : Record<string, unknown>;
type MutationFunctionContext = {
    client: QueryClient;
    meta: MutationMeta | undefined;
    mutationKey?: MutationKey;
};
type MutationFunction<TData = unknown, TVariables = unknown> = (variables: TVariables, context: MutationFunctionContext) => Promise<TData>;
interface MutationOptions<TData = unknown, TError = DefaultError, TVariables = void, TOnMutateResult = unknown> {
    mutationFn?: MutationFunction<TData, TVariables>;
    mutationKey?: MutationKey;
    onMutate?: (variables: TVariables, context: MutationFunctionContext) => Promise<TOnMutateResult> | TOnMutateResult;
    onSuccess?: (data: TData, variables: TVariables, onMutateResult: TOnMutateResult, context: MutationFunctionContext) => Promise<unknown> | unknown;
    onError?: (error: TError, variables: TVariables, onMutateResult: TOnMutateResult | undefined, context: MutationFunctionContext) => Promise<unknown> | unknown;
    onSettled?: (data: TData | undefined, error: TError | null, variables: TVariables, onMutateResult: TOnMutateResult | undefined, context: MutationFunctionContext) => Promise<unknown> | unknown;
    retry?: RetryValue<TError>;
    retryDelay?: RetryDelayValue<TError>;
    networkMode?: NetworkMode;
    gcTime?: number;
    _defaulted?: boolean;
    meta?: MutationMeta;
    scope?: MutationScope;
}
interface MutationObserverOptions<TData = unknown, TError = DefaultError, TVariables = void, TOnMutateResult = unknown> extends MutationOptions<TData, TError, TVariables, TOnMutateResult> {
    throwOnError?: boolean | ((error: TError) => boolean);
}
interface MutateOptions<TData = unknown, TError = DefaultError, TVariables = void, TOnMutateResult = unknown> {
    onSuccess?: (data: TData, variables: TVariables, onMutateResult: TOnMutateResult | undefined, context: MutationFunctionContext) => void;
    onError?: (error: TError, variables: TVariables, onMutateResult: TOnMutateResult | undefined, context: MutationFunctionContext) => void;
    onSettled?: (data: TData | undefined, error: TError | null, variables: TVariables, onMutateResult: TOnMutateResult | undefined, context: MutationFunctionContext) => void;
}
type MutateFunctionRest<TData = unknown, TError = DefaultError, TVariables = void, TOnMutateResult = unknown> = undefined extends TVariables ? [
    variables?: TVariables,
    options?: MutateOptions<TData, TError, TVariables, TOnMutateResult>
] : [
    variables: TVariables,
    options?: MutateOptions<TData, TError, TVariables, TOnMutateResult>
];
type MutateFunction<TData = unknown, TError = DefaultError, TVariables = void, TOnMutateResult = unknown> = (...rest: MutateFunctionRest<TData, TError, TVariables, TOnMutateResult>) => Promise<TData>;
interface MutationObserverBaseResult<TData = unknown, TError = DefaultError, TVariables = void, TOnMutateResult = unknown> extends MutationState<TData, TError, TVariables, TOnMutateResult> {
    data: TData | undefined;
    variables: TVariables | undefined;
    error: TError | null;
    isError: boolean;
    isIdle: boolean;
    isPending: boolean;
    isSuccess: boolean;
    status: MutationStatus;
    mutate: MutateFunction<TData, TError, TVariables, TOnMutateResult>;
    reset: () => void;
}
interface MutationObserverIdleResult<TData = unknown, TError = DefaultError, TVariables = void, TOnMutateResult = unknown> extends MutationObserverBaseResult<TData, TError, TVariables, TOnMutateResult> {
    data: undefined;
    variables: undefined;
    error: null;
    isError: false;
    isIdle: true;
    isPending: false;
    isSuccess: false;
    status: 'idle';
}
interface MutationObserverLoadingResult<TData = unknown, TError = DefaultError, TVariables = void, TOnMutateResult = unknown> extends MutationObserverBaseResult<TData, TError, TVariables, TOnMutateResult> {
    data: undefined;
    variables: TVariables;
    error: null;
    isError: false;
    isIdle: false;
    isPending: true;
    isSuccess: false;
    status: 'pending';
}
interface MutationObserverErrorResult<TData = unknown, TError = DefaultError, TVariables = void, TOnMutateResult = unknown> extends MutationObserverBaseResult<TData, TError, TVariables, TOnMutateResult> {
    data: undefined;
    error: TError;
    variables: TVariables;
    isError: true;
    isIdle: false;
    isPending: false;
    isSuccess: false;
    status: 'error';
}
interface MutationObserverSuccessResult<TData = unknown, TError = DefaultError, TVariables = void, TOnMutateResult = unknown> extends MutationObserverBaseResult<TData, TError, TVariables, TOnMutateResult> {
    data: TData;
    error: null;
    variables: TVariables;
    isError: false;
    isIdle: false;
    isPending: false;
    isSuccess: true;
    status: 'success';
}
type MutationObserverResult<TData = unknown, TError = DefaultError, TVariables = void, TOnMutateResult = unknown> = MutationObserverIdleResult<TData, TError, TVariables, TOnMutateResult> | MutationObserverLoadingResult<TData, TError, TVariables, TOnMutateResult> | MutationObserverErrorResult<TData, TError, TVariables, TOnMutateResult> | MutationObserverSuccessResult<TData, TError, TVariables, TOnMutateResult>;
interface QueryClientConfig {
    queryCache?: QueryCache;
    mutationCache?: MutationCache;
    defaultOptions?: DefaultOptions;
}
interface DefaultOptions<TError = DefaultError> {
    queries?: OmitKeyof<QueryObserverOptions<unknown, TError>, 'suspense' | 'queryKey'>;
    mutations?: MutationObserverOptions<unknown, TError, unknown, unknown>;
    hydrate?: HydrateOptions['defaultOptions'];
    dehydrate?: DehydrateOptions;
}
interface CancelOptions {
    revert?: boolean;
    silent?: boolean;
}
interface SetDataOptions {
    updatedAt?: number;
}
type NotifyEventType = 'added' | 'removed' | 'updated' | 'observerAdded' | 'observerRemoved' | 'observerResultsUpdated' | 'observerOptionsUpdated';
interface NotifyEvent {
    type: NotifyEventType;
}
type TransformerFn = (data: any) => any;
interface DehydrateOptions {
    serializeData?: TransformerFn;
    shouldDehydrateMutation?: (mutation: Mutation) => boolean;
    shouldDehydrateQuery?: (query: Query) => boolean;
    shouldRedactErrors?: (error: unknown) => boolean;
}
interface HydrateOptions {
    defaultOptions?: {
        deserializeData?: TransformerFn;
        queries?: QueryOptions;
        mutations?: MutationOptions<unknown, DefaultError, unknown, unknown>;
    };
}
type InfiniteQueryObserverListener<TData, TError> = (result: InfiniteQueryObserverResult<TData, TError>) => void;
declare class InfiniteQueryObserver<TQueryFnData = unknown, TError = DefaultError, TData = InfiniteData<TQueryFnData>, TQueryKey extends QueryKey = QueryKey, TPageParam = unknown> extends QueryObserver<TQueryFnData, TError, TData, InfiniteData<TQueryFnData, TPageParam>, TQueryKey> {
    subscribe: Subscribable<InfiniteQueryObserverListener<TData, TError>>['subscribe'];
    getCurrentResult: ReplaceReturnType<QueryObserver<TQueryFnData, TError, TData, InfiniteData<TQueryFnData, TPageParam>, TQueryKey>['getCurrentResult'], InfiniteQueryObserverResult<TData, TError>>;
    protected fetch: ReplaceReturnType<QueryObserver<TQueryFnData, TError, TData, InfiniteData<TQueryFnData, TPageParam>, TQueryKey>['fetch'], Promise<InfiniteQueryObserverResult<TData, TError>>>;
    constructor(client: QueryClient, options: InfiniteQueryObserverOptions<TQueryFnData, TError, TData, TQueryKey, TPageParam>);
    protected bindMethods(): void;
    setOptions(options: InfiniteQueryObserverOptions<TQueryFnData, TError, TData, TQueryKey, TPageParam>): void;
    getOptimisticResult(options: DefaultedInfiniteQueryObserverOptions<TQueryFnData, TError, TData, TQueryKey, TPageParam>): InfiniteQueryObserverResult<TData, TError>;
    fetchNextPage(options?: FetchNextPageOptions): Promise<InfiniteQueryObserverResult<TData, TError>>;
    fetchPreviousPage(options?: FetchPreviousPageOptions): Promise<InfiniteQueryObserverResult<TData, TError>>;
    protected createResult(query: Query<TQueryFnData, TError, InfiniteData<TQueryFnData, TPageParam>, TQueryKey>, options: InfiniteQueryObserverOptions<TQueryFnData, TError, TData, TQueryKey, TPageParam>): InfiniteQueryObserverResult<TData, TError>;
}
type ReplaceReturnType<TFunction extends (...args: Array<any>) => unknown, TReturn> = (...args: Parameters<TFunction>) => TReturn;
interface UseBaseQueryOptions<TQueryFnData = unknown, TError = DefaultError, TData = TQueryFnData, TQueryData = TQueryFnData, TQueryKey extends QueryKey = QueryKey> extends QueryObserverOptions<TQueryFnData, TError, TData, TQueryData, TQueryKey> {
    subscribed?: boolean;
}
interface UseQueryOptions<TQueryFnData = unknown, TError = DefaultError, TData = TQueryFnData, TQueryKey extends QueryKey = QueryKey> extends OmitKeyof<UseBaseQueryOptions<TQueryFnData, TError, TData, TQueryFnData, TQueryKey>, 'suspense'> {
}
interface UseInfiniteQueryOptions<TQueryFnData = unknown, TError = DefaultError, TData = InfiniteData<TQueryFnData>, TQueryKey extends QueryKey = QueryKey, TPageParam = unknown> extends OmitKeyof<InfiniteQueryObserverOptions<TQueryFnData, TError, TData, TQueryKey, TPageParam>, 'suspense'> {
    subscribed?: boolean;
}
interface UseMutationOptions<TData = unknown, TError = DefaultError, TVariables = void, TOnMutateResult = unknown> extends OmitKeyof<MutationObserverOptions<TData, TError, TVariables, TOnMutateResult>, '_defaulted'> {
}
type UndefinedInitialDataOptions<TQueryFnData = unknown, TError = DefaultError, TData = TQueryFnData, TQueryKey extends QueryKey = QueryKey> = UseQueryOptions<TQueryFnData, TError, TData, TQueryKey> & {
    initialData?: undefined | InitialDataFunction<NonUndefinedGuard<TQueryFnData>> | NonUndefinedGuard<TQueryFnData>;
};
type UnusedSkipTokenOptions<TQueryFnData = unknown, TError = DefaultError, TData = TQueryFnData, TQueryKey extends QueryKey = QueryKey> = OmitKeyof<UseQueryOptions<TQueryFnData, TError, TData, TQueryKey>, 'queryFn'> & {
    queryFn?: Exclude<UseQueryOptions<TQueryFnData, TError, TData, TQueryKey>['queryFn'], SkipToken | undefined>;
};
type DefinedInitialDataOptions<TQueryFnData = unknown, TError = DefaultError, TData = TQueryFnData, TQueryKey extends QueryKey = QueryKey> = Omit<UseQueryOptions<TQueryFnData, TError, TData, TQueryKey>, 'queryFn'> & {
    initialData: NonUndefinedGuard<TQueryFnData> | (() => NonUndefinedGuard<TQueryFnData>);
    queryFn?: QueryFunction<TQueryFnData, TQueryKey>;
};
type UndefinedInitialDataInfiniteOptions<TQueryFnData, TError = DefaultError, TData = InfiniteData<TQueryFnData>, TQueryKey extends QueryKey = QueryKey, TPageParam = unknown> = UseInfiniteQueryOptions<TQueryFnData, TError, TData, TQueryKey, TPageParam> & {
    initialData?: undefined | NonUndefinedGuard<InfiniteData<TQueryFnData, TPageParam>> | InitialDataFunction<NonUndefinedGuard<InfiniteData<TQueryFnData, TPageParam>>>;
};
type UnusedSkipTokenInfiniteOptions<TQueryFnData, TError = DefaultError, TData = InfiniteData<TQueryFnData>, TQueryKey extends QueryKey = QueryKey, TPageParam = unknown> = OmitKeyof<UseInfiniteQueryOptions<TQueryFnData, TError, TData, TQueryKey, TPageParam>, 'queryFn'> & {
    queryFn?: Exclude<UseInfiniteQueryOptions<TQueryFnData, TError, TData, TQueryKey, TPageParam>['queryFn'], SkipToken | undefined>;
};
type DefinedInitialDataInfiniteOptions<TQueryFnData, TError = DefaultError, TData = InfiniteData<TQueryFnData>, TQueryKey extends QueryKey = QueryKey, TPageParam = unknown> = UseInfiniteQueryOptions<TQueryFnData, TError, TData, TQueryKey, TPageParam> & {
    initialData: NonUndefinedGuard<InfiniteData<TQueryFnData, TPageParam>> | (() => NonUndefinedGuard<InfiniteData<TQueryFnData, TPageParam>>) | undefined;
};
export { InfiniteQueryObserver, MutationObserver, QueryClient, QueryObserver, hashKey, keepPreviousData, skipToken };
export type { DataTag, DefinedInitialDataInfiniteOptions, DefinedInitialDataOptions, InfiniteData, QueryFilters, SkipToken, UndefinedInitialDataInfiniteOptions, UndefinedInitialDataOptions, UnusedSkipTokenInfiniteOptions, UnusedSkipTokenOptions, UseMutationOptions };
