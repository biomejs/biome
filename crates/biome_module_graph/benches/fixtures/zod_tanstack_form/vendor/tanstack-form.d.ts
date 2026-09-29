/**
 * @private
 */
type AnyUpdater = (prev: any) => any;
/**
 * @private
 */
interface ListenerValue<T> {
    prevVal: T;
    currentVal: T;
}
/**
 * @private
 */
type Listener<T> = (value: ListenerValue<T>) => void;

interface StoreOptions<TState, TUpdater extends AnyUpdater = (cb: TState) => TState> {
    /**
     * Replace the default update function with a custom one.
     */
    updateFn?: (previous: TState) => (updater: TUpdater) => TState;
    /**
     * Called when a listener subscribes to the store.
     *
     * @return a function to unsubscribe the listener
     */
    onSubscribe?: (listener: Listener<TState>, store: Store<TState, TUpdater>) => () => void;
    /**
     * Called after the state has been updated, used to derive other state.
     */
    onUpdate?: () => void;
}
declare class Store<TState, TUpdater extends AnyUpdater = (cb: TState) => TState> {
    listeners: Set<Listener<TState>>;
    state: TState;
    prevState: TState;
    options?: StoreOptions<TState, TUpdater>;
    constructor(initialState: TState, options?: StoreOptions<TState, TUpdater>);
    subscribe: (listener: Listener<TState>) => () => void;
    setState: (updater: TUpdater) => void;
}

type UnwrapDerivedOrStore<T> = T extends Derived<infer InnerD> ? InnerD : T extends Store<infer InnerS> ? InnerS : never;
type UnwrapReadonlyDerivedOrStoreArray<TArr extends ReadonlyArray<Derived<any> | Store<any>>> = TArr extends readonly [infer Head, ...infer Tail] ? Head extends Derived<any> | Store<any> ? Tail extends ReadonlyArray<Derived<any> | Store<any>> ? [UnwrapDerivedOrStore<Head>, ...UnwrapReadonlyDerivedOrStoreArray<Tail>] : [] : [] : [];
interface DerivedFnProps<TArr extends ReadonlyArray<Derived<any> | Store<any>> = ReadonlyArray<any>, TUnwrappedArr extends UnwrapReadonlyDerivedOrStoreArray<TArr> = UnwrapReadonlyDerivedOrStoreArray<TArr>> {
    /**
     * `undefined` if it's the first run
     * @privateRemarks this also cannot be typed as TState, as it breaks the inferencing of the function's return type when an argument is used - even with `NoInfer` usage
     */
    prevVal: unknown | undefined;
    prevDepVals: TUnwrappedArr | undefined;
    currDepVals: TUnwrappedArr;
}
interface DerivedOptions<TState, TArr extends ReadonlyArray<Derived<any> | Store<any>> = ReadonlyArray<any>> {
    onSubscribe?: (listener: Listener<TState>, derived: Derived<TState>) => () => void;
    onUpdate?: () => void;
    deps: TArr;
    /**
     * Values of the `deps` from before and after the current invocation of `fn`
     */
    fn: (props: DerivedFnProps<TArr>) => TState;
}
declare class Derived<TState, const TArr extends ReadonlyArray<Derived<any> | Store<any>> = ReadonlyArray<any>> {
    listeners: Set<Listener<TState>>;
    state: TState;
    prevState: TState | undefined;
    options: DerivedOptions<TState, TArr>;
    /**
     * Functions representing the subscriptions. Call a function to cleanup
     * @private
     */
    _subscriptions: Array<() => void>;
    lastSeenDepValues: Array<unknown>;
    getDepVals: () => {
        prevDepVals: unknown[];
        currDepVals: unknown[];
        prevVal: NonNullable<TState> | undefined;
    };
    constructor(options: DerivedOptions<TState, TArr>);
    registerOnGraph(deps?: ReadonlyArray<Derived<any> | Store<any>>): void;
    unregisterFromGraph(deps?: ReadonlyArray<Derived<any> | Store<any>>): void;
    recompute: () => void;
    checkIfRecalculationNeededDeeply: () => void;
    mount: () => () => void;
    subscribe: (listener: Listener<TState>) => () => void;
}

/**
 * @private
 */
type UnwrapOneLevelOfArray<T> = T extends (infer U)[] ? U : T;
type Narrowable = string | number | bigint | boolean;
type NarrowRaw<A> = (A extends [] ? [] : never) | (A extends Narrowable ? A : never) | {
    [K in keyof A]: A[K] extends Function ? A[K] : NarrowRaw<A[K]>;
};
type Try<A1, A2, Catch = never> = A1 extends A2 ? A1 : Catch;
/**
 * @private
 */
type Narrow<A> = Try<A, [], NarrowRaw<A>>;
type ComputeRange<N extends number, Result extends Array<unknown> = []> = Result['length'] extends N ? Result : ComputeRange<N, [...Result, Result['length']]>;
type Index40 = ComputeRange<40>[number];
type IsTuple<T> = T extends readonly any[] & {
    length: infer Length;
} ? Length extends Index40 ? T : never : never;
type AllowedIndexes<Tuple extends ReadonlyArray<any>, Keys extends number = never> = Tuple extends readonly [] ? Keys : Tuple extends readonly [infer _, ...infer Tail] ? AllowedIndexes<Tail, Keys | Tail['length']> : Keys;
type PrefixArrayAccessor<T extends any[], TDepth extends any[]> = {
    [K in keyof T]: `[${number}]${DeepKeys<T[K], TDepth>}`;
}[number];
type PrefixTupleAccessor<T extends any[], TIndex extends number, TDepth extends any[]> = {
    [K in TIndex]: `[${K}]` | `[${K}]${DeepKeys<T[K], TDepth>}`;
}[TIndex];
type PrefixObjectAccessor<T extends object, TDepth extends any[]> = {
    [K in keyof T]-?: K extends string | number ? PrefixFromDepth<K, TDepth> | `${PrefixFromDepth<K, TDepth>}${DeepKeys<T[K], [TDepth]>}` : never;
}[keyof T];
/**
 * The keys of an object or array, deeply nested.
 */
type DeepKeys<T, TDepth extends any[] = []> = TDepth['length'] extends 5 ? never : unknown extends T ? PrefixFromDepth<string, TDepth> : T extends readonly any[] & IsTuple<T> ? PrefixTupleAccessor<T, AllowedIndexes<T>, TDepth> : T extends any[] ? PrefixArrayAccessor<T, [...TDepth, any]> : T extends Date ? never : T extends object ? PrefixObjectAccessor<T, TDepth> : T extends string | number | boolean | bigint ? '' : never;
type PrefixFromDepth<T extends string | number, TDepth extends any[]> = TDepth['length'] extends 0 ? T : `.${T}`;
type Get<T, K extends string> = T extends {
    [Key in K]: infer V;
} ? V : T extends {
    [Key in K]?: infer W;
} ? W | undefined : never;
type ApplyNull<T> = null extends T ? null : never;
type ApplyUndefined<T> = undefined extends T ? undefined : never;
/**
 * Infer the type of a deeply nested property within an object or an array.
 */
type DeepValue<TValue, TAccessor, TDepth extends ReadonlyArray<any> = []> = unknown extends TValue ? TValue : TDepth['length'] extends 10 ? unknown : TValue extends ReadonlyArray<any> ? TAccessor extends `[${infer TBrackets}].${infer TAfter}` ? DeepValue<DeepValue<TValue, TBrackets, [...TDepth, any]>, TAfter, [
    ...TDepth,
    any
]> : TAccessor extends `[${infer TBrackets}]` ? DeepValue<TValue, TBrackets, [...TDepth, any]> : TAccessor extends keyof TValue ? TValue[TAccessor] : TValue[TAccessor & number] : TAccessor extends `${infer TBefore}[${infer TEverythingElse}` ? DeepValue<DeepValue<TValue, TBefore, [...TDepth, any]>, `[${TEverythingElse}`, [
    ...TDepth,
    any
]> : TAccessor extends `[${infer TBrackets}]` ? DeepValue<TValue, TBrackets, [...TDepth, any]> : TAccessor extends `${infer TBefore}.${infer TAfter}` ? DeepValue<DeepValue<TValue, TBefore, [...TDepth, any]>, TAfter, [
    ...TDepth,
    any
]> : TAccessor extends string ? Get<TValue, TAccessor> | (ApplyNull<TValue> | ApplyUndefined<TValue>) : never;

type ValidationError = unknown;
type ValidationSource = 'form' | 'field';
/**
 * "server" is only intended for SSR/SSG validation and should not execute anything
 * @private
 */
type ValidationCause = 'change' | 'blur' | 'submit' | 'mount' | 'server';
/**
 * @private
 */
type ValidationErrorMapKeys = `on${Capitalize<ValidationCause>}`;
/**
 * @private
 */
