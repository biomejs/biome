type Fallback<TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>>> = MaybeDeepReadonly<InferOutput<TSchema>> | ((dataset?: OutputDataset<InferOutput<TSchema>, InferIssue<TSchema>>, config?: Config<InferIssue<TSchema>>) => MaybeDeepReadonly<InferOutput<TSchema>>);
type SchemaWithFallback<TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, TFallback$1 extends Fallback<TSchema>> = TSchema & {
    readonly fallback: TFallback$1;
};
type FallbackAsync<TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>> = MaybeDeepReadonly<InferOutput<TSchema>> | ((dataset?: OutputDataset<InferOutput<TSchema>, InferIssue<TSchema>>, config?: Config<InferIssue<TSchema>>) => MaybePromise<MaybeDeepReadonly<InferOutput<TSchema>>>);
type SchemaWithFallbackAsync<TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, TFallback$1 extends FallbackAsync<TSchema>> = Omit<TSchema, "async" | "~standard" | "~run"> & {
    readonly fallback: TFallback$1;
    readonly async: true;
    readonly "~standard": StandardProps<InferInput<TSchema>, InferOutput<TSchema>>;
    readonly "~run": (dataset: UnknownDataset, config: Config<BaseIssue<unknown>>) => Promise<OutputDataset<InferOutput<TSchema>, InferIssue<TSchema>>>;
};
type FlatErrors<TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>> | undefined> = Prettify<{
    readonly root?: [
        string,
        ...string[]
    ];
    readonly nested?: Prettify<Readonly<Partial<Record<TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>> ? IssueDotPath<TSchema> : string, [
        string,
        ...string[]
    ]>>>>;
    readonly other?: [
        string,
        ...string[]
    ];
}>;
declare function flatten(issues: readonly [
    BaseIssue<unknown>,
    ...BaseIssue<unknown>[]
]): FlatErrors<undefined>;
declare function flatten<TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>>(issues: readonly [
    InferIssue<TSchema>,
    ...InferIssue<TSchema>[]
]): FlatErrors<TSchema>;
type SchemaWithPipe<TPipe$1 extends readonly [
    BaseSchema<unknown, unknown, BaseIssue<unknown>>,
    ...PipeItem<any, unknown, BaseIssue<unknown>>[]
]> = Omit<FirstTupleItem<TPipe$1>, "pipe" | "~standard" | "~run" | "~types"> & {
    readonly pipe: TPipe$1;
    readonly "~standard": StandardProps<InferInput<FirstTupleItem<TPipe$1>>, InferOutput<LastTupleItem<TPipe$1>>>;
    readonly "~run": (dataset: UnknownDataset, config: Config<BaseIssue<unknown>>) => OutputDataset<InferOutput<LastTupleItem<TPipe$1>>, InferIssue<TPipe$1[number]>>;
    readonly "~types"?: {
        readonly input: InferInput<FirstTupleItem<TPipe$1>>;
        readonly output: InferOutput<LastTupleItem<TPipe$1>>;
        readonly issue: InferIssue<TPipe$1[number]>;
    } | undefined;
};
declare function pipe<const TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TItem1 extends PipeItem<InferOutput<TSchema>, unknown, BaseIssue<unknown>>>(schema: TSchema, item1: TItem1 | PipeAction<InferOutput<TSchema>, InferOutput<TItem1>, InferIssue<TItem1>>): SchemaWithPipe<readonly [
    TSchema,
    TItem1
]>;
declare function pipe<const TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TItem1 extends PipeItem<InferOutput<TSchema>, unknown, BaseIssue<unknown>>, const TItem2 extends PipeItem<InferOutput<TItem1>, unknown, BaseIssue<unknown>>>(schema: TSchema, item1: TItem1 | PipeAction<InferOutput<TSchema>, InferOutput<TItem1>, InferIssue<TItem1>>, item2: TItem2 | PipeAction<InferOutput<TItem1>, InferOutput<TItem2>, InferIssue<TItem2>>): SchemaWithPipe<readonly [
    TSchema,
    TItem1,
    TItem2
]>;
declare function pipe<const TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TItem1 extends PipeItem<InferOutput<TSchema>, unknown, BaseIssue<unknown>>, const TItem2 extends PipeItem<InferOutput<TItem1>, unknown, BaseIssue<unknown>>, const TItem3 extends PipeItem<InferOutput<TItem2>, unknown, BaseIssue<unknown>>>(schema: TSchema, item1: TItem1 | PipeAction<InferOutput<TSchema>, InferOutput<TItem1>, InferIssue<TItem1>>, item2: TItem2 | PipeAction<InferOutput<TItem1>, InferOutput<TItem2>, InferIssue<TItem2>>, item3: TItem3 | PipeAction<InferOutput<TItem2>, InferOutput<TItem3>, InferIssue<TItem3>>): SchemaWithPipe<readonly [
    TSchema,
    TItem1,
    TItem2,
    TItem3
]>;
declare function pipe<const TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TItem1 extends PipeItem<InferOutput<TSchema>, unknown, BaseIssue<unknown>>, const TItem2 extends PipeItem<InferOutput<TItem1>, unknown, BaseIssue<unknown>>, const TItem3 extends PipeItem<InferOutput<TItem2>, unknown, BaseIssue<unknown>>, const TItem4 extends PipeItem<InferOutput<TItem3>, unknown, BaseIssue<unknown>>>(schema: TSchema, item1: TItem1 | PipeAction<InferOutput<TSchema>, InferOutput<TItem1>, InferIssue<TItem1>>, item2: TItem2 | PipeAction<InferOutput<TItem1>, InferOutput<TItem2>, InferIssue<TItem2>>, item3: TItem3 | PipeAction<InferOutput<TItem2>, InferOutput<TItem3>, InferIssue<TItem3>>, item4: TItem4 | PipeAction<InferOutput<TItem3>, InferOutput<TItem4>, InferIssue<TItem4>>): SchemaWithPipe<readonly [
    TSchema,
    TItem1,
    TItem2,
    TItem3,
    TItem4
]>;
declare function pipe<const TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TItem1 extends PipeItem<InferOutput<TSchema>, unknown, BaseIssue<unknown>>, const TItem2 extends PipeItem<InferOutput<TItem1>, unknown, BaseIssue<unknown>>, const TItem3 extends PipeItem<InferOutput<TItem2>, unknown, BaseIssue<unknown>>, const TItem4 extends PipeItem<InferOutput<TItem3>, unknown, BaseIssue<unknown>>, const TItem5 extends PipeItem<InferOutput<TItem4>, unknown, BaseIssue<unknown>>>(schema: TSchema, item1: TItem1 | PipeAction<InferOutput<TSchema>, InferOutput<TItem1>, InferIssue<TItem1>>, item2: TItem2 | PipeAction<InferOutput<TItem1>, InferOutput<TItem2>, InferIssue<TItem2>>, item3: TItem3 | PipeAction<InferOutput<TItem2>, InferOutput<TItem3>, InferIssue<TItem3>>, item4: TItem4 | PipeAction<InferOutput<TItem3>, InferOutput<TItem4>, InferIssue<TItem4>>, item5: TItem5 | PipeAction<InferOutput<TItem4>, InferOutput<TItem5>, InferIssue<TItem5>>): SchemaWithPipe<readonly [
    TSchema,
    TItem1,
    TItem2,
    TItem3,
    TItem4,
    TItem5
]>;
declare function pipe<const TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TItem1 extends PipeItem<InferOutput<TSchema>, unknown, BaseIssue<unknown>>, const TItem2 extends PipeItem<InferOutput<TItem1>, unknown, BaseIssue<unknown>>, const TItem3 extends PipeItem<InferOutput<TItem2>, unknown, BaseIssue<unknown>>, const TItem4 extends PipeItem<InferOutput<TItem3>, unknown, BaseIssue<unknown>>, const TItem5 extends PipeItem<InferOutput<TItem4>, unknown, BaseIssue<unknown>>, const TItem6 extends PipeItem<InferOutput<TItem5>, unknown, BaseIssue<unknown>>>(schema: TSchema, item1: TItem1 | PipeAction<InferOutput<TSchema>, InferOutput<TItem1>, InferIssue<TItem1>>, item2: TItem2 | PipeAction<InferOutput<TItem1>, InferOutput<TItem2>, InferIssue<TItem2>>, item3: TItem3 | PipeAction<InferOutput<TItem2>, InferOutput<TItem3>, InferIssue<TItem3>>, item4: TItem4 | PipeAction<InferOutput<TItem3>, InferOutput<TItem4>, InferIssue<TItem4>>, item5: TItem5 | PipeAction<InferOutput<TItem4>, InferOutput<TItem5>, InferIssue<TItem5>>, item6: TItem6 | PipeAction<InferOutput<TItem5>, InferOutput<TItem6>, InferIssue<TItem6>>): SchemaWithPipe<readonly [
    TSchema,
    TItem1,
    TItem2,
    TItem3,
    TItem4,
    TItem5,
    TItem6
]>;
declare function pipe<const TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TItem1 extends PipeItem<InferOutput<TSchema>, unknown, BaseIssue<unknown>>, const TItem2 extends PipeItem<InferOutput<TItem1>, unknown, BaseIssue<unknown>>, const TItem3 extends PipeItem<InferOutput<TItem2>, unknown, BaseIssue<unknown>>, const TItem4 extends PipeItem<InferOutput<TItem3>, unknown, BaseIssue<unknown>>, const TItem5 extends PipeItem<InferOutput<TItem4>, unknown, BaseIssue<unknown>>, const TItem6 extends PipeItem<InferOutput<TItem5>, unknown, BaseIssue<unknown>>, const TItem7 extends PipeItem<InferOutput<TItem6>, unknown, BaseIssue<unknown>>>(schema: TSchema, item1: TItem1 | PipeAction<InferOutput<TSchema>, InferOutput<TItem1>, InferIssue<TItem1>>, item2: TItem2 | PipeAction<InferOutput<TItem1>, InferOutput<TItem2>, InferIssue<TItem2>>, item3: TItem3 | PipeAction<InferOutput<TItem2>, InferOutput<TItem3>, InferIssue<TItem3>>, item4: TItem4 | PipeAction<InferOutput<TItem3>, InferOutput<TItem4>, InferIssue<TItem4>>, item5: TItem5 | PipeAction<InferOutput<TItem4>, InferOutput<TItem5>, InferIssue<TItem5>>, item6: TItem6 | PipeAction<InferOutput<TItem5>, InferOutput<TItem6>, InferIssue<TItem6>>, item7: TItem7 | PipeAction<InferOutput<TItem6>, InferOutput<TItem7>, InferIssue<TItem7>>): SchemaWithPipe<readonly [
    TSchema,
    TItem1,
    TItem2,
    TItem3,
    TItem4,
    TItem5,
    TItem6,
    TItem7
]>;
declare function pipe<const TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TItem1 extends PipeItem<InferOutput<TSchema>, unknown, BaseIssue<unknown>>, const TItem2 extends PipeItem<InferOutput<TItem1>, unknown, BaseIssue<unknown>>, const TItem3 extends PipeItem<InferOutput<TItem2>, unknown, BaseIssue<unknown>>, const TItem4 extends PipeItem<InferOutput<TItem3>, unknown, BaseIssue<unknown>>, const TItem5 extends PipeItem<InferOutput<TItem4>, unknown, BaseIssue<unknown>>, const TItem6 extends PipeItem<InferOutput<TItem5>, unknown, BaseIssue<unknown>>, const TItem7 extends PipeItem<InferOutput<TItem6>, unknown, BaseIssue<unknown>>, const TItem8 extends PipeItem<InferOutput<TItem7>, unknown, BaseIssue<unknown>>>(schema: TSchema, item1: TItem1 | PipeAction<InferOutput<TSchema>, InferOutput<TItem1>, InferIssue<TItem1>>, item2: TItem2 | PipeAction<InferOutput<TItem1>, InferOutput<TItem2>, InferIssue<TItem2>>, item3: TItem3 | PipeAction<InferOutput<TItem2>, InferOutput<TItem3>, InferIssue<TItem3>>, item4: TItem4 | PipeAction<InferOutput<TItem3>, InferOutput<TItem4>, InferIssue<TItem4>>, item5: TItem5 | PipeAction<InferOutput<TItem4>, InferOutput<TItem5>, InferIssue<TItem5>>, item6: TItem6 | PipeAction<InferOutput<TItem5>, InferOutput<TItem6>, InferIssue<TItem6>>, item7: TItem7 | PipeAction<InferOutput<TItem6>, InferOutput<TItem7>, InferIssue<TItem7>>, item8: TItem8 | PipeAction<InferOutput<TItem7>, InferOutput<TItem8>, InferIssue<TItem8>>): SchemaWithPipe<readonly [
    TSchema,
    TItem1,
    TItem2,
    TItem3,
    TItem4,
    TItem5,
    TItem6,
    TItem7,
    TItem8
]>;
declare function pipe<const TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TItem1 extends PipeItem<InferOutput<TSchema>, unknown, BaseIssue<unknown>>, const TItem2 extends PipeItem<InferOutput<TItem1>, unknown, BaseIssue<unknown>>, const TItem3 extends PipeItem<InferOutput<TItem2>, unknown, BaseIssue<unknown>>, const TItem4 extends PipeItem<InferOutput<TItem3>, unknown, BaseIssue<unknown>>, const TItem5 extends PipeItem<InferOutput<TItem4>, unknown, BaseIssue<unknown>>, const TItem6 extends PipeItem<InferOutput<TItem5>, unknown, BaseIssue<unknown>>, const TItem7 extends PipeItem<InferOutput<TItem6>, unknown, BaseIssue<unknown>>, const TItem8 extends PipeItem<InferOutput<TItem7>, unknown, BaseIssue<unknown>>, const TItem9 extends PipeItem<InferOutput<TItem8>, unknown, BaseIssue<unknown>>>(schema: TSchema, item1: TItem1 | PipeAction<InferOutput<TSchema>, InferOutput<TItem1>, InferIssue<TItem1>>, item2: TItem2 | PipeAction<InferOutput<TItem1>, InferOutput<TItem2>, InferIssue<TItem2>>, item3: TItem3 | PipeAction<InferOutput<TItem2>, InferOutput<TItem3>, InferIssue<TItem3>>, item4: TItem4 | PipeAction<InferOutput<TItem3>, InferOutput<TItem4>, InferIssue<TItem4>>, item5: TItem5 | PipeAction<InferOutput<TItem4>, InferOutput<TItem5>, InferIssue<TItem5>>, item6: TItem6 | PipeAction<InferOutput<TItem5>, InferOutput<TItem6>, InferIssue<TItem6>>, item7: TItem7 | PipeAction<InferOutput<TItem6>, InferOutput<TItem7>, InferIssue<TItem7>>, item8: TItem8 | PipeAction<InferOutput<TItem7>, InferOutput<TItem8>, InferIssue<TItem8>>, item9: TItem9 | PipeAction<InferOutput<TItem8>, InferOutput<TItem9>, InferIssue<TItem9>>): SchemaWithPipe<readonly [
    TSchema,
    TItem1,
    TItem2,
    TItem3,
    TItem4,
    TItem5,
    TItem6,
    TItem7,
    TItem8,
    TItem9
]>;
declare function pipe<const TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TItem1 extends PipeItem<InferOutput<TSchema>, unknown, BaseIssue<unknown>>, const TItem2 extends PipeItem<InferOutput<TItem1>, unknown, BaseIssue<unknown>>, const TItem3 extends PipeItem<InferOutput<TItem2>, unknown, BaseIssue<unknown>>, const TItem4 extends PipeItem<InferOutput<TItem3>, unknown, BaseIssue<unknown>>, const TItem5 extends PipeItem<InferOutput<TItem4>, unknown, BaseIssue<unknown>>, const TItem6 extends PipeItem<InferOutput<TItem5>, unknown, BaseIssue<unknown>>, const TItem7 extends PipeItem<InferOutput<TItem6>, unknown, BaseIssue<unknown>>, const TItem8 extends PipeItem<InferOutput<TItem7>, unknown, BaseIssue<unknown>>, const TItem9 extends PipeItem<InferOutput<TItem8>, unknown, BaseIssue<unknown>>, const TItem10 extends PipeItem<InferOutput<TItem9>, unknown, BaseIssue<unknown>>>(schema: TSchema, item1: TItem1 | PipeAction<InferOutput<TSchema>, InferOutput<TItem1>, InferIssue<TItem1>>, item2: TItem2 | PipeAction<InferOutput<TItem1>, InferOutput<TItem2>, InferIssue<TItem2>>, item3: TItem3 | PipeAction<InferOutput<TItem2>, InferOutput<TItem3>, InferIssue<TItem3>>, item4: TItem4 | PipeAction<InferOutput<TItem3>, InferOutput<TItem4>, InferIssue<TItem4>>, item5: TItem5 | PipeAction<InferOutput<TItem4>, InferOutput<TItem5>, InferIssue<TItem5>>, item6: TItem6 | PipeAction<InferOutput<TItem5>, InferOutput<TItem6>, InferIssue<TItem6>>, item7: TItem7 | PipeAction<InferOutput<TItem6>, InferOutput<TItem7>, InferIssue<TItem7>>, item8: TItem8 | PipeAction<InferOutput<TItem7>, InferOutput<TItem8>, InferIssue<TItem8>>, item9: TItem9 | PipeAction<InferOutput<TItem8>, InferOutput<TItem9>, InferIssue<TItem9>>, item10: TItem10 | PipeAction<InferOutput<TItem9>, InferOutput<TItem10>, InferIssue<TItem10>>): SchemaWithPipe<readonly [
    TSchema,
    TItem1,
    TItem2,
    TItem3,
    TItem4,
    TItem5,
    TItem6,
    TItem7,
    TItem8,
    TItem9,
    TItem10
]>;
declare function pipe<const TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TItem1 extends PipeItem<InferOutput<TSchema>, unknown, BaseIssue<unknown>>, const TItem2 extends PipeItem<InferOutput<TItem1>, unknown, BaseIssue<unknown>>, const TItem3 extends PipeItem<InferOutput<TItem2>, unknown, BaseIssue<unknown>>, const TItem4 extends PipeItem<InferOutput<TItem3>, unknown, BaseIssue<unknown>>, const TItem5 extends PipeItem<InferOutput<TItem4>, unknown, BaseIssue<unknown>>, const TItem6 extends PipeItem<InferOutput<TItem5>, unknown, BaseIssue<unknown>>, const TItem7 extends PipeItem<InferOutput<TItem6>, unknown, BaseIssue<unknown>>, const TItem8 extends PipeItem<InferOutput<TItem7>, unknown, BaseIssue<unknown>>, const TItem9 extends PipeItem<InferOutput<TItem8>, unknown, BaseIssue<unknown>>, const TItem10 extends PipeItem<InferOutput<TItem9>, unknown, BaseIssue<unknown>>, const TItem11 extends PipeItem<InferOutput<TItem10>, unknown, BaseIssue<unknown>>>(schema: TSchema, item1: TItem1 | PipeAction<InferOutput<TSchema>, InferOutput<TItem1>, InferIssue<TItem1>>, item2: TItem2 | PipeAction<InferOutput<TItem1>, InferOutput<TItem2>, InferIssue<TItem2>>, item3: TItem3 | PipeAction<InferOutput<TItem2>, InferOutput<TItem3>, InferIssue<TItem3>>, item4: TItem4 | PipeAction<InferOutput<TItem3>, InferOutput<TItem4>, InferIssue<TItem4>>, item5: TItem5 | PipeAction<InferOutput<TItem4>, InferOutput<TItem5>, InferIssue<TItem5>>, item6: TItem6 | PipeAction<InferOutput<TItem5>, InferOutput<TItem6>, InferIssue<TItem6>>, item7: TItem7 | PipeAction<InferOutput<TItem6>, InferOutput<TItem7>, InferIssue<TItem7>>, item8: TItem8 | PipeAction<InferOutput<TItem7>, InferOutput<TItem8>, InferIssue<TItem8>>, item9: TItem9 | PipeAction<InferOutput<TItem8>, InferOutput<TItem9>, InferIssue<TItem9>>, item10: TItem10 | PipeAction<InferOutput<TItem9>, InferOutput<TItem10>, InferIssue<TItem10>>, item11: TItem11 | PipeAction<InferOutput<TItem10>, InferOutput<TItem11>, InferIssue<TItem11>>): SchemaWithPipe<readonly [
    TSchema,
    TItem1,
    TItem2,
    TItem3,
    TItem4,
    TItem5,
    TItem6,
    TItem7,
    TItem8,
    TItem9,
    TItem10,
    TItem11
]>;
declare function pipe<const TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TItem1 extends PipeItem<InferOutput<TSchema>, unknown, BaseIssue<unknown>>, const TItem2 extends PipeItem<InferOutput<TItem1>, unknown, BaseIssue<unknown>>, const TItem3 extends PipeItem<InferOutput<TItem2>, unknown, BaseIssue<unknown>>, const TItem4 extends PipeItem<InferOutput<TItem3>, unknown, BaseIssue<unknown>>, const TItem5 extends PipeItem<InferOutput<TItem4>, unknown, BaseIssue<unknown>>, const TItem6 extends PipeItem<InferOutput<TItem5>, unknown, BaseIssue<unknown>>, const TItem7 extends PipeItem<InferOutput<TItem6>, unknown, BaseIssue<unknown>>, const TItem8 extends PipeItem<InferOutput<TItem7>, unknown, BaseIssue<unknown>>, const TItem9 extends PipeItem<InferOutput<TItem8>, unknown, BaseIssue<unknown>>, const TItem10 extends PipeItem<InferOutput<TItem9>, unknown, BaseIssue<unknown>>, const TItem11 extends PipeItem<InferOutput<TItem10>, unknown, BaseIssue<unknown>>, const TItem12 extends PipeItem<InferOutput<TItem11>, unknown, BaseIssue<unknown>>>(schema: TSchema, item1: TItem1 | PipeAction<InferOutput<TSchema>, InferOutput<TItem1>, InferIssue<TItem1>>, item2: TItem2 | PipeAction<InferOutput<TItem1>, InferOutput<TItem2>, InferIssue<TItem2>>, item3: TItem3 | PipeAction<InferOutput<TItem2>, InferOutput<TItem3>, InferIssue<TItem3>>, item4: TItem4 | PipeAction<InferOutput<TItem3>, InferOutput<TItem4>, InferIssue<TItem4>>, item5: TItem5 | PipeAction<InferOutput<TItem4>, InferOutput<TItem5>, InferIssue<TItem5>>, item6: TItem6 | PipeAction<InferOutput<TItem5>, InferOutput<TItem6>, InferIssue<TItem6>>, item7: TItem7 | PipeAction<InferOutput<TItem6>, InferOutput<TItem7>, InferIssue<TItem7>>, item8: TItem8 | PipeAction<InferOutput<TItem7>, InferOutput<TItem8>, InferIssue<TItem8>>, item9: TItem9 | PipeAction<InferOutput<TItem8>, InferOutput<TItem9>, InferIssue<TItem9>>, item10: TItem10 | PipeAction<InferOutput<TItem9>, InferOutput<TItem10>, InferIssue<TItem10>>, item11: TItem11 | PipeAction<InferOutput<TItem10>, InferOutput<TItem11>, InferIssue<TItem11>>, item12: TItem12 | PipeAction<InferOutput<TItem11>, InferOutput<TItem12>, InferIssue<TItem12>>): SchemaWithPipe<readonly [
    TSchema,
    TItem1,
    TItem2,
    TItem3,
    TItem4,
    TItem5,
    TItem6,
    TItem7,
    TItem8,
    TItem9,
    TItem10,
    TItem11,
    TItem12
]>;
declare function pipe<const TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TItem1 extends PipeItem<InferOutput<TSchema>, unknown, BaseIssue<unknown>>, const TItem2 extends PipeItem<InferOutput<TItem1>, unknown, BaseIssue<unknown>>, const TItem3 extends PipeItem<InferOutput<TItem2>, unknown, BaseIssue<unknown>>, const TItem4 extends PipeItem<InferOutput<TItem3>, unknown, BaseIssue<unknown>>, const TItem5 extends PipeItem<InferOutput<TItem4>, unknown, BaseIssue<unknown>>, const TItem6 extends PipeItem<InferOutput<TItem5>, unknown, BaseIssue<unknown>>, const TItem7 extends PipeItem<InferOutput<TItem6>, unknown, BaseIssue<unknown>>, const TItem8 extends PipeItem<InferOutput<TItem7>, unknown, BaseIssue<unknown>>, const TItem9 extends PipeItem<InferOutput<TItem8>, unknown, BaseIssue<unknown>>, const TItem10 extends PipeItem<InferOutput<TItem9>, unknown, BaseIssue<unknown>>, const TItem11 extends PipeItem<InferOutput<TItem10>, unknown, BaseIssue<unknown>>, const TItem12 extends PipeItem<InferOutput<TItem11>, unknown, BaseIssue<unknown>>, const TItem13 extends PipeItem<InferOutput<TItem12>, unknown, BaseIssue<unknown>>>(schema: TSchema, item1: TItem1 | PipeAction<InferOutput<TSchema>, InferOutput<TItem1>, InferIssue<TItem1>>, item2: TItem2 | PipeAction<InferOutput<TItem1>, InferOutput<TItem2>, InferIssue<TItem2>>, item3: TItem3 | PipeAction<InferOutput<TItem2>, InferOutput<TItem3>, InferIssue<TItem3>>, item4: TItem4 | PipeAction<InferOutput<TItem3>, InferOutput<TItem4>, InferIssue<TItem4>>, item5: TItem5 | PipeAction<InferOutput<TItem4>, InferOutput<TItem5>, InferIssue<TItem5>>, item6: TItem6 | PipeAction<InferOutput<TItem5>, InferOutput<TItem6>, InferIssue<TItem6>>, item7: TItem7 | PipeAction<InferOutput<TItem6>, InferOutput<TItem7>, InferIssue<TItem7>>, item8: TItem8 | PipeAction<InferOutput<TItem7>, InferOutput<TItem8>, InferIssue<TItem8>>, item9: TItem9 | PipeAction<InferOutput<TItem8>, InferOutput<TItem9>, InferIssue<TItem9>>, item10: TItem10 | PipeAction<InferOutput<TItem9>, InferOutput<TItem10>, InferIssue<TItem10>>, item11: TItem11 | PipeAction<InferOutput<TItem10>, InferOutput<TItem11>, InferIssue<TItem11>>, item12: TItem12 | PipeAction<InferOutput<TItem11>, InferOutput<TItem12>, InferIssue<TItem12>>, item13: TItem13 | PipeAction<InferOutput<TItem12>, InferOutput<TItem13>, InferIssue<TItem13>>): SchemaWithPipe<readonly [
    TSchema,
    TItem1,
    TItem2,
    TItem3,
    TItem4,
    TItem5,
    TItem6,
    TItem7,
    TItem8,
    TItem9,
    TItem10,
    TItem11,
    TItem12,
    TItem13
]>;
declare function pipe<const TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TItem1 extends PipeItem<InferOutput<TSchema>, unknown, BaseIssue<unknown>>, const TItem2 extends PipeItem<InferOutput<TItem1>, unknown, BaseIssue<unknown>>, const TItem3 extends PipeItem<InferOutput<TItem2>, unknown, BaseIssue<unknown>>, const TItem4 extends PipeItem<InferOutput<TItem3>, unknown, BaseIssue<unknown>>, const TItem5 extends PipeItem<InferOutput<TItem4>, unknown, BaseIssue<unknown>>, const TItem6 extends PipeItem<InferOutput<TItem5>, unknown, BaseIssue<unknown>>, const TItem7 extends PipeItem<InferOutput<TItem6>, unknown, BaseIssue<unknown>>, const TItem8 extends PipeItem<InferOutput<TItem7>, unknown, BaseIssue<unknown>>, const TItem9 extends PipeItem<InferOutput<TItem8>, unknown, BaseIssue<unknown>>, const TItem10 extends PipeItem<InferOutput<TItem9>, unknown, BaseIssue<unknown>>, const TItem11 extends PipeItem<InferOutput<TItem10>, unknown, BaseIssue<unknown>>, const TItem12 extends PipeItem<InferOutput<TItem11>, unknown, BaseIssue<unknown>>, const TItem13 extends PipeItem<InferOutput<TItem12>, unknown, BaseIssue<unknown>>, const TItem14 extends PipeItem<InferOutput<TItem13>, unknown, BaseIssue<unknown>>>(schema: TSchema, item1: TItem1 | PipeAction<InferOutput<TSchema>, InferOutput<TItem1>, InferIssue<TItem1>>, item2: TItem2 | PipeAction<InferOutput<TItem1>, InferOutput<TItem2>, InferIssue<TItem2>>, item3: TItem3 | PipeAction<InferOutput<TItem2>, InferOutput<TItem3>, InferIssue<TItem3>>, item4: TItem4 | PipeAction<InferOutput<TItem3>, InferOutput<TItem4>, InferIssue<TItem4>>, item5: TItem5 | PipeAction<InferOutput<TItem4>, InferOutput<TItem5>, InferIssue<TItem5>>, item6: TItem6 | PipeAction<InferOutput<TItem5>, InferOutput<TItem6>, InferIssue<TItem6>>, item7: TItem7 | PipeAction<InferOutput<TItem6>, InferOutput<TItem7>, InferIssue<TItem7>>, item8: TItem8 | PipeAction<InferOutput<TItem7>, InferOutput<TItem8>, InferIssue<TItem8>>, item9: TItem9 | PipeAction<InferOutput<TItem8>, InferOutput<TItem9>, InferIssue<TItem9>>, item10: TItem10 | PipeAction<InferOutput<TItem9>, InferOutput<TItem10>, InferIssue<TItem10>>, item11: TItem11 | PipeAction<InferOutput<TItem10>, InferOutput<TItem11>, InferIssue<TItem11>>, item12: TItem12 | PipeAction<InferOutput<TItem11>, InferOutput<TItem12>, InferIssue<TItem12>>, item13: TItem13 | PipeAction<InferOutput<TItem12>, InferOutput<TItem13>, InferIssue<TItem13>>, item14: TItem14 | PipeAction<InferOutput<TItem13>, InferOutput<TItem14>, InferIssue<TItem14>>): SchemaWithPipe<readonly [
    TSchema,
    TItem1,
    TItem2,
    TItem3,
    TItem4,
    TItem5,
    TItem6,
    TItem7,
    TItem8,
    TItem9,
    TItem10,
    TItem11,
    TItem12,
    TItem13,
    TItem14
]>;
declare function pipe<const TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TItem1 extends PipeItem<InferOutput<TSchema>, unknown, BaseIssue<unknown>>, const TItem2 extends PipeItem<InferOutput<TItem1>, unknown, BaseIssue<unknown>>, const TItem3 extends PipeItem<InferOutput<TItem2>, unknown, BaseIssue<unknown>>, const TItem4 extends PipeItem<InferOutput<TItem3>, unknown, BaseIssue<unknown>>, const TItem5 extends PipeItem<InferOutput<TItem4>, unknown, BaseIssue<unknown>>, const TItem6 extends PipeItem<InferOutput<TItem5>, unknown, BaseIssue<unknown>>, const TItem7 extends PipeItem<InferOutput<TItem6>, unknown, BaseIssue<unknown>>, const TItem8 extends PipeItem<InferOutput<TItem7>, unknown, BaseIssue<unknown>>, const TItem9 extends PipeItem<InferOutput<TItem8>, unknown, BaseIssue<unknown>>, const TItem10 extends PipeItem<InferOutput<TItem9>, unknown, BaseIssue<unknown>>, const TItem11 extends PipeItem<InferOutput<TItem10>, unknown, BaseIssue<unknown>>, const TItem12 extends PipeItem<InferOutput<TItem11>, unknown, BaseIssue<unknown>>, const TItem13 extends PipeItem<InferOutput<TItem12>, unknown, BaseIssue<unknown>>, const TItem14 extends PipeItem<InferOutput<TItem13>, unknown, BaseIssue<unknown>>, const TItem15 extends PipeItem<InferOutput<TItem14>, unknown, BaseIssue<unknown>>>(schema: TSchema, item1: TItem1 | PipeAction<InferOutput<TSchema>, InferOutput<TItem1>, InferIssue<TItem1>>, item2: TItem2 | PipeAction<InferOutput<TItem1>, InferOutput<TItem2>, InferIssue<TItem2>>, item3: TItem3 | PipeAction<InferOutput<TItem2>, InferOutput<TItem3>, InferIssue<TItem3>>, item4: TItem4 | PipeAction<InferOutput<TItem3>, InferOutput<TItem4>, InferIssue<TItem4>>, item5: TItem5 | PipeAction<InferOutput<TItem4>, InferOutput<TItem5>, InferIssue<TItem5>>, item6: TItem6 | PipeAction<InferOutput<TItem5>, InferOutput<TItem6>, InferIssue<TItem6>>, item7: TItem7 | PipeAction<InferOutput<TItem6>, InferOutput<TItem7>, InferIssue<TItem7>>, item8: TItem8 | PipeAction<InferOutput<TItem7>, InferOutput<TItem8>, InferIssue<TItem8>>, item9: TItem9 | PipeAction<InferOutput<TItem8>, InferOutput<TItem9>, InferIssue<TItem9>>, item10: TItem10 | PipeAction<InferOutput<TItem9>, InferOutput<TItem10>, InferIssue<TItem10>>, item11: TItem11 | PipeAction<InferOutput<TItem10>, InferOutput<TItem11>, InferIssue<TItem11>>, item12: TItem12 | PipeAction<InferOutput<TItem11>, InferOutput<TItem12>, InferIssue<TItem12>>, item13: TItem13 | PipeAction<InferOutput<TItem12>, InferOutput<TItem13>, InferIssue<TItem13>>, item14: TItem14 | PipeAction<InferOutput<TItem13>, InferOutput<TItem14>, InferIssue<TItem14>>, item15: TItem15 | PipeAction<InferOutput<TItem14>, InferOutput<TItem15>, InferIssue<TItem15>>): SchemaWithPipe<readonly [
    TSchema,
    TItem1,
    TItem2,
    TItem3,
    TItem4,
    TItem5,
    TItem6,
    TItem7,
    TItem8,
    TItem9,
    TItem10,
    TItem11,
    TItem12,
    TItem13,
    TItem14,
    TItem15
]>;
declare function pipe<const TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TItem1 extends PipeItem<InferOutput<TSchema>, unknown, BaseIssue<unknown>>, const TItem2 extends PipeItem<InferOutput<TItem1>, unknown, BaseIssue<unknown>>, const TItem3 extends PipeItem<InferOutput<TItem2>, unknown, BaseIssue<unknown>>, const TItem4 extends PipeItem<InferOutput<TItem3>, unknown, BaseIssue<unknown>>, const TItem5 extends PipeItem<InferOutput<TItem4>, unknown, BaseIssue<unknown>>, const TItem6 extends PipeItem<InferOutput<TItem5>, unknown, BaseIssue<unknown>>, const TItem7 extends PipeItem<InferOutput<TItem6>, unknown, BaseIssue<unknown>>, const TItem8 extends PipeItem<InferOutput<TItem7>, unknown, BaseIssue<unknown>>, const TItem9 extends PipeItem<InferOutput<TItem8>, unknown, BaseIssue<unknown>>, const TItem10 extends PipeItem<InferOutput<TItem9>, unknown, BaseIssue<unknown>>, const TItem11 extends PipeItem<InferOutput<TItem10>, unknown, BaseIssue<unknown>>, const TItem12 extends PipeItem<InferOutput<TItem11>, unknown, BaseIssue<unknown>>, const TItem13 extends PipeItem<InferOutput<TItem12>, unknown, BaseIssue<unknown>>, const TItem14 extends PipeItem<InferOutput<TItem13>, unknown, BaseIssue<unknown>>, const TItem15 extends PipeItem<InferOutput<TItem14>, unknown, BaseIssue<unknown>>, const TItem16 extends PipeItem<InferOutput<TItem15>, unknown, BaseIssue<unknown>>>(schema: TSchema, item1: TItem1 | PipeAction<InferOutput<TSchema>, InferOutput<TItem1>, InferIssue<TItem1>>, item2: TItem2 | PipeAction<InferOutput<TItem1>, InferOutput<TItem2>, InferIssue<TItem2>>, item3: TItem3 | PipeAction<InferOutput<TItem2>, InferOutput<TItem3>, InferIssue<TItem3>>, item4: TItem4 | PipeAction<InferOutput<TItem3>, InferOutput<TItem4>, InferIssue<TItem4>>, item5: TItem5 | PipeAction<InferOutput<TItem4>, InferOutput<TItem5>, InferIssue<TItem5>>, item6: TItem6 | PipeAction<InferOutput<TItem5>, InferOutput<TItem6>, InferIssue<TItem6>>, item7: TItem7 | PipeAction<InferOutput<TItem6>, InferOutput<TItem7>, InferIssue<TItem7>>, item8: TItem8 | PipeAction<InferOutput<TItem7>, InferOutput<TItem8>, InferIssue<TItem8>>, item9: TItem9 | PipeAction<InferOutput<TItem8>, InferOutput<TItem9>, InferIssue<TItem9>>, item10: TItem10 | PipeAction<InferOutput<TItem9>, InferOutput<TItem10>, InferIssue<TItem10>>, item11: TItem11 | PipeAction<InferOutput<TItem10>, InferOutput<TItem11>, InferIssue<TItem11>>, item12: TItem12 | PipeAction<InferOutput<TItem11>, InferOutput<TItem12>, InferIssue<TItem12>>, item13: TItem13 | PipeAction<InferOutput<TItem12>, InferOutput<TItem13>, InferIssue<TItem13>>, item14: TItem14 | PipeAction<InferOutput<TItem13>, InferOutput<TItem14>, InferIssue<TItem14>>, item15: TItem15 | PipeAction<InferOutput<TItem14>, InferOutput<TItem15>, InferIssue<TItem15>>, item16: TItem16 | PipeAction<InferOutput<TItem15>, InferOutput<TItem16>, InferIssue<TItem16>>): SchemaWithPipe<readonly [
    TSchema,
    TItem1,
    TItem2,
    TItem3,
    TItem4,
    TItem5,
    TItem6,
    TItem7,
    TItem8,
    TItem9,
    TItem10,
    TItem11,
    TItem12,
    TItem13,
    TItem14,
    TItem15,
    TItem16
]>;
declare function pipe<const TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TItem1 extends PipeItem<InferOutput<TSchema>, unknown, BaseIssue<unknown>>, const TItem2 extends PipeItem<InferOutput<TItem1>, unknown, BaseIssue<unknown>>, const TItem3 extends PipeItem<InferOutput<TItem2>, unknown, BaseIssue<unknown>>, const TItem4 extends PipeItem<InferOutput<TItem3>, unknown, BaseIssue<unknown>>, const TItem5 extends PipeItem<InferOutput<TItem4>, unknown, BaseIssue<unknown>>, const TItem6 extends PipeItem<InferOutput<TItem5>, unknown, BaseIssue<unknown>>, const TItem7 extends PipeItem<InferOutput<TItem6>, unknown, BaseIssue<unknown>>, const TItem8 extends PipeItem<InferOutput<TItem7>, unknown, BaseIssue<unknown>>, const TItem9 extends PipeItem<InferOutput<TItem8>, unknown, BaseIssue<unknown>>, const TItem10 extends PipeItem<InferOutput<TItem9>, unknown, BaseIssue<unknown>>, const TItem11 extends PipeItem<InferOutput<TItem10>, unknown, BaseIssue<unknown>>, const TItem12 extends PipeItem<InferOutput<TItem11>, unknown, BaseIssue<unknown>>, const TItem13 extends PipeItem<InferOutput<TItem12>, unknown, BaseIssue<unknown>>, const TItem14 extends PipeItem<InferOutput<TItem13>, unknown, BaseIssue<unknown>>, const TItem15 extends PipeItem<InferOutput<TItem14>, unknown, BaseIssue<unknown>>, const TItem16 extends PipeItem<InferOutput<TItem15>, unknown, BaseIssue<unknown>>>(schema: TSchema, item1: TItem1 | PipeAction<InferOutput<TSchema>, InferOutput<TItem1>, InferIssue<TItem1>>, item2: TItem2 | PipeAction<InferOutput<TItem1>, InferOutput<TItem2>, InferIssue<TItem2>>, item3: TItem3 | PipeAction<InferOutput<TItem2>, InferOutput<TItem3>, InferIssue<TItem3>>, item4: TItem4 | PipeAction<InferOutput<TItem3>, InferOutput<TItem4>, InferIssue<TItem4>>, item5: TItem5 | PipeAction<InferOutput<TItem4>, InferOutput<TItem5>, InferIssue<TItem5>>, item6: TItem6 | PipeAction<InferOutput<TItem5>, InferOutput<TItem6>, InferIssue<TItem6>>, item7: TItem7 | PipeAction<InferOutput<TItem6>, InferOutput<TItem7>, InferIssue<TItem7>>, item8: TItem8 | PipeAction<InferOutput<TItem7>, InferOutput<TItem8>, InferIssue<TItem8>>, item9: TItem9 | PipeAction<InferOutput<TItem8>, InferOutput<TItem9>, InferIssue<TItem9>>, item10: TItem10 | PipeAction<InferOutput<TItem9>, InferOutput<TItem10>, InferIssue<TItem10>>, item11: TItem11 | PipeAction<InferOutput<TItem10>, InferOutput<TItem11>, InferIssue<TItem11>>, item12: TItem12 | PipeAction<InferOutput<TItem11>, InferOutput<TItem12>, InferIssue<TItem12>>, item13: TItem13 | PipeAction<InferOutput<TItem12>, InferOutput<TItem13>, InferIssue<TItem13>>, item14: TItem14 | PipeAction<InferOutput<TItem13>, InferOutput<TItem14>, InferIssue<TItem14>>, item15: TItem15 | PipeAction<InferOutput<TItem14>, InferOutput<TItem15>, InferIssue<TItem15>>, item16: TItem16 | PipeAction<InferOutput<TItem15>, InferOutput<TItem16>, InferIssue<TItem16>>): SchemaWithPipe<readonly [
    TSchema,
    TItem1,
    TItem2,
    TItem3,
    TItem4,
    TItem5,
    TItem6,
    TItem7,
    TItem8,
    TItem9,
    TItem10,
    TItem11,
    TItem12,
    TItem13,
    TItem14,
    TItem15,
    TItem16
]>;
declare function pipe<const TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TItem1 extends PipeItem<InferOutput<TSchema>, unknown, BaseIssue<unknown>>, const TItem2 extends PipeItem<InferOutput<TItem1>, unknown, BaseIssue<unknown>>, const TItem3 extends PipeItem<InferOutput<TItem2>, unknown, BaseIssue<unknown>>, const TItem4 extends PipeItem<InferOutput<TItem3>, unknown, BaseIssue<unknown>>, const TItem5 extends PipeItem<InferOutput<TItem4>, unknown, BaseIssue<unknown>>, const TItem6 extends PipeItem<InferOutput<TItem5>, unknown, BaseIssue<unknown>>, const TItem7 extends PipeItem<InferOutput<TItem6>, unknown, BaseIssue<unknown>>, const TItem8 extends PipeItem<InferOutput<TItem7>, unknown, BaseIssue<unknown>>, const TItem9 extends PipeItem<InferOutput<TItem8>, unknown, BaseIssue<unknown>>, const TItem10 extends PipeItem<InferOutput<TItem9>, unknown, BaseIssue<unknown>>, const TItem11 extends PipeItem<InferOutput<TItem10>, unknown, BaseIssue<unknown>>, const TItem12 extends PipeItem<InferOutput<TItem11>, unknown, BaseIssue<unknown>>, const TItem13 extends PipeItem<InferOutput<TItem12>, unknown, BaseIssue<unknown>>, const TItem14 extends PipeItem<InferOutput<TItem13>, unknown, BaseIssue<unknown>>, const TItem15 extends PipeItem<InferOutput<TItem14>, unknown, BaseIssue<unknown>>, const TItem16 extends PipeItem<InferOutput<TItem15>, unknown, BaseIssue<unknown>>, const TItem17 extends PipeItem<InferOutput<TItem16>, unknown, BaseIssue<unknown>>>(schema: TSchema, item1: TItem1 | PipeAction<InferOutput<TSchema>, InferOutput<TItem1>, InferIssue<TItem1>>, item2: TItem2 | PipeAction<InferOutput<TItem1>, InferOutput<TItem2>, InferIssue<TItem2>>, item3: TItem3 | PipeAction<InferOutput<TItem2>, InferOutput<TItem3>, InferIssue<TItem3>>, item4: TItem4 | PipeAction<InferOutput<TItem3>, InferOutput<TItem4>, InferIssue<TItem4>>, item5: TItem5 | PipeAction<InferOutput<TItem4>, InferOutput<TItem5>, InferIssue<TItem5>>, item6: TItem6 | PipeAction<InferOutput<TItem5>, InferOutput<TItem6>, InferIssue<TItem6>>, item7: TItem7 | PipeAction<InferOutput<TItem6>, InferOutput<TItem7>, InferIssue<TItem7>>, item8: TItem8 | PipeAction<InferOutput<TItem7>, InferOutput<TItem8>, InferIssue<TItem8>>, item9: TItem9 | PipeAction<InferOutput<TItem8>, InferOutput<TItem9>, InferIssue<TItem9>>, item10: TItem10 | PipeAction<InferOutput<TItem9>, InferOutput<TItem10>, InferIssue<TItem10>>, item11: TItem11 | PipeAction<InferOutput<TItem10>, InferOutput<TItem11>, InferIssue<TItem11>>, item12: TItem12 | PipeAction<InferOutput<TItem11>, InferOutput<TItem12>, InferIssue<TItem12>>, item13: TItem13 | PipeAction<InferOutput<TItem12>, InferOutput<TItem13>, InferIssue<TItem13>>, item14: TItem14 | PipeAction<InferOutput<TItem13>, InferOutput<TItem14>, InferIssue<TItem14>>, item15: TItem15 | PipeAction<InferOutput<TItem14>, InferOutput<TItem15>, InferIssue<TItem15>>, item16: TItem16 | PipeAction<InferOutput<TItem15>, InferOutput<TItem16>, InferIssue<TItem16>>, item17: TItem17 | PipeAction<InferOutput<TItem16>, InferOutput<TItem17>, InferIssue<TItem17>>): SchemaWithPipe<readonly [
    TSchema,
    TItem1,
    TItem2,
    TItem3,
    TItem4,
    TItem5,
    TItem6,
    TItem7,
    TItem8,
    TItem9,
    TItem10,
    TItem11,
    TItem12,
    TItem13,
    TItem14,
    TItem15,
    TItem16,
    TItem17
]>;
declare function pipe<const TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TItem1 extends PipeItem<InferOutput<TSchema>, unknown, BaseIssue<unknown>>, const TItem2 extends PipeItem<InferOutput<TItem1>, unknown, BaseIssue<unknown>>, const TItem3 extends PipeItem<InferOutput<TItem2>, unknown, BaseIssue<unknown>>, const TItem4 extends PipeItem<InferOutput<TItem3>, unknown, BaseIssue<unknown>>, const TItem5 extends PipeItem<InferOutput<TItem4>, unknown, BaseIssue<unknown>>, const TItem6 extends PipeItem<InferOutput<TItem5>, unknown, BaseIssue<unknown>>, const TItem7 extends PipeItem<InferOutput<TItem6>, unknown, BaseIssue<unknown>>, const TItem8 extends PipeItem<InferOutput<TItem7>, unknown, BaseIssue<unknown>>, const TItem9 extends PipeItem<InferOutput<TItem8>, unknown, BaseIssue<unknown>>, const TItem10 extends PipeItem<InferOutput<TItem9>, unknown, BaseIssue<unknown>>, const TItem11 extends PipeItem<InferOutput<TItem10>, unknown, BaseIssue<unknown>>, const TItem12 extends PipeItem<InferOutput<TItem11>, unknown, BaseIssue<unknown>>, const TItem13 extends PipeItem<InferOutput<TItem12>, unknown, BaseIssue<unknown>>, const TItem14 extends PipeItem<InferOutput<TItem13>, unknown, BaseIssue<unknown>>, const TItem15 extends PipeItem<InferOutput<TItem14>, unknown, BaseIssue<unknown>>, const TItem16 extends PipeItem<InferOutput<TItem15>, unknown, BaseIssue<unknown>>, const TItem17 extends PipeItem<InferOutput<TItem16>, unknown, BaseIssue<unknown>>, const TItem18 extends PipeItem<InferOutput<TItem17>, unknown, BaseIssue<unknown>>>(schema: TSchema, item1: TItem1 | PipeAction<InferOutput<TSchema>, InferOutput<TItem1>, InferIssue<TItem1>>, item2: TItem2 | PipeAction<InferOutput<TItem1>, InferOutput<TItem2>, InferIssue<TItem2>>, item3: TItem3 | PipeAction<InferOutput<TItem2>, InferOutput<TItem3>, InferIssue<TItem3>>, item4: TItem4 | PipeAction<InferOutput<TItem3>, InferOutput<TItem4>, InferIssue<TItem4>>, item5: TItem5 | PipeAction<InferOutput<TItem4>, InferOutput<TItem5>, InferIssue<TItem5>>, item6: TItem6 | PipeAction<InferOutput<TItem5>, InferOutput<TItem6>, InferIssue<TItem6>>, item7: TItem7 | PipeAction<InferOutput<TItem6>, InferOutput<TItem7>, InferIssue<TItem7>>, item8: TItem8 | PipeAction<InferOutput<TItem7>, InferOutput<TItem8>, InferIssue<TItem8>>, item9: TItem9 | PipeAction<InferOutput<TItem8>, InferOutput<TItem9>, InferIssue<TItem9>>, item10: TItem10 | PipeAction<InferOutput<TItem9>, InferOutput<TItem10>, InferIssue<TItem10>>, item11: TItem11 | PipeAction<InferOutput<TItem10>, InferOutput<TItem11>, InferIssue<TItem11>>, item12: TItem12 | PipeAction<InferOutput<TItem11>, InferOutput<TItem12>, InferIssue<TItem12>>, item13: TItem13 | PipeAction<InferOutput<TItem12>, InferOutput<TItem13>, InferIssue<TItem13>>, item14: TItem14 | PipeAction<InferOutput<TItem13>, InferOutput<TItem14>, InferIssue<TItem14>>, item15: TItem15 | PipeAction<InferOutput<TItem14>, InferOutput<TItem15>, InferIssue<TItem15>>, item16: TItem16 | PipeAction<InferOutput<TItem15>, InferOutput<TItem16>, InferIssue<TItem16>>, item17: TItem17 | PipeAction<InferOutput<TItem16>, InferOutput<TItem17>, InferIssue<TItem17>>, item18: TItem18 | PipeAction<InferOutput<TItem17>, InferOutput<TItem18>, InferIssue<TItem18>>): SchemaWithPipe<readonly [
    TSchema,
    TItem1,
    TItem2,
    TItem3,
    TItem4,
    TItem5,
    TItem6,
    TItem7,
    TItem8,
    TItem9,
    TItem10,
    TItem11,
    TItem12,
    TItem13,
    TItem14,
    TItem15,
    TItem16,
    TItem17,
    TItem18
]>;
declare function pipe<const TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TItem1 extends PipeItem<InferOutput<TSchema>, unknown, BaseIssue<unknown>>, const TItem2 extends PipeItem<InferOutput<TItem1>, unknown, BaseIssue<unknown>>, const TItem3 extends PipeItem<InferOutput<TItem2>, unknown, BaseIssue<unknown>>, const TItem4 extends PipeItem<InferOutput<TItem3>, unknown, BaseIssue<unknown>>, const TItem5 extends PipeItem<InferOutput<TItem4>, unknown, BaseIssue<unknown>>, const TItem6 extends PipeItem<InferOutput<TItem5>, unknown, BaseIssue<unknown>>, const TItem7 extends PipeItem<InferOutput<TItem6>, unknown, BaseIssue<unknown>>, const TItem8 extends PipeItem<InferOutput<TItem7>, unknown, BaseIssue<unknown>>, const TItem9 extends PipeItem<InferOutput<TItem8>, unknown, BaseIssue<unknown>>, const TItem10 extends PipeItem<InferOutput<TItem9>, unknown, BaseIssue<unknown>>, const TItem11 extends PipeItem<InferOutput<TItem10>, unknown, BaseIssue<unknown>>, const TItem12 extends PipeItem<InferOutput<TItem11>, unknown, BaseIssue<unknown>>, const TItem13 extends PipeItem<InferOutput<TItem12>, unknown, BaseIssue<unknown>>, const TItem14 extends PipeItem<InferOutput<TItem13>, unknown, BaseIssue<unknown>>, const TItem15 extends PipeItem<InferOutput<TItem14>, unknown, BaseIssue<unknown>>, const TItem16 extends PipeItem<InferOutput<TItem15>, unknown, BaseIssue<unknown>>, const TItem17 extends PipeItem<InferOutput<TItem16>, unknown, BaseIssue<unknown>>, const TItem18 extends PipeItem<InferOutput<TItem17>, unknown, BaseIssue<unknown>>, const TItem19 extends PipeItem<InferOutput<TItem18>, unknown, BaseIssue<unknown>>>(schema: TSchema, item1: TItem1 | PipeAction<InferOutput<TSchema>, InferOutput<TItem1>, InferIssue<TItem1>>, item2: TItem2 | PipeAction<InferOutput<TItem1>, InferOutput<TItem2>, InferIssue<TItem2>>, item3: TItem3 | PipeAction<InferOutput<TItem2>, InferOutput<TItem3>, InferIssue<TItem3>>, item4: TItem4 | PipeAction<InferOutput<TItem3>, InferOutput<TItem4>, InferIssue<TItem4>>, item5: TItem5 | PipeAction<InferOutput<TItem4>, InferOutput<TItem5>, InferIssue<TItem5>>, item6: TItem6 | PipeAction<InferOutput<TItem5>, InferOutput<TItem6>, InferIssue<TItem6>>, item7: TItem7 | PipeAction<InferOutput<TItem6>, InferOutput<TItem7>, InferIssue<TItem7>>, item8: TItem8 | PipeAction<InferOutput<TItem7>, InferOutput<TItem8>, InferIssue<TItem8>>, item9: TItem9 | PipeAction<InferOutput<TItem8>, InferOutput<TItem9>, InferIssue<TItem9>>, item10: TItem10 | PipeAction<InferOutput<TItem9>, InferOutput<TItem10>, InferIssue<TItem10>>, item11: TItem11 | PipeAction<InferOutput<TItem10>, InferOutput<TItem11>, InferIssue<TItem11>>, item12: TItem12 | PipeAction<InferOutput<TItem11>, InferOutput<TItem12>, InferIssue<TItem12>>, item13: TItem13 | PipeAction<InferOutput<TItem12>, InferOutput<TItem13>, InferIssue<TItem13>>, item14: TItem14 | PipeAction<InferOutput<TItem13>, InferOutput<TItem14>, InferIssue<TItem14>>, item15: TItem15 | PipeAction<InferOutput<TItem14>, InferOutput<TItem15>, InferIssue<TItem15>>, item16: TItem16 | PipeAction<InferOutput<TItem15>, InferOutput<TItem16>, InferIssue<TItem16>>, item17: TItem17 | PipeAction<InferOutput<TItem16>, InferOutput<TItem17>, InferIssue<TItem17>>, item18: TItem18 | PipeAction<InferOutput<TItem17>, InferOutput<TItem18>, InferIssue<TItem18>>, item19: TItem19 | PipeAction<InferOutput<TItem18>, InferOutput<TItem19>, InferIssue<TItem19>>): SchemaWithPipe<readonly [
    TSchema,
    TItem1,
    TItem2,
    TItem3,
    TItem4,
    TItem5,
    TItem6,
    TItem7,
    TItem8,
    TItem9,
    TItem10,
    TItem11,
    TItem12,
    TItem13,
    TItem14,
    TItem15,
    TItem16,
    TItem17,
    TItem18,
    TItem19
]>;
declare function pipe<const TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TItems$1 extends readonly PipeItem<InferOutput<TSchema>, InferOutput<TSchema>, BaseIssue<unknown>>[]>(schema: TSchema, ...items: TItems$1): SchemaWithPipe<readonly [
    TSchema,
    ...TItems$1
]>;
type SchemaWithPipeAsync<TPipe$1 extends readonly [
    (BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>),
    ...(PipeItem<any, unknown, BaseIssue<unknown>> | PipeItemAsync<any, unknown, BaseIssue<unknown>>)[]
]> = Omit<FirstTupleItem<TPipe$1>, "async" | "pipe" | "~standard" | "~run" | "~types"> & {
    readonly pipe: TPipe$1;
    readonly async: true;
    readonly "~standard": StandardProps<InferInput<FirstTupleItem<TPipe$1>>, InferOutput<LastTupleItem<TPipe$1>>>;
    readonly "~run": (dataset: UnknownDataset, config: Config<BaseIssue<unknown>>) => Promise<OutputDataset<InferOutput<LastTupleItem<TPipe$1>>, InferIssue<TPipe$1[number]>>>;
    readonly "~types"?: {
        readonly input: InferInput<FirstTupleItem<TPipe$1>>;
        readonly output: InferOutput<LastTupleItem<TPipe$1>>;
        readonly issue: InferIssue<TPipe$1[number]>;
    } | undefined;
};
declare function parse<const TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>>>(schema: TSchema, input: unknown, config?: Config<InferIssue<TSchema>>): InferOutput<TSchema>;
type Schema$8 = SchemaWithoutPipe<LooseObjectSchema<ObjectEntries, ErrorMessage<LooseObjectIssue> | undefined> | ObjectSchema<ObjectEntries, ErrorMessage<ObjectIssue> | undefined> | ObjectWithRestSchema<ObjectEntries, BaseSchema<unknown, unknown, BaseIssue<unknown>>, ErrorMessage<ObjectWithRestIssue> | undefined> | StrictObjectSchema<ObjectEntries, ErrorMessage<StrictObjectIssue> | undefined>>;
type PartialEntries$1<TEntries$1 extends ObjectEntries, TKeys extends readonly (keyof TEntries$1)[] | undefined> = {
    [TKey in keyof TEntries$1]: TKeys extends readonly (keyof TEntries$1)[] ? TKey extends TKeys[number] ? OptionalSchema<TEntries$1[TKey], undefined> : TEntries$1[TKey] : OptionalSchema<TEntries$1[TKey], undefined>;
};
type SchemaWithPartial<TSchema extends Schema$8, TKeys extends ObjectKeys<TSchema> | undefined> = TSchema extends ObjectSchema<infer TEntries, ErrorMessage<ObjectIssue> | undefined> | StrictObjectSchema<infer TEntries, ErrorMessage<StrictObjectIssue> | undefined> ? Omit<TSchema, "entries" | "~standard" | "~run" | "~types"> & {
    readonly entries: PartialEntries$1<TEntries, TKeys>;
    readonly "~standard": StandardProps<InferObjectInput<PartialEntries$1<TEntries, TKeys>>, InferObjectOutput<PartialEntries$1<TEntries, TKeys>>>;
    readonly "~run": (dataset: UnknownDataset, config: Config<BaseIssue<unknown>>) => OutputDataset<InferObjectOutput<PartialEntries$1<TEntries, TKeys>>, InferIssue<TSchema>>;
    readonly "~types"?: {
        readonly input: InferObjectInput<PartialEntries$1<TEntries, TKeys>>;
        readonly output: InferObjectOutput<PartialEntries$1<TEntries, TKeys>>;
        readonly issue: InferIssue<TSchema>;
    } | undefined;
} : TSchema extends LooseObjectSchema<infer TEntries, ErrorMessage<LooseObjectIssue> | undefined> ? Omit<TSchema, "entries" | "~standard" | "~run" | "~types"> & {
    readonly entries: PartialEntries$1<TEntries, TKeys>;
    readonly "~standard": StandardProps<InferObjectInput<PartialEntries$1<TEntries, TKeys>> & {
        [key: string]: unknown;
    }, InferObjectOutput<PartialEntries$1<TEntries, TKeys>> & {
        [key: string]: unknown;
    }>;
    readonly "~run": (dataset: UnknownDataset, config: Config<BaseIssue<unknown>>) => OutputDataset<InferObjectOutput<PartialEntries$1<TEntries, TKeys>> & {
        [key: string]: unknown;
    }, InferIssue<TSchema>>;
    readonly "~types"?: {
        readonly input: InferObjectInput<PartialEntries$1<TEntries, TKeys>> & {
            [key: string]: unknown;
        };
        readonly output: InferObjectOutput<PartialEntries$1<TEntries, TKeys>> & {
            [key: string]: unknown;
        };
        readonly issue: InferIssue<TSchema>;
    } | undefined;
} : TSchema extends ObjectWithRestSchema<infer TEntries, infer TRest, ErrorMessage<ObjectWithRestIssue> | undefined> ? Omit<TSchema, "entries" | "~standard" | "~run" | "~types"> & {
    readonly entries: PartialEntries$1<TEntries, TKeys>;
    readonly "~standard": StandardProps<InferObjectInput<PartialEntries$1<TEntries, TKeys>> & {
        [key: string]: InferInput<TRest>;
    }, InferObjectOutput<PartialEntries$1<TEntries, TKeys>> & {
        [key: string]: InferOutput<TRest>;
    }>;
    readonly "~run": (dataset: UnknownDataset, config: Config<BaseIssue<unknown>>) => OutputDataset<InferObjectOutput<PartialEntries$1<TEntries, TKeys>> & {
        [key: string]: InferOutput<TRest>;
    }, InferIssue<TSchema>>;
    readonly "~types"?: {
        readonly input: InferObjectInput<PartialEntries$1<TEntries, TKeys>> & {
            [key: string]: InferInput<TRest>;
        };
        readonly output: InferObjectOutput<PartialEntries$1<TEntries, TKeys>> & {
            [key: string]: InferOutput<TRest>;
        };
        readonly issue: InferIssue<TSchema>;
    } | undefined;
} : never;
declare function partial<const TSchema extends Schema$8>(schema: TSchema): SchemaWithPartial<TSchema, undefined>;
declare function partial<const TSchema extends Schema$8, const TKeys extends ObjectKeys<TSchema>>(schema: TSchema, keys: TKeys): SchemaWithPartial<TSchema, TKeys>;
type Schema$6 = SchemaWithoutPipe<LooseObjectSchema<ObjectEntries, ErrorMessage<LooseObjectIssue> | undefined> | LooseObjectSchemaAsync<ObjectEntriesAsync, ErrorMessage<LooseObjectIssue> | undefined> | ObjectSchema<ObjectEntries, ErrorMessage<ObjectIssue> | undefined> | ObjectSchemaAsync<ObjectEntriesAsync, ErrorMessage<ObjectIssue> | undefined> | ObjectWithRestSchema<ObjectEntries, BaseSchema<unknown, unknown, BaseIssue<unknown>>, ErrorMessage<ObjectWithRestIssue> | undefined> | ObjectWithRestSchemaAsync<ObjectEntriesAsync, BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, ErrorMessage<ObjectWithRestIssue> | undefined> | StrictObjectSchema<ObjectEntries, ErrorMessage<StrictObjectIssue> | undefined> | StrictObjectSchemaAsync<ObjectEntriesAsync, ErrorMessage<StrictObjectIssue> | undefined>>;
type SchemaWithPick<TSchema extends Schema$6, TKeys extends ObjectKeys<TSchema>> = TSchema extends ObjectSchema<infer TEntries, ErrorMessage<ObjectIssue> | undefined> | StrictObjectSchema<infer TEntries, ErrorMessage<StrictObjectIssue> | undefined> ? Omit<TSchema, "entries" | "~standard" | "~run" | "~types"> & {
    readonly entries: Pick<TEntries, TKeys[number]>;
    readonly "~standard": StandardProps<InferObjectInput<Pick<TEntries, TKeys[number]>>, InferObjectOutput<Pick<TEntries, TKeys[number]>>>;
    readonly "~run": (dataset: UnknownDataset, config: Config<BaseIssue<unknown>>) => OutputDataset<InferObjectOutput<Pick<TEntries, TKeys[number]>>, Extract<InferIssue<TSchema>, {
        type: TSchema["type"];
    }> | InferObjectIssue<Pick<TEntries, TKeys[number]>>>;
    readonly "~types"?: {
        readonly input: InferObjectInput<Pick<TEntries, TKeys[number]>>;
        readonly output: InferObjectOutput<Pick<TEntries, TKeys[number]>>;
        readonly issue: Extract<InferIssue<TSchema>, {
            type: TSchema["type"];
        }> | InferObjectIssue<Pick<TEntries, TKeys[number]>>;
    } | undefined;
} : TSchema extends ObjectSchemaAsync<infer TEntries, ErrorMessage<ObjectIssue> | undefined> | StrictObjectSchemaAsync<infer TEntries, ErrorMessage<StrictObjectIssue> | undefined> ? Omit<TSchema, "entries" | "~standard" | "~run" | "~types"> & {
    readonly entries: Pick<TEntries, TKeys[number]>;
    readonly "~standard": StandardProps<InferObjectInput<Pick<TEntries, TKeys[number]>>, InferObjectOutput<Pick<TEntries, TKeys[number]>>>;
    readonly "~run": (dataset: UnknownDataset, config: Config<BaseIssue<unknown>>) => Promise<OutputDataset<InferObjectOutput<Pick<TEntries, TKeys[number]>>, Extract<InferIssue<TSchema>, {
        type: TSchema["type"];
    }> | InferObjectIssue<Pick<TEntries, TKeys[number]>>>>;
    readonly "~types"?: {
        readonly input: InferObjectInput<Pick<TEntries, TKeys[number]>>;
        readonly output: InferObjectOutput<Pick<TEntries, TKeys[number]>>;
        readonly issue: Extract<InferIssue<TSchema>, {
            type: TSchema["type"];
        }> | InferObjectIssue<Pick<TEntries, TKeys[number]>>;
    } | undefined;
} : TSchema extends LooseObjectSchema<infer TEntries, ErrorMessage<LooseObjectIssue> | undefined> ? Omit<TSchema, "entries" | "~standard" | "~run" | "~types"> & {
    readonly entries: Pick<TEntries, TKeys[number]>;
    readonly "~standard": StandardProps<InferObjectInput<Pick<TEntries, TKeys[number]>> & {
        [key: string]: unknown;
    }, InferObjectOutput<Pick<TEntries, TKeys[number]>> & {
        [key: string]: unknown;
    }>;
    readonly "~run": (dataset: UnknownDataset, config: Config<BaseIssue<unknown>>) => OutputDataset<InferObjectOutput<Pick<TEntries, TKeys[number]>> & {
        [key: string]: unknown;
    }, Extract<InferIssue<TSchema>, {
        type: TSchema["type"];
    }> | InferObjectIssue<Pick<TEntries, TKeys[number]>>>;
    readonly "~types"?: {
        readonly input: InferObjectInput<Pick<TEntries, TKeys[number]>> & {
            [key: string]: unknown;
        };
        readonly output: InferObjectOutput<Pick<TEntries, TKeys[number]>> & {
            [key: string]: unknown;
        };
        readonly issue: Extract<InferIssue<TSchema>, {
            type: TSchema["type"];
        }> | InferObjectIssue<Pick<TEntries, TKeys[number]>>;
    } | undefined;
} : TSchema extends LooseObjectSchemaAsync<infer TEntries, ErrorMessage<LooseObjectIssue> | undefined> ? Omit<TSchema, "entries" | "~standard" | "~run" | "~types"> & {
    readonly entries: Pick<TEntries, TKeys[number]>;
    readonly "~standard": StandardProps<InferObjectInput<Pick<TEntries, TKeys[number]>> & {
        [key: string]: unknown;
    }, InferObjectOutput<Pick<TEntries, TKeys[number]>> & {
        [key: string]: unknown;
    }>;
    readonly "~run": (dataset: UnknownDataset, config: Config<BaseIssue<unknown>>) => Promise<OutputDataset<InferObjectOutput<Pick<TEntries, TKeys[number]>> & {
        [key: string]: unknown;
    }, Extract<InferIssue<TSchema>, {
        type: TSchema["type"];
    }> | InferObjectIssue<Pick<TEntries, TKeys[number]>>>>;
    readonly "~types"?: {
        readonly input: InferObjectInput<Pick<TEntries, TKeys[number]>> & {
            [key: string]: unknown;
        };
        readonly output: InferObjectOutput<Pick<TEntries, TKeys[number]>> & {
            [key: string]: unknown;
        };
        readonly issue: Extract<InferIssue<TSchema>, {
            type: TSchema["type"];
        }> | InferObjectIssue<Pick<TEntries, TKeys[number]>>;
    } | undefined;
} : TSchema extends ObjectWithRestSchema<infer TEntries, BaseSchema<unknown, unknown, BaseIssue<unknown>>, ErrorMessage<ObjectWithRestIssue> | undefined> ? Omit<TSchema, "entries" | "~standard" | "~run" | "~types"> & {
    readonly entries: Pick<TEntries, TKeys[number]>;
    readonly "~standard": StandardProps<InferObjectInput<Pick<TEntries, TKeys[number]>> & {
        [key: string]: InferInput<TSchema["rest"]>;
    }, InferObjectOutput<Pick<TEntries, TKeys[number]>> & {
        [key: string]: InferOutput<TSchema["rest"]>;
    }>;
    readonly "~run": (dataset: UnknownDataset, config: Config<BaseIssue<unknown>>) => OutputDataset<InferObjectOutput<Pick<TEntries, TKeys[number]>> & {
        [key: string]: InferOutput<TSchema["rest"]>;
    }, Extract<InferIssue<TSchema>, {
        type: TSchema["type"];
    }> | InferObjectIssue<Pick<TEntries, TKeys[number]>> | InferIssue<TSchema["rest"]>>;
    readonly "~types"?: {
        readonly input: InferObjectInput<Pick<TEntries, TKeys[number]>> & {
            [key: string]: InferInput<TSchema["rest"]>;
        };
        readonly output: InferObjectOutput<Pick<TEntries, TKeys[number]>> & {
            [key: string]: InferOutput<TSchema["rest"]>;
        };
        readonly issue: Extract<InferIssue<TSchema>, {
            type: TSchema["type"];
        }> | InferObjectIssue<Pick<TEntries, TKeys[number]>> | InferIssue<TSchema["rest"]>;
    } | undefined;
} : TSchema extends ObjectWithRestSchemaAsync<infer TEntries, BaseSchema<unknown, unknown, BaseIssue<unknown>>, ErrorMessage<ObjectWithRestIssue> | undefined> ? Omit<TSchema, "entries" | "~standard" | "~run" | "~types"> & {
    readonly entries: Pick<TEntries, TKeys[number]>;
    readonly "~standard": StandardProps<InferObjectInput<Pick<TEntries, TKeys[number]>> & {
        [key: string]: InferInput<TSchema["rest"]>;
    }, InferObjectOutput<Pick<TEntries, TKeys[number]>> & {
        [key: string]: InferOutput<TSchema["rest"]>;
    }>;
    readonly "~run": (dataset: UnknownDataset, config: Config<BaseIssue<unknown>>) => Promise<OutputDataset<InferObjectOutput<Pick<TEntries, TKeys[number]>> & {
        [key: string]: InferOutput<TSchema["rest"]>;
    }, Extract<InferIssue<TSchema>, {
        type: TSchema["type"];
    }> | InferObjectIssue<Pick<TEntries, TKeys[number]>> | InferIssue<TSchema["rest"]>>>;
    readonly "~types"?: {
        readonly input: InferObjectInput<Pick<TEntries, TKeys[number]>> & {
            [key: string]: InferInput<TSchema["rest"]>;
        };
        readonly output: InferObjectOutput<Pick<TEntries, TKeys[number]>> & {
            [key: string]: InferOutput<TSchema["rest"]>;
        };
        readonly issue: Extract<InferIssue<TSchema>, {
            type: TSchema["type"];
        }> | InferObjectIssue<Pick<TEntries, TKeys[number]>> | InferIssue<TSchema["rest"]>;
    } | undefined;
} : never;
declare function pick<const TSchema extends Schema$6, const TKeys extends ObjectKeys<TSchema>>(schema: TSchema, keys: TKeys): SchemaWithPick<TSchema, TKeys>;
type SafeParseResult<TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>> = {
    readonly typed: true;
    readonly success: true;
    readonly output: InferOutput<TSchema>;
    readonly issues: undefined;
} | {
    readonly typed: true;
    readonly success: false;
    readonly output: InferOutput<TSchema>;
    readonly issues: [
        InferIssue<TSchema>,
        ...InferIssue<TSchema>[]
    ];
} | {
    readonly typed: false;
    readonly success: false;
    readonly output: unknown;
    readonly issues: [
        InferIssue<TSchema>,
        ...InferIssue<TSchema>[]
    ];
};
declare function safeParse<const TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>>>(schema: TSchema, input: unknown, config?: Config<InferIssue<TSchema>>): SafeParseResult<TSchema>;
interface BaseMetadata<TInput$1> {
    readonly kind: "metadata";
    readonly type: string;
    readonly reference: (...args: any[]) => BaseMetadata<any>;
    readonly "~types"?: {
        readonly input: TInput$1;
        readonly output: TInput$1;
        readonly issue: never;
    } | undefined;
}
interface UnknownDataset {
    typed?: false;
    value: unknown;
    issues?: undefined;
}
interface SuccessDataset<TValue$1> {
    typed: true;
    value: TValue$1;
    issues?: undefined;
}
interface PartialDataset<TValue$1, TIssue extends BaseIssue<unknown>> {
    typed: true;
    value: TValue$1;
    issues: [
        TIssue,
        ...TIssue[]
    ];
}
interface FailureDataset<TIssue extends BaseIssue<unknown>> {
    typed: false;
    value: unknown;
    issues: [
        TIssue,
        ...TIssue[]
    ];
}
type OutputDataset<TValue$1, TIssue extends BaseIssue<unknown>> = SuccessDataset<TValue$1> | PartialDataset<TValue$1, TIssue> | FailureDataset<TIssue>;
interface StandardProps<TInput$1, TOutput$1> {
    readonly version: 1;
    readonly vendor: "valibot";
    readonly validate: (value: unknown) => StandardResult<TOutput$1> | Promise<StandardResult<TOutput$1>>;
    readonly types?: StandardTypes<TInput$1, TOutput$1> | undefined;
}
type StandardResult<TOutput$1> = StandardSuccessResult<TOutput$1> | StandardFailureResult;
interface StandardSuccessResult<TOutput$1> {
    readonly value: TOutput$1;
    readonly issues?: undefined;
}
interface StandardFailureResult {
    readonly issues: readonly StandardIssue[];
}
interface StandardIssue {
    readonly message: string;
    readonly path?: readonly (PropertyKey | StandardPathItem)[] | undefined;
}
interface StandardPathItem {
    readonly key: PropertyKey;
}
interface StandardTypes<TInput$1, TOutput$1> {
    readonly input: TInput$1;
    readonly output: TOutput$1;
}
interface BaseSchema<TInput$1, TOutput$1, TIssue extends BaseIssue<unknown>> {
    readonly kind: "schema";
    readonly type: string;
    readonly reference: (...args: any[]) => BaseSchema<unknown, unknown, BaseIssue<unknown>>;
    readonly expects: string;
    readonly async: false;
    readonly "~standard": StandardProps<TInput$1, TOutput$1>;
    readonly "~run": (dataset: UnknownDataset, config: Config<BaseIssue<unknown>>) => OutputDataset<TOutput$1, TIssue>;
    readonly "~types"?: {
        readonly input: TInput$1;
        readonly output: TOutput$1;
        readonly issue: TIssue;
    } | undefined;
}
interface BaseSchemaAsync<TInput$1, TOutput$1, TIssue extends BaseIssue<unknown>> extends Omit<BaseSchema<TInput$1, TOutput$1, TIssue>, "reference" | "async" | "~run"> {
    readonly reference: (...args: any[]) => BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>;
    readonly async: true;
    readonly "~run": (dataset: UnknownDataset, config: Config<BaseIssue<unknown>>) => Promise<OutputDataset<TOutput$1, TIssue>>;
}
interface BaseTransformation<TInput$1, TOutput$1, TIssue extends BaseIssue<unknown>> {
    readonly kind: "transformation";
    readonly type: string;
    readonly reference: (...args: any[]) => BaseTransformation<any, any, BaseIssue<unknown>>;
    readonly async: false;
    readonly "~run": (dataset: SuccessDataset<TInput$1>, config: Config<BaseIssue<unknown>>) => OutputDataset<TOutput$1, BaseIssue<unknown> | TIssue>;
    readonly "~types"?: {
        readonly input: TInput$1;
        readonly output: TOutput$1;
        readonly issue: TIssue;
    } | undefined;
}
interface BaseTransformationAsync<TInput$1, TOutput$1, TIssue extends BaseIssue<unknown>> extends Omit<BaseTransformation<TInput$1, TOutput$1, TIssue>, "reference" | "async" | "~run"> {
    readonly reference: (...args: any[]) => BaseTransformation<any, any, BaseIssue<unknown>> | BaseTransformationAsync<any, any, BaseIssue<unknown>>;
    readonly async: true;
    readonly "~run": (dataset: SuccessDataset<TInput$1>, config: Config<BaseIssue<unknown>>) => Promise<OutputDataset<TOutput$1, BaseIssue<unknown> | TIssue>>;
}
interface BaseValidation<TInput$1, TOutput$1, TIssue extends BaseIssue<unknown>> {
    readonly kind: "validation";
    readonly type: string;
    readonly reference: (...args: any[]) => BaseValidation<any, any, BaseIssue<unknown>>;
    readonly expects: string | null;
    readonly async: false;
    readonly "~run": (dataset: OutputDataset<TInput$1, BaseIssue<unknown>>, config: Config<BaseIssue<unknown>>) => OutputDataset<TOutput$1, BaseIssue<unknown> | TIssue>;
    readonly "~types"?: {
        readonly input: TInput$1;
        readonly output: TOutput$1;
        readonly issue: TIssue;
    } | undefined;
}
interface BaseValidationAsync<TInput$1, TOutput$1, TIssue extends BaseIssue<unknown>> extends Omit<BaseValidation<TInput$1, TOutput$1, TIssue>, "reference" | "async" | "~run"> {
    readonly reference: (...args: any[]) => BaseValidation<any, any, BaseIssue<unknown>> | BaseValidationAsync<any, any, BaseIssue<unknown>>;
    readonly async: true;
    readonly "~run": (dataset: OutputDataset<TInput$1, BaseIssue<unknown>>, config: Config<BaseIssue<unknown>>) => Promise<OutputDataset<TOutput$1, BaseIssue<unknown> | TIssue>>;
}
type InferInput<TItem$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>> | BaseValidation<any, unknown, BaseIssue<unknown>> | BaseValidationAsync<any, unknown, BaseIssue<unknown>> | BaseTransformation<any, unknown, BaseIssue<unknown>> | BaseTransformationAsync<any, unknown, BaseIssue<unknown>> | BaseMetadata<any>> = NonNullable<TItem$1["~types"]>["input"];
type InferOutput<TItem$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>> | BaseValidation<any, unknown, BaseIssue<unknown>> | BaseValidationAsync<any, unknown, BaseIssue<unknown>> | BaseTransformation<any, unknown, BaseIssue<unknown>> | BaseTransformationAsync<any, unknown, BaseIssue<unknown>> | BaseMetadata<any>> = NonNullable<TItem$1["~types"]>["output"];
type InferIssue<TItem$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>> | BaseValidation<any, unknown, BaseIssue<unknown>> | BaseValidationAsync<any, unknown, BaseIssue<unknown>> | BaseTransformation<any, unknown, BaseIssue<unknown>> | BaseTransformationAsync<any, unknown, BaseIssue<unknown>> | BaseMetadata<any>> = NonNullable<TItem$1["~types"]>["issue"];
type IsNever<Type> = [
    Type
] extends [
    never
] ? true : false;
type NonNullable$1<TValue$1> = TValue$1 extends null ? never : TValue$1;
type NonNullish<TValue$1> = TValue$1 extends null | undefined ? never : TValue$1;
type NonOptional<TValue$1> = TValue$1 extends undefined ? never : TValue$1;
type MaybeReadonly<TValue$1> = TValue$1 | Readonly<TValue$1>;
type DeepReadonly<TValue$1> = TValue$1 extends Record<string, unknown> | readonly unknown[] ? {
    readonly [TKey in keyof TValue$1]: DeepReadonly<TValue$1[TKey]>;
} : TValue$1;
type MaybeDeepReadonly<TValue$1> = TValue$1 | DeepReadonly<TValue$1>;
type MaybePromise<TValue$1> = TValue$1 | Promise<TValue$1>;
type Prettify<TObject> = {
    [TKey in keyof TObject]: TObject[TKey];
} & {};
type MarkOptional<TObject, TKeys extends keyof TObject> = {
    [TKey in keyof TObject]?: unknown;
} & Omit<TObject, TKeys> & Partial<Pick<TObject, TKeys>>;
type FirstTupleItem<TTuple extends readonly [
    unknown,
    ...unknown[]
]> = TTuple[0];
type LastTupleItem<TTuple extends readonly [
    unknown,
    ...unknown[]
]> = TTuple[TTuple extends readonly [
    unknown,
    ...infer TRest
] ? TRest["length"] : never];
type UnionToIntersect<TUnion> = (TUnion extends any ? (arg: TUnion) => void : never) extends ((arg: infer Intersect) => void) ? Intersect : never;
type ErrorMessage<TIssue extends BaseIssue<unknown>> = ((issue: TIssue) => string) | string;
type Default<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, TInput$1 extends null | undefined> = MaybeDeepReadonly<InferInput<TWrapped$1> | TInput$1> | ((dataset?: UnknownDataset, config?: Config<InferIssue<TWrapped$1>>) => MaybeDeepReadonly<InferInput<TWrapped$1> | TInput$1>) | undefined;
type DefaultAsync<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, TInput$1 extends null | undefined> = MaybeDeepReadonly<InferInput<TWrapped$1> | TInput$1> | ((dataset?: UnknownDataset, config?: Config<InferIssue<TWrapped$1>>) => MaybePromise<MaybeDeepReadonly<InferInput<TWrapped$1> | TInput$1>>) | undefined;
type DefaultValue<TDefault extends Default<BaseSchema<unknown, unknown, BaseIssue<unknown>>, null | undefined> | DefaultAsync<BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, null | undefined>> = TDefault extends DefaultAsync<infer TWrapped extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, infer TInput> ? TDefault extends ((dataset?: UnknownDataset, config?: Config<InferIssue<TWrapped>>) => MaybePromise<MaybeDeepReadonly<InferInput<TWrapped> | TInput>>) ? Awaited<ReturnType<TDefault>> : TDefault : never;
type OptionalEntrySchema = ExactOptionalSchema<BaseSchema<unknown, unknown, BaseIssue<unknown>>, unknown> | NullishSchema<BaseSchema<unknown, unknown, BaseIssue<unknown>>, unknown> | OptionalSchema<BaseSchema<unknown, unknown, BaseIssue<unknown>>, unknown>;
type OptionalEntrySchemaAsync = ExactOptionalSchemaAsync<BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, unknown> | NullishSchemaAsync<BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, unknown> | OptionalSchemaAsync<BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, unknown>;
interface ObjectEntries {
    [key: string]: BaseSchema<unknown, unknown, BaseIssue<unknown>> | SchemaWithFallback<BaseSchema<unknown, unknown, BaseIssue<unknown>>, unknown> | OptionalEntrySchema;
}
interface ObjectEntriesAsync {
    [key: string]: BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>> | SchemaWithFallback<BaseSchema<unknown, unknown, BaseIssue<unknown>>, unknown> | SchemaWithFallbackAsync<BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, unknown> | OptionalEntrySchema | OptionalEntrySchemaAsync;
}
type ObjectKeys<TSchema extends LooseObjectSchema<ObjectEntries, ErrorMessage<LooseObjectIssue> | undefined> | LooseObjectSchemaAsync<ObjectEntriesAsync, ErrorMessage<LooseObjectIssue> | undefined> | ObjectSchema<ObjectEntries, ErrorMessage<ObjectIssue> | undefined> | ObjectSchemaAsync<ObjectEntriesAsync, ErrorMessage<ObjectIssue> | undefined> | ObjectWithRestSchema<ObjectEntries, BaseSchema<unknown, unknown, BaseIssue<unknown>>, ErrorMessage<ObjectWithRestIssue> | undefined> | ObjectWithRestSchemaAsync<ObjectEntriesAsync, BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, ErrorMessage<ObjectWithRestIssue> | undefined> | StrictObjectSchema<ObjectEntries, ErrorMessage<StrictObjectIssue> | undefined> | StrictObjectSchemaAsync<ObjectEntriesAsync, ErrorMessage<StrictObjectIssue> | undefined>> = MaybeReadonly<[
    keyof TSchema["entries"],
    ...(keyof TSchema["entries"])[]
]>;
type InferEntriesInput<TEntries$1 extends ObjectEntries | ObjectEntriesAsync> = {
    -readonly [TKey in keyof TEntries$1]: InferInput<TEntries$1[TKey]>;
};
type InferEntriesOutput<TEntries$1 extends ObjectEntries | ObjectEntriesAsync> = {
    -readonly [TKey in keyof TEntries$1]: InferOutput<TEntries$1[TKey]>;
};
type OptionalInputKeys<TEntries$1 extends ObjectEntries | ObjectEntriesAsync> = {
    [TKey in keyof TEntries$1]: TEntries$1[TKey] extends OptionalEntrySchema | OptionalEntrySchemaAsync ? TKey : never;
}[keyof TEntries$1];
type OptionalOutputKeys<TEntries$1 extends ObjectEntries | ObjectEntriesAsync> = {
    [TKey in keyof TEntries$1]: TEntries$1[TKey] extends OptionalEntrySchema | OptionalEntrySchemaAsync ? undefined extends TEntries$1[TKey]["default"] ? TKey : never : never;
}[keyof TEntries$1];
type InputWithQuestionMarks<TEntries$1 extends ObjectEntries | ObjectEntriesAsync, TObject extends InferEntriesInput<TEntries$1>> = MarkOptional<TObject, OptionalInputKeys<TEntries$1>>;
type OutputWithQuestionMarks<TEntries$1 extends ObjectEntries | ObjectEntriesAsync, TObject extends InferEntriesOutput<TEntries$1>> = MarkOptional<TObject, OptionalOutputKeys<TEntries$1>>;
type ReadonlyOutputKeys<TEntries$1 extends ObjectEntries | ObjectEntriesAsync> = {
    [TKey in keyof TEntries$1]: TEntries$1[TKey] extends {
        readonly pipe: readonly unknown[];
    } ? ReadonlyAction<any> extends TEntries$1[TKey]["pipe"][number] ? TKey : never : never;
}[keyof TEntries$1];
type OutputWithReadonly<TEntries$1 extends ObjectEntries | ObjectEntriesAsync, TObject extends OutputWithQuestionMarks<TEntries$1, InferEntriesOutput<TEntries$1>>> = ReadonlyOutputKeys<TEntries$1> extends never ? TObject : Readonly<TObject> & Pick<TObject, Exclude<keyof TObject, ReadonlyOutputKeys<TEntries$1>>>;
type InferObjectInput<TEntries$1 extends ObjectEntries | ObjectEntriesAsync> = Prettify<InputWithQuestionMarks<TEntries$1, InferEntriesInput<TEntries$1>>>;
type InferObjectOutput<TEntries$1 extends ObjectEntries | ObjectEntriesAsync> = Prettify<OutputWithReadonly<TEntries$1, OutputWithQuestionMarks<TEntries$1, InferEntriesOutput<TEntries$1>>>>;
type InferObjectIssue<TEntries$1 extends ObjectEntries | ObjectEntriesAsync> = InferIssue<TEntries$1[keyof TEntries$1]>;
type TupleItems = MaybeReadonly<BaseSchema<unknown, unknown, BaseIssue<unknown>>[]>;
type TupleItemsAsync = MaybeReadonly<(BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>)[]>;
type InferTupleInput<TItems$1 extends TupleItems | TupleItemsAsync> = {
    -readonly [TKey in keyof TItems$1]: InferInput<TItems$1[TKey]>;
};
type InferTupleOutput<TItems$1 extends TupleItems | TupleItemsAsync> = {
    -readonly [TKey in keyof TItems$1]: InferOutput<TItems$1[TKey]>;
};
type InferTupleIssue<TItems$1 extends TupleItems | TupleItemsAsync> = InferIssue<TItems$1[number]>;
interface ArrayPathItem {
    readonly type: "array";
    readonly origin: "value";
    readonly input: MaybeReadonly<unknown[]>;
    readonly key: number;
    readonly value: unknown;
}
interface MapPathItem {
    readonly type: "map";
    readonly origin: "key" | "value";
    readonly input: Map<unknown, unknown>;
    readonly key: unknown;
    readonly value: unknown;
}
interface ObjectPathItem {
    readonly type: "object";
    readonly origin: "key" | "value";
    readonly input: Record<string, unknown>;
    readonly key: string;
    readonly value: unknown;
}
interface SetPathItem {
    readonly type: "set";
    readonly origin: "value";
    readonly input: Set<unknown>;
    readonly key: null;
    readonly value: unknown;
}
interface UnknownPathItem {
    readonly type: "unknown";
    readonly origin: "key" | "value";
    readonly input: unknown;
    readonly key: unknown;
    readonly value: unknown;
}
type IssuePathItem = ArrayPathItem | MapPathItem | ObjectPathItem | SetPathItem | UnknownPathItem;
interface BaseIssue<TInput$1> extends Config<BaseIssue<TInput$1>> {
    readonly kind: "schema" | "validation" | "transformation";
    readonly type: string;
    readonly input: TInput$1;
    readonly expected: string | null;
    readonly received: string;
    readonly message: string;
    readonly requirement?: unknown | undefined;
    readonly path?: [
        IssuePathItem,
        ...IssuePathItem[]
    ] | undefined;
    readonly issues?: [
        BaseIssue<TInput$1>,
        ...BaseIssue<TInput$1>[]
    ] | undefined;
}
type DotPath<TKey$1 extends string | number | symbol, TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>> = TKey$1 extends string | number ? `${TKey$1}` | `${TKey$1}.${IssueDotPath<TSchema>}` : never;
type ObjectPath<TEntries$1 extends ObjectEntries | ObjectEntriesAsync> = {
    [TKey in keyof TEntries$1]: DotPath<TKey, TEntries$1[TKey]>;
}[keyof TEntries$1];
type TupleKeys<TItems$1 extends TupleItems | TupleItemsAsync> = Exclude<keyof TItems$1, keyof [
]>;
type TuplePath<TItems$1 extends TupleItems | TupleItemsAsync> = {
    [TKey in TupleKeys<TItems$1>]: TItems$1[TKey] extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>> ? DotPath<TKey, TItems$1[TKey]> : never;
}[TupleKeys<TItems$1>];
type IssueDotPath<TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>> = TSchema extends SchemaWithPipe<infer TPipe> ? IssueDotPath<FirstTupleItem<TPipe>> : TSchema extends SchemaWithPipeAsync<infer TPipe> ? IssueDotPath<FirstTupleItem<TPipe>> : TSchema extends ArraySchema<infer TItem, ErrorMessage<ArrayIssue> | undefined> ? DotPath<number, TItem> : TSchema extends ArraySchemaAsync<infer TItem, ErrorMessage<ArrayIssue> | undefined> ? DotPath<number, TItem> : TSchema extends IntersectSchema<infer TOptions, ErrorMessage<IntersectIssue> | undefined> | UnionSchema<infer TOptions, ErrorMessage<UnionIssue<BaseIssue<unknown>>> | undefined> | VariantSchema<string, infer TOptions, ErrorMessage<VariantIssue> | undefined> ? IssueDotPath<TOptions[number]> : TSchema extends IntersectSchemaAsync<infer TOptions, ErrorMessage<IntersectIssue> | undefined> | UnionSchemaAsync<infer TOptions, ErrorMessage<UnionIssue<BaseIssue<unknown>>> | undefined> | VariantSchemaAsync<string, infer TOptions, ErrorMessage<VariantIssue> | undefined> ? IssueDotPath<TOptions[number]> : TSchema extends MapSchema<infer TKey, infer TValue, ErrorMessage<MapIssue> | undefined> | RecordSchema<infer TKey, infer TValue, ErrorMessage<RecordIssue> | undefined> ? DotPath<InferInput<TKey>, TValue> : TSchema extends MapSchemaAsync<infer TKey, infer TValue, ErrorMessage<MapIssue> | undefined> | RecordSchemaAsync<infer TKey, infer TValue, ErrorMessage<RecordIssue> | undefined> ? DotPath<InferInput<TKey>, TValue> : TSchema extends LooseObjectSchema<infer TEntries, ErrorMessage<LooseObjectIssue> | undefined> | ObjectSchema<infer TEntries, ErrorMessage<ObjectIssue> | undefined> | StrictObjectSchema<infer TEntries, ErrorMessage<StrictObjectIssue> | undefined> ? ObjectPath<TEntries> : TSchema extends LooseObjectSchemaAsync<infer TEntries, ErrorMessage<LooseObjectIssue> | undefined> | ObjectSchemaAsync<infer TEntries, ErrorMessage<ObjectIssue> | undefined> | StrictObjectSchemaAsync<infer TEntries, ErrorMessage<StrictObjectIssue> | undefined> ? ObjectPath<TEntries> : TSchema extends ObjectWithRestSchema<ObjectEntries, BaseSchema<unknown, unknown, BaseIssue<unknown>>, ErrorMessage<ObjectWithRestIssue> | undefined> ? string : TSchema extends ObjectWithRestSchemaAsync<ObjectEntriesAsync, BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, ErrorMessage<ObjectWithRestIssue> | undefined> ? string : TSchema extends SetSchema<infer TValue, ErrorMessage<SetIssue> | undefined> ? DotPath<number, TValue> : TSchema extends SetSchemaAsync<infer TValue, ErrorMessage<SetIssue> | undefined> ? DotPath<number, TValue> : TSchema extends LooseTupleSchema<infer TItems, ErrorMessage<LooseTupleIssue> | undefined> | StrictTupleSchema<infer TItems, ErrorMessage<StrictTupleIssue> | undefined> | TupleSchema<infer TItems, ErrorMessage<TupleIssue> | undefined> ? TuplePath<TItems> : TSchema extends LooseTupleSchemaAsync<infer TItems, ErrorMessage<LooseTupleIssue> | undefined> | StrictTupleSchemaAsync<infer TItems, ErrorMessage<StrictTupleIssue> | undefined> | TupleSchemaAsync<infer TItems, ErrorMessage<TupleIssue> | undefined> ? TuplePath<TItems> : TSchema extends TupleWithRestSchema<infer TItems, infer TRest, ErrorMessage<TupleWithRestIssue> | undefined> ? TuplePath<TItems> | DotPath<number, TRest> : TSchema extends TupleWithRestSchemaAsync<infer TItems, infer TRest, ErrorMessage<TupleWithRestIssue> | undefined> ? TuplePath<TItems> | DotPath<number, TRest> : TSchema extends ExactOptionalSchema<infer TWrapped, Default<BaseSchema<unknown, unknown, BaseIssue<unknown>>, never>> | LazySchema<infer TWrapped> | NonNullableSchema<infer TWrapped, ErrorMessage<NonNullableIssue> | undefined> | NonNullishSchema<infer TWrapped, ErrorMessage<NonNullishIssue> | undefined> | NonOptionalSchema<infer TWrapped, ErrorMessage<NonOptionalIssue> | undefined> | NullableSchema<infer TWrapped, Default<BaseSchema<unknown, unknown, BaseIssue<unknown>>, null>> | NullishSchema<infer TWrapped, Default<BaseSchema<unknown, unknown, BaseIssue<unknown>>, null | undefined>> | OptionalSchema<infer TWrapped, Default<BaseSchema<unknown, unknown, BaseIssue<unknown>>, undefined>> | UndefinedableSchema<infer TWrapped, Default<BaseSchema<unknown, unknown, BaseIssue<unknown>>, undefined>> ? IssueDotPath<TWrapped> : TSchema extends ExactOptionalSchemaAsync<infer TWrapped, DefaultAsync<BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, never>> | LazySchemaAsync<infer TWrapped> | NonNullableSchemaAsync<infer TWrapped, ErrorMessage<NonNullableIssue> | undefined> | NonNullishSchemaAsync<infer TWrapped, ErrorMessage<NonNullishIssue> | undefined> | NonOptionalSchemaAsync<infer TWrapped, ErrorMessage<NonOptionalIssue> | undefined> | NullableSchemaAsync<infer TWrapped, DefaultAsync<BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, null>> | NullishSchemaAsync<infer TWrapped, DefaultAsync<BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, null | undefined>> | OptionalSchemaAsync<infer TWrapped, DefaultAsync<BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, undefined>> | UndefinedableSchemaAsync<infer TWrapped, DefaultAsync<BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, undefined>> ? IssueDotPath<TWrapped> : never;
interface Config<TIssue extends BaseIssue<unknown>> {
    readonly lang?: string | undefined;
    readonly message?: ErrorMessage<TIssue> | undefined;
    readonly abortEarly?: boolean | undefined;
    readonly abortPipeEarly?: boolean | undefined;
}
type PipeAction<TInput$1, TOutput$1, TIssue extends BaseIssue<unknown>> = BaseValidation<TInput$1, TOutput$1, TIssue> | BaseTransformation<TInput$1, TOutput$1, TIssue> | BaseMetadata<TInput$1>;
type PipeActionAsync<TInput$1, TOutput$1, TIssue extends BaseIssue<unknown>> = BaseValidationAsync<TInput$1, TOutput$1, TIssue> | BaseTransformationAsync<TInput$1, TOutput$1, TIssue>;
type PipeItem<TInput$1, TOutput$1, TIssue extends BaseIssue<unknown>> = BaseSchema<TInput$1, TOutput$1, TIssue> | PipeAction<TInput$1, TOutput$1, TIssue>;
type PipeItemAsync<TInput$1, TOutput$1, TIssue extends BaseIssue<unknown>> = BaseSchemaAsync<TInput$1, TOutput$1, TIssue> | PipeActionAsync<TInput$1, TOutput$1, TIssue>;
type SchemaWithoutPipe<TSchema extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>> = TSchema & {
    pipe?: never;
};
interface ArrayIssue extends BaseIssue<unknown> {
    readonly kind: "schema";
    readonly type: "array";
    readonly expected: "Array";
}
interface ArraySchema<TItem$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, TMessage extends ErrorMessage<ArrayIssue> | undefined> extends BaseSchema<InferInput<TItem$1>[], InferOutput<TItem$1>[], ArrayIssue | InferIssue<TItem$1>> {
    readonly type: "array";
    readonly reference: typeof array;
    readonly expects: "Array";
    readonly item: TItem$1;
    readonly message: TMessage;
}
declare function array<const TItem$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>>(item: TItem$1): ArraySchema<TItem$1, undefined>;
declare function array<const TItem$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TMessage extends ErrorMessage<ArrayIssue> | undefined>(item: TItem$1, message: TMessage): ArraySchema<TItem$1, TMessage>;
interface ArraySchemaAsync<TItem$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, TMessage extends ErrorMessage<ArrayIssue> | undefined> extends BaseSchemaAsync<InferInput<TItem$1>[], InferOutput<TItem$1>[], ArrayIssue | InferIssue<TItem$1>> {
    readonly type: "array";
    readonly reference: typeof array | typeof arrayAsync;
    readonly expects: "Array";
    readonly item: TItem$1;
    readonly message: TMessage;
}
declare function arrayAsync<const TItem$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>>(item: TItem$1): ArraySchemaAsync<TItem$1, undefined>;
declare function arrayAsync<const TItem$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, const TMessage extends ErrorMessage<ArrayIssue> | undefined>(item: TItem$1, message: TMessage): ArraySchemaAsync<TItem$1, TMessage>;
interface BooleanIssue extends BaseIssue<unknown> {
    readonly kind: "schema";
    readonly type: "boolean";
    readonly expected: "boolean";
}
interface BooleanSchema<TMessage extends ErrorMessage<BooleanIssue> | undefined> extends BaseSchema<boolean, boolean, BooleanIssue> {
    readonly type: "boolean";
    readonly reference: typeof boolean;
    readonly expects: "boolean";
    readonly message: TMessage;
}
declare function boolean(): BooleanSchema<undefined>;
declare function boolean<const TMessage extends ErrorMessage<BooleanIssue> | undefined>(message: TMessage): BooleanSchema<TMessage>;
interface ExactOptionalSchema<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, TDefault extends Default<TWrapped$1, never>> extends BaseSchema<InferInput<TWrapped$1>, InferOutput<TWrapped$1>, InferIssue<TWrapped$1>> {
    readonly type: "exact_optional";
    readonly reference: typeof exactOptional;
    readonly expects: TWrapped$1["expects"];
    readonly wrapped: TWrapped$1;
    readonly default: TDefault;
}
declare function exactOptional<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>>(wrapped: TWrapped$1): ExactOptionalSchema<TWrapped$1, undefined>;
declare function exactOptional<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TDefault extends Default<TWrapped$1, never>>(wrapped: TWrapped$1, default_: TDefault): ExactOptionalSchema<TWrapped$1, TDefault>;
interface ExactOptionalSchemaAsync<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, TDefault extends DefaultAsync<TWrapped$1, never>> extends BaseSchemaAsync<InferInput<TWrapped$1>, InferOutput<TWrapped$1>, InferIssue<TWrapped$1>> {
    readonly type: "exact_optional";
    readonly reference: typeof exactOptional | typeof exactOptionalAsync;
    readonly expects: TWrapped$1["expects"];
    readonly wrapped: TWrapped$1;
    readonly default: TDefault;
}
declare function exactOptionalAsync<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>>(wrapped: TWrapped$1): ExactOptionalSchemaAsync<TWrapped$1, undefined>;
declare function exactOptionalAsync<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, const TDefault extends DefaultAsync<TWrapped$1, never>>(wrapped: TWrapped$1, default_: TDefault): ExactOptionalSchemaAsync<TWrapped$1, TDefault>;
interface IntersectIssue extends BaseIssue<unknown> {
    readonly kind: "schema";
    readonly type: "intersect";
    readonly expected: string;
}
type IntersectOptions = MaybeReadonly<BaseSchema<unknown, unknown, BaseIssue<unknown>>[]>;
type IntersectOptionsAsync = MaybeReadonly<(BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>)[]>;
type InferOption<TInput$1, TOutput$1> = BaseSchema<TInput$1, TOutput$1, BaseIssue<unknown>> | BaseSchemaAsync<TInput$1, TOutput$1, BaseIssue<unknown>>;
type InferIntersectInput<TOptions$1 extends IntersectOptions | IntersectOptionsAsync> = TOptions$1 extends readonly [
    InferOption<infer TInput, unknown>,
    ...infer TRest
] ? TRest extends readonly [
    InferOption<unknown, unknown>,
    ...InferOption<unknown, unknown>[]
] ? TInput & InferIntersectInput<TRest> : TInput : IsNever<TOptions$1[number]> extends true ? never : UnionToIntersect<InferInput<TOptions$1[number]>>;
type InferIntersectOutput<TOptions$1 extends IntersectOptions | IntersectOptionsAsync> = TOptions$1 extends readonly [
    InferOption<unknown, infer TOutput>,
    ...infer TRest
] ? TRest extends readonly [
    InferOption<unknown, unknown>,
    ...InferOption<unknown, unknown>[]
] ? TOutput & InferIntersectOutput<TRest> : TOutput : IsNever<TOptions$1[number]> extends true ? never : UnionToIntersect<InferOutput<TOptions$1[number]>>;
interface IntersectSchema<TOptions$1 extends IntersectOptions, TMessage extends ErrorMessage<IntersectIssue> | undefined> extends BaseSchema<InferIntersectInput<TOptions$1>, InferIntersectOutput<TOptions$1>, IntersectIssue | InferIssue<TOptions$1[number]>> {
    readonly type: "intersect";
    readonly reference: typeof intersect;
    readonly options: TOptions$1;
    readonly message: TMessage;
}
declare function intersect<const TOptions$1 extends IntersectOptions>(options: TOptions$1): IntersectSchema<TOptions$1, undefined>;
declare function intersect<const TOptions$1 extends IntersectOptions, const TMessage extends ErrorMessage<IntersectIssue> | undefined>(options: TOptions$1, message: TMessage): IntersectSchema<TOptions$1, TMessage>;
interface IntersectSchemaAsync<TOptions$1 extends IntersectOptionsAsync, TMessage extends ErrorMessage<IntersectIssue> | undefined> extends BaseSchemaAsync<InferIntersectInput<TOptions$1>, InferIntersectOutput<TOptions$1>, IntersectIssue | InferIssue<TOptions$1[number]>> {
    readonly type: "intersect";
    readonly reference: typeof intersect | typeof intersectAsync;
    readonly options: TOptions$1;
    readonly message: TMessage;
}
declare function intersectAsync<const TOptions$1 extends IntersectOptionsAsync>(options: TOptions$1): IntersectSchemaAsync<TOptions$1, undefined>;
declare function intersectAsync<const TOptions$1 extends IntersectOptionsAsync, const TMessage extends ErrorMessage<IntersectIssue> | undefined>(options: TOptions$1, message: TMessage): IntersectSchemaAsync<TOptions$1, TMessage>;
interface LazySchema<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>> extends BaseSchema<InferInput<TWrapped$1>, InferOutput<TWrapped$1>, InferIssue<TWrapped$1>> {
    readonly type: "lazy";
    readonly reference: typeof lazy;
    readonly expects: "unknown";
    readonly getter: (input: unknown) => TWrapped$1;
}
declare function lazy<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>>(getter: (input: unknown) => TWrapped$1): LazySchema<TWrapped$1>;
interface LazySchemaAsync<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>> extends BaseSchemaAsync<InferInput<TWrapped$1>, InferOutput<TWrapped$1>, InferIssue<TWrapped$1>> {
    readonly type: "lazy";
    readonly reference: typeof lazy | typeof lazyAsync;
    readonly expects: "unknown";
    readonly getter: (input: unknown) => MaybePromise<TWrapped$1>;
}
declare function lazyAsync<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>>(getter: (input: unknown) => MaybePromise<TWrapped$1>): LazySchemaAsync<TWrapped$1>;
type Literal = bigint | boolean | number | string | symbol;
interface LiteralIssue extends BaseIssue<unknown> {
    readonly kind: "schema";
    readonly type: "literal";
    readonly expected: string;
}
interface LiteralSchema<TLiteral extends Literal, TMessage extends ErrorMessage<LiteralIssue> | undefined> extends BaseSchema<TLiteral, TLiteral, LiteralIssue> {
    readonly type: "literal";
    readonly reference: typeof literal;
    readonly literal: TLiteral;
    readonly message: TMessage;
}
declare function literal<const TLiteral extends Literal>(literal_: TLiteral): LiteralSchema<TLiteral, undefined>;
declare function literal<const TLiteral extends Literal, const TMessage extends ErrorMessage<LiteralIssue> | undefined>(literal_: TLiteral, message: TMessage): LiteralSchema<TLiteral, TMessage>;
interface LooseObjectIssue extends BaseIssue<unknown> {
    readonly kind: "schema";
    readonly type: "loose_object";
    readonly expected: "Object" | `"${string}"`;
}
interface LooseObjectSchema<TEntries$1 extends ObjectEntries, TMessage extends ErrorMessage<LooseObjectIssue> | undefined> extends BaseSchema<InferObjectInput<TEntries$1> & {
    [key: string]: unknown;
}, InferObjectOutput<TEntries$1> & {
    [key: string]: unknown;
}, LooseObjectIssue | InferObjectIssue<TEntries$1>> {
    readonly type: "loose_object";
    readonly reference: typeof looseObject;
    readonly expects: "Object";
    readonly entries: TEntries$1;
    readonly message: TMessage;
}
declare function looseObject<const TEntries$1 extends ObjectEntries>(entries: TEntries$1): LooseObjectSchema<TEntries$1, undefined>;
declare function looseObject<const TEntries$1 extends ObjectEntries, const TMessage extends ErrorMessage<LooseObjectIssue> | undefined>(entries: TEntries$1, message: TMessage): LooseObjectSchema<TEntries$1, TMessage>;
interface LooseObjectSchemaAsync<TEntries$1 extends ObjectEntriesAsync, TMessage extends ErrorMessage<LooseObjectIssue> | undefined> extends BaseSchemaAsync<InferObjectInput<TEntries$1> & {
    [key: string]: unknown;
}, InferObjectOutput<TEntries$1> & {
    [key: string]: unknown;
}, LooseObjectIssue | InferObjectIssue<TEntries$1>> {
    readonly type: "loose_object";
    readonly reference: typeof looseObject | typeof looseObjectAsync;
    readonly expects: "Object";
    readonly entries: TEntries$1;
    readonly message: TMessage;
}
declare function looseObjectAsync<const TEntries$1 extends ObjectEntriesAsync>(entries: TEntries$1): LooseObjectSchemaAsync<TEntries$1, undefined>;
declare function looseObjectAsync<const TEntries$1 extends ObjectEntriesAsync, const TMessage extends ErrorMessage<LooseObjectIssue> | undefined>(entries: TEntries$1, message: TMessage): LooseObjectSchemaAsync<TEntries$1, TMessage>;
interface LooseTupleIssue extends BaseIssue<unknown> {
    readonly kind: "schema";
    readonly type: "loose_tuple";
    readonly expected: "Array";
}
interface LooseTupleSchema<TItems$1 extends TupleItems, TMessage extends ErrorMessage<LooseTupleIssue> | undefined> extends BaseSchema<[
    ...InferTupleInput<TItems$1>,
    ...unknown[]
], [
    ...InferTupleOutput<TItems$1>,
    ...unknown[]
], LooseTupleIssue | InferTupleIssue<TItems$1>> {
    readonly type: "loose_tuple";
    readonly reference: typeof looseTuple;
    readonly expects: "Array";
    readonly items: TItems$1;
    readonly message: TMessage;
}
declare function looseTuple<const TItems$1 extends TupleItems>(items: TItems$1): LooseTupleSchema<TItems$1, undefined>;
declare function looseTuple<const TItems$1 extends TupleItems, const TMessage extends ErrorMessage<LooseTupleIssue> | undefined>(items: TItems$1, message: TMessage): LooseTupleSchema<TItems$1, TMessage>;
interface LooseTupleSchemaAsync<TItems$1 extends TupleItemsAsync, TMessage extends ErrorMessage<LooseTupleIssue> | undefined> extends BaseSchemaAsync<[
    ...InferTupleInput<TItems$1>,
    ...unknown[]
], [
    ...InferTupleOutput<TItems$1>,
    ...unknown[]
], LooseTupleIssue | InferTupleIssue<TItems$1>> {
    readonly type: "loose_tuple";
    readonly reference: typeof looseTuple | typeof looseTupleAsync;
    readonly expects: "Array";
    readonly items: TItems$1;
    readonly message: TMessage;
}
declare function looseTupleAsync<const TItems$1 extends TupleItemsAsync>(items: TItems$1): LooseTupleSchemaAsync<TItems$1, undefined>;
declare function looseTupleAsync<const TItems$1 extends TupleItemsAsync, const TMessage extends ErrorMessage<LooseTupleIssue> | undefined>(items: TItems$1, message: TMessage): LooseTupleSchemaAsync<TItems$1, TMessage>;
interface MapIssue extends BaseIssue<unknown> {
    readonly kind: "schema";
    readonly type: "map";
    readonly expected: "Map";
}
type InferMapInput<TKey$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, TValue$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>> = Map<InferInput<TKey$1>, InferInput<TValue$1>>;
type InferMapOutput<TKey$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, TValue$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>> = Map<InferOutput<TKey$1>, InferOutput<TValue$1>>;
interface MapSchema<TKey$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, TValue$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, TMessage extends ErrorMessage<MapIssue> | undefined> extends BaseSchema<InferMapInput<TKey$1, TValue$1>, InferMapOutput<TKey$1, TValue$1>, MapIssue | InferIssue<TKey$1> | InferIssue<TValue$1>> {
    readonly type: "map";
    readonly reference: typeof map;
    readonly expects: "Map";
    readonly key: TKey$1;
    readonly value: TValue$1;
    readonly message: TMessage;
}
declare function map<const TKey$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TValue$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>>(key: TKey$1, value: TValue$1): MapSchema<TKey$1, TValue$1, undefined>;
declare function map<const TKey$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TValue$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TMessage extends ErrorMessage<MapIssue> | undefined>(key: TKey$1, value: TValue$1, message: TMessage): MapSchema<TKey$1, TValue$1, TMessage>;
interface MapSchemaAsync<TKey$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, TValue$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, TMessage extends ErrorMessage<MapIssue> | undefined> extends BaseSchemaAsync<InferMapInput<TKey$1, TValue$1>, InferMapOutput<TKey$1, TValue$1>, MapIssue | InferIssue<TKey$1> | InferIssue<TValue$1>> {
    readonly type: "map";
    readonly reference: typeof map | typeof mapAsync;
    readonly expects: "Map";
    readonly key: TKey$1;
    readonly value: TValue$1;
    readonly message: TMessage;
}
declare function mapAsync<const TKey$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, const TValue$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>>(key: TKey$1, value: TValue$1): MapSchemaAsync<TKey$1, TValue$1, undefined>;
declare function mapAsync<const TKey$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, const TValue$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, const TMessage extends ErrorMessage<MapIssue> | undefined>(key: TKey$1, value: TValue$1, message: TMessage): MapSchemaAsync<TKey$1, TValue$1, TMessage>;
interface UnionIssue<TSubIssue extends BaseIssue<unknown>> extends BaseIssue<unknown> {
    readonly kind: "schema";
    readonly type: "union";
    readonly expected: string;
    readonly issues?: [
        TSubIssue,
        ...TSubIssue[]
    ];
}
type UnionOptions = MaybeReadonly<BaseSchema<unknown, unknown, BaseIssue<unknown>>[]>;
interface UnionSchema<TOptions$1 extends UnionOptions, TMessage extends ErrorMessage<UnionIssue<InferIssue<TOptions$1[number]>>> | undefined> extends BaseSchema<InferInput<TOptions$1[number]>, InferOutput<TOptions$1[number]>, UnionIssue<InferIssue<TOptions$1[number]>> | InferIssue<TOptions$1[number]>> {
    readonly type: "union";
    readonly reference: typeof union;
    readonly options: TOptions$1;
    readonly message: TMessage;
}
declare function union<const TOptions$1 extends UnionOptions>(options: TOptions$1): UnionSchema<TOptions$1, undefined>;
declare function union<const TOptions$1 extends UnionOptions, const TMessage extends ErrorMessage<UnionIssue<InferIssue<TOptions$1[number]>>> | undefined>(options: TOptions$1, message: TMessage): UnionSchema<TOptions$1, TMessage>;
type UnionOptionsAsync = MaybeReadonly<(BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>)[]>;
interface UnionSchemaAsync<TOptions$1 extends UnionOptionsAsync, TMessage extends ErrorMessage<UnionIssue<InferIssue<TOptions$1[number]>>> | undefined> extends BaseSchemaAsync<InferInput<TOptions$1[number]>, InferOutput<TOptions$1[number]>, UnionIssue<InferIssue<TOptions$1[number]>> | InferIssue<TOptions$1[number]>> {
    readonly type: "union";
    readonly reference: typeof union | typeof unionAsync;
    readonly options: TOptions$1;
    readonly message: TMessage;
}
declare function unionAsync<const TOptions$1 extends UnionOptionsAsync>(options: TOptions$1): UnionSchemaAsync<TOptions$1, undefined>;
declare function unionAsync<const TOptions$1 extends UnionOptionsAsync, const TMessage extends ErrorMessage<UnionIssue<InferIssue<TOptions$1[number]>>> | undefined>(options: TOptions$1, message: TMessage): UnionSchemaAsync<TOptions$1, TMessage>;
interface NonNullableIssue extends BaseIssue<unknown> {
    readonly kind: "schema";
    readonly type: "non_nullable";
    readonly expected: "!null";
}
type InferNonNullableInput<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>> = NonNullable$1<InferInput<TWrapped$1>>;
type InferNonNullableOutput<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>> = NonNullable$1<InferOutput<TWrapped$1>>;
type InferNonNullableIssue<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>> = TWrapped$1 extends UnionSchema<UnionOptions, ErrorMessage<UnionIssue<BaseIssue<unknown>>> | undefined> | UnionSchemaAsync<UnionOptionsAsync, ErrorMessage<UnionIssue<BaseIssue<unknown>>> | undefined> ? Exclude<InferIssue<TWrapped$1>, {
    type: "null" | "union";
}> | UnionIssue<InferNonNullableIssue<TWrapped$1["options"][number]>> : Exclude<InferIssue<TWrapped$1>, {
    type: "null";
}>;
interface NonNullableSchema<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, TMessage extends ErrorMessage<NonNullableIssue> | undefined> extends BaseSchema<InferNonNullableInput<TWrapped$1>, InferNonNullableOutput<TWrapped$1>, NonNullableIssue | InferNonNullableIssue<TWrapped$1>> {
    readonly type: "non_nullable";
    readonly reference: typeof nonNullable;
    readonly expects: "!null";
    readonly wrapped: TWrapped$1;
    readonly message: TMessage;
}
declare function nonNullable<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>>(wrapped: TWrapped$1): NonNullableSchema<TWrapped$1, undefined>;
declare function nonNullable<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TMessage extends ErrorMessage<NonNullableIssue> | undefined>(wrapped: TWrapped$1, message: TMessage): NonNullableSchema<TWrapped$1, TMessage>;
interface NonNullableSchemaAsync<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, TMessage extends ErrorMessage<NonNullableIssue> | undefined> extends BaseSchemaAsync<InferNonNullableInput<TWrapped$1>, InferNonNullableOutput<TWrapped$1>, NonNullableIssue | InferNonNullableIssue<TWrapped$1>> {
    readonly type: "non_nullable";
    readonly reference: typeof nonNullable | typeof nonNullableAsync;
    readonly expects: "!null";
    readonly wrapped: TWrapped$1;
    readonly message: TMessage;
}
declare function nonNullableAsync<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>>(wrapped: TWrapped$1): NonNullableSchemaAsync<TWrapped$1, undefined>;
declare function nonNullableAsync<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, const TMessage extends ErrorMessage<NonNullableIssue> | undefined>(wrapped: TWrapped$1, message: TMessage): NonNullableSchemaAsync<TWrapped$1, TMessage>;
interface NonNullishIssue extends BaseIssue<unknown> {
    readonly kind: "schema";
    readonly type: "non_nullish";
    readonly expected: "(!null & !undefined)";
}
type InferNonNullishInput<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>> = NonNullish<InferInput<TWrapped$1>>;
type InferNonNullishOutput<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>> = NonNullish<InferOutput<TWrapped$1>>;
type InferNonNullishIssue<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>> = TWrapped$1 extends UnionSchema<UnionOptions, ErrorMessage<UnionIssue<BaseIssue<unknown>>> | undefined> | UnionSchemaAsync<UnionOptionsAsync, ErrorMessage<UnionIssue<BaseIssue<unknown>>> | undefined> ? Exclude<InferIssue<TWrapped$1>, {
    type: "null" | "undefined" | "union";
}> | UnionIssue<InferNonNullishIssue<TWrapped$1["options"][number]>> : Exclude<InferIssue<TWrapped$1>, {
    type: "null" | "undefined";
}>;
interface NonNullishSchema<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, TMessage extends ErrorMessage<NonNullishIssue> | undefined> extends BaseSchema<InferNonNullishInput<TWrapped$1>, InferNonNullishOutput<TWrapped$1>, NonNullishIssue | InferNonNullishIssue<TWrapped$1>> {
    readonly type: "non_nullish";
    readonly reference: typeof nonNullish;
    readonly expects: "(!null & !undefined)";
    readonly wrapped: TWrapped$1;
    readonly message: TMessage;
}
declare function nonNullish<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>>(wrapped: TWrapped$1): NonNullishSchema<TWrapped$1, undefined>;
declare function nonNullish<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TMessage extends ErrorMessage<NonNullishIssue> | undefined>(wrapped: TWrapped$1, message: TMessage): NonNullishSchema<TWrapped$1, TMessage>;
interface NonNullishSchemaAsync<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, TMessage extends ErrorMessage<NonNullishIssue> | undefined> extends BaseSchemaAsync<InferNonNullishInput<TWrapped$1>, InferNonNullishOutput<TWrapped$1>, NonNullishIssue | InferNonNullishIssue<TWrapped$1>> {
    readonly type: "non_nullish";
    readonly reference: typeof nonNullish | typeof nonNullishAsync;
    readonly expects: "(!null & !undefined)";
    readonly wrapped: TWrapped$1;
    readonly message: TMessage;
}
declare function nonNullishAsync<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>>(wrapped: TWrapped$1): NonNullishSchemaAsync<TWrapped$1, undefined>;
declare function nonNullishAsync<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, const TMessage extends ErrorMessage<NonNullishIssue> | undefined>(wrapped: TWrapped$1, message: TMessage): NonNullishSchemaAsync<TWrapped$1, TMessage>;
interface NonOptionalIssue extends BaseIssue<unknown> {
    readonly kind: "schema";
    readonly type: "non_optional";
    readonly expected: "!undefined";
}
type InferNonOptionalInput<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>> = NonOptional<InferInput<TWrapped$1>>;
type InferNonOptionalOutput<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>> = NonOptional<InferOutput<TWrapped$1>>;
type InferNonOptionalIssue<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>> = TWrapped$1 extends UnionSchema<UnionOptions, ErrorMessage<UnionIssue<BaseIssue<unknown>>> | undefined> | UnionSchemaAsync<UnionOptionsAsync, ErrorMessage<UnionIssue<BaseIssue<unknown>>> | undefined> ? Exclude<InferIssue<TWrapped$1>, {
    type: "undefined" | "union";
}> | UnionIssue<InferNonOptionalIssue<TWrapped$1["options"][number]>> : Exclude<InferIssue<TWrapped$1>, {
    type: "undefined";
}>;
interface NonOptionalSchema<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, TMessage extends ErrorMessage<NonOptionalIssue> | undefined> extends BaseSchema<InferNonOptionalInput<TWrapped$1>, InferNonOptionalOutput<TWrapped$1>, NonOptionalIssue | InferNonOptionalIssue<TWrapped$1>> {
    readonly type: "non_optional";
    readonly reference: typeof nonOptional;
    readonly expects: "!undefined";
    readonly wrapped: TWrapped$1;
    readonly message: TMessage;
}
declare function nonOptional<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>>(wrapped: TWrapped$1): NonOptionalSchema<TWrapped$1, undefined>;
declare function nonOptional<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TMessage extends ErrorMessage<NonOptionalIssue> | undefined>(wrapped: TWrapped$1, message: TMessage): NonOptionalSchema<TWrapped$1, TMessage>;
interface NonOptionalSchemaAsync<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, TMessage extends ErrorMessage<NonOptionalIssue> | undefined> extends BaseSchemaAsync<InferNonOptionalInput<TWrapped$1>, InferNonOptionalOutput<TWrapped$1>, NonOptionalIssue | InferNonOptionalIssue<TWrapped$1>> {
    readonly type: "non_optional";
    readonly reference: typeof nonOptional | typeof nonOptionalAsync;
    readonly expects: "!undefined";
    readonly wrapped: TWrapped$1;
    readonly message: TMessage;
}
declare function nonOptionalAsync<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>>(wrapped: TWrapped$1): NonOptionalSchemaAsync<TWrapped$1, undefined>;
declare function nonOptionalAsync<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, const TMessage extends ErrorMessage<NonOptionalIssue> | undefined>(wrapped: TWrapped$1, message: TMessage): NonOptionalSchemaAsync<TWrapped$1, TMessage>;
type InferNullableOutput<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, TDefault extends DefaultAsync<TWrapped$1, null>> = undefined extends TDefault ? InferOutput<TWrapped$1> | null : InferOutput<TWrapped$1> | Extract<DefaultValue<TDefault>, null>;
interface NullableSchema<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, TDefault extends Default<TWrapped$1, null>> extends BaseSchema<InferInput<TWrapped$1> | null, InferNullableOutput<TWrapped$1, TDefault>, InferIssue<TWrapped$1>> {
    readonly type: "nullable";
    readonly reference: typeof nullable;
    readonly expects: `(${TWrapped$1["expects"]} | null)`;
    readonly wrapped: TWrapped$1;
    readonly default: TDefault;
}
declare function nullable<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>>(wrapped: TWrapped$1): NullableSchema<TWrapped$1, undefined>;
declare function nullable<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TDefault extends Default<TWrapped$1, null>>(wrapped: TWrapped$1, default_: TDefault): NullableSchema<TWrapped$1, TDefault>;
interface NullableSchemaAsync<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, TDefault extends DefaultAsync<TWrapped$1, null>> extends BaseSchemaAsync<InferInput<TWrapped$1> | null, InferNullableOutput<TWrapped$1, TDefault>, InferIssue<TWrapped$1>> {
    readonly type: "nullable";
    readonly reference: typeof nullable | typeof nullableAsync;
    readonly expects: `(${TWrapped$1["expects"]} | null)`;
    readonly wrapped: TWrapped$1;
    readonly default: TDefault;
}
declare function nullableAsync<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>>(wrapped: TWrapped$1): NullableSchemaAsync<TWrapped$1, undefined>;
declare function nullableAsync<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, const TDefault extends DefaultAsync<TWrapped$1, null>>(wrapped: TWrapped$1, default_: TDefault): NullableSchemaAsync<TWrapped$1, TDefault>;
type InferNullishOutput<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, TDefault extends DefaultAsync<TWrapped$1, null | undefined>> = undefined extends TDefault ? InferOutput<TWrapped$1> | null | undefined : InferOutput<TWrapped$1> | Extract<DefaultValue<TDefault>, null | undefined>;
interface NullishSchema<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, TDefault extends Default<TWrapped$1, null | undefined>> extends BaseSchema<InferInput<TWrapped$1> | null | undefined, InferNullishOutput<TWrapped$1, TDefault>, InferIssue<TWrapped$1>> {
    readonly type: "nullish";
    readonly reference: typeof nullish;
    readonly expects: `(${TWrapped$1["expects"]} | null | undefined)`;
    readonly wrapped: TWrapped$1;
    readonly default: TDefault;
}
declare function nullish<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>>(wrapped: TWrapped$1): NullishSchema<TWrapped$1, undefined>;
declare function nullish<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TDefault extends Default<TWrapped$1, null | undefined>>(wrapped: TWrapped$1, default_: TDefault): NullishSchema<TWrapped$1, TDefault>;
interface NullishSchemaAsync<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, TDefault extends DefaultAsync<TWrapped$1, null | undefined>> extends BaseSchemaAsync<InferInput<TWrapped$1> | null | undefined, InferNullishOutput<TWrapped$1, TDefault>, InferIssue<TWrapped$1>> {
    readonly type: "nullish";
    readonly reference: typeof nullish | typeof nullishAsync;
    readonly expects: `(${TWrapped$1["expects"]} | null | undefined)`;
    readonly wrapped: TWrapped$1;
    readonly default: TDefault;
}
declare function nullishAsync<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>>(wrapped: TWrapped$1): NullishSchemaAsync<TWrapped$1, undefined>;
declare function nullishAsync<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, const TDefault extends DefaultAsync<TWrapped$1, null | undefined>>(wrapped: TWrapped$1, default_: TDefault): NullishSchemaAsync<TWrapped$1, TDefault>;
interface NumberIssue extends BaseIssue<unknown> {
    readonly kind: "schema";
    readonly type: "number";
    readonly expected: "number";
}
interface NumberSchema<TMessage extends ErrorMessage<NumberIssue> | undefined> extends BaseSchema<number, number, NumberIssue> {
    readonly type: "number";
    readonly reference: typeof number;
    readonly expects: "number";
    readonly message: TMessage;
}
declare function number(): NumberSchema<undefined>;
declare function number<const TMessage extends ErrorMessage<NumberIssue> | undefined>(message: TMessage): NumberSchema<TMessage>;
interface ObjectIssue extends BaseIssue<unknown> {
    readonly kind: "schema";
    readonly type: "object";
    readonly expected: "Object" | `"${string}"`;
}
interface ObjectSchema<TEntries$1 extends ObjectEntries, TMessage extends ErrorMessage<ObjectIssue> | undefined> extends BaseSchema<InferObjectInput<TEntries$1>, InferObjectOutput<TEntries$1>, ObjectIssue | InferObjectIssue<TEntries$1>> {
    readonly type: "object";
    readonly reference: typeof object;
    readonly expects: "Object";
    readonly entries: TEntries$1;
    readonly message: TMessage;
}
declare function object<const TEntries$1 extends ObjectEntries>(entries: TEntries$1): ObjectSchema<TEntries$1, undefined>;
declare function object<const TEntries$1 extends ObjectEntries, const TMessage extends ErrorMessage<ObjectIssue> | undefined>(entries: TEntries$1, message: TMessage): ObjectSchema<TEntries$1, TMessage>;
interface ObjectSchemaAsync<TEntries$1 extends ObjectEntriesAsync, TMessage extends ErrorMessage<ObjectIssue> | undefined> extends BaseSchemaAsync<InferObjectInput<TEntries$1>, InferObjectOutput<TEntries$1>, ObjectIssue | InferObjectIssue<TEntries$1>> {
    readonly type: "object";
    readonly reference: typeof object | typeof objectAsync;
    readonly expects: "Object";
    readonly entries: TEntries$1;
    readonly message: TMessage;
}
declare function objectAsync<const TEntries$1 extends ObjectEntriesAsync>(entries: TEntries$1): ObjectSchemaAsync<TEntries$1, undefined>;
declare function objectAsync<const TEntries$1 extends ObjectEntriesAsync, const TMessage extends ErrorMessage<ObjectIssue> | undefined>(entries: TEntries$1, message: TMessage): ObjectSchemaAsync<TEntries$1, TMessage>;
interface ObjectWithRestIssue extends BaseIssue<unknown> {
    readonly kind: "schema";
    readonly type: "object_with_rest";
    readonly expected: "Object" | `"${string}"`;
}
interface ObjectWithRestSchema<TEntries$1 extends ObjectEntries, TRest$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, TMessage extends ErrorMessage<ObjectWithRestIssue> | undefined> extends BaseSchema<InferObjectInput<TEntries$1> & {
    [key: string]: InferInput<TRest$1>;
}, InferObjectOutput<TEntries$1> & {
    [key: string]: InferOutput<TRest$1>;
}, ObjectWithRestIssue | InferObjectIssue<TEntries$1> | InferIssue<TRest$1>> {
    readonly type: "object_with_rest";
    readonly reference: typeof objectWithRest;
    readonly expects: "Object";
    readonly entries: TEntries$1;
    readonly rest: TRest$1;
    readonly message: TMessage;
}
declare function objectWithRest<const TEntries$1 extends ObjectEntries, const TRest$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>>(entries: TEntries$1, rest: TRest$1): ObjectWithRestSchema<TEntries$1, TRest$1, undefined>;
declare function objectWithRest<const TEntries$1 extends ObjectEntries, const TRest$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TMessage extends ErrorMessage<ObjectWithRestIssue> | undefined>(entries: TEntries$1, rest: TRest$1, message: TMessage): ObjectWithRestSchema<TEntries$1, TRest$1, TMessage>;
interface ObjectWithRestSchemaAsync<TEntries$1 extends ObjectEntriesAsync, TRest$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, TMessage extends ErrorMessage<ObjectWithRestIssue> | undefined> extends BaseSchemaAsync<InferObjectInput<TEntries$1> & {
    [key: string]: InferInput<TRest$1>;
}, InferObjectOutput<TEntries$1> & {
    [key: string]: InferOutput<TRest$1>;
}, ObjectWithRestIssue | InferObjectIssue<TEntries$1> | InferIssue<TRest$1>> {
    readonly type: "object_with_rest";
    readonly reference: typeof objectWithRest | typeof objectWithRestAsync;
    readonly expects: "Object";
    readonly entries: TEntries$1;
    readonly rest: TRest$1;
    readonly message: TMessage;
}
declare function objectWithRestAsync<const TEntries$1 extends ObjectEntriesAsync, const TRest$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>>(entries: TEntries$1, rest: TRest$1): ObjectWithRestSchemaAsync<TEntries$1, TRest$1, undefined>;
declare function objectWithRestAsync<const TEntries$1 extends ObjectEntriesAsync, const TRest$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, const TMessage extends ErrorMessage<ObjectWithRestIssue> | undefined>(entries: TEntries$1, rest: TRest$1, message: TMessage): ObjectWithRestSchemaAsync<TEntries$1, TRest$1, TMessage>;
type InferOptionalOutput<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, TDefault extends DefaultAsync<TWrapped$1, undefined>> = undefined extends TDefault ? InferOutput<TWrapped$1> | undefined : InferOutput<TWrapped$1> | Extract<DefaultValue<TDefault>, undefined>;
interface OptionalSchema<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, TDefault extends Default<TWrapped$1, undefined>> extends BaseSchema<InferInput<TWrapped$1> | undefined, InferOptionalOutput<TWrapped$1, TDefault>, InferIssue<TWrapped$1>> {
    readonly type: "optional";
    readonly reference: typeof optional;
    readonly expects: `(${TWrapped$1["expects"]} | undefined)`;
    readonly wrapped: TWrapped$1;
    readonly default: TDefault;
}
declare function optional<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>>(wrapped: TWrapped$1): OptionalSchema<TWrapped$1, undefined>;
declare function optional<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TDefault extends Default<TWrapped$1, undefined>>(wrapped: TWrapped$1, default_: TDefault): OptionalSchema<TWrapped$1, TDefault>;
interface OptionalSchemaAsync<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, TDefault extends DefaultAsync<TWrapped$1, undefined>> extends BaseSchemaAsync<InferInput<TWrapped$1> | undefined, InferOptionalOutput<TWrapped$1, TDefault>, InferIssue<TWrapped$1>> {
    readonly type: "optional";
    readonly reference: typeof optional | typeof optionalAsync;
    readonly expects: `(${TWrapped$1["expects"]} | undefined)`;
    readonly wrapped: TWrapped$1;
    readonly default: TDefault;
}
declare function optionalAsync<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>>(wrapped: TWrapped$1): OptionalSchemaAsync<TWrapped$1, undefined>;
declare function optionalAsync<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, const TDefault extends DefaultAsync<TWrapped$1, undefined>>(wrapped: TWrapped$1, default_: TDefault): OptionalSchemaAsync<TWrapped$1, TDefault>;
type PicklistOptions = MaybeReadonly<(string | number | bigint)[]>;
interface PicklistIssue extends BaseIssue<unknown> {
    readonly kind: "schema";
    readonly type: "picklist";
    readonly expected: string;
}
interface PicklistSchema<TOptions$1 extends PicklistOptions, TMessage extends ErrorMessage<PicklistIssue> | undefined> extends BaseSchema<TOptions$1[number], TOptions$1[number], PicklistIssue> {
    readonly type: "picklist";
    readonly reference: typeof picklist;
    readonly options: TOptions$1;
    readonly message: TMessage;
}
declare function picklist<const TOptions$1 extends PicklistOptions>(options: TOptions$1): PicklistSchema<TOptions$1, undefined>;
declare function picklist<const TOptions$1 extends PicklistOptions, const TMessage extends ErrorMessage<PicklistIssue> | undefined>(options: TOptions$1, message: TMessage): PicklistSchema<TOptions$1, TMessage>;
interface RecordIssue extends BaseIssue<unknown> {
    readonly kind: "schema";
    readonly type: "record";
    readonly expected: "Object";
}
type IsLiteral<TKey$1 extends string | number | symbol> = string extends TKey$1 ? false : number extends TKey$1 ? false : symbol extends TKey$1 ? false : TKey$1 extends Brand<string | number | symbol> ? false : true;
type OptionalKeys<TObject extends Record<string | number | symbol, unknown>> = {
    [TKey in keyof TObject]: IsLiteral<TKey> extends true ? TKey : never;
}[keyof TObject];
type WithQuestionMarks<TObject extends Record<string | number | symbol, unknown>> = MarkOptional<TObject, OptionalKeys<TObject>>;
type WithReadonly<TValue$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, TObject extends WithQuestionMarks<Record<string | number | symbol, unknown>>> = TValue$1 extends {
    readonly pipe: readonly unknown[];
} ? ReadonlyAction<any> extends TValue$1["pipe"][number] ? Readonly<TObject> : TObject : TObject;
type InferRecordInput<TKey$1 extends BaseSchema<string, string | number | symbol, BaseIssue<unknown>> | BaseSchemaAsync<string, string | number | symbol, BaseIssue<unknown>>, TValue$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>> = Prettify<WithQuestionMarks<Record<InferInput<TKey$1>, InferInput<TValue$1>>>>;
type InferRecordOutput<TKey$1 extends BaseSchema<string, string | number | symbol, BaseIssue<unknown>> | BaseSchemaAsync<string, string | number | symbol, BaseIssue<unknown>>, TValue$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>> = Prettify<WithReadonly<TValue$1, WithQuestionMarks<Record<InferOutput<TKey$1>, InferOutput<TValue$1>>>>>;
interface RecordSchema<TKey$1 extends BaseSchema<string, string | number | symbol, BaseIssue<unknown>>, TValue$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, TMessage extends ErrorMessage<RecordIssue> | undefined> extends BaseSchema<InferRecordInput<TKey$1, TValue$1>, InferRecordOutput<TKey$1, TValue$1>, RecordIssue | InferIssue<TKey$1> | InferIssue<TValue$1>> {
    readonly type: "record";
    readonly reference: typeof record;
    readonly expects: "Object";
    readonly key: TKey$1;
    readonly value: TValue$1;
    readonly message: TMessage;
}
declare function record<const TKey$1 extends BaseSchema<string, string | number | symbol, BaseIssue<unknown>>, const TValue$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>>(key: TKey$1, value: TValue$1): RecordSchema<TKey$1, TValue$1, undefined>;
declare function record<const TKey$1 extends BaseSchema<string, string | number | symbol, BaseIssue<unknown>>, const TValue$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TMessage extends ErrorMessage<RecordIssue> | undefined>(key: TKey$1, value: TValue$1, message: TMessage): RecordSchema<TKey$1, TValue$1, TMessage>;
interface RecordSchemaAsync<TKey$1 extends BaseSchema<string, string | number | symbol, BaseIssue<unknown>> | BaseSchemaAsync<string, string | number | symbol, BaseIssue<unknown>>, TValue$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, TMessage extends ErrorMessage<RecordIssue> | undefined> extends BaseSchemaAsync<InferRecordInput<TKey$1, TValue$1>, InferRecordOutput<TKey$1, TValue$1>, RecordIssue | InferIssue<TKey$1> | InferIssue<TValue$1>> {
    readonly type: "record";
    readonly reference: typeof record | typeof recordAsync;
    readonly expects: "Object";
    readonly key: TKey$1;
    readonly value: TValue$1;
    readonly message: TMessage;
}
declare function recordAsync<const TKey$1 extends BaseSchema<string, string | number | symbol, BaseIssue<unknown>> | BaseSchemaAsync<string, string | number | symbol, BaseIssue<unknown>>, const TValue$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>>(key: TKey$1, value: TValue$1): RecordSchemaAsync<TKey$1, TValue$1, undefined>;
declare function recordAsync<const TKey$1 extends BaseSchema<string, string | number | symbol, BaseIssue<unknown>> | BaseSchemaAsync<string, string | number | symbol, BaseIssue<unknown>>, const TValue$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, const TMessage extends ErrorMessage<RecordIssue> | undefined>(key: TKey$1, value: TValue$1, message: TMessage): RecordSchemaAsync<TKey$1, TValue$1, TMessage>;
interface SetIssue extends BaseIssue<unknown> {
    readonly kind: "schema";
    readonly type: "set";
    readonly expected: "Set";
}
type InferSetInput<TValue$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>> = Set<InferInput<TValue$1>>;
type InferSetOutput<TValue$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>> = Set<InferOutput<TValue$1>>;
interface SetSchema<TValue$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, TMessage extends ErrorMessage<SetIssue> | undefined> extends BaseSchema<InferSetInput<TValue$1>, InferSetOutput<TValue$1>, SetIssue | InferIssue<TValue$1>> {
    readonly type: "set";
    readonly reference: typeof set;
    readonly expects: "Set";
    readonly value: TValue$1;
    readonly message: TMessage;
}
declare function set<const TValue$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>>(value: TValue$1): SetSchema<TValue$1, undefined>;
declare function set<const TValue$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TMessage extends ErrorMessage<SetIssue> | undefined>(value: TValue$1, message: TMessage): SetSchema<TValue$1, TMessage>;
interface SetSchemaAsync<TValue$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, TMessage extends ErrorMessage<SetIssue> | undefined> extends BaseSchemaAsync<InferSetInput<TValue$1>, InferSetOutput<TValue$1>, SetIssue | InferIssue<TValue$1>> {
    readonly type: "set";
    readonly reference: typeof set | typeof setAsync;
    readonly expects: "Set";
    readonly value: TValue$1;
    readonly message: TMessage;
}
declare function setAsync<const TValue$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>>(value: TValue$1): SetSchemaAsync<TValue$1, undefined>;
declare function setAsync<const TValue$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, const TMessage extends ErrorMessage<SetIssue> | undefined>(value: TValue$1, message: TMessage): SetSchemaAsync<TValue$1, TMessage>;
interface StrictObjectIssue extends BaseIssue<unknown> {
    readonly kind: "schema";
    readonly type: "strict_object";
    readonly expected: "Object" | `"${string}"` | "never";
}
interface StrictObjectSchema<TEntries$1 extends ObjectEntries, TMessage extends ErrorMessage<StrictObjectIssue> | undefined> extends BaseSchema<InferObjectInput<TEntries$1>, InferObjectOutput<TEntries$1>, StrictObjectIssue | InferObjectIssue<TEntries$1>> {
    readonly type: "strict_object";
    readonly reference: typeof strictObject;
    readonly expects: "Object";
    readonly entries: TEntries$1;
    readonly message: TMessage;
}
declare function strictObject<const TEntries$1 extends ObjectEntries>(entries: TEntries$1): StrictObjectSchema<TEntries$1, undefined>;
declare function strictObject<const TEntries$1 extends ObjectEntries, const TMessage extends ErrorMessage<StrictObjectIssue> | undefined>(entries: TEntries$1, message: TMessage): StrictObjectSchema<TEntries$1, TMessage>;
interface StrictObjectSchemaAsync<TEntries$1 extends ObjectEntriesAsync, TMessage extends ErrorMessage<StrictObjectIssue> | undefined> extends BaseSchemaAsync<InferObjectInput<TEntries$1>, InferObjectOutput<TEntries$1>, StrictObjectIssue | InferObjectIssue<TEntries$1>> {
    readonly type: "strict_object";
    readonly reference: typeof strictObject | typeof strictObjectAsync;
    readonly expects: "Object";
    readonly entries: TEntries$1;
    readonly message: TMessage;
}
declare function strictObjectAsync<const TEntries$1 extends ObjectEntriesAsync>(entries: TEntries$1): StrictObjectSchemaAsync<TEntries$1, undefined>;
declare function strictObjectAsync<const TEntries$1 extends ObjectEntriesAsync, const TMessage extends ErrorMessage<StrictObjectIssue> | undefined>(entries: TEntries$1, message: TMessage): StrictObjectSchemaAsync<TEntries$1, TMessage>;
interface StrictTupleIssue extends BaseIssue<unknown> {
    readonly kind: "schema";
    readonly type: "strict_tuple";
    readonly expected: "Array" | "never";
}
interface StrictTupleSchema<TItems$1 extends TupleItems, TMessage extends ErrorMessage<StrictTupleIssue> | undefined> extends BaseSchema<InferTupleInput<TItems$1>, InferTupleOutput<TItems$1>, StrictTupleIssue | InferTupleIssue<TItems$1>> {
    readonly type: "strict_tuple";
    readonly reference: typeof strictTuple;
    readonly expects: "Array";
    readonly items: TItems$1;
    readonly message: TMessage;
}
declare function strictTuple<const TItems$1 extends TupleItems>(items: TItems$1): StrictTupleSchema<TItems$1, undefined>;
declare function strictTuple<const TItems$1 extends TupleItems, const TMessage extends ErrorMessage<StrictTupleIssue> | undefined>(items: TItems$1, message: TMessage): StrictTupleSchema<TItems$1, TMessage>;
interface StrictTupleSchemaAsync<TItems$1 extends TupleItemsAsync, TMessage extends ErrorMessage<StrictTupleIssue> | undefined> extends BaseSchemaAsync<InferTupleInput<TItems$1>, InferTupleOutput<TItems$1>, StrictTupleIssue | InferTupleIssue<TItems$1>> {
    readonly type: "strict_tuple";
    readonly reference: typeof strictTuple | typeof strictTupleAsync;
    readonly expects: "Array";
    readonly items: TItems$1;
    readonly message: TMessage;
}
declare function strictTupleAsync<const TItems$1 extends TupleItemsAsync>(items: TItems$1): StrictTupleSchemaAsync<TItems$1, undefined>;
declare function strictTupleAsync<const TItems$1 extends TupleItemsAsync, const TMessage extends ErrorMessage<StrictTupleIssue> | undefined>(items: TItems$1, message: TMessage): StrictTupleSchemaAsync<TItems$1, TMessage>;
interface StringIssue extends BaseIssue<unknown> {
    readonly kind: "schema";
    readonly type: "string";
    readonly expected: "string";
}
interface StringSchema<TMessage extends ErrorMessage<StringIssue> | undefined> extends BaseSchema<string, string, StringIssue> {
    readonly type: "string";
    readonly reference: typeof string;
    readonly expects: "string";
    readonly message: TMessage;
}
declare function string(): StringSchema<undefined>;
declare function string<const TMessage extends ErrorMessage<StringIssue> | undefined>(message: TMessage): StringSchema<TMessage>;
interface TupleIssue extends BaseIssue<unknown> {
    readonly kind: "schema";
    readonly type: "tuple";
    readonly expected: "Array";
}
interface TupleSchema<TItems$1 extends TupleItems, TMessage extends ErrorMessage<TupleIssue> | undefined> extends BaseSchema<InferTupleInput<TItems$1>, InferTupleOutput<TItems$1>, TupleIssue | InferTupleIssue<TItems$1>> {
    readonly type: "tuple";
    readonly reference: typeof tuple;
    readonly expects: "Array";
    readonly items: TItems$1;
    readonly message: TMessage;
}
declare function tuple<const TItems$1 extends TupleItems>(items: TItems$1): TupleSchema<TItems$1, undefined>;
declare function tuple<const TItems$1 extends TupleItems, const TMessage extends ErrorMessage<TupleIssue> | undefined>(items: TItems$1, message: TMessage): TupleSchema<TItems$1, TMessage>;
interface TupleSchemaAsync<TItems$1 extends TupleItemsAsync, TMessage extends ErrorMessage<TupleIssue> | undefined> extends BaseSchemaAsync<InferTupleInput<TItems$1>, InferTupleOutput<TItems$1>, TupleIssue | InferTupleIssue<TItems$1>> {
    readonly type: "tuple";
    readonly reference: typeof tuple | typeof tupleAsync;
    readonly expects: "Array";
    readonly items: TItems$1;
    readonly message: TMessage;
}
declare function tupleAsync<const TItems$1 extends TupleItemsAsync>(items: TItems$1): TupleSchemaAsync<TItems$1, undefined>;
declare function tupleAsync<const TItems$1 extends TupleItemsAsync, const TMessage extends ErrorMessage<TupleIssue> | undefined>(items: TItems$1, message: TMessage): TupleSchemaAsync<TItems$1, TMessage>;
interface TupleWithRestIssue extends BaseIssue<unknown> {
    readonly kind: "schema";
    readonly type: "tuple_with_rest";
    readonly expected: "Array";
}
interface TupleWithRestSchema<TItems$1 extends TupleItems, TRest$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, TMessage extends ErrorMessage<TupleWithRestIssue> | undefined> extends BaseSchema<[
    ...InferTupleInput<TItems$1>,
    ...InferInput<TRest$1>[]
], [
    ...InferTupleOutput<TItems$1>,
    ...InferOutput<TRest$1>[]
], TupleWithRestIssue | InferTupleIssue<TItems$1> | InferIssue<TRest$1>> {
    readonly type: "tuple_with_rest";
    readonly reference: typeof tupleWithRest;
    readonly expects: "Array";
    readonly items: TItems$1;
    readonly rest: TRest$1;
    readonly message: TMessage;
}
declare function tupleWithRest<const TItems$1 extends TupleItems, const TRest$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>>(items: TItems$1, rest: TRest$1): TupleWithRestSchema<TItems$1, TRest$1, undefined>;
declare function tupleWithRest<const TItems$1 extends TupleItems, const TRest$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TMessage extends ErrorMessage<TupleWithRestIssue> | undefined>(items: TItems$1, rest: TRest$1, message: TMessage): TupleWithRestSchema<TItems$1, TRest$1, TMessage>;
interface TupleWithRestSchemaAsync<TItems$1 extends TupleItemsAsync, TRest$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, TMessage extends ErrorMessage<TupleWithRestIssue> | undefined> extends BaseSchemaAsync<[
    ...InferTupleInput<TItems$1>,
    ...InferInput<TRest$1>[]
], [
    ...InferTupleOutput<TItems$1>,
    ...InferOutput<TRest$1>[]
], TupleWithRestIssue | InferTupleIssue<TItems$1> | InferIssue<TRest$1>> {
    readonly type: "tuple_with_rest";
    readonly reference: typeof tupleWithRest | typeof tupleWithRestAsync;
    readonly expects: "Array";
    readonly items: TItems$1;
    readonly rest: TRest$1;
    readonly message: TMessage;
}
declare function tupleWithRestAsync<const TItems$1 extends TupleItemsAsync, const TRest$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>>(items: TItems$1, rest: TRest$1): TupleWithRestSchemaAsync<TItems$1, TRest$1, undefined>;
declare function tupleWithRestAsync<const TItems$1 extends TupleItemsAsync, const TRest$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, const TMessage extends ErrorMessage<TupleWithRestIssue> | undefined>(items: TItems$1, rest: TRest$1, message: TMessage): TupleWithRestSchemaAsync<TItems$1, TRest$1, TMessage>;
type InferUndefinedableOutput<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, TDefault extends DefaultAsync<TWrapped$1, undefined>> = undefined extends TDefault ? InferOutput<TWrapped$1> | undefined : InferOutput<TWrapped$1> | Extract<DefaultValue<TDefault>, undefined>;
interface UndefinedableSchema<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, TDefault extends Default<TWrapped$1, undefined>> extends BaseSchema<InferInput<TWrapped$1> | undefined, InferUndefinedableOutput<TWrapped$1, TDefault>, InferIssue<TWrapped$1>> {
    readonly type: "undefinedable";
    readonly reference: typeof undefinedable;
    readonly expects: `(${TWrapped$1["expects"]} | undefined)`;
    readonly wrapped: TWrapped$1;
    readonly default: TDefault;
}
declare function undefinedable<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>>(wrapped: TWrapped$1): UndefinedableSchema<TWrapped$1, undefined>;
declare function undefinedable<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>>, const TDefault extends Default<TWrapped$1, undefined>>(wrapped: TWrapped$1, default_: TDefault): UndefinedableSchema<TWrapped$1, TDefault>;
interface UndefinedableSchemaAsync<TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, TDefault extends DefaultAsync<TWrapped$1, undefined>> extends BaseSchemaAsync<InferInput<TWrapped$1> | undefined, InferUndefinedableOutput<TWrapped$1, TDefault>, InferIssue<TWrapped$1>> {
    readonly type: "undefinedable";
    readonly reference: typeof undefinedable | typeof undefinedableAsync;
    readonly expects: `(${TWrapped$1["expects"]} | undefined)`;
    readonly wrapped: TWrapped$1;
    readonly default: TDefault;
}
declare function undefinedableAsync<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>>(wrapped: TWrapped$1): UndefinedableSchemaAsync<TWrapped$1, undefined>;
declare function undefinedableAsync<const TWrapped$1 extends BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, const TDefault extends DefaultAsync<TWrapped$1, undefined>>(wrapped: TWrapped$1, default_: TDefault): UndefinedableSchemaAsync<TWrapped$1, TDefault>;
interface VariantSchema<TKey$1 extends string, TOptions$1 extends VariantOptions<TKey$1>, TMessage extends ErrorMessage<VariantIssue> | undefined> extends BaseSchema<InferInput<TOptions$1[number]>, InferOutput<TOptions$1[number]>, VariantIssue | InferVariantIssue<TOptions$1>> {
    readonly type: "variant";
    readonly reference: typeof variant;
    readonly expects: "Object";
    readonly key: TKey$1;
    readonly options: TOptions$1;
    readonly message: TMessage;
}
declare function variant<const TKey$1 extends string, const TOptions$1 extends VariantOptions<TKey$1>>(key: TKey$1, options: TOptions$1): VariantSchema<TKey$1, TOptions$1, undefined>;
declare function variant<const TKey$1 extends string, const TOptions$1 extends VariantOptions<TKey$1>, const TMessage extends ErrorMessage<VariantIssue> | undefined>(key: TKey$1, options: TOptions$1, message: TMessage): VariantSchema<TKey$1, TOptions$1, TMessage>;
interface VariantSchemaAsync<TKey$1 extends string, TOptions$1 extends VariantOptionsAsync<TKey$1>, TMessage extends ErrorMessage<VariantIssue> | undefined> extends BaseSchemaAsync<InferInput<TOptions$1[number]>, InferOutput<TOptions$1[number]>, VariantIssue | InferVariantIssue<TOptions$1>> {
    readonly type: "variant";
    readonly reference: typeof variant | typeof variantAsync;
    readonly expects: "Object";
    readonly key: TKey$1;
    readonly options: TOptions$1;
    readonly message: TMessage;
}
declare function variantAsync<const TKey$1 extends string, const TOptions$1 extends VariantOptionsAsync<TKey$1>>(key: TKey$1, options: TOptions$1): VariantSchemaAsync<TKey$1, TOptions$1, undefined>;
declare function variantAsync<const TKey$1 extends string, const TOptions$1 extends VariantOptionsAsync<TKey$1>, const TMessage extends ErrorMessage<VariantIssue> | undefined>(key: TKey$1, options: TOptions$1, message: TMessage): VariantSchemaAsync<TKey$1, TOptions$1, TMessage>;
interface VariantIssue extends BaseIssue<unknown> {
    readonly kind: "schema";
    readonly type: "variant";
    readonly expected: string;
}
interface VariantOptionSchema<TKey$1 extends string> extends BaseSchema<unknown, unknown, VariantIssue | BaseIssue<unknown>> {
    readonly type: "variant";
    readonly reference: typeof variant;
    readonly key: string;
    readonly options: VariantOptions<TKey$1>;
    readonly message: ErrorMessage<VariantIssue> | undefined;
}
interface VariantOptionSchemaAsync<TKey$1 extends string> extends BaseSchemaAsync<unknown, unknown, VariantIssue | BaseIssue<unknown>> {
    readonly type: "variant";
    readonly reference: typeof variant | typeof variantAsync;
    readonly key: string;
    readonly options: VariantOptionsAsync<TKey$1>;
    readonly message: ErrorMessage<VariantIssue> | undefined;
}
type VariantObjectEntries<TKey$1 extends string> = Record<TKey$1, BaseSchema<unknown, unknown, BaseIssue<unknown>> | OptionalEntrySchema> & ObjectEntries;
type VariantObjectEntriesAsync<TKey$1 extends string> = Record<TKey$1, BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>> | OptionalEntrySchema | OptionalEntrySchemaAsync> & ObjectEntriesAsync;
type VariantOption<TKey$1 extends string> = LooseObjectSchema<VariantObjectEntries<TKey$1>, ErrorMessage<LooseObjectIssue> | undefined> | ObjectSchema<VariantObjectEntries<TKey$1>, ErrorMessage<ObjectIssue> | undefined> | ObjectWithRestSchema<VariantObjectEntries<TKey$1>, BaseSchema<unknown, unknown, BaseIssue<unknown>>, ErrorMessage<ObjectWithRestIssue> | undefined> | StrictObjectSchema<VariantObjectEntries<TKey$1>, ErrorMessage<StrictObjectIssue> | undefined> | VariantOptionSchema<TKey$1>;
type VariantOptionAsync<TKey$1 extends string> = LooseObjectSchemaAsync<VariantObjectEntriesAsync<TKey$1>, ErrorMessage<LooseObjectIssue> | undefined> | ObjectSchemaAsync<VariantObjectEntriesAsync<TKey$1>, ErrorMessage<ObjectIssue> | undefined> | ObjectWithRestSchemaAsync<VariantObjectEntriesAsync<TKey$1>, BaseSchema<unknown, unknown, BaseIssue<unknown>> | BaseSchemaAsync<unknown, unknown, BaseIssue<unknown>>, ErrorMessage<ObjectWithRestIssue> | undefined> | StrictObjectSchemaAsync<VariantObjectEntriesAsync<TKey$1>, ErrorMessage<StrictObjectIssue> | undefined> | VariantOptionSchemaAsync<TKey$1>;
type VariantOptions<TKey$1 extends string> = MaybeReadonly<VariantOption<TKey$1>[]>;
type VariantOptionsAsync<TKey$1 extends string> = MaybeReadonly<(VariantOption<TKey$1> | VariantOptionAsync<TKey$1>)[]>;
type InferVariantIssue<TOptions$1 extends VariantOptions<string> | VariantOptionsAsync<string>> = Exclude<InferIssue<TOptions$1[number]>, {
    type: "loose_object" | "object" | "object_with_rest";
}>;
declare const BrandSymbol: unique symbol;
type BrandName = string | number | symbol;
interface Brand<TName extends BrandName> {
    [BrandSymbol]: {
        [TValue in TName]: TValue;
    };
}
interface BrandAction<TInput$1, TName extends BrandName> extends BaseTransformation<TInput$1, TInput$1 & Brand<TName>, never> {
    readonly type: "brand";
    readonly reference: typeof brand;
    readonly name: TName;
}
declare function brand<TInput$1, TName extends BrandName>(name: TName): BrandAction<TInput$1, TName>;
interface CheckIssue<TInput$1> extends BaseIssue<TInput$1> {
    readonly kind: "validation";
    readonly type: "check";
    readonly expected: null;
    readonly requirement: (input: TInput$1) => MaybePromise<boolean>;
}
interface CheckAction<TInput$1, TMessage extends ErrorMessage<CheckIssue<TInput$1>> | undefined> extends BaseValidation<TInput$1, TInput$1, CheckIssue<TInput$1>> {
    readonly type: "check";
    readonly reference: typeof check;
    readonly expects: null;
    readonly requirement: (input: TInput$1) => boolean;
    readonly message: TMessage;
}
declare function check<TInput$1>(requirement: (input: TInput$1) => boolean): CheckAction<TInput$1, undefined>;
declare function check<TInput$1, const TMessage extends ErrorMessage<CheckIssue<TInput$1>> | undefined>(requirement: (input: TInput$1) => boolean, message: TMessage): CheckAction<TInput$1, TMessage>;
type LengthInput = string | ArrayLike<unknown>;
type ValueInput = string | number | bigint | boolean | Date;
interface EmailIssue<TInput$1 extends string> extends BaseIssue<TInput$1> {
    readonly kind: "validation";
    readonly type: "email";
    readonly expected: null;
    readonly received: `"${string}"`;
    readonly requirement: RegExp;
}
interface EmailAction<TInput$1 extends string, TMessage extends ErrorMessage<EmailIssue<TInput$1>> | undefined> extends BaseValidation<TInput$1, TInput$1, EmailIssue<TInput$1>> {
    readonly type: "email";
    readonly reference: typeof email;
    readonly expects: null;
    readonly requirement: RegExp;
    readonly message: TMessage;
}
declare function email<TInput$1 extends string>(): EmailAction<TInput$1, undefined>;
declare function email<TInput$1 extends string, const TMessage extends ErrorMessage<EmailIssue<TInput$1>> | undefined>(message: TMessage): EmailAction<TInput$1, TMessage>;
interface IntegerIssue<TInput$1 extends number> extends BaseIssue<TInput$1> {
    readonly kind: "validation";
    readonly type: "integer";
    readonly expected: null;
    readonly received: `${number}`;
    readonly requirement: (input: number) => boolean;
}
interface IntegerAction<TInput$1 extends number, TMessage extends ErrorMessage<IntegerIssue<TInput$1>> | undefined> extends BaseValidation<TInput$1, TInput$1, IntegerIssue<TInput$1>> {
    readonly type: "integer";
    readonly reference: typeof integer;
    readonly expects: null;
    readonly requirement: (input: number) => boolean;
    readonly message: TMessage;
}
declare function integer<TInput$1 extends number>(): IntegerAction<TInput$1, undefined>;
declare function integer<TInput$1 extends number, const TMessage extends ErrorMessage<IntegerIssue<TInput$1>> | undefined>(message: TMessage): IntegerAction<TInput$1, TMessage>;
interface MaxValueIssue<TInput$1 extends ValueInput, TRequirement extends ValueInput> extends BaseIssue<TInput$1> {
    readonly kind: "validation";
    readonly type: "max_value";
    readonly expected: `<=${string}`;
    readonly requirement: TRequirement;
}
interface MaxValueAction<TInput$1 extends ValueInput, TRequirement extends TInput$1, TMessage extends ErrorMessage<MaxValueIssue<TInput$1, TRequirement>> | undefined> extends BaseValidation<TInput$1, TInput$1, MaxValueIssue<TInput$1, TRequirement>> {
    readonly type: "max_value";
    readonly reference: typeof maxValue;
    readonly expects: `<=${string}`;
    readonly requirement: TRequirement;
    readonly message: TMessage;
}
declare function maxValue<TInput$1 extends ValueInput, const TRequirement extends TInput$1>(requirement: TRequirement): MaxValueAction<TInput$1, TRequirement, undefined>;
declare function maxValue<TInput$1 extends ValueInput, const TRequirement extends TInput$1, const TMessage extends ErrorMessage<MaxValueIssue<TInput$1, TRequirement>> | undefined>(requirement: TRequirement, message: TMessage): MaxValueAction<TInput$1, TRequirement, TMessage>;
interface MinLengthIssue<TInput$1 extends LengthInput, TRequirement extends number> extends BaseIssue<TInput$1> {
    readonly kind: "validation";
    readonly type: "min_length";
    readonly expected: `>=${TRequirement}`;
    readonly received: `${number}`;
    readonly requirement: TRequirement;
}
interface MinLengthAction<TInput$1 extends LengthInput, TRequirement extends number, TMessage extends ErrorMessage<MinLengthIssue<TInput$1, TRequirement>> | undefined> extends BaseValidation<TInput$1, TInput$1, MinLengthIssue<TInput$1, TRequirement>> {
    readonly type: "min_length";
    readonly reference: typeof minLength;
    readonly expects: `>=${TRequirement}`;
    readonly requirement: TRequirement;
    readonly message: TMessage;
}
declare function minLength<TInput$1 extends LengthInput, const TRequirement extends number>(requirement: TRequirement): MinLengthAction<TInput$1, TRequirement, undefined>;
declare function minLength<TInput$1 extends LengthInput, const TRequirement extends number, const TMessage extends ErrorMessage<MinLengthIssue<TInput$1, TRequirement>> | undefined>(requirement: TRequirement, message: TMessage): MinLengthAction<TInput$1, TRequirement, TMessage>;
interface MinValueIssue<TInput$1 extends ValueInput, TRequirement extends ValueInput> extends BaseIssue<TInput$1> {
    readonly kind: "validation";
    readonly type: "min_value";
    readonly expected: `>=${string}`;
    readonly requirement: TRequirement;
}
interface MinValueAction<TInput$1 extends ValueInput, TRequirement extends TInput$1, TMessage extends ErrorMessage<MinValueIssue<TInput$1, TRequirement>> | undefined> extends BaseValidation<TInput$1, TInput$1, MinValueIssue<TInput$1, TRequirement>> {
    readonly type: "min_value";
    readonly reference: typeof minValue;
    readonly expects: `>=${string}`;
    readonly requirement: TRequirement;
    readonly message: TMessage;
}
declare function minValue<TInput$1 extends ValueInput, const TRequirement extends TInput$1>(requirement: TRequirement): MinValueAction<TInput$1, TRequirement, undefined>;
declare function minValue<TInput$1 extends ValueInput, const TRequirement extends TInput$1, const TMessage extends ErrorMessage<MinValueIssue<TInput$1, TRequirement>> | undefined>(requirement: TRequirement, message: TMessage): MinValueAction<TInput$1, TRequirement, TMessage>;
type ReadonlyOutput<TInput$1> = TInput$1 extends Map<infer TKey, infer TValue> ? ReadonlyMap<TKey, TValue> : TInput$1 extends Set<infer TValue> ? ReadonlySet<TValue> : Readonly<TInput$1>;
interface ReadonlyAction<TInput$1> extends BaseTransformation<TInput$1, ReadonlyOutput<TInput$1>, never> {
    readonly type: "readonly";
    readonly reference: typeof readonly;
}
declare function readonly<TInput$1>(): ReadonlyAction<TInput$1>;
interface ToLowerCaseAction extends BaseTransformation<string, string, never> {
    readonly type: "to_lower_case";
    readonly reference: typeof toLowerCase;
}
declare function toLowerCase(): ToLowerCaseAction;
interface TransformAction<TInput$1, TOutput$1> extends BaseTransformation<TInput$1, TOutput$1, never> {
    readonly type: "transform";
    readonly reference: typeof transform;
    readonly operation: (input: TInput$1) => TOutput$1;
}
declare function transform<TInput$1, TOutput$1>(operation: (input: TInput$1) => TOutput$1): TransformAction<TInput$1, TOutput$1>;
interface TrimAction extends BaseTransformation<string, string, never> {
    readonly type: "trim";
    readonly reference: typeof trim;
}
declare function trim(): TrimAction;
interface UrlIssue<TInput$1 extends string> extends BaseIssue<TInput$1> {
    readonly kind: "validation";
    readonly type: "url";
    readonly expected: null;
    readonly received: `"${string}"`;
    readonly requirement: (input: string) => boolean;
}
interface UrlAction<TInput$1 extends string, TMessage extends ErrorMessage<UrlIssue<TInput$1>> | undefined> extends BaseValidation<TInput$1, TInput$1, UrlIssue<TInput$1>> {
    readonly type: "url";
    readonly reference: typeof url;
    readonly expects: null;
    readonly requirement: (input: string) => boolean;
    readonly message: TMessage;
}
declare function url<TInput$1 extends string>(): UrlAction<TInput$1, undefined>;
declare function url<TInput$1 extends string, const TMessage extends ErrorMessage<UrlIssue<TInput$1>> | undefined>(message: TMessage): UrlAction<TInput$1, TMessage>;
interface UuidIssue<TInput$1 extends string> extends BaseIssue<TInput$1> {
    readonly kind: "validation";
    readonly type: "uuid";
    readonly expected: null;
    readonly received: `"${string}"`;
    readonly requirement: RegExp;
}
interface UuidAction<TInput$1 extends string, TMessage extends ErrorMessage<UuidIssue<TInput$1>> | undefined> extends BaseValidation<TInput$1, TInput$1, UuidIssue<TInput$1>> {
    readonly type: "uuid";
    readonly reference: typeof uuid;
    readonly expects: null;
    readonly requirement: RegExp;
    readonly message: TMessage;
}
declare function uuid<TInput$1 extends string>(): UuidAction<TInput$1, undefined>;
declare function uuid<TInput$1 extends string, const TMessage extends ErrorMessage<UuidIssue<TInput$1>> | undefined>(message: TMessage): UuidAction<TInput$1, TMessage>;
export { type InferInput, type InferOutput, array, boolean, brand, check, email, flatten, integer, literal, maxValue, minLength, minValue, nullable, number, object, optional, parse, partial, pick, picklist, pipe, record, safeParse, strictObject, string, toLowerCase, transform, trim, tuple, union, url, uuid, variant };