type ValidationErrorMap<TOnMountReturn = unknown, TOnChangeReturn = unknown, TOnChangeAsyncReturn = unknown, TOnBlurReturn = unknown, TOnBlurAsyncReturn = unknown, TOnSubmitReturn = unknown, TOnSubmitAsyncReturn = unknown, TOnServerReturn = unknown> = {
    onMount?: TOnMountReturn;
    onChange?: TOnChangeReturn | TOnChangeAsyncReturn;
    onBlur?: TOnBlurReturn | TOnBlurAsyncReturn;
    onSubmit?: TOnSubmitReturn | TOnSubmitAsyncReturn;
    onServer?: TOnServerReturn;
};
/**
 * @private
 */
type FormValidationErrorMap<TOnMountReturn = unknown, TOnChangeReturn = unknown, TOnChangeAsyncReturn = unknown, TOnBlurReturn = unknown, TOnBlurAsyncReturn = unknown, TOnSubmitReturn = unknown, TOnSubmitAsyncReturn = unknown, TOnServerReturn = unknown> = {
    onMount?: TOnMountReturn;
    onChange?: TOnChangeReturn | TOnChangeAsyncReturn;
    onBlur?: TOnBlurReturn | TOnBlurAsyncReturn;
    onSubmit?: TOnSubmitReturn | TOnSubmitAsyncReturn;
    onServer?: TOnServerReturn;
};
type FormValidationError<TFormData> = ValidationError | GlobalFormValidationError<TFormData>;
/**
 * @private
 *
 * @example
 * ```tsx
 * {
 *   form: 'This form contains an error',
 *   fields: {
 *     age: "Must be 13 or older to register"
 *   }
 * }
 * ````
 */
type GlobalFormValidationError<TFormData> = {
    form?: ValidationError;
    fields: Partial<Record<DeepKeys<TFormData>, ValidationError>>;
};
/**
 * @private
 */
interface UpdateMetaOptions {
    /**
     * @default false
     */
    dontUpdateMeta?: boolean;
}

type TStandardSchemaValidatorValue<TData> = {
    value: TData;
    validationSource: ValidationSource;
};
declare const standardSchemaValidators: {
    validate({ value, validationSource }: TStandardSchemaValidatorValue<unknown>, schema: StandardSchemaV1): readonly StandardSchemaV1Issue[] | {
        form: {
            [k: string]: StandardSchemaV1Issue[];
        };
        fields: {
            [k: string]: StandardSchemaV1Issue[];
        };
    } | undefined;
    validateAsync({ value, validationSource }: TStandardSchemaValidatorValue<unknown>, schema: StandardSchemaV1): Promise<readonly StandardSchemaV1Issue[] | {
        form: {
            [k: string]: StandardSchemaV1Issue[];
        };
        fields: {
            [k: string]: StandardSchemaV1Issue[];
        };
    } | undefined>;
};
declare const isStandardSchemaValidator: (validator: unknown) => validator is StandardSchemaV1;
/**
 * The Standard Schema interface.
 */
type StandardSchemaV1<Input = unknown, Output = Input> = {
    /**
     * The Standard Schema properties.
     */
    readonly '~standard': StandardSchemaV1Props<Input, Output>;
};
/**
 * The Standard Schema properties interface.
 */
interface StandardSchemaV1Props<Input = unknown, Output = Input> {
    /**
     * The version number of the standard.
     */
    readonly version: 1;
    /**
     * The vendor name of the schema library.
     */
    readonly vendor: string;
    /**
     * Validates unknown input values.
     */
    readonly validate: (value: unknown) => StandardSchemaV1Result<Output> | Promise<StandardSchemaV1Result<Output>>;
    /**
     * Inferred types associated with the schema.
     */
    readonly types?: StandardSchemaV1Types<Input, Output> | undefined;
}
/**
 * The result interface of the validate function.
 */
type StandardSchemaV1Result<Output> = StandardSchemaV1SuccessResult<Output> | StandardSchemaV1FailureResult;
/**
 * The result interface if validation succeeds.
 */
interface StandardSchemaV1SuccessResult<Output> {
    /**
     * The typed output value.
     */
    readonly value: Output;
    /**
     * The non-existent issues.
     */
    readonly issues?: undefined;
}
/**
 * The result interface if validation fails.
 */
interface StandardSchemaV1FailureResult {
    /**
     * The issues of failed validation.
     */
    readonly issues: ReadonlyArray<StandardSchemaV1Issue>;
}
/**
 * The issue interface of the failure output.
 */
interface StandardSchemaV1Issue {
    /**
     * The error message of the issue.
     */
    readonly message: string;
    /**
     * The path of the issue, if any.
     */
    readonly path?: ReadonlyArray<PropertyKey | StandardSchemaV1PathSegment> | undefined;
}
/**
 * The path segment interface of the issue.
 */
interface StandardSchemaV1PathSegment {
    /**
     * The key representing a path segment.
     */
    readonly key: PropertyKey;
}
/**
 * The Standard Schema types interface.
 */
interface StandardSchemaV1Types<Input = unknown, Output = Input> {
    /**
     * The input type of the schema.
     */
    readonly input: Input;
    /**
     * The output type of the schema.
     */
    readonly output: Output;
}

type UpdaterFn<TInput, TOutput = TInput> = (input: TInput) => TOutput;
type Updater<TInput, TOutput = TInput> = TOutput | UpdaterFn<TInput, TOutput>;
/**
 * @private
 */
declare function functionalUpdate<TInput, TOutput = TInput>(updater: Updater<TInput, TOutput>, input: TInput): TOutput;
/**
 * Get a value from an object using a path, including dot notation.
 * @private
 */
declare function getBy(obj: any, path: any): any;
/**
 * Set a value on an object using a path, including dot notation.
 * @private
 */
declare function setBy(obj: any, _path: any, updater: Updater<any>): any;
/**
 * Delete a field on an object using a path, including dot notation.
 * @private
 */
declare function deleteBy(obj: any, _path: any): any;
/**
 * @private
 */
declare function makePathArray(str: string | Array<string | number>): (string | number)[];
/**
 * @private
 */
declare function isNonEmptyArray(obj: any): boolean;
interface AsyncValidatorArrayPartialOptions<T> {
    validators?: T;
    asyncDebounceMs?: number;
}
/**
 * @private
 */
interface AsyncValidator<T> {
    cause: ValidationCause;
    validate: T;
    debounceMs: number;
}
/**
 * @private
 */
declare function getAsyncValidatorArray<T>(cause: ValidationCause, options: AsyncValidatorArrayPartialOptions<T>): T extends FieldValidators<any, any, any, any, any, any, any, any, any, any> ? Array<AsyncValidator<T['onChangeAsync'] | T['onBlurAsync'] | T['onSubmitAsync']>> : T extends FormValidators<any, any, any, any, any, any, any, any> ? Array<AsyncValidator<T['onChangeAsync'] | T['onBlurAsync'] | T['onSubmitAsync']>> : never;
interface SyncValidatorArrayPartialOptions<T> {
    validators?: T;
}
/**
 * @private
 */
interface SyncValidator<T> {
    cause: ValidationCause;
    validate: T;
}
/**
 * @private
 */
declare function getSyncValidatorArray<T>(cause: ValidationCause, options: SyncValidatorArrayPartialOptions<T>): T extends FieldValidators<any, any, any, any, any, any, any, any, any, any> ? Array<SyncValidator<T['onChange'] | T['onBlur'] | T['onSubmit'] | T['onMount']>> : T extends FormValidators<any, any, any, any, any, any, any, any> ? Array<SyncValidator<T['onChange'] | T['onBlur'] | T['onSubmit'] | T['onMount']>> : never;
declare const isGlobalFormValidationError: (error: unknown) => error is GlobalFormValidationError<unknown>;
declare function shallow<T>(objA: T, objB: T): boolean;

/**
 * @private
 */
type FieldErrorMapFromValidator<TFormData, TName extends DeepKeys<TFormData>, TData extends DeepValue<TFormData, TName>, TOnMount extends undefined | FieldValidateOrFn<TFormData, TName, TData>, TOnChange extends undefined | FieldValidateOrFn<TFormData, TName, TData>, TOnChangeAsync extends undefined | FieldAsyncValidateOrFn<TFormData, TName, TData>, TOnBlur extends undefined | FieldValidateOrFn<TFormData, TName, TData>, TOnBlurAsync extends undefined | FieldAsyncValidateOrFn<TFormData, TName, TData>, TOnSubmit extends undefined | FieldValidateOrFn<TFormData, TName, TData>, TOnSubmitAsync extends undefined | FieldAsyncValidateOrFn<TFormData, TName, TData>> = Partial<Record<DeepKeys<TFormData>, ValidationErrorMap<TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync>>>;
/**
 * @private
 */
type FieldValidateFn<TParentData, TName extends DeepKeys<TParentData>, TData extends DeepValue<TParentData, TName> = DeepValue<TParentData, TName>> = (props: {
    value: TData;
    fieldApi: FieldApi<TParentData, TName, TData, any, any, any, any, any, any, any, any, any, any, any, any, any, any, any, any>;
}) => unknown;
/**
 * @private
 */
type FieldValidateOrFn<TParentData, TName extends DeepKeys<TParentData>, TData extends DeepValue<TParentData, TName> = DeepValue<TParentData, TName>> = FieldValidateFn<TParentData, TName, TData> | StandardSchemaV1<TData, unknown>;
type StandardBrandedSchemaV1<T> = T & {
    __standardSchemaV1: true;
};
type UnwrapFormValidateOrFnForInner<TValidateOrFn extends undefined | FormValidateOrFn<any>> = [TValidateOrFn] extends [FormValidateFn<any>] ? ReturnType<TValidateOrFn> : [TValidateOrFn] extends [StandardSchemaV1<infer TOut, any>] ? StandardBrandedSchemaV1<TOut> : undefined;
type UnwrapFieldValidateOrFn<TParentData, TName extends DeepKeys<TParentData>, TValidateOrFn extends undefined | FieldValidateOrFn<any, any, any>, TFormValidateOrFn extends undefined | FormValidateOrFn<any>> = ([TFormValidateOrFn] extends [StandardSchemaV1<any, infer TStandardOut>] ? TName extends keyof TStandardOut ? StandardSchemaV1Issue[] : undefined : undefined) | (UnwrapFormValidateOrFnForInner<TFormValidateOrFn> extends infer TFormValidateVal ? TFormValidateVal extends {
    __standardSchemaV1: true;
} ? [DeepValue<TFormValidateVal, TName>] extends [never] ? undefined : StandardSchemaV1Issue[] : TFormValidateVal extends {
    fields: any;
} ? TName extends keyof TFormValidateVal['fields'] ? TFormValidateVal['fields'][TName] : undefined : undefined : never) | ([TValidateOrFn] extends [FieldValidateFn<any, any, any>] ? ReturnType<TValidateOrFn> : [TValidateOrFn] extends [StandardSchemaV1<any, any>] ? StandardSchemaV1Issue[] : undefined);
/**
 * @private
 */
type FieldValidateAsyncFn<TParentData, TName extends DeepKeys<TParentData>, TData extends DeepValue<TParentData, TName> = DeepValue<TParentData, TName>> = (options: {
    value: TData;
    fieldApi: FieldApi<TParentData, TName, TData, any, any, any, any, any, any, any, any, any, any, any, any, any, any, any, any>;
    signal: AbortSignal;
}) => unknown | Promise<unknown>;
/**
 * @private
 */
type FieldAsyncValidateOrFn<TParentData, TName extends DeepKeys<TParentData>, TData extends DeepValue<TParentData, TName> = DeepValue<TParentData, TName>> = FieldValidateAsyncFn<TParentData, TName, TData> | StandardSchemaV1<TData, unknown>;
type UnwrapFormAsyncValidateOrFnForInner<TValidateOrFn extends undefined | FormAsyncValidateOrFn<any>> = [TValidateOrFn] extends [FormValidateAsyncFn<any>] ? Awaited<ReturnType<TValidateOrFn>> : [TValidateOrFn] extends [StandardSchemaV1<infer TOut, any>] ? StandardBrandedSchemaV1<TOut> : undefined;
type UnwrapFieldAsyncValidateOrFn<TParentData, TName extends DeepKeys<TParentData>, TValidateOrFn extends undefined | FieldAsyncValidateOrFn<any, any, any>, TFormValidateOrFn extends undefined | FormAsyncValidateOrFn<any>> = ([TFormValidateOrFn] extends [StandardSchemaV1<any, infer TStandardOut>] ? TName extends keyof TStandardOut ? StandardSchemaV1Issue[] : undefined : undefined) | (UnwrapFormAsyncValidateOrFnForInner<TFormValidateOrFn> extends infer TFormValidateVal ? TFormValidateVal extends {
    __standardSchemaV1: true;
} ? [DeepValue<TFormValidateVal, TName>] extends [never] ? undefined : StandardSchemaV1Issue[] : TFormValidateVal extends {
    fields: any;
} ? TName extends keyof TFormValidateVal['fields'] ? TFormValidateVal['fields'][TName] : undefined : undefined : never) | ([TValidateOrFn] extends [FieldValidateAsyncFn<any, any, any>] ? Awaited<ReturnType<TValidateOrFn>> : [TValidateOrFn] extends [StandardSchemaV1<any, any>] ? StandardSchemaV1Issue[] : undefined);
/**
 * @private
 */
type FieldListenerFn<TParentData, TName extends DeepKeys<TParentData>, TData extends DeepValue<TParentData, TName> = DeepValue<TParentData, TName>> = (props: {
    value: TData;
    fieldApi: FieldApi<TParentData, TName, TData, any, any, any, any, any, any, any, any, any, any, any, any, any, any, any, any>;
}) => void;
interface FieldValidators<TParentData, TName extends DeepKeys<TParentData>, TData extends DeepValue<TParentData, TName>, TOnMount extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnChange extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnChangeAsync extends undefined | FieldAsyncValidateOrFn<TParentData, TName, TData>, TOnBlur extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnBlurAsync extends undefined | FieldAsyncValidateOrFn<TParentData, TName, TData>, TOnSubmit extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnSubmitAsync extends undefined | FieldAsyncValidateOrFn<TParentData, TName, TData>> {
    /**
     * An optional function, that runs on the mount event of input.
     */
    onMount?: TOnMount;
    /**
     * An optional function, that runs on the change event of input.
     *
     * @example z.string().min(1)
     */
    onChange?: TOnChange;
    /**
     * An optional property similar to `onChange` but async validation
     *
     * @example z.string().refine(async (val) => val.length > 3, { message: 'Testing 123' })
     */
    onChangeAsync?: TOnChangeAsync;
    /**
     * An optional number to represent how long the `onChangeAsync` should wait before running
     *
     * If set to a number larger than 0, will debounce the async validation event by this length of time in milliseconds
     */
    onChangeAsyncDebounceMs?: number;
    /**
     * An optional list of field names that should trigger this field's `onChange` and `onChangeAsync` events when its value changes
     */
    onChangeListenTo?: DeepKeys<TParentData>[];
    /**
     * An optional function, that runs on the blur event of input.
     *
     * @example z.string().min(1)
     */
    onBlur?: TOnBlur;
    /**
     * An optional property similar to `onBlur` but async validation.
     *
     * @example z.string().refine(async (val) => val.length > 3, { message: 'Testing 123' })
     */
    onBlurAsync?: TOnBlurAsync;
    /**
     * An optional number to represent how long the `onBlurAsync` should wait before running
     *
     * If set to a number larger than 0, will debounce the async validation event by this length of time in milliseconds
     */
    onBlurAsyncDebounceMs?: number;
    /**
     * An optional list of field names that should trigger this field's `onBlur` and `onBlurAsync` events when its value changes
     */
    onBlurListenTo?: DeepKeys<TParentData>[];
    /**
     * An optional function, that runs on the submit event of form.
     *
     * @example z.string().min(1)
     */
    onSubmit?: TOnSubmit;
    /**
     * An optional property similar to `onSubmit` but async validation.
     *
     * @example z.string().refine(async (val) => val.length > 3, { message: 'Testing 123' })
     */
    onSubmitAsync?: TOnSubmitAsync;
}
interface FieldListeners<TParentData, TName extends DeepKeys<TParentData>, TData extends DeepValue<TParentData, TName> = DeepValue<TParentData, TName>> {
    onChange?: FieldListenerFn<TParentData, TName, TData>;
    onBlur?: FieldListenerFn<TParentData, TName, TData>;
    onMount?: FieldListenerFn<TParentData, TName, TData>;
    onSubmit?: FieldListenerFn<TParentData, TName, TData>;
}
/**
 * An object type representing the options for a field in a form.
 */
interface FieldOptions<TParentData, TName extends DeepKeys<TParentData>, TData extends DeepValue<TParentData, TName>, TOnMount extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnChange extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnChangeAsync extends undefined | FieldAsyncValidateOrFn<TParentData, TName, TData>, TOnBlur extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnBlurAsync extends undefined | FieldAsyncValidateOrFn<TParentData, TName, TData>, TOnSubmit extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnSubmitAsync extends undefined | FieldAsyncValidateOrFn<TParentData, TName, TData>> {
    /**
     * The field name. The type will be `DeepKeys<TParentData>` to ensure your name is a deep key of the parent dataset.
     */
    name: TName;
    /**
     * An optional default value for the field.
     */
    defaultValue?: NoInfer<TData>;
    /**
     * The default time to debounce async validation if there is not a more specific debounce time passed.
     */
    asyncDebounceMs?: number;
    /**
     * If `true`, always run async validation, even if there are errors emitted during synchronous validation.
     */
    asyncAlways?: boolean;
    /**
     * A list of validators to pass to the field
     */
    validators?: FieldValidators<TParentData, TName, TData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync>;
    /**
     * An optional object with default metadata for the field.
     */
    defaultMeta?: Partial<FieldMeta<TParentData, TName, TData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, any, any, any, any, any, any, any>>;
    /**
     * A list of listeners which attach to the corresponding events
     */
    listeners?: FieldListeners<TParentData, TName, TData>;
    /**
     * Disable the `flat(1)` operation on `field.errors`. This is useful if you want to keep the error structure as is. Not suggested for most use-cases.
     */
    disableErrorFlat?: boolean;
}
/**
 * An object type representing the required options for the FieldApi class.
 */
interface FieldApiOptions<TParentData, TName extends DeepKeys<TParentData>, TData extends DeepValue<TParentData, TName>, TOnMount extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnChange extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnChangeAsync extends undefined | FieldAsyncValidateOrFn<TParentData, TName, TData>, TOnBlur extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnBlurAsync extends undefined | FieldAsyncValidateOrFn<TParentData, TName, TData>, TOnSubmit extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnSubmitAsync extends undefined | FieldAsyncValidateOrFn<TParentData, TName, TData>, TFormOnMount extends undefined | FormValidateOrFn<TParentData>, TFormOnChange extends undefined | FormValidateOrFn<TParentData>, TFormOnChangeAsync extends undefined | FormAsyncValidateOrFn<TParentData>, TFormOnBlur extends undefined | FormValidateOrFn<TParentData>, TFormOnBlurAsync extends undefined | FormAsyncValidateOrFn<TParentData>, TFormOnSubmit extends undefined | FormValidateOrFn<TParentData>, TFormOnSubmitAsync extends undefined | FormAsyncValidateOrFn<TParentData>, TFormOnServer extends undefined | FormAsyncValidateOrFn<TParentData>, TParentSubmitMeta> extends FieldOptions<TParentData, TName, TData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync> {
    form: FormApi<TParentData, TFormOnMount, TFormOnChange, TFormOnChangeAsync, TFormOnBlur, TFormOnBlurAsync, TFormOnSubmit, TFormOnSubmitAsync, TFormOnServer, TParentSubmitMeta>;
}
type FieldMetaBase<TParentData, TName extends DeepKeys<TParentData>, TData extends DeepValue<TParentData, TName>, TOnMount extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnChange extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnChangeAsync extends undefined | FieldAsyncValidateOrFn<TParentData, TName, TData>, TOnBlur extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnBlurAsync extends undefined | FieldAsyncValidateOrFn<TParentData, TName, TData>, TOnSubmit extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnSubmitAsync extends undefined | FieldAsyncValidateOrFn<TParentData, TName, TData>, TFormOnMount extends undefined | FormValidateOrFn<TParentData>, TFormOnChange extends undefined | FormValidateOrFn<TParentData>, TFormOnChangeAsync extends undefined | FormAsyncValidateOrFn<TParentData>, TFormOnBlur extends undefined | FormValidateOrFn<TParentData>, TFormOnBlurAsync extends undefined | FormAsyncValidateOrFn<TParentData>, TFormOnSubmit extends undefined | FormValidateOrFn<TParentData>, TFormOnSubmitAsync extends undefined | FormAsyncValidateOrFn<TParentData>> = {
    /**
     * A flag indicating whether the field has been touched.
     */
    isTouched: boolean;
    /**
     * A flag indicating whether the field has been blurred.
     */
    isBlurred: boolean;
    /**
     * A flag that is `true` if the field's value has been modified by the user. Opposite of `isPristine`.
     */
    isDirty: boolean;
    /**
     * A map of errors related to the field value.
     */
    errorMap: ValidationErrorMap<UnwrapFieldValidateOrFn<TParentData, TName, TOnMount, TFormOnMount>, UnwrapFieldValidateOrFn<TParentData, TName, TOnChange, TFormOnChange>, UnwrapFieldAsyncValidateOrFn<TParentData, TName, TOnChangeAsync, TFormOnChangeAsync>, UnwrapFieldValidateOrFn<TParentData, TName, TOnBlur, TFormOnBlur>, UnwrapFieldAsyncValidateOrFn<TParentData, TName, TOnBlurAsync, TFormOnBlurAsync>, UnwrapFieldValidateOrFn<TParentData, TName, TOnSubmit, TFormOnSubmit>, UnwrapFieldAsyncValidateOrFn<TParentData, TName, TOnSubmitAsync, TFormOnSubmitAsync>>;
    /**
     * A flag indicating whether the field is currently being validated.
     */
    isValidating: boolean;
};
type AnyFieldMetaBase = FieldMetaBase<any, any, any, any, any, any, any, any, any, any, any, any, any, any, any, any, any>;
type FieldMetaDerived<TParentData, TName extends DeepKeys<TParentData>, TData extends DeepValue<TParentData, TName>, TOnMount extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnChange extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnChangeAsync extends undefined | FieldAsyncValidateOrFn<TParentData, TName, TData>, TOnBlur extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnBlurAsync extends undefined | FieldAsyncValidateOrFn<TParentData, TName, TData>, TOnSubmit extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnSubmitAsync extends undefined | FieldAsyncValidateOrFn<TParentData, TName, TData>, TFormOnMount extends undefined | FormValidateOrFn<TParentData>, TFormOnChange extends undefined | FormValidateOrFn<TParentData>, TFormOnChangeAsync extends undefined | FormAsyncValidateOrFn<TParentData>, TFormOnBlur extends undefined | FormValidateOrFn<TParentData>, TFormOnBlurAsync extends undefined | FormAsyncValidateOrFn<TParentData>, TFormOnSubmit extends undefined | FormValidateOrFn<TParentData>, TFormOnSubmitAsync extends undefined | FormAsyncValidateOrFn<TParentData>> = {
    /**
     * An array of errors related to the field value.
     */
    errors: Array<UnwrapOneLevelOfArray<UnwrapFieldValidateOrFn<TParentData, TName, TOnMount, TFormOnMount>> | UnwrapOneLevelOfArray<UnwrapFieldValidateOrFn<TParentData, TName, TOnChange, TFormOnChange>> | UnwrapOneLevelOfArray<UnwrapFieldAsyncValidateOrFn<TParentData, TName, TOnChangeAsync, TFormOnChangeAsync>> | UnwrapOneLevelOfArray<UnwrapFieldValidateOrFn<TParentData, TName, TOnBlur, TFormOnBlur>> | UnwrapOneLevelOfArray<UnwrapFieldAsyncValidateOrFn<TParentData, TName, TOnBlurAsync, TFormOnBlurAsync>> | UnwrapOneLevelOfArray<UnwrapFieldValidateOrFn<TParentData, TName, TOnSubmit, TFormOnSubmit>> | UnwrapOneLevelOfArray<UnwrapFieldAsyncValidateOrFn<TParentData, TName, TOnSubmitAsync, TFormOnSubmitAsync>>>;
    /**
     * A flag that is `true` if the field's value has not been modified by the user. Opposite of `isDirty`.
     */
    isPristine: boolean;
};
type AnyFieldMetaDerived = FieldMetaDerived<any, any, any, any, any, any, any, any, any, any, any, any, any, any, any, any, any>;
/**
 * An object type representing the metadata of a field in a form.
 */
type FieldMeta<TParentData, TName extends DeepKeys<TParentData>, TData extends DeepValue<TParentData, TName>, TOnMount extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnChange extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnChangeAsync extends undefined | FieldAsyncValidateOrFn<TParentData, TName, TData>, TOnBlur extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnBlurAsync extends undefined | FieldAsyncValidateOrFn<TParentData, TName, TData>, TOnSubmit extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnSubmitAsync extends undefined | FieldAsyncValidateOrFn<TParentData, TName, TData>, TFormOnMount extends undefined | FormValidateOrFn<TParentData>, TFormOnChange extends undefined | FormValidateOrFn<TParentData>, TFormOnChangeAsync extends undefined | FormAsyncValidateOrFn<TParentData>, TFormOnBlur extends undefined | FormValidateOrFn<TParentData>, TFormOnBlurAsync extends undefined | FormAsyncValidateOrFn<TParentData>, TFormOnSubmit extends undefined | FormValidateOrFn<TParentData>, TFormOnSubmitAsync extends undefined | FormAsyncValidateOrFn<TParentData>> = FieldMetaBase<TParentData, TName, TData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TFormOnMount, TFormOnChange, TFormOnChangeAsync, TFormOnBlur, TFormOnBlurAsync, TFormOnSubmit, TFormOnSubmitAsync> & FieldMetaDerived<TParentData, TName, TData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TFormOnMount, TFormOnChange, TFormOnChangeAsync, TFormOnBlur, TFormOnBlurAsync, TFormOnSubmit, TFormOnSubmitAsync>;
type AnyFieldMeta = FieldMeta<any, any, any, any, any, any, any, any, any, any, any, any, any, any, any, any, any>;
/**
 * An object type representing the state of a field.
 */
type FieldState<TParentData, TName extends DeepKeys<TParentData>, TData extends DeepValue<TParentData, TName>, TOnMount extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnChange extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnChangeAsync extends undefined | FieldAsyncValidateOrFn<TParentData, TName, TData>, TOnBlur extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnBlurAsync extends undefined | FieldAsyncValidateOrFn<TParentData, TName, TData>, TOnSubmit extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnSubmitAsync extends undefined | FieldAsyncValidateOrFn<TParentData, TName, TData>, TFormOnMount extends undefined | FormValidateOrFn<TParentData>, TFormOnChange extends undefined | FormValidateOrFn<TParentData>, TFormOnChangeAsync extends undefined | FormAsyncValidateOrFn<TParentData>, TFormOnBlur extends undefined | FormValidateOrFn<TParentData>, TFormOnBlurAsync extends undefined | FormAsyncValidateOrFn<TParentData>, TFormOnSubmit extends undefined | FormValidateOrFn<TParentData>, TFormOnSubmitAsync extends undefined | FormAsyncValidateOrFn<TParentData>> = {
    /**
     * The current value of the field.
     */
    value: TData;
    /**
     * The current metadata of the field.
     */
    meta: FieldMeta<TParentData, TName, TData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TFormOnMount, TFormOnChange, TFormOnChangeAsync, TFormOnBlur, TFormOnBlurAsync, TFormOnSubmit, TFormOnSubmitAsync>;
};
/**
 * @public
 *
 * A type representing the Field API with all generics set to `any` for convenience.
 */
type AnyFieldApi = FieldApi<any, any, any, any, any, any, any, any, any, any, any, any, any, any, any, any, any, any, any>;
/**
 * A class representing the API for managing a form field.
 *
 * Normally, you will not need to create a new `FieldApi` instance directly.
 * Instead, you will use a framework hook/function like `useField` or `createField`
 * to create a new instance for you that uses your framework's reactivity model.
 * However, if you need to create a new instance manually, you can do so by calling
 * the `new FieldApi` constructor.
 */
declare class FieldApi<TParentData, TName extends DeepKeys<TParentData>, TData extends DeepValue<TParentData, TName>, TOnMount extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnChange extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnChangeAsync extends undefined | FieldAsyncValidateOrFn<TParentData, TName, TData>, TOnBlur extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnBlurAsync extends undefined | FieldAsyncValidateOrFn<TParentData, TName, TData>, TOnSubmit extends undefined | FieldValidateOrFn<TParentData, TName, TData>, TOnSubmitAsync extends undefined | FieldAsyncValidateOrFn<TParentData, TName, TData>, TFormOnMount extends undefined | FormValidateOrFn<TParentData>, TFormOnChange extends undefined | FormValidateOrFn<TParentData>, TFormOnChangeAsync extends undefined | FormAsyncValidateOrFn<TParentData>, TFormOnBlur extends undefined | FormValidateOrFn<TParentData>, TFormOnBlurAsync extends undefined | FormAsyncValidateOrFn<TParentData>, TFormOnSubmit extends undefined | FormValidateOrFn<TParentData>, TFormOnSubmitAsync extends undefined | FormAsyncValidateOrFn<TParentData>, TFormOnServer extends undefined | FormAsyncValidateOrFn<TParentData>, TParentSubmitMeta> {
    /**
     * A reference to the form API instance.
     */
    form: FieldApiOptions<TParentData, TName, TData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TFormOnMount, TFormOnChange, TFormOnChangeAsync, TFormOnBlur, TFormOnBlurAsync, TFormOnSubmit, TFormOnSubmitAsync, TFormOnServer, TParentSubmitMeta>['form'];
    /**
     * The field name.
     */
    name: DeepKeys<TParentData>;
    /**
     * The field options.
     */
    options: FieldApiOptions<TParentData, TName, TData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TFormOnMount, TFormOnChange, TFormOnChangeAsync, TFormOnBlur, TFormOnBlurAsync, TFormOnSubmit, TFormOnSubmitAsync, TFormOnServer, TParentSubmitMeta>;
    /**
     * The field state store.
     */
    store: Derived<FieldState<TParentData, TName, TData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TFormOnMount, TFormOnChange, TFormOnChangeAsync, TFormOnBlur, TFormOnBlurAsync, TFormOnSubmit, TFormOnSubmitAsync>>;
    /**
     * The current field state.
     */
    get state(): FieldState<TParentData, TName, TData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TFormOnMount, TFormOnChange, TFormOnChangeAsync, TFormOnBlur, TFormOnBlurAsync, TFormOnSubmit, TFormOnSubmitAsync>;
    timeoutIds: Record<ValidationCause, ReturnType<typeof setTimeout> | null>;
    /**
     * Initializes a new `FieldApi` instance.
     */
    constructor(opts: FieldApiOptions<TParentData, TName, TData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TFormOnMount, TFormOnChange, TFormOnChangeAsync, TFormOnBlur, TFormOnBlurAsync, TFormOnSubmit, TFormOnSubmitAsync, TFormOnServer, TParentSubmitMeta>);
    /**
     * @private
     */
    runValidator<TValue extends TStandardSchemaValidatorValue<TData> & {
        fieldApi: AnyFieldApi;
    }, TType extends 'validate' | 'validateAsync'>(props: {
        validate: TType extends 'validate' ? FieldValidateOrFn<any, any, any> : FieldAsyncValidateOrFn<any, any, any>;
        value: TValue;
        type: TType;
    }): unknown;
    /**
     * Mounts the field instance to the form.
     */
    mount: () => () => void;
    /**
     * Updates the field instance with new options.
     */
    update: (opts: FieldApiOptions<TParentData, TName, TData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TFormOnMount, TFormOnChange, TFormOnChangeAsync, TFormOnBlur, TFormOnBlurAsync, TFormOnSubmit, TFormOnSubmitAsync, TFormOnServer, TParentSubmitMeta>) => void;
    /**
     * Gets the current field value.
     * @deprecated Use `field.state.value` instead.
     */
    getValue: () => TData;
    /**
     * Sets the field value and run the `change` validator.
     */
    setValue: (updater: Updater<TData>, options?: UpdateMetaOptions) => void;
    getMeta: () => FieldMeta<TParentData, TName, TData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TFormOnMount, TFormOnChange, TFormOnChangeAsync, TFormOnBlur, TFormOnBlurAsync, TFormOnSubmit, TFormOnSubmitAsync>;
    /**
     * Sets the field metadata.
     */
    setMeta: (updater: Updater<FieldMeta<TParentData, TName, TData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TFormOnMount, TFormOnChange, TFormOnChangeAsync, TFormOnBlur, TFormOnBlurAsync, TFormOnSubmit, TFormOnSubmitAsync>>) => void;
    /**
     * Gets the field information object.
     */
    getInfo: () => FieldInfo<TParentData>;
    /**
     * Pushes a new value to the field.
     */
    pushValue: (value: TData extends any[] ? TData[number] : never, opts?: UpdateMetaOptions) => void;
    /**
     * Inserts a value at the specified index, shifting the subsequent values to the right.
     */
    insertValue: (index: number, value: TData extends any[] ? TData[number] : never, opts?: UpdateMetaOptions) => Promise<void>;
    /**
     * Replaces a value at the specified index.
     */
    replaceValue: (index: number, value: TData extends any[] ? TData[number] : never, opts?: UpdateMetaOptions) => Promise<void>;
    /**
     * Removes a value at the specified index.
     */
    removeValue: (index: number, opts?: UpdateMetaOptions) => Promise<void>;
    /**
     * Swaps the values at the specified indices.
     */
    swapValues: (aIndex: number, bIndex: number, opts?: UpdateMetaOptions) => void;
    /**
     * Moves the value at the first specified index to the second specified index.
     */
    moveValue: (aIndex: number, bIndex: number, opts?: UpdateMetaOptions) => void;
    /**
     * @private
     */
    getLinkedFields: (cause: ValidationCause) => AnyFieldApi[];
    /**
     * @private
     */
    validateSync: (cause: ValidationCause, errorFromForm: ValidationErrorMap) => {
        hasErrored: boolean;
    };
    /**
     * @private
     */
    validateAsync: (cause: ValidationCause, formValidationResultPromise: Promise<FieldErrorMapFromValidator<TParentData, TName, TData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync>>) => Promise<unknown[]>;
    /**
     * Validates the field value.
     */
    validate: (cause: ValidationCause, opts?: {
        skipFormValidation?: boolean;
    }) => ValidationError[] | Promise<ValidationError[]>;
    /**
     * Handles the change event.
     */
    handleChange: (updater: Updater<TData>) => void;
    /**
     * Handles the blur event.
     */
    handleBlur: () => void;
    /**
     * Updates the field's errorMap
     */
    setErrorMap(errorMap: ValidationErrorMap): void;
}

/**
 * @private
 */
type FormErrorMapFromValidator<TFormData, TOnMount extends undefined | FormValidateOrFn<TFormData>, TOnChange extends undefined | FormValidateOrFn<TFormData>, TOnChangeAsync extends undefined | FormAsyncValidateOrFn<TFormData>, TOnBlur extends undefined | FormValidateOrFn<TFormData>, TOnBlurAsync extends undefined | FormAsyncValidateOrFn<TFormData>, TOnSubmit extends undefined | FormValidateOrFn<TFormData>, TOnSubmitAsync extends undefined | FormAsyncValidateOrFn<TFormData>> = Partial<Record<DeepKeys<TFormData>, ValidationErrorMap<TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync>>>;
type FormValidateFn<TFormData> = (props: {
    value: TFormData;
    formApi: FormApi<TFormData, any, any, any, any, any, any, any, any, any>;
}) => unknown;
/**
 * @private
 */
type FormValidateOrFn<TFormData> = FormValidateFn<TFormData> | StandardSchemaV1<TFormData, unknown>;
type UnwrapFormValidateOrFn<TValidateOrFn extends undefined | FormValidateOrFn<any>> = [TValidateOrFn] extends [FormValidateFn<any>] ? ReturnType<TValidateOrFn> : [TValidateOrFn] extends [StandardSchemaV1<any, any>] ? Record<string, StandardSchemaV1Issue[]> : undefined;
/**
 * @private
 */
type FormValidateAsyncFn<TFormData> = (props: {
    value: TFormData;
    formApi: FormApi<TFormData, any, any, any, any, any, any, any, any, any>;
    signal: AbortSignal;
}) => unknown | Promise<unknown>;
type FormValidator<TFormData, TType, TFn = unknown> = {
    validate(options: {
        value: TType;
    }, fn: TFn): ValidationError;
    validateAsync(options: {
        value: TType;
    }, fn: TFn): Promise<FormValidationError<TFormData>>;
};
/**
 * @private
 */
type FormAsyncValidateOrFn<TFormData> = FormValidateAsyncFn<TFormData> | StandardSchemaV1<TFormData, unknown>;
type UnwrapFormAsyncValidateOrFn<TValidateOrFn extends undefined | FormAsyncValidateOrFn<any>> = [TValidateOrFn] extends [FormValidateAsyncFn<any>] ? Awaited<ReturnType<TValidateOrFn>> : [TValidateOrFn] extends [StandardSchemaV1<any, any>] ? Record<string, StandardSchemaV1Issue[]> : undefined;
interface FormValidators<TFormData, TOnMount extends undefined | FormValidateOrFn<TFormData>, TOnChange extends undefined | FormValidateOrFn<TFormData>, TOnChangeAsync extends undefined | FormAsyncValidateOrFn<TFormData>, TOnBlur extends undefined | FormValidateOrFn<TFormData>, TOnBlurAsync extends undefined | FormAsyncValidateOrFn<TFormData>, TOnSubmit extends undefined | FormValidateOrFn<TFormData>, TOnSubmitAsync extends undefined | FormAsyncValidateOrFn<TFormData>> {
    /**
     * Optional function that fires as soon as the component mounts.
     */
    onMount?: TOnMount;
    /**
     * Optional function that checks the validity of your data whenever a value changes
     */
    onChange?: TOnChange;
    /**
     * Optional onChange asynchronous counterpart to onChange. Useful for more complex validation logic that might involve server requests.
     */
    onChangeAsync?: TOnChangeAsync;
    /**
     * The default time in milliseconds that if set to a number larger than 0, will debounce the async validation event by this length of time in milliseconds.
     */
    onChangeAsyncDebounceMs?: number;
    /**
     * Optional function that validates the form data when a field loses focus, returns a `FormValidationError`
     */
    onBlur?: TOnBlur;
    /**
     * Optional onBlur asynchronous validation method for when a field loses focus returns a ` FormValidationError` or a promise of `Promise<FormValidationError>`
     */
    onBlurAsync?: TOnBlurAsync;
    /**
     * The default time in milliseconds that if set to a number larger than 0, will debounce the async validation event by this length of time in milliseconds.
     */
    onBlurAsyncDebounceMs?: number;
    onSubmit?: TOnSubmit;
    onSubmitAsync?: TOnSubmitAsync;
}
/**
 * @private
 */
interface FormTransform<TFormData, TOnMount extends undefined | FormValidateOrFn<TFormData>, TOnChange extends undefined | FormValidateOrFn<TFormData>, TOnChangeAsync extends undefined | FormAsyncValidateOrFn<TFormData>, TOnBlur extends undefined | FormValidateOrFn<TFormData>, TOnBlurAsync extends undefined | FormAsyncValidateOrFn<TFormData>, TOnSubmit extends undefined | FormValidateOrFn<TFormData>, TOnSubmitAsync extends undefined | FormAsyncValidateOrFn<TFormData>, TOnServer extends undefined | FormAsyncValidateOrFn<TFormData>, TSubmitMeta = never> {
    fn: (formBase: FormApi<TFormData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TOnServer, TSubmitMeta>) => FormApi<TFormData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TOnServer, TSubmitMeta>;
    deps: unknown[];
}
/**
 * An object representing the options for a form.
 */
interface FormOptions<TFormData, TOnMount extends undefined | FormValidateOrFn<TFormData>, TOnChange extends undefined | FormValidateOrFn<TFormData>, TOnChangeAsync extends undefined | FormAsyncValidateOrFn<TFormData>, TOnBlur extends undefined | FormValidateOrFn<TFormData>, TOnBlurAsync extends undefined | FormAsyncValidateOrFn<TFormData>, TOnSubmit extends undefined | FormValidateOrFn<TFormData>, TOnSubmitAsync extends undefined | FormAsyncValidateOrFn<TFormData>, TOnServer extends undefined | FormAsyncValidateOrFn<TFormData>, TSubmitMeta = never> {
    /**
     * Set initial values for your form.
     */
    defaultValues?: TFormData;
    /**
     * The default state for the form.
     */
    defaultState?: Partial<FormState<TFormData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TOnServer>>;
    /**
     * If true, always run async validation, even when sync validation has produced an error. Defaults to undefined.
     */
    asyncAlways?: boolean;
    /**
     * Optional time in milliseconds if you want to introduce a delay before firing off an async action.
     */
    asyncDebounceMs?: number;
    /**
     * A list of validators to pass to the form
     */
    validators?: FormValidators<TFormData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync>;
    /**
     * onSubmitMeta, the data passed from the handleSubmit handler, to the onSubmit function props
     */
    onSubmitMeta?: TSubmitMeta;
    /**
     * A function to be called when the form is submitted, what should happen once the user submits a valid form returns `any` or a promise `Promise<any>`
     */
    onSubmit?: (props: {
        value: TFormData;
        formApi: FormApi<TFormData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TOnServer, TSubmitMeta>;
        meta: TSubmitMeta;
    }) => any | Promise<any>;
    /**
     * Specify an action for scenarios where the user tries to submit an invalid form.
     */
    onSubmitInvalid?: (props: {
        value: TFormData;
        formApi: FormApi<TFormData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TOnServer, TSubmitMeta>;
    }) => void;
    transform?: FormTransform<TFormData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TOnServer, TSubmitMeta>;
}
/**
 * An object representing the validation metadata for a field. Not intended for public usage.
 */
type ValidationMeta = {
    /**
     * An abort controller stored in memory to cancel previous async validation attempts.
     */
    lastAbortController: AbortController;
};
/**
 * An object representing the field information for a specific field within the form.
 */
type FieldInfo<TFormData> = {
    /**
     * An instance of the FieldAPI.
     */
    instance: FieldApi<TFormData, any, any, any, any, any, any, any, any, any, any, any, any, any, any, any, any, any, any> | null;
    /**
     * A record of field validation internal handling.
     */
    validationMetaMap: Record<ValidationErrorMapKeys, ValidationMeta | undefined>;
};
/**
 * An object representing the current state of the form.
 */
type BaseFormState<TFormData, TOnMount extends undefined | FormValidateOrFn<TFormData>, TOnChange extends undefined | FormValidateOrFn<TFormData>, TOnChangeAsync extends undefined | FormAsyncValidateOrFn<TFormData>, TOnBlur extends undefined | FormValidateOrFn<TFormData>, TOnBlurAsync extends undefined | FormAsyncValidateOrFn<TFormData>, TOnSubmit extends undefined | FormValidateOrFn<TFormData>, TOnSubmitAsync extends undefined | FormAsyncValidateOrFn<TFormData>, TOnServer extends undefined | FormAsyncValidateOrFn<TFormData>> = {
    /**
     * The current values of the form fields.
     */
    values: TFormData;
    /**
     * The error map for the form itself.
     */
    errorMap: FormValidationErrorMap<UnwrapFormValidateOrFn<TOnMount>, UnwrapFormValidateOrFn<TOnChange>, UnwrapFormAsyncValidateOrFn<TOnChangeAsync>, UnwrapFormValidateOrFn<TOnBlur>, UnwrapFormAsyncValidateOrFn<TOnBlurAsync>, UnwrapFormValidateOrFn<TOnSubmit>, UnwrapFormAsyncValidateOrFn<TOnSubmitAsync>, UnwrapFormAsyncValidateOrFn<TOnServer>>;
    /**
     * An internal mechanism used for keeping track of validation logic in a form.
     */
    validationMetaMap: Record<ValidationErrorMapKeys, ValidationMeta | undefined>;
    /**
     * A record of field metadata for each field in the form, not including the derived properties, like `errors` and such
     */
    fieldMetaBase: Record<DeepKeys<TFormData>, AnyFieldMetaBase>;
    /**
     * A boolean indicating if the form is currently in the process of being submitted after `handleSubmit` is called.
     *
     * Goes back to `false` when submission completes for one of the following reasons:
     * - the validation step returned errors.
     * - the `onSubmit` function has completed.
     *
     * Note: if you're running async operations in your `onSubmit` function make sure to await them to ensure `isSubmitting` is set to `false` only when the async operation completes.
     *
     * This is useful for displaying loading indicators or disabling form inputs during submission.
     *
     */
    isSubmitting: boolean;
    /**
     * A boolean indicating if the form has been submitted.
     */
    isSubmitted: boolean;
    /**
     * A boolean indicating if the form or any of its fields are currently validating.
     */
    isValidating: boolean;
    /**
     * A counter for tracking the number of submission attempts.
     */
    submissionAttempts: number;
    /**
     * A boolean indicating if the last submission was successful.
     */
    isSubmitSuccessful: boolean;
    /**
     * @private, used to force a re-evaluation of the form state when options change
     */
    _force_re_eval?: boolean;
};
type DerivedFormState<TFormData, TOnMount extends undefined | FormValidateOrFn<TFormData>, TOnChange extends undefined | FormValidateOrFn<TFormData>, TOnChangeAsync extends undefined | FormAsyncValidateOrFn<TFormData>, TOnBlur extends undefined | FormValidateOrFn<TFormData>, TOnBlurAsync extends undefined | FormAsyncValidateOrFn<TFormData>, TOnSubmit extends undefined | FormValidateOrFn<TFormData>, TOnSubmitAsync extends undefined | FormAsyncValidateOrFn<TFormData>, TOnServer extends undefined | FormAsyncValidateOrFn<TFormData>> = {
    /**
     * A boolean indicating if the form is currently validating.
     */
    isFormValidating: boolean;
    /**
     * A boolean indicating if the form is valid.
     */
    isFormValid: boolean;
    /**
     * The error array for the form itself.
     */
    errors: Array<UnwrapFormValidateOrFn<TOnMount> | UnwrapFormValidateOrFn<TOnChange> | UnwrapFormAsyncValidateOrFn<TOnChangeAsync> | UnwrapFormValidateOrFn<TOnBlur> | UnwrapFormAsyncValidateOrFn<TOnBlurAsync> | UnwrapFormValidateOrFn<TOnSubmit> | UnwrapFormAsyncValidateOrFn<TOnSubmitAsync> | UnwrapFormAsyncValidateOrFn<TOnServer>>;
    /**
     * A boolean indicating if any of the form fields are currently validating.
     */
    isFieldsValidating: boolean;
    /**
     * A boolean indicating if all the form fields are valid.
     */
    isFieldsValid: boolean;
    /**
     * A boolean indicating if any of the form fields have been touched.
     */
    isTouched: boolean;
    /**
     * A boolean indicating if any of the form fields have been blurred.
     */
    isBlurred: boolean;
    /**
     * A boolean indicating if any of the form's fields' values have been modified by the user. `True` if the user have modified at least one of the fields. Opposite of `isPristine`.
     */
    isDirty: boolean;
    /**
     * A boolean indicating if none of the form's fields' values have been modified by the user. `True` if the user have not modified any of the fields. Opposite of `isDirty`.
     */
    isPristine: boolean;
    /**
     * A boolean indicating if the form and all its fields are valid.
     */
    isValid: boolean;
    /**
     * A boolean indicating if the form can be submitted based on its current state.
     */
    canSubmit: boolean;
    /**
     * A record of field metadata for each field in the form.
     */
    fieldMeta: Record<DeepKeys<TFormData>, AnyFieldMeta>;
};
type FormState<TFormData, TOnMount extends undefined | FormValidateOrFn<TFormData>, TOnChange extends undefined | FormValidateOrFn<TFormData>, TOnChangeAsync extends undefined | FormAsyncValidateOrFn<TFormData>, TOnBlur extends undefined | FormValidateOrFn<TFormData>, TOnBlurAsync extends undefined | FormAsyncValidateOrFn<TFormData>, TOnSubmit extends undefined | FormValidateOrFn<TFormData>, TOnSubmitAsync extends undefined | FormAsyncValidateOrFn<TFormData>, TOnServer extends undefined | FormAsyncValidateOrFn<TFormData>> = BaseFormState<TFormData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TOnServer> & DerivedFormState<TFormData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TOnServer>;
type AnyFormState = FormState<any, any, any, any, any, any, any, any, any>;
/**
 * @public
 *
 * A type representing the Form API with all generics set to `any` for convenience.
 */
type AnyFormApi = FormApi<any, any, any, any, any, any, any, any, any, any>;
/**
 * A class representing the Form API. It handles the logic and interactions with the form state.
 *
 * Normally, you will not need to create a new `FormApi` instance directly. Instead, you will use a framework
 * hook/function like `useForm` or `createForm` to create a new instance for you that uses your framework's reactivity model.
 * However, if you need to create a new instance manually, you can do so by calling the `new FormApi` constructor.
 */
declare class FormApi<TFormData, TOnMount extends undefined | FormValidateOrFn<TFormData>, TOnChange extends undefined | FormValidateOrFn<TFormData>, TOnChangeAsync extends undefined | FormAsyncValidateOrFn<TFormData>, TOnBlur extends undefined | FormValidateOrFn<TFormData>, TOnBlurAsync extends undefined | FormAsyncValidateOrFn<TFormData>, TOnSubmit extends undefined | FormValidateOrFn<TFormData>, TOnSubmitAsync extends undefined | FormAsyncValidateOrFn<TFormData>, TOnServer extends undefined | FormAsyncValidateOrFn<TFormData>, TSubmitMeta = never> {
    /**
     * The options for the form.
     */
    options: FormOptions<TFormData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TOnServer, TSubmitMeta>;
    baseStore: Store<BaseFormState<TFormData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TOnServer>>;
    fieldMetaDerived: Derived<Record<DeepKeys<TFormData>, AnyFieldMeta>>;
    store: Derived<FormState<TFormData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TOnServer>>;
    /**
     * A record of field information for each field in the form.
     */
    fieldInfo: Record<DeepKeys<TFormData>, FieldInfo<TFormData>>;
    get state(): FormState<TFormData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TOnServer>;
    /**
     * @private
     */
    prevTransformArray: unknown[];
    /**
     * @private Persistent store of all field validation errors originating from form-level validators.
     * Maintains the cumulative state across validation cycles, including cleared errors (undefined values).
     * This map preserves the complete validation state for all fields.
     */
    cumulativeFieldsErrorMap: FormErrorMapFromValidator<TFormData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync>;
    /**
     * Constructs a new `FormApi` instance with the given form options.
     */
    constructor(opts?: FormOptions<TFormData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TOnServer, TSubmitMeta>);
    /**
     * @private
     */
    runValidator<TValue extends TStandardSchemaValidatorValue<TFormData> & {
        formApi: AnyFormApi;
    }, TType extends 'validate' | 'validateAsync'>(props: {
        validate: TType extends 'validate' ? FormValidateOrFn<TFormData> : FormAsyncValidateOrFn<TFormData>;
        value: TValue;
        type: TType;
    }): unknown;
    mount: () => () => void;
    /**
     * Updates the form options and form state.
     */
    update: (options?: FormOptions<TFormData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TOnServer, TSubmitMeta>) => void;
    /**
     * Resets the form state to the default values.
     * If values are provided, the form will be reset to those values instead and the default values will be updated.
     *
     * @param values - Optional values to reset the form to.
     * @param opts - Optional options to control the reset behavior.
     */
    reset: (values?: TFormData, opts?: {
        keepDefaultValues?: boolean;
    }) => void;
    /**
     * Validates all fields using the correct handlers for a given validation cause.
     */
    validateAllFields: (cause: ValidationCause) => Promise<unknown[]>;
    /**
     * Validates the children of a specified array in the form starting from a given index until the end using the correct handlers for a given validation type.
     */
    validateArrayFieldsStartingFrom: <TField extends DeepKeys<TFormData>>(field: TField, index: number, cause: ValidationCause) => Promise<unknown[]>;
    /**
     * Validates a specified field in the form using the correct handlers for a given validation type.
     */
    validateField: <TField extends DeepKeys<TFormData>>(field: TField, cause: ValidationCause) => unknown[] | Promise<unknown[]>;
    /**
     * TODO: This code is copied from FieldApi, we should refactor to share
     * @private
     */
    validateSync: (cause: ValidationCause) => {
        hasErrored: boolean;
        fieldsErrorMap: FormErrorMapFromValidator<TFormData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync>;
    };
    /**
     * @private
     */
    validateAsync: (cause: ValidationCause) => Promise<FormErrorMapFromValidator<TFormData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync>>;
    /**
     * @private
     */
    validate: (cause: ValidationCause) => FormErrorMapFromValidator<TFormData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync> | Promise<FormErrorMapFromValidator<TFormData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync>>;
    /**
     * Handles the form submission, performs validation, and calls the appropriate onSubmit or onInvalidSubmit callbacks.
     */
    handleSubmit(): Promise<void>;
    handleSubmit(submitMeta: TSubmitMeta): Promise<void>;
    /**
     * Gets the value of the specified field.
     */
    getFieldValue: <TField extends DeepKeys<TFormData>>(field: TField) => DeepValue<TFormData, TField>;
    /**
     * Gets the metadata of the specified field.
     */
    getFieldMeta: <TField extends DeepKeys<TFormData>>(field: TField) => AnyFieldMeta | undefined;
    /**
     * Gets the field info of the specified field.
     */
    getFieldInfo: <TField extends DeepKeys<TFormData>>(field: TField) => FieldInfo<TFormData>;
    /**
     * Updates the metadata of the specified field.
     */
    setFieldMeta: <TField extends DeepKeys<TFormData>>(field: TField, updater: Updater<AnyFieldMeta>) => void;
    resetFieldMeta: <TField extends DeepKeys<TFormData>>(fieldMeta: Record<TField, AnyFieldMeta>) => Record<TField, AnyFieldMeta>;
    /**
     * Sets the value of the specified field and optionally updates the touched state.
     */
    setFieldValue: <TField extends DeepKeys<TFormData>>(field: TField, updater: Updater<DeepValue<TFormData, TField>>, opts?: UpdateMetaOptions) => void;
    deleteField: <TField extends DeepKeys<TFormData>>(field: TField) => void;
    /**
     * Pushes a value into an array field.
     */
    pushFieldValue: <TField extends DeepKeys<TFormData>>(field: TField, value: DeepValue<TFormData, TField> extends any[] ? DeepValue<TFormData, TField>[number] : never, opts?: UpdateMetaOptions) => void;
    insertFieldValue: <TField extends DeepKeys<TFormData>>(field: TField, index: number, value: DeepValue<TFormData, TField> extends any[] ? DeepValue<TFormData, TField>[number] : never, opts?: UpdateMetaOptions) => Promise<void>;
    /**
     * Replaces a value into an array field at the specified index.
     */
    replaceFieldValue: <TField extends DeepKeys<TFormData>>(field: TField, index: number, value: DeepValue<TFormData, TField> extends any[] ? DeepValue<TFormData, TField>[number] : never, opts?: UpdateMetaOptions) => Promise<void>;
    /**
     * Removes a value from an array field at the specified index.
     */
    removeFieldValue: <TField extends DeepKeys<TFormData>>(field: TField, index: number, opts?: UpdateMetaOptions) => Promise<void>;
    /**
     * Swaps the values at the specified indices within an array field.
     */
    swapFieldValues: <TField extends DeepKeys<TFormData>>(field: TField, index1: number, index2: number, opts?: UpdateMetaOptions) => void;
    /**
     * Moves the value at the first specified index to the second specified index within an array field.
     */
    moveFieldValues: <TField extends DeepKeys<TFormData>>(field: TField, index1: number, index2: number, opts?: UpdateMetaOptions) => void;
    /**
     * Updates the form's errorMap
     */
    setErrorMap(errorMap: ValidationErrorMap<TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync>): void;
    /**
     * Returns form and field level errors
     */
    getAllErrors: () => {
        form: {
            errors: Array<UnwrapFormValidateOrFn<TOnMount> | UnwrapFormValidateOrFn<TOnChange> | UnwrapFormAsyncValidateOrFn<TOnChangeAsync> | UnwrapFormValidateOrFn<TOnBlur> | UnwrapFormAsyncValidateOrFn<TOnBlurAsync> | UnwrapFormValidateOrFn<TOnSubmit> | UnwrapFormAsyncValidateOrFn<TOnSubmitAsync> | UnwrapFormAsyncValidateOrFn<TOnServer>>;
            errorMap: FormValidationErrorMap<UnwrapFormValidateOrFn<TOnMount>, UnwrapFormValidateOrFn<TOnChange>, UnwrapFormAsyncValidateOrFn<TOnChangeAsync>, UnwrapFormValidateOrFn<TOnBlur>, UnwrapFormAsyncValidateOrFn<TOnBlurAsync>, UnwrapFormValidateOrFn<TOnSubmit>, UnwrapFormAsyncValidateOrFn<TOnSubmitAsync>, UnwrapFormAsyncValidateOrFn<TOnServer>>;
        };
        fields: Record<DeepKeys<TFormData>, {
            errors: ValidationError[];
            errorMap: ValidationErrorMap;
        }>;
    };
}

/**
 * @private
 */
declare function mutateMergeDeep(target: object | null | undefined, source: object | null | undefined): object;
declare function mergeForm<TFormData>(baseForm: FormApi<NoInfer<TFormData>, any, any, any, any, any, any, any, any, any>, state: Partial<FormApi<TFormData, any, any, any, any, any, any, any, any, any>['state']>): FormApi<NoInfer<TFormData>, any, any, any, any, any, any, any, any, any>;

declare function formOptions<TFormData, TOnMount extends undefined | FormValidateOrFn<TFormData>, TOnChange extends undefined | FormValidateOrFn<TFormData>, TOnChangeAsync extends undefined | FormAsyncValidateOrFn<TFormData>, TOnBlur extends undefined | FormValidateOrFn<TFormData>, TOnBlurAsync extends undefined | FormAsyncValidateOrFn<TFormData>, TOnSubmit extends undefined | FormValidateOrFn<TFormData>, TOnSubmitAsync extends undefined | FormAsyncValidateOrFn<TFormData>, TOnServer extends undefined | FormAsyncValidateOrFn<TFormData>, TSubmitMeta>(defaultOpts?: FormOptions<TFormData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TOnServer, TSubmitMeta>): FormOptions<TFormData, TOnMount, TOnChange, TOnChangeAsync, TOnBlur, TOnBlurAsync, TOnSubmit, TOnSubmitAsync, TOnServer, TSubmitMeta> | undefined;

export { type AnyFieldApi, type AnyFieldMeta, type AnyFieldMetaBase, type AnyFieldMetaDerived, type AnyFormApi, type AnyFormState, type AsyncValidator, type BaseFormState, type DeepKeys, type DeepValue, type DerivedFormState, FieldApi, type FieldApiOptions, type FieldAsyncValidateOrFn, type FieldInfo, type FieldListenerFn, type FieldListeners, type FieldMeta, type FieldMetaBase, type FieldMetaDerived, type FieldOptions, type FieldState, type FieldValidateAsyncFn, type FieldValidateFn, type FieldValidateOrFn, type FieldValidators, FormApi, type FormAsyncValidateOrFn, type FormOptions, type FormState, type FormTransform, type FormValidateAsyncFn, type FormValidateFn, type FormValidateOrFn, type FormValidationError, type FormValidationErrorMap, type FormValidator, type FormValidators, type GlobalFormValidationError, type Narrow, type StandardSchemaV1, type StandardSchemaV1Issue, type SyncValidator, type TStandardSchemaValidatorValue, type UnwrapFieldAsyncValidateOrFn, type UnwrapFieldValidateOrFn, type UnwrapFormAsyncValidateOrFn, type UnwrapFormValidateOrFn, type UnwrapOneLevelOfArray, type UpdateMetaOptions, type Updater, type UpdaterFn, type ValidationCause, type ValidationError, type ValidationErrorMap, type ValidationErrorMapKeys, type ValidationMeta, type ValidationSource, deleteBy, formOptions, functionalUpdate, getAsyncValidatorArray, getBy, getSyncValidatorArray, isGlobalFormValidationError, isNonEmptyArray, isStandardSchemaValidator, makePathArray, mergeForm, mutateMergeDeep, setBy, shallow, standardSchemaValidators };
