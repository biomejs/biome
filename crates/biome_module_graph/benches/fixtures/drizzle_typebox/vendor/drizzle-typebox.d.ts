import * as t from './typebox';
import { TSchema, Kind, Static } from './typebox';
export { Static, TSchema, Type } from './typebox';
declare const entityKind: unique symbol;
type CacheConfig = {
    ex?: number;
    px?: number;
    exat?: number;
    pxat?: number;
    keepTtl?: boolean;
    hexOptions?: 'NX' | 'nx' | 'XX' | 'xx' | 'GT' | 'gt' | 'LT' | 'lt';
};
type WithCacheConfig = {
    enable: boolean;
    config?: CacheConfig;
    tag?: string;
    autoInvalidate?: boolean;
};
declare abstract class Cache {
    static readonly [entityKind]: string;
    abstract strategy(): 'explicit' | 'all';
    abstract get(key: string, tables: string[], isTag: boolean, isAutoInvalidate?: boolean): Promise<any[] | undefined>;
    abstract put(hashedQuery: string, response: any, tables: string[], isTag: boolean, config?: CacheConfig): Promise<void>;
    abstract onMutate(params: MutationOption): Promise<void>;
}
type MutationOption = {
    tags?: string | string[];
    tables?: Table<any> | Table<any>[] | string | string[];
};
declare class CasingCache {
    static readonly [entityKind]: string;
    private cachedTables;
    private convert;
    constructor(casing?: Casing);
    getColumnCasing(column: Column): string;
    private cacheTable;
    clearCache(): void;
}
interface Subquery<TAlias extends string = string, TSelectedFields extends Record<string, unknown> = Record<string, unknown>> extends SQLWrapper {
}
declare class Subquery<TAlias extends string = string, TSelectedFields extends Record<string, unknown> = Record<string, unknown>> implements SQLWrapper {
    static readonly [entityKind]: string;
    _: {
        brand: 'Subquery';
        sql: SQL;
        selectedFields: TSelectedFields;
        alias: TAlias;
        isWith: boolean;
        usedTables?: string[];
    };
    constructor(sql: SQL, fields: TSelectedFields, alias: string, isWith?: boolean, usedTables?: string[]);
}
declare class WithSubquery<TAlias extends string = string, TSelection extends Record<string, unknown> = Record<string, unknown>> extends Subquery<TAlias, TSelection> {
    static readonly [entityKind]: string;
}
type WithSubqueryWithoutSelection<TAlias extends string> = WithSubquery<TAlias, {}>;
interface TableConfig$5<TColumn extends Column = Column<any>> {
    name: string;
    schema: string | undefined;
    columns: Record<string, TColumn>;
    dialect: string;
}
type UpdateTableConfig<T extends TableConfig$5, TUpdate extends Partial<TableConfig$5>> = Required<Update<T, TUpdate>>;
interface Table<T extends TableConfig$5 = TableConfig$5> extends SQLWrapper {
}
declare class Table<T extends TableConfig$5 = TableConfig$5> implements SQLWrapper {
    static readonly [entityKind]: string;
    readonly _: {
        readonly brand: 'Table';
        readonly config: T;
        readonly name: T['name'];
        readonly schema: T['schema'];
        readonly columns: T['columns'];
        readonly inferSelect: InferSelectModel<Table<T>>;
        readonly inferInsert: InferInsertModel<Table<T>>;
    };
    readonly $inferSelect: InferSelectModel<Table<T>>;
    readonly $inferInsert: InferInsertModel<Table<T>>;
    constructor(name: string, schema: string | undefined, baseName: string);
}
type AnyTable<TPartial extends Partial<TableConfig$5>> = Table<UpdateTableConfig<TableConfig$5, TPartial>>;
type MapColumnName<TName extends string, TColumn extends Column, TDBColumNames extends boolean> = TDBColumNames extends true ? TColumn['_']['name'] : TName;
type InferModelFromColumns<TColumns extends Record<string, Column>, TInferMode extends 'select' | 'insert' = 'select', TConfig extends {
    dbColumnNames: boolean;
    override?: boolean;
} = {
    dbColumnNames: false;
    override: false;
}> = Simplify<TInferMode extends 'insert' ? {
    [Key in keyof TColumns & string as RequiredKeyOnly<MapColumnName<Key, TColumns[Key], TConfig['dbColumnNames']>, TColumns[Key]>]: GetColumnData<TColumns[Key], 'query'>;
} & {
    [Key in keyof TColumns & string as OptionalKeyOnly<MapColumnName<Key, TColumns[Key], TConfig['dbColumnNames']>, TColumns[Key], TConfig['override']>]?: GetColumnData<TColumns[Key], 'query'> | undefined;
} : {
    [Key in keyof TColumns & string as MapColumnName<Key, TColumns[Key], TConfig['dbColumnNames']>]: GetColumnData<TColumns[Key], 'query'>;
}>;
type InferSelectModel<TTable extends Table, TConfig extends {
    dbColumnNames: boolean;
} = {
    dbColumnNames: false;
}> = InferModelFromColumns<TTable['_']['columns'], 'select', TConfig>;
type InferInsertModel<TTable extends Table, TConfig extends {
    dbColumnNames: boolean;
    override?: boolean;
} = {
    dbColumnNames: false;
    override: false;
}> = InferModelFromColumns<TTable['_']['columns'], 'insert', TConfig>;
type RequiredKeyOnly<TKey extends string, T extends Column> = T extends AnyColumn<{
    notNull: true;
    hasDefault: false;
}> ? TKey : never;
type OptionalKeyOnly<TKey extends string, T extends Column, OverrideT extends boolean | undefined = false> = TKey extends RequiredKeyOnly<TKey, T> ? never : T extends {
    _: {
        generated: undefined;
    };
} ? (T extends {
    _: {
        identity: undefined;
    };
} ? TKey : T['_']['identity'] extends 'always' ? OverrideT extends true ? TKey : never : TKey) : never;
type SelectedFieldsFlat$1<TColumn extends Column> = Record<string, TColumn | SQL | SQL.Aliased | Subquery>;
type SelectedFields$1<TColumn extends Column, TTable extends Table> = Record<string, SelectedFieldsFlat$1<TColumn>[string] | TTable | SelectedFieldsFlat$1<TColumn>>;
type SelectedFieldsOrdered$1<TColumn extends Column> = {
    path: string[];
    field: TColumn | SQL | SQL.Aliased | Subquery;
}[];
type JoinType = 'inner' | 'left' | 'right' | 'full' | 'cross';
type JoinNullability = 'nullable' | 'not-null';
type ApplyNullability<T, TNullability extends JoinNullability> = TNullability extends 'nullable' ? T | null : TNullability extends 'null' ? null : T;
type ApplyNullabilityToColumn<TColumn extends Column, TNullability extends JoinNullability> = TNullability extends 'not-null' ? TColumn : Column<Assume<UpdateColConfig<TColumn['_'], {
    notNull: TNullability extends 'nullable' ? false : TColumn['_']['notNull'];
}>, ColumnBaseConfig<ColumnDataType, string>>>;
type ApplyNotNullMapToJoins<TResult, TNullabilityMap extends Record<string, JoinNullability>> = {
    [TTableName in keyof TResult & keyof TNullabilityMap & string]: ApplyNullability<TResult[TTableName], TNullabilityMap[TTableName]>;
} & {};
type SelectMode = 'partial' | 'single' | 'multiple';
type SelectResult<TResult, TSelectMode extends SelectMode, TNullabilityMap extends Record<string, JoinNullability>> = TSelectMode extends 'partial' ? SelectPartialResult<TResult, TNullabilityMap> : TSelectMode extends 'single' ? SelectResultFields<TResult> : ApplyNotNullMapToJoins<SelectResultFields<TResult>, TNullabilityMap>;
type SelectPartialResult<TFields, TNullability extends Record<string, JoinNullability>> = TNullability extends TNullability ? {
    [Key in keyof TFields]: TFields[Key] extends infer TField ? TField extends Table ? TField['_']['name'] extends keyof TNullability ? ApplyNullability<SelectResultFields<TField['_']['columns']>, TNullability[TField['_']['name']]> : never : TField extends Column ? TField['_']['tableName'] extends keyof TNullability ? ApplyNullability<SelectResultField<TField>, TNullability[TField['_']['tableName']]> : never : TField extends SQL | SQL.Aliased ? SelectResultField<TField> : TField extends Subquery ? FromSingleKeyObject<TField['_']['selectedFields'], TField['_']['selectedFields'] extends {
        [key: string]: infer TValue;
    } ? SelectResultField<TValue> : never, 'You can only select one column in the subquery'> : TField extends Record<string, any> ? TField[keyof TField] extends AnyColumn<{
        tableName: infer TTableName extends string;
    }> | SQL | SQL.Aliased ? Not<IsUnion<TTableName>> extends true ? ApplyNullability<SelectResultFields<TField>, TNullability[TTableName]> : SelectPartialResult<TField, TNullability> : never : never : never;
} : never;
type MapColumnsToTableAlias<TColumns extends ColumnsSelection, TAlias extends string, TDialect extends Dialect> = {
    [Key in keyof TColumns]: TColumns[Key] extends Column ? ChangeColumnTableName<Assume<TColumns[Key], Column>, TAlias, TDialect> : TColumns[Key];
} & {};
type AddAliasToSelection<TSelection extends ColumnsSelection, TAlias extends string, TDialect extends Dialect> = Simplify<IsAny<TSelection> extends true ? any : {
    [Key in keyof TSelection]: TSelection[Key] extends Column ? ChangeColumnTableName<TSelection[Key], TAlias, TDialect> : TSelection[Key] extends Table ? AddAliasToSelection<TSelection[Key]['_']['columns'], TAlias, TDialect> : TSelection[Key] extends SQL | SQL.Aliased ? TSelection[Key] : TSelection[Key] extends ColumnsSelection ? MapColumnsToTableAlias<TSelection[Key], TAlias, TDialect> : never;
}>;
type AppendToResult<TTableName extends string | undefined, TResult, TJoinedName extends string | undefined, TSelectedFields extends SelectedFields$1<Column, Table>, TOldSelectMode extends SelectMode> = TOldSelectMode extends 'partial' ? TResult : TOldSelectMode extends 'single' ? (TTableName extends string ? Record<TTableName, TResult> : TResult) & (TJoinedName extends string ? Record<TJoinedName, TSelectedFields> : TSelectedFields) : TResult & (TJoinedName extends string ? Record<TJoinedName, TSelectedFields> : TSelectedFields);
type BuildSubquerySelection<TSelection extends ColumnsSelection, TNullability extends Record<string, JoinNullability>> = TSelection extends never ? any : {
    [Key in keyof TSelection]: TSelection[Key] extends SQL ? DrizzleTypeError<'You cannot reference this field without assigning it an alias first - use `.as(<alias>)`'> : TSelection[Key] extends SQL.Aliased ? TSelection[Key] : TSelection[Key] extends Table ? BuildSubquerySelection<TSelection[Key]['_']['columns'], TNullability> : TSelection[Key] extends Column ? ApplyNullabilityToColumn<TSelection[Key], TNullability[TSelection[Key]['_']['tableName']]> : TSelection[Key] extends ColumnsSelection ? BuildSubquerySelection<TSelection[Key], TNullability> : never;
} & {};
type SetJoinsNullability<TNullabilityMap extends Record<string, JoinNullability>, TValue extends JoinNullability> = {
    [Key in keyof TNullabilityMap]: TValue;
};
type AppendToNullabilityMap<TJoinsNotNull extends Record<string, JoinNullability>, TJoinedName extends string | undefined, TJoinType extends JoinType> = TJoinedName extends string ? 'left' extends TJoinType ? TJoinsNotNull & {
    [name in TJoinedName]: 'nullable';
} : 'right' extends TJoinType ? SetJoinsNullability<TJoinsNotNull, 'nullable'> & {
    [name in TJoinedName]: 'not-null';
} : 'inner' extends TJoinType ? TJoinsNotNull & {
    [name in TJoinedName]: 'not-null';
} : 'cross' extends TJoinType ? TJoinsNotNull & {
    [name in TJoinedName]: 'not-null';
} : 'full' extends TJoinType ? SetJoinsNullability<TJoinsNotNull, 'nullable'> & {
    [name in TJoinedName]: 'nullable';
} : never : TJoinsNotNull;
type TableLike = Table | Subquery | View | SQL;
type GetSelectTableName<TTable extends TableLike> = TTable extends Table ? TTable['_']['name'] : TTable extends Subquery ? TTable['_']['alias'] : TTable extends View ? TTable['_']['name'] : TTable extends SQL ? undefined : never;
type GetSelectTableSelection<TTable extends TableLike> = TTable extends Table ? TTable['_']['columns'] : TTable extends Subquery | View ? Assume<TTable['_']['selectedFields'], ColumnsSelection> : TTable extends SQL ? {} : never;
type SelectResultField<T, TDeep extends boolean = true> = T extends DrizzleTypeError<any> ? T : T extends Table ? Equal<TDeep, true> extends true ? SelectResultField<T['_']['columns'], false> : never : T extends Column<any> ? GetColumnData<T> : T extends SQL | SQL.Aliased ? T['_']['type'] : T extends Record<string, any> ? SelectResultFields<T, true> : never;
type SelectResultFields<TSelectedFields, TDeep extends boolean = true> = Simplify<{
    [Key in keyof TSelectedFields]: SelectResultField<TSelectedFields[Key], TDeep>;
}>;
type SetOperator = 'union' | 'intersect' | 'except';
declare class FakePrimitiveParam {
    static readonly [entityKind]: string;
}
interface BuildQueryConfig {
    casing: CasingCache;
    escapeName(name: string): string;
    escapeParam(num: number, value: unknown): string;
    escapeString(str: string): string;
    prepareTyping?: (encoder: DriverValueEncoder<unknown, unknown>) => QueryTypingsValue;
    paramStartIndex?: {
        value: number;
    };
    inlineParams?: boolean;
    invokeSource?: 'indexes' | undefined;
}
type QueryTypingsValue = 'json' | 'decimal' | 'time' | 'timestamp' | 'uuid' | 'date' | 'none';
interface Query {
    sql: string;
    params: unknown[];
}
interface QueryWithTypings extends Query {
    typings?: QueryTypingsValue[];
}
interface SQLWrapper {
    getSQL(): SQL;
    shouldOmitSQLParens?(): boolean;
}
declare class StringChunk implements SQLWrapper {
    static readonly [entityKind]: string;
    readonly value: string[];
    constructor(value: string | string[]);
    getSQL(): SQL<unknown>;
}
type GetDecoderResult<T> = T extends Column ? T['_']['data'] : T extends DriverValueDecoder<infer TData, any> | DriverValueDecoder<infer TData, any>['mapFromDriverValue'] ? TData : never;
declare class Name implements SQLWrapper {
    readonly value: string;
    static readonly [entityKind]: string;
    protected brand: 'Name';
    constructor(value: string);
    getSQL(): SQL<unknown>;
}
interface DriverValueDecoder<TData, TDriverParam> {
    mapFromDriverValue(value: TDriverParam): TData;
}
interface DriverValueEncoder<TData, TDriverParam> {
    mapToDriverValue(value: TData): TDriverParam | SQL;
}
interface DriverValueMapper<TData, TDriverParam> extends DriverValueDecoder<TData, TDriverParam>, DriverValueEncoder<TData, TDriverParam> {
}
declare class Param<TDataType = unknown, TDriverParamType = TDataType> implements SQLWrapper {
    readonly value: TDataType;
    readonly encoder: DriverValueEncoder<TDataType, TDriverParamType>;
    static readonly [entityKind]: string;
    protected brand: 'BoundParamValue';
    constructor(value: TDataType, encoder?: DriverValueEncoder<TDataType, TDriverParamType>);
    getSQL(): SQL<unknown>;
}
type SQLChunk = StringChunk | SQLChunk[] | SQLWrapper | SQL | Table | View | Subquery | AnyColumn | Param | Name | undefined | FakePrimitiveParam | Placeholder;
declare function sql<T>(strings: TemplateStringsArray, ...params: any[]): SQL<T>;
declare namespace sql {
    function empty(): SQL;
    function fromList(list: SQLChunk[]): SQL;
    function raw(str: string): SQL;
    function join(chunks: SQLChunk[], separator?: SQLChunk): SQL;
    function identifier(value: string): Name;
    function placeholder<TName extends string>(name: TName): Placeholder<TName>;
    function param<TData, TDriver>(value: TData, encoder?: DriverValueEncoder<TData, TDriver>): Param<TData, TDriver>;
}
declare class SQL<T = unknown> implements SQLWrapper {
    readonly queryChunks: SQLChunk[];
    static readonly [entityKind]: string;
    _: {
        brand: 'SQL';
        type: T;
    };
    private shouldInlineParams;
    constructor(queryChunks: SQLChunk[]);
    append(query: SQL): this;
    toQuery(config: BuildQueryConfig): QueryWithTypings;
    buildQueryFromSourceParams(chunks: SQLChunk[], _config: BuildQueryConfig): Query;
    private mapInlineParam;
    getSQL(): SQL;
    as(alias: string): SQL.Aliased<T>;
    as<TData>(): SQL<TData>;
    as<TData>(alias: string): SQL.Aliased<TData>;
    mapWith<TDecoder extends DriverValueDecoder<any, any> | DriverValueDecoder<any, any>['mapFromDriverValue']>(decoder: TDecoder): SQL<GetDecoderResult<TDecoder>>;
    inlineParams(): this;
    if(condition: any | undefined): this | undefined;
}
declare namespace SQL {
    class Aliased<T = unknown> implements SQLWrapper {
        readonly sql: SQL;
        readonly fieldAlias: string;
        static readonly [entityKind]: string;
        _: {
            brand: 'SQL.Aliased';
            type: T;
        };
        constructor(sql: SQL, fieldAlias: string);
        getSQL(): SQL;
    }
}
declare class Placeholder<TName extends string = string, TValue = any> implements SQLWrapper {
    readonly name: TName;
    static readonly [entityKind]: string;
    protected: TValue;
    constructor(name: TName);
    getSQL(): SQL;
}
type ColumnsSelection = Record<string, unknown>;
declare abstract class View<TName extends string = string, TExisting extends boolean = boolean, TSelection extends ColumnsSelection = ColumnsSelection> implements SQLWrapper {
    static readonly [entityKind]: string;
    _: {
        brand: 'View';
        viewBrand: string;
        name: TName;
        existing: TExisting;
        selectedFields: TSelection;
    };
    readonly $inferSelect: InferSelectViewModel<View<Assume<TName, string>, TExisting, TSelection>>;
    constructor({ name, schema, selectedFields, query }: {
        name: TName;
        schema: string | undefined;
        selectedFields: ColumnsSelection;
        query: SQL | undefined;
    });
    getSQL(): SQL<unknown>;
}
type InferSelectViewModel<TView extends View> = Equal<TView['_']['selectedFields'], {
    [x: string]: unknown;
}> extends true ? {
    [x: string]: unknown;
} : SelectResult<TView['_']['selectedFields'], 'single', Record<TView['_']['name'], 'not-null'>>;
type UpdateSet = Record<string, SQL | Param | AnyColumn | null | undefined>;
type Update<T, TUpdate> = {
    [K in Exclude<keyof T, keyof TUpdate>]: T[K];
} & TUpdate;
type Simplify<T> = {
    [K in keyof T]: T[K];
} & {};
type Not<T extends boolean> = T extends true ? false : true;
type IsNever$1<T> = [
    T
] extends [
    never
] ? true : false;
type IsUnion<T, U extends T = T> = (T extends any ? (U extends T ? false : true) : never) extends false ? false : true;
type FromSingleKeyObject<T, Result, TError extends string, K = keyof T> = IsNever$1<K> extends true ? never : IsUnion<K> extends true ? DrizzleTypeError<TError> : Result;
type Assume<T, U> = T extends U ? T : U;
type Equal<X, Y> = (<T>() => T extends X ? 1 : 2) extends (<T>() => T extends Y ? 1 : 2) ? true : false;
interface DrizzleTypeError<T extends string> {
    $drizzleTypeError: T;
}
type ValueOrArray<T> = T | T[];
type Writable<T> = {
    -readonly [P in keyof T]: T[P];
};
type NonArray<T> = T extends any[] ? never : T;
declare function getTableColumns<T extends Table>(table: T): T['_']['columns'];
type ColumnsWithTable<TTableName extends string, TForeignTableName extends string, TColumns extends AnyColumn<{
    tableName: TTableName;
}>[]> = {
    [Key in keyof TColumns]: AnyColumn<{
        tableName: TForeignTableName;
    }>;
};
type Casing = 'snake_case' | 'camelCase';
type ValidateShape<T, ValidShape, TResult = T> = T extends ValidShape ? Exclude<keyof T, keyof ValidShape> extends never ? TResult : DrizzleTypeError<`Invalid key(s): ${Exclude<(keyof T) & (string | number | bigint | boolean | null | undefined), keyof ValidShape>}`> : never;
type KnownKeysOnly<T, U> = {
    [K in keyof T]: K extends keyof U ? T[K] : never;
};
type IsAny<T> = 0 extends (1 & T) ? true : false;
type RequireAtLeastOne<T, Keys extends keyof T = keyof T> = Keys extends any ? Required<Pick<T, Keys>> & Partial<Omit<T, Keys>> : never;
type NeonAuthToken = string | (() => string | Promise<string>);
interface BinaryOperator {
    <TColumn extends Column>(left: TColumn, right: GetColumnData<TColumn, 'raw'> | SQLWrapper): SQL;
    <T>(left: SQL.Aliased<T>, right: T | SQLWrapper): SQL;
    <T extends SQLWrapper>(left: Exclude<T, SQL.Aliased | Column>, right: unknown): SQL;
}
declare const eq: BinaryOperator;
declare function and(...conditions: (SQLWrapper | undefined)[]): SQL | undefined;
declare function or(...conditions: (SQLWrapper | undefined)[]): SQL | undefined;
declare function not(condition: SQLWrapper): SQL;
declare const gte: BinaryOperator;
declare function inArray<T>(column: SQL.Aliased<T>, values: (T | Placeholder)[] | SQLWrapper): SQL;
declare function inArray<TColumn extends Column>(column: TColumn, values: ReadonlyArray<GetColumnData<TColumn, 'raw'> | Placeholder> | SQLWrapper): SQL;
declare function inArray<T extends SQLWrapper>(column: Exclude<T, SQL.Aliased | Column>, values: ReadonlyArray<unknown | Placeholder> | SQLWrapper): SQL;
declare function notInArray<T>(column: SQL.Aliased<T>, values: (T | Placeholder)[] | SQLWrapper): SQL;
declare function notInArray<TColumn extends Column>(column: TColumn, values: (GetColumnData<TColumn, 'raw'> | Placeholder)[] | SQLWrapper): SQL;
declare function notInArray<T extends SQLWrapper>(column: Exclude<T, SQL.Aliased | Column>, values: (unknown | Placeholder)[] | SQLWrapper): SQL;
declare function isNull(value: SQLWrapper): SQL;
declare function isNotNull(value: SQLWrapper): SQL;
declare function exists(subquery: SQLWrapper): SQL;
declare function notExists(subquery: SQLWrapper): SQL;
declare function between<T>(column: SQL.Aliased, min: T | SQLWrapper, max: T | SQLWrapper): SQL;
declare function between<TColumn extends AnyColumn>(column: TColumn, min: GetColumnData<TColumn, 'raw'> | SQLWrapper, max: GetColumnData<TColumn, 'raw'> | SQLWrapper): SQL;
declare function between<T extends SQLWrapper>(column: Exclude<T, SQL.Aliased | Column>, min: unknown, max: unknown): SQL;
declare function notBetween<T>(column: SQL.Aliased, min: T | SQLWrapper, max: T | SQLWrapper): SQL;
declare function notBetween<TColumn extends AnyColumn>(column: TColumn, min: GetColumnData<TColumn, 'raw'> | SQLWrapper, max: GetColumnData<TColumn, 'raw'> | SQLWrapper): SQL;
declare function notBetween<T extends SQLWrapper>(column: Exclude<T, SQL.Aliased | Column>, min: unknown, max: unknown): SQL;
declare function like(column: Column | SQL.Aliased | SQL, value: string | SQLWrapper): SQL;
declare function notLike(column: Column | SQL.Aliased | SQL, value: string | SQLWrapper): SQL;
declare function ilike(column: Column | SQL.Aliased | SQL, value: string | SQLWrapper): SQL;
declare function notIlike(column: Column | SQL.Aliased | SQL, value: string | SQLWrapper): SQL;
declare function asc(column: AnyColumn | SQLWrapper): SQL;
declare function desc(column: AnyColumn | SQLWrapper): SQL;
declare function count(expression?: SQLWrapper): SQL<number>;
declare function sum(expression: SQLWrapper): SQL<string | null>;
declare abstract class TypedQueryBuilder<TSelection, TResult = unknown, TConfig = unknown> implements SQLWrapper {
    static readonly [entityKind]: string;
    _: {
        selectedFields: TSelection;
        result: TResult;
        config?: TConfig;
    };
    abstract getSQL(): SQL;
}
type GelIndexOpClass = 'abstime_ops' | 'access_method' | 'anyarray_eq' | 'anyarray_ge' | 'anyarray_gt' | 'anyarray_le' | 'anyarray_lt' | 'anyarray_ne' | 'bigint_ops' | 'bit_ops' | 'bool_ops' | 'box_ops' | 'bpchar_ops' | 'char_ops' | 'cidr_ops' | 'cstring_ops' | 'date_ops' | 'float_ops' | 'int2_ops' | 'int4_ops' | 'int8_ops' | 'interval_ops' | 'jsonb_ops' | 'macaddr_ops' | 'name_ops' | 'numeric_ops' | 'oid_ops' | 'oidint4_ops' | 'oidint8_ops' | 'oidname_ops' | 'oidvector_ops' | 'point_ops' | 'polygon_ops' | 'range_ops' | 'record_eq' | 'record_ge' | 'record_gt' | 'record_le' | 'record_lt' | 'record_ne' | 'text_ops' | 'time_ops' | 'timestamp_ops' | 'timestamptz_ops' | 'timetz_ops' | 'uuid_ops' | 'varbit_ops' | 'varchar_ops' | 'xml_ops' | 'vector_l2_ops' | 'vector_ip_ops' | 'vector_cosine_ops' | 'vector_l1_ops' | 'bit_hamming_ops' | 'bit_jaccard_ops' | 'halfvec_l2_ops' | 'sparsevec_l2_op' | (string & {});
type TableConfig$4 = TableConfig$5<GelColumn>;
declare class GelTable<T extends TableConfig$4 = TableConfig$4> extends Table<T> {
    static readonly [entityKind]: string;
}
declare abstract class GelColumn<T extends ColumnBaseConfig<ColumnDataType, string> = ColumnBaseConfig<ColumnDataType, string>, TRuntimeConfig extends object = {}, TTypeConfig extends object = {}> extends Column<T, TRuntimeConfig, TTypeConfig & {
    dialect: 'gel';
}> {
    readonly table: GelTable;
    static readonly [entityKind]: string;
    constructor(table: GelTable, config: ColumnBuilderRuntimeConfig<T['data'], TRuntimeConfig>);
}
type IndexedExtraConfigType$1 = {
    order?: 'asc' | 'desc';
    nulls?: 'first' | 'last';
    opClass?: string;
};
declare class GelExtraConfigColumn<T extends ColumnBaseConfig<ColumnDataType, string> = ColumnBaseConfig<ColumnDataType, string>> extends GelColumn<T, IndexedExtraConfigType$1> {
    static readonly [entityKind]: string;
    getSQLType(): string;
    indexConfig: IndexedExtraConfigType$1;
    defaultConfig: IndexedExtraConfigType$1;
    asc(): Omit<this, 'asc' | 'desc'>;
    desc(): Omit<this, 'asc' | 'desc'>;
    nullsFirst(): Omit<this, 'nullsFirst' | 'nullsLast'>;
    nullsLast(): Omit<this, 'nullsFirst' | 'nullsLast'>;
    op(opClass: GelIndexOpClass): Omit<this, 'op'>;
}
declare abstract class Relation<TTableName extends string = string> {
    readonly sourceTable: Table;
    readonly referencedTable: AnyTable<{
        name: TTableName;
    }>;
    readonly relationName: string | undefined;
    static readonly [entityKind]: string;
    readonly $brand: 'Relation';
    readonly referencedTableName: TTableName;
    fieldName: string;
    constructor(sourceTable: Table, referencedTable: AnyTable<{
        name: TTableName;
    }>, relationName: string | undefined);
    abstract withFieldName(fieldName: string): Relation<TTableName>;
}
declare class Relations<TTableName extends string = string, TConfig extends Record<string, Relation> = Record<string, Relation>> {
    readonly table: AnyTable<{
        name: TTableName;
    }>;
    readonly config: (helpers: TableRelationsHelpers<TTableName>) => TConfig;
    static readonly [entityKind]: string;
    readonly $brand: 'Relations';
    constructor(table: AnyTable<{
        name: TTableName;
    }>, config: (helpers: TableRelationsHelpers<TTableName>) => TConfig);
}
declare class One<TTableName extends string = string, TIsNullable extends boolean = boolean> extends Relation<TTableName> {
    readonly config: RelationConfig<TTableName, string, AnyColumn<{
        tableName: TTableName;
    }>[]> | undefined;
    readonly isNullable: TIsNullable;
    static readonly [entityKind]: string;
    protected $relationBrand: 'One';
    constructor(sourceTable: Table, referencedTable: AnyTable<{
        name: TTableName;
    }>, config: RelationConfig<TTableName, string, AnyColumn<{
        tableName: TTableName;
    }>[]> | undefined, isNullable: TIsNullable);
    withFieldName(fieldName: string): One<TTableName>;
}
declare class Many<TTableName extends string> extends Relation<TTableName> {
    readonly config: {
        relationName: string;
    } | undefined;
    static readonly [entityKind]: string;
    protected $relationBrand: 'Many';
    constructor(sourceTable: Table, referencedTable: AnyTable<{
        name: TTableName;
    }>, config: {
        relationName: string;
    } | undefined);
    withFieldName(fieldName: string): Many<TTableName>;
}
type TableRelationsKeysOnly<TSchema extends Record<string, unknown>, TTableName extends string, K extends keyof TSchema> = TSchema[K] extends Relations<TTableName> ? K : never;
type ExtractTableRelationsFromSchema<TSchema extends Record<string, unknown>, TTableName extends string> = ExtractObjectValues<{
    [K in keyof TSchema as TableRelationsKeysOnly<TSchema, TTableName, K>]: TSchema[K] extends Relations<TTableName, infer TConfig> ? TConfig : never;
}>;
type ExtractObjectValues<T> = T[keyof T];
declare function getOperators(): {
    and: typeof and;
    between: typeof between;
    eq: BinaryOperator;
    exists: typeof exists;
    gt: BinaryOperator;
    gte: BinaryOperator;
    ilike: typeof ilike;
    inArray: typeof inArray;
    isNull: typeof isNull;
    isNotNull: typeof isNotNull;
    like: typeof like;
    lt: BinaryOperator;
    lte: BinaryOperator;
    ne: BinaryOperator;
    not: typeof not;
    notBetween: typeof notBetween;
    notExists: typeof notExists;
    notLike: typeof notLike;
    notIlike: typeof notIlike;
    notInArray: typeof notInArray;
    or: typeof or;
    sql: typeof sql;
};
type Operators = ReturnType<typeof getOperators>;
declare function getOrderByOperators(): {
    sql: typeof sql;
    asc: typeof asc;
    desc: typeof desc;
};
type OrderByOperators = ReturnType<typeof getOrderByOperators>;
type FindTableByDBName<TSchema extends TablesRelationalConfig, TTableName extends string> = ExtractObjectValues<{
    [K in keyof TSchema as TSchema[K]['dbName'] extends TTableName ? K : never]: TSchema[K];
}>;
type DBQueryConfig<TRelationType extends 'one' | 'many' = 'one' | 'many', TIsRoot extends boolean = boolean, TSchema extends TablesRelationalConfig = TablesRelationalConfig, TTableConfig extends TableRelationalConfig = TableRelationalConfig> = {
    columns?: {
        [K in keyof TTableConfig['columns']]?: boolean;
    } | undefined;
    with?: {
        [K in keyof TTableConfig['relations']]?: true | DBQueryConfig<TTableConfig['relations'][K] extends One ? 'one' : 'many', false, TSchema, FindTableByDBName<TSchema, TTableConfig['relations'][K]['referencedTableName']>> | undefined;
    } | undefined;
    extras?: Record<string, SQL.Aliased> | ((fields: Simplify<[
        TTableConfig['columns']
    ] extends [
        never
    ] ? {} : TTableConfig['columns']>, operators: {
        sql: Operators['sql'];
    }) => Record<string, SQL.Aliased>) | undefined;
} & (TRelationType extends 'many' ? {
    where?: SQL | undefined | ((fields: Simplify<[
        TTableConfig['columns']
    ] extends [
        never
    ] ? {} : TTableConfig['columns']>, operators: Operators) => SQL | undefined);
    orderBy?: ValueOrArray<AnyColumn | SQL> | ((fields: Simplify<[
        TTableConfig['columns']
    ] extends [
        never
    ] ? {} : TTableConfig['columns']>, operators: OrderByOperators) => ValueOrArray<AnyColumn | SQL>) | undefined;
    limit?: number | Placeholder | undefined;
} & (TIsRoot extends true ? {
    offset?: number | Placeholder | undefined;
} : {}) : {});
interface TableRelationalConfig {
    tsName: string;
    dbName: string;
    columns: Record<string, Column>;
    relations: Record<string, Relation>;
    primaryKey: AnyColumn[];
    schema?: string;
}
type TablesRelationalConfig = Record<string, TableRelationalConfig>;
interface RelationalSchemaConfig<TSchema extends TablesRelationalConfig> {
    fullSchema: Record<string, unknown>;
    schema: TSchema;
    tableNamesMap: Record<string, string>;
}
type ExtractTablesWithRelations<TSchema extends Record<string, unknown>> = {
    [K in keyof TSchema as TSchema[K] extends Table ? K : never]: TSchema[K] extends Table ? {
        tsName: K & string;
        dbName: TSchema[K]['_']['name'];
        columns: TSchema[K]['_']['columns'];
        relations: ExtractTableRelationsFromSchema<TSchema, TSchema[K]['_']['name']>;
        primaryKey: AnyColumn[];
    } : never;
};
type ReturnTypeOrValue<T> = T extends (...args: any[]) => infer R ? R : T;
type BuildRelationResult<TSchema extends TablesRelationalConfig, TInclude, TRelations extends Record<string, Relation>> = {
    [K in NonUndefinedKeysOnly<TInclude> & keyof TRelations]: TRelations[K] extends infer TRel extends Relation ? BuildQueryResult<TSchema, FindTableByDBName<TSchema, TRel['referencedTableName']>, Assume<TInclude[K], true | Record<string, unknown>>> extends infer TResult ? TRel extends One ? TResult | (Equal<TRel['isNullable'], false> extends true ? null : never) : TResult[] : never : never;
};
type NonUndefinedKeysOnly<T> = ExtractObjectValues<{
    [K in keyof T as T[K] extends undefined ? never : K]: K;
}> & keyof T;
type BuildQueryResult<TSchema extends TablesRelationalConfig, TTableConfig extends TableRelationalConfig, TFullSelection extends true | Record<string, unknown>> = Equal<TFullSelection, true> extends true ? InferModelFromColumns<TTableConfig['columns']> : TFullSelection extends Record<string, unknown> ? Simplify<(TFullSelection['columns'] extends Record<string, unknown> ? InferModelFromColumns<{
    [K in Equal<Exclude<TFullSelection['columns'][keyof TFullSelection['columns'] & keyof TTableConfig['columns']], undefined>, false> extends true ? Exclude<keyof TTableConfig['columns'], NonUndefinedKeysOnly<TFullSelection['columns']>> : {
        [K in keyof TFullSelection['columns']]: Equal<TFullSelection['columns'][K], true> extends true ? K : never;
    }[keyof TFullSelection['columns']] & keyof TTableConfig['columns']]: TTableConfig['columns'][K];
}> : InferModelFromColumns<TTableConfig['columns']>) & (TFullSelection['extras'] extends Record<string, unknown> | ((...args: any[]) => Record<string, unknown>) ? {
    [K in NonUndefinedKeysOnly<ReturnTypeOrValue<TFullSelection['extras']>>]: Assume<ReturnTypeOrValue<TFullSelection['extras']>[K], SQL.Aliased>['_']['type'];
} : {}) & (TFullSelection['with'] extends Record<string, unknown> ? BuildRelationResult<TSchema, TFullSelection['with'], TTableConfig['relations']> : {})> : never;
interface RelationConfig<TTableName extends string, TForeignTableName extends string, TColumns extends AnyColumn<{
    tableName: TTableName;
}>[]> {
    relationName?: string;
    fields: TColumns;
    references: ColumnsWithTable<TTableName, TForeignTableName, TColumns>;
}
declare function relations<TTableName extends string, TRelations extends Record<string, Relation<any>>>(table: AnyTable<{
    name: TTableName;
}>, relations: (helpers: TableRelationsHelpers<TTableName>) => TRelations): Relations<TTableName, TRelations>;
declare function createTableRelationsHelpers<TTableName extends string>(sourceTable: AnyTable<{
    name: TTableName;
}>): {
    one: <TForeignTable extends Table, TColumns extends [
        AnyColumn<{
            tableName: TTableName;
        }>,
        ...AnyColumn<{
            tableName: TTableName;
        }>[]
    ]>(table: TForeignTable, config?: RelationConfig<TTableName, TForeignTable["_"]["name"], TColumns> | undefined) => One<TForeignTable["_"]["name"], Equal<TColumns[number]["_"]["notNull"], true>>;
    many: <TForeignTable extends Table>(referencedTable: TForeignTable, config?: {
        relationName: string;
    }) => Many<TForeignTable["_"]["name"]>;
};
type TableRelationsHelpers<TTableName extends string> = ReturnType<typeof createTableRelationsHelpers<TTableName>>;
interface BuildRelationalQueryResult<TTable extends Table = Table, TColumn extends Column = Column> {
    tableTsKey: string;
    selection: {
        dbKey: string;
        tsKey: string;
        field: TColumn | SQL | SQL.Aliased;
        relationTableTsKey: string | undefined;
        isJson: boolean;
        isExtra?: boolean;
        selection: BuildRelationalQueryResult<TTable>['selection'];
    }[];
    sql: TTable | SQL;
}
interface PreparedQuery {
    getQuery(): Query;
    mapResult(response: unknown, isFromBatch?: boolean): unknown;
}
declare abstract class QueryPromise<T> implements Promise<T> {
    static readonly [entityKind]: string;
    [Symbol.toStringTag]: string;
    catch<TResult = never>(onRejected?: ((reason: any) => TResult | PromiseLike<TResult>) | null | undefined): Promise<T | TResult>;
    finally(onFinally?: (() => void) | null | undefined): Promise<T>;
    then<TResult1 = T, TResult2 = never>(onFulfilled?: ((value: T) => TResult1 | PromiseLike<TResult1>) | undefined | null, onRejected?: ((reason: any) => TResult2 | PromiseLike<TResult2>) | undefined | null): Promise<TResult1 | TResult2>;
    abstract execute(): Promise<T>;
}
interface RunnableQuery<T, TDialect extends Dialect> {
    readonly _: {
        readonly dialect: TDialect;
        readonly result: T;
    };
}
type TableConfig$3 = TableConfig$5<MySqlColumn>;
declare class MySqlTable<T extends TableConfig$3 = TableConfig$3> extends Table<T> {
    static readonly [entityKind]: string;
    protected $columns: T['columns'];
}
declare abstract class MySqlColumn<T extends ColumnBaseConfig<ColumnDataType, string> = ColumnBaseConfig<ColumnDataType, string>, TRuntimeConfig extends object = {}, TTypeConfig extends object = {}> extends Column<T, TRuntimeConfig, TTypeConfig & {
    dialect: 'mysql';
}> {
    readonly table: MySqlTable;
    static readonly [entityKind]: string;
    constructor(table: MySqlTable, config: ColumnBuilderRuntimeConfig<T['data'], TRuntimeConfig>);
}
interface MigrationConfig {
    migrationsFolder: string;
    migrationsTable?: string;
    migrationsSchema?: string;
}
interface MigrationMeta {
    sql: string[];
    folderMillis: number;
    hash: string;
    bps: boolean;
}
declare class CheckBuilder {
    name: string;
    value: SQL;
    static readonly [entityKind]: string;
    protected brand: 'PgConstraintBuilder';
    constructor(name: string, value: SQL);
}
type PgBigSerial53BuilderInitial<TName extends string> = NotNull<HasDefault<PgBigSerial53Builder<{
    name: TName;
    dataType: 'number';
    columnType: 'PgBigSerial53';
    data: number;
    driverParam: number;
    enumValues: undefined;
}>>>;
declare class PgBigSerial53Builder<T extends ColumnBuilderBaseConfig<'number', 'PgBigSerial53'>> extends PgColumnBuilder<T> {
    static readonly [entityKind]: string;
    constructor(name: string);
}
type PgBigSerial64BuilderInitial<TName extends string> = NotNull<HasDefault<PgBigSerial64Builder<{
    name: TName;
    dataType: 'bigint';
    columnType: 'PgBigSerial64';
    data: bigint;
    driverParam: string;
    enumValues: undefined;
}>>>;
declare class PgBigSerial64Builder<T extends ColumnBuilderBaseConfig<'bigint', 'PgBigSerial64'>> extends PgColumnBuilder<T> {
    static readonly [entityKind]: string;
    constructor(name: string);
}
interface PgBigSerialConfig<T extends 'number' | 'bigint' = 'number' | 'bigint'> {
    mode: T;
}
declare function bigserial<TMode extends PgBigSerialConfig['mode']>(config: PgBigSerialConfig<TMode>): TMode extends 'number' ? PgBigSerial53BuilderInitial<''> : PgBigSerial64BuilderInitial<''>;
declare function bigserial<TName extends string, TMode extends PgBigSerialConfig['mode']>(name: TName, config: PgBigSerialConfig<TMode>): TMode extends 'number' ? PgBigSerial53BuilderInitial<TName> : PgBigSerial64BuilderInitial<TName>;
type PgBooleanBuilderInitial<TName extends string> = PgBooleanBuilder<{
    name: TName;
    dataType: 'boolean';
    columnType: 'PgBoolean';
    data: boolean;
    driverParam: boolean;
    enumValues: undefined;
}>;
declare class PgBooleanBuilder<T extends ColumnBuilderBaseConfig<'boolean', 'PgBoolean'>> extends PgColumnBuilder<T> {
    static readonly [entityKind]: string;
    constructor(name: T['name']);
}
declare function boolean(): PgBooleanBuilderInitial<''>;
declare function boolean<TName extends string>(name: TName): PgBooleanBuilderInitial<TName>;
type PgCharBuilderInitial<TName extends string, TEnum extends [
    string,
    ...string[]
], TLength extends number | undefined> = PgCharBuilder<{
    name: TName;
    dataType: 'string';
    columnType: 'PgChar';
    data: TEnum[number];
    enumValues: TEnum;
    driverParam: string;
    length: TLength;
}>;
declare class PgCharBuilder<T extends ColumnBuilderBaseConfig<'string', 'PgChar'> & {
    length?: number | undefined;
}> extends PgColumnBuilder<T, {
    length: T['length'];
    enumValues: T['enumValues'];
}, {
    length: T['length'];
}> {
    static readonly [entityKind]: string;
    constructor(name: T['name'], config: PgCharConfig<T['enumValues'], T['length']>);
}
interface PgCharConfig<TEnum extends readonly string[] | string[] | undefined = readonly string[] | string[] | undefined, TLength extends number | undefined = number | undefined> {
    enum?: TEnum;
    length?: TLength;
}
declare function char(): PgCharBuilderInitial<'', [
    string,
    ...string[]
], undefined>;
declare function char<U extends string, T extends Readonly<[
    U,
    ...U[]
]>, L extends number | undefined>(config?: PgCharConfig<T | Writable<T>, L>): PgCharBuilderInitial<'', Writable<T>, L>;
declare function char<TName extends string, U extends string, T extends Readonly<[
    U,
    ...U[]
]>, L extends number | undefined>(name: TName, config?: PgCharConfig<T | Writable<T>, L>): PgCharBuilderInitial<TName, Writable<T>, L>;
type PgCidrBuilderInitial<TName extends string> = PgCidrBuilder<{
    name: TName;
    dataType: 'string';
    columnType: 'PgCidr';
    data: string;
    driverParam: string;
    enumValues: undefined;
}>;
declare class PgCidrBuilder<T extends ColumnBuilderBaseConfig<'string', 'PgCidr'>> extends PgColumnBuilder<T> {
    static readonly [entityKind]: string;
    constructor(name: T['name']);
}
declare function cidr(): PgCidrBuilderInitial<''>;
declare function cidr<TName extends string>(name: TName): PgCidrBuilderInitial<TName>;
type ConvertCustomConfig<TName extends string, T extends Partial<CustomTypeValues>> = {
    name: TName;
    dataType: 'custom';
    columnType: 'PgCustomColumn';
    data: T['data'];
    driverParam: T['driverData'];
    enumValues: undefined;
} & (T['notNull'] extends true ? {
    notNull: true;
} : {}) & (T['default'] extends true ? {
    hasDefault: true;
} : {});
declare class PgCustomColumnBuilder<T extends ColumnBuilderBaseConfig<'custom', 'PgCustomColumn'>> extends PgColumnBuilder<T, {
    fieldConfig: CustomTypeValues['config'];
    customTypeParams: CustomTypeParams<any>;
}, {
    pgColumnBuilderBrand: 'PgCustomColumnBuilderBrand';
}> {
    static readonly [entityKind]: string;
    constructor(name: T['name'], fieldConfig: CustomTypeValues['config'], customTypeParams: CustomTypeParams<any>);
}
type CustomTypeValues = {
    data: unknown;
    driverData?: unknown;
    config?: Record<string, any>;
    configRequired?: boolean;
    notNull?: boolean;
    default?: boolean;
};
interface CustomTypeParams<T extends CustomTypeValues> {
    dataType: (config: T['config'] | (Equal<T['configRequired'], true> extends true ? never : undefined)) => string;
    toDriver?: (value: T['data']) => T['driverData'] | SQL;
    fromDriver?: (value: T['driverData']) => T['data'];
}
declare function customType<T extends CustomTypeValues = CustomTypeValues>(customTypeParams: CustomTypeParams<T>): Equal<T['configRequired'], true> extends true ? {
    <TConfig extends Record<string, any> & T['config']>(fieldConfig: TConfig): PgCustomColumnBuilder<ConvertCustomConfig<'', T>>;
    <TName extends string>(dbName: TName, fieldConfig: T['config']): PgCustomColumnBuilder<ConvertCustomConfig<TName, T>>;
} : {
    (): PgCustomColumnBuilder<ConvertCustomConfig<'', T>>;
    <TConfig extends Record<string, any> & T['config']>(fieldConfig?: TConfig): PgCustomColumnBuilder<ConvertCustomConfig<'', T>>;
    <TName extends string>(dbName: TName, fieldConfig?: T['config']): PgCustomColumnBuilder<ConvertCustomConfig<TName, T>>;
};
declare abstract class PgDateColumnBaseBuilder<T extends ColumnBuilderBaseConfig<ColumnDataType, string>, TRuntimeConfig extends object = object> extends PgColumnBuilder<T, TRuntimeConfig> {
    static readonly [entityKind]: string;
    defaultNow(): HasDefault<this>;
}
type PgDateBuilderInitial<TName extends string> = PgDateBuilder<{
    name: TName;
    dataType: 'date';
    columnType: 'PgDate';
    data: Date;
    driverParam: string;
    enumValues: undefined;
}>;
declare class PgDateBuilder<T extends ColumnBuilderBaseConfig<'date', 'PgDate'>> extends PgDateColumnBaseBuilder<T> {
    static readonly [entityKind]: string;
    constructor(name: T['name']);
}
type PgDateStringBuilderInitial<TName extends string> = PgDateStringBuilder<{
    name: TName;
    dataType: 'string';
    columnType: 'PgDateString';
    data: string;
    driverParam: string;
    enumValues: undefined;
}>;
declare class PgDateStringBuilder<T extends ColumnBuilderBaseConfig<'string', 'PgDateString'>> extends PgDateColumnBaseBuilder<T> {
    static readonly [entityKind]: string;
    constructor(name: T['name']);
}
interface PgDateConfig<T extends 'date' | 'string' = 'date' | 'string'> {
    mode: T;
}
declare function date(): PgDateStringBuilderInitial<''>;
declare function date<TMode extends PgDateConfig['mode'] & {}>(config?: PgDateConfig<TMode>): Equal<TMode, 'date'> extends true ? PgDateBuilderInitial<''> : PgDateStringBuilderInitial<''>;
declare function date<TName extends string, TMode extends PgDateConfig['mode'] & {}>(name: TName, config?: PgDateConfig<TMode>): Equal<TMode, 'date'> extends true ? PgDateBuilderInitial<TName> : PgDateStringBuilderInitial<TName>;
type PgDoublePrecisionBuilderInitial<TName extends string> = PgDoublePrecisionBuilder<{
    name: TName;
    dataType: 'number';
    columnType: 'PgDoublePrecision';
    data: number;
    driverParam: string | number;
    enumValues: undefined;
}>;
declare class PgDoublePrecisionBuilder<T extends ColumnBuilderBaseConfig<'number', 'PgDoublePrecision'>> extends PgColumnBuilder<T> {
    static readonly [entityKind]: string;
    constructor(name: T['name']);
}
declare function doublePrecision(): PgDoublePrecisionBuilderInitial<''>;
declare function doublePrecision<TName extends string>(name: TName): PgDoublePrecisionBuilderInitial<TName>;
type PgInetBuilderInitial<TName extends string> = PgInetBuilder<{
    name: TName;
    dataType: 'string';
    columnType: 'PgInet';
    data: string;
    driverParam: string;
    enumValues: undefined;
}>;
declare class PgInetBuilder<T extends ColumnBuilderBaseConfig<'string', 'PgInet'>> extends PgColumnBuilder<T> {
    static readonly [entityKind]: string;
    constructor(name: T['name']);
}
declare function inet(): PgInetBuilderInitial<''>;
declare function inet<TName extends string>(name: TName): PgInetBuilderInitial<TName>;
type PgSequenceOptions = {
    increment?: number | string;
    minValue?: number | string;
    maxValue?: number | string;
    startWith?: number | string;
    cache?: number | string;
    cycle?: boolean;
};
declare abstract class PgIntColumnBaseBuilder<T extends ColumnBuilderBaseConfig<ColumnDataType, string>> extends PgColumnBuilder<T, {
    generatedIdentity: GeneratedIdentityConfig;
}> {
    static readonly [entityKind]: string;
    generatedAlwaysAsIdentity(sequence?: PgSequenceOptions & {
        name?: string;
    }): IsIdentity<this, 'always'>;
    generatedByDefaultAsIdentity(sequence?: PgSequenceOptions & {
        name?: string;
    }): IsIdentity<this, 'byDefault'>;
}
type PgIntegerBuilderInitial<TName extends string> = PgIntegerBuilder<{
    name: TName;
    dataType: 'number';
    columnType: 'PgInteger';
    data: number;
    driverParam: number | string;
    enumValues: undefined;
}>;
declare class PgIntegerBuilder<T extends ColumnBuilderBaseConfig<'number', 'PgInteger'>> extends PgIntColumnBaseBuilder<T> {
    static readonly [entityKind]: string;
    constructor(name: T['name']);
}
declare function integer(): PgIntegerBuilderInitial<''>;
declare function integer<TName extends string>(name: TName): PgIntegerBuilderInitial<TName>;
type PgTimestampBuilderInitial<TName extends string> = PgTimestampBuilder<{
    name: TName;
    dataType: 'date';
    columnType: 'PgTimestamp';
    data: Date;
    driverParam: string;
    enumValues: undefined;
}>;
declare class PgTimestampBuilder<T extends ColumnBuilderBaseConfig<'date', 'PgTimestamp'>> extends PgDateColumnBaseBuilder<T, {
    withTimezone: boolean;
    precision: number | undefined;
}> {
    static readonly [entityKind]: string;
    constructor(name: T['name'], withTimezone: boolean, precision: number | undefined);
}
type PgTimestampStringBuilderInitial<TName extends string> = PgTimestampStringBuilder<{
    name: TName;
    dataType: 'string';
    columnType: 'PgTimestampString';
    data: string;
    driverParam: string;
    enumValues: undefined;
}>;
declare class PgTimestampStringBuilder<T extends ColumnBuilderBaseConfig<'string', 'PgTimestampString'>> extends PgDateColumnBaseBuilder<T, {
    withTimezone: boolean;
    precision: number | undefined;
}> {
    static readonly [entityKind]: string;
    constructor(name: T['name'], withTimezone: boolean, precision: number | undefined);
}
type Precision = 0 | 1 | 2 | 3 | 4 | 5 | 6;
interface PgTimestampConfig<TMode extends 'date' | 'string' = 'date' | 'string'> {
    mode?: TMode;
    precision?: Precision;
    withTimezone?: boolean;
}
declare function timestamp(): PgTimestampBuilderInitial<''>;
declare function timestamp<TMode extends PgTimestampConfig['mode'] & {}>(config?: PgTimestampConfig<TMode>): Equal<TMode, 'string'> extends true ? PgTimestampStringBuilderInitial<''> : PgTimestampBuilderInitial<''>;
declare function timestamp<TName extends string, TMode extends PgTimestampConfig['mode'] & {}>(name: TName, config?: PgTimestampConfig<TMode>): Equal<TMode, 'string'> extends true ? PgTimestampStringBuilderInitial<TName> : PgTimestampBuilderInitial<TName>;
type PgIntervalBuilderInitial<TName extends string> = PgIntervalBuilder<{
    name: TName;
    dataType: 'string';
    columnType: 'PgInterval';
    data: string;
    driverParam: string;
    enumValues: undefined;
}>;
declare class PgIntervalBuilder<T extends ColumnBuilderBaseConfig<'string', 'PgInterval'>> extends PgColumnBuilder<T, {
    intervalConfig: IntervalConfig;
}> {
    static readonly [entityKind]: string;
    constructor(name: T['name'], intervalConfig: IntervalConfig);
}
interface IntervalConfig {
    fields?: 'year' | 'month' | 'day' | 'hour' | 'minute' | 'second' | 'year to month' | 'day to hour' | 'day to minute' | 'day to second' | 'hour to minute' | 'hour to second' | 'minute to second';
    precision?: Precision;
}
declare function interval(): PgIntervalBuilderInitial<''>;
declare function interval(config?: IntervalConfig): PgIntervalBuilderInitial<''>;
declare function interval<TName extends string>(name: TName, config?: IntervalConfig): PgIntervalBuilderInitial<TName>;
type PgJsonBuilderInitial<TName extends string> = PgJsonBuilder<{
    name: TName;
    dataType: 'json';
    columnType: 'PgJson';
    data: unknown;
    driverParam: unknown;
    enumValues: undefined;
}>;
declare class PgJsonBuilder<T extends ColumnBuilderBaseConfig<'json', 'PgJson'>> extends PgColumnBuilder<T> {
    static readonly [entityKind]: string;
    constructor(name: T['name']);
}
declare function json(): PgJsonBuilderInitial<''>;
declare function json<TName extends string>(name: TName): PgJsonBuilderInitial<TName>;
type PgJsonbBuilderInitial<TName extends string> = PgJsonbBuilder<{
    name: TName;
    dataType: 'json';
    columnType: 'PgJsonb';
    data: unknown;
    driverParam: unknown;
    enumValues: undefined;
}>;
declare class PgJsonbBuilder<T extends ColumnBuilderBaseConfig<'json', 'PgJsonb'>> extends PgColumnBuilder<T> {
    static readonly [entityKind]: string;
    constructor(name: T['name']);
}
declare function jsonb(): PgJsonbBuilderInitial<''>;
declare function jsonb<TName extends string>(name: TName): PgJsonbBuilderInitial<TName>;
type PgLineBuilderInitial<TName extends string> = PgLineBuilder<{
    name: TName;
    dataType: 'array';
    columnType: 'PgLine';
    data: [
        number,
        number,
        number
    ];
    driverParam: number | string;
    enumValues: undefined;
}>;
declare class PgLineBuilder<T extends ColumnBuilderBaseConfig<'array', 'PgLine'>> extends PgColumnBuilder<T> {
    static readonly [entityKind]: string;
    constructor(name: T['name']);
}
type PgLineABCBuilderInitial<TName extends string> = PgLineABCBuilder<{
    name: TName;
    dataType: 'json';
    columnType: 'PgLineABC';
    data: {
        a: number;
        b: number;
        c: number;
    };
    driverParam: string;
    enumValues: undefined;
}>;
declare class PgLineABCBuilder<T extends ColumnBuilderBaseConfig<'json', 'PgLineABC'>> extends PgColumnBuilder<T> {
    static readonly [entityKind]: string;
    constructor(name: T['name']);
}
interface PgLineTypeConfig<T extends 'tuple' | 'abc' = 'tuple' | 'abc'> {
    mode?: T;
}
declare function line(): PgLineBuilderInitial<''>;
declare function line<TMode extends PgLineTypeConfig['mode'] & {}>(config?: PgLineTypeConfig<TMode>): Equal<TMode, 'abc'> extends true ? PgLineABCBuilderInitial<''> : PgLineBuilderInitial<''>;
declare function line<TName extends string, TMode extends PgLineTypeConfig['mode'] & {}>(name: TName, config?: PgLineTypeConfig<TMode>): Equal<TMode, 'abc'> extends true ? PgLineABCBuilderInitial<TName> : PgLineBuilderInitial<TName>;
type PgMacaddrBuilderInitial<TName extends string> = PgMacaddrBuilder<{
    name: TName;
    dataType: 'string';
    columnType: 'PgMacaddr';
    data: string;
    driverParam: string;
    enumValues: undefined;
}>;
declare class PgMacaddrBuilder<T extends ColumnBuilderBaseConfig<'string', 'PgMacaddr'>> extends PgColumnBuilder<T> {
    static readonly [entityKind]: string;
    constructor(name: T['name']);
}
declare function macaddr(): PgMacaddrBuilderInitial<''>;
declare function macaddr<TName extends string>(name: TName): PgMacaddrBuilderInitial<TName>;
type PgMacaddr8BuilderInitial<TName extends string> = PgMacaddr8Builder<{
    name: TName;
    dataType: 'string';
    columnType: 'PgMacaddr8';
    data: string;
    driverParam: string;
    enumValues: undefined;
}>;
declare class PgMacaddr8Builder<T extends ColumnBuilderBaseConfig<'string', 'PgMacaddr8'>> extends PgColumnBuilder<T> {
    static readonly [entityKind]: string;
    constructor(name: T['name']);
}
declare function macaddr8(): PgMacaddr8BuilderInitial<''>;
declare function macaddr8<TName extends string>(name: TName): PgMacaddr8BuilderInitial<TName>;
type PgNumericBuilderInitial<TName extends string> = PgNumericBuilder<{
    name: TName;
    dataType: 'string';
    columnType: 'PgNumeric';
    data: string;
    driverParam: string;
    enumValues: undefined;
}>;
declare class PgNumericBuilder<T extends ColumnBuilderBaseConfig<'string', 'PgNumeric'>> extends PgColumnBuilder<T, {
    precision: number | undefined;
    scale: number | undefined;
}> {
    static readonly [entityKind]: string;
    constructor(name: T['name'], precision?: number, scale?: number);
}
type PgNumericNumberBuilderInitial<TName extends string> = PgNumericNumberBuilder<{
    name: TName;
    dataType: 'number';
    columnType: 'PgNumericNumber';
    data: number;
    driverParam: string;
    enumValues: undefined;
}>;
declare class PgNumericNumberBuilder<T extends ColumnBuilderBaseConfig<'number', 'PgNumericNumber'>> extends PgColumnBuilder<T, {
    precision: number | undefined;
    scale: number | undefined;
}> {
    static readonly [entityKind]: string;
    constructor(name: T['name'], precision?: number, scale?: number);
}
type PgNumericBigIntBuilderInitial<TName extends string> = PgNumericBigIntBuilder<{
    name: TName;
    dataType: 'bigint';
    columnType: 'PgNumericBigInt';
    data: bigint;
    driverParam: string;
    enumValues: undefined;
}>;
declare class PgNumericBigIntBuilder<T extends ColumnBuilderBaseConfig<'bigint', 'PgNumericBigInt'>> extends PgColumnBuilder<T, {
    precision: number | undefined;
    scale: number | undefined;
}> {
    static readonly [entityKind]: string;
    constructor(name: T['name'], precision?: number, scale?: number);
}
type PgNumericConfig<T extends 'string' | 'number' | 'bigint' = 'string' | 'number' | 'bigint'> = {
    precision: number;
    scale?: number;
    mode?: T;
} | {
    precision?: number;
    scale: number;
    mode?: T;
} | {
    precision?: number;
    scale?: number;
    mode: T;
};
declare function numeric<TMode extends 'string' | 'number' | 'bigint'>(config?: PgNumericConfig<TMode>): Equal<TMode, 'number'> extends true ? PgNumericNumberBuilderInitial<''> : Equal<TMode, 'bigint'> extends true ? PgNumericBigIntBuilderInitial<''> : PgNumericBuilderInitial<''>;
declare function numeric<TName extends string, TMode extends 'string' | 'number' | 'bigint'>(name: TName, config?: PgNumericConfig<TMode>): Equal<TMode, 'number'> extends true ? PgNumericNumberBuilderInitial<TName> : Equal<TMode, 'bigint'> extends true ? PgNumericBigIntBuilderInitial<TName> : PgNumericBuilderInitial<TName>;
type PgPointTupleBuilderInitial<TName extends string> = PgPointTupleBuilder<{
    name: TName;
    dataType: 'array';
    columnType: 'PgPointTuple';
    data: [
        number,
        number
    ];
    driverParam: number | string;
    enumValues: undefined;
}>;
declare class PgPointTupleBuilder<T extends ColumnBuilderBaseConfig<'array', 'PgPointTuple'>> extends PgColumnBuilder<T> {
    static readonly [entityKind]: string;
    constructor(name: string);
}
type PgPointObjectBuilderInitial<TName extends string> = PgPointObjectBuilder<{
    name: TName;
    dataType: 'json';
    columnType: 'PgPointObject';
    data: {
        x: number;
        y: number;
    };
    driverParam: string;
    enumValues: undefined;
}>;
declare class PgPointObjectBuilder<T extends ColumnBuilderBaseConfig<'json', 'PgPointObject'>> extends PgColumnBuilder<T> {
    static readonly [entityKind]: string;
    constructor(name: string);
}
interface PgPointConfig<T extends 'tuple' | 'xy' = 'tuple' | 'xy'> {
    mode?: T;
}
declare function point(): PgPointTupleBuilderInitial<''>;
declare function point<TMode extends PgPointConfig['mode'] & {}>(config?: PgPointConfig<TMode>): Equal<TMode, 'xy'> extends true ? PgPointObjectBuilderInitial<''> : PgPointTupleBuilderInitial<''>;
declare function point<TName extends string, TMode extends PgPointConfig['mode'] & {}>(name: TName, config?: PgPointConfig<TMode>): Equal<TMode, 'xy'> extends true ? PgPointObjectBuilderInitial<TName> : PgPointTupleBuilderInitial<TName>;
type PgGeometryBuilderInitial<TName extends string> = PgGeometryBuilder<{
    name: TName;
    dataType: 'array';
    columnType: 'PgGeometry';
    data: [
        number,
        number
    ];
    driverParam: string;
    enumValues: undefined;
}>;
declare class PgGeometryBuilder<T extends ColumnBuilderBaseConfig<'array', 'PgGeometry'>> extends PgColumnBuilder<T> {
    static readonly [entityKind]: string;
    constructor(name: T['name']);
}
type PgGeometryObjectBuilderInitial<TName extends string> = PgGeometryObjectBuilder<{
    name: TName;
    dataType: 'json';
    columnType: 'PgGeometryObject';
    data: {
        x: number;
        y: number;
    };
    driverParam: string;
    enumValues: undefined;
}>;
declare class PgGeometryObjectBuilder<T extends ColumnBuilderBaseConfig<'json', 'PgGeometryObject'>> extends PgColumnBuilder<T> {
    static readonly [entityKind]: string;
    constructor(name: T['name']);
}
interface PgGeometryConfig<T extends 'tuple' | 'xy' = 'tuple' | 'xy'> {
    mode?: T;
    type?: 'point' | (string & {});
    srid?: number;
}
declare function geometry(): PgGeometryBuilderInitial<''>;
declare function geometry<TMode extends PgGeometryConfig['mode'] & {}>(config?: PgGeometryConfig<TMode>): Equal<TMode, 'xy'> extends true ? PgGeometryObjectBuilderInitial<''> : PgGeometryBuilderInitial<''>;
declare function geometry<TName extends string, TMode extends PgGeometryConfig['mode'] & {}>(name: TName, config?: PgGeometryConfig<TMode>): Equal<TMode, 'xy'> extends true ? PgGeometryObjectBuilderInitial<TName> : PgGeometryBuilderInitial<TName>;
type PgRealBuilderInitial<TName extends string> = PgRealBuilder<{
    name: TName;
    dataType: 'number';
    columnType: 'PgReal';
    data: number;
    driverParam: string | number;
    enumValues: undefined;
}>;
declare class PgRealBuilder<T extends ColumnBuilderBaseConfig<'number', 'PgReal'>> extends PgColumnBuilder<T, {
    length: number | undefined;
}> {
    static readonly [entityKind]: string;
    constructor(name: T['name'], length?: number);
}
declare function real(): PgRealBuilderInitial<''>;
declare function real<TName extends string>(name: TName): PgRealBuilderInitial<TName>;
type PgSerialBuilderInitial<TName extends string> = NotNull<HasDefault<PgSerialBuilder<{
    name: TName;
    dataType: 'number';
    columnType: 'PgSerial';
    data: number;
    driverParam: number;
    enumValues: undefined;
}>>>;
declare class PgSerialBuilder<T extends ColumnBuilderBaseConfig<'number', 'PgSerial'>> extends PgColumnBuilder<T> {
    static readonly [entityKind]: string;
    constructor(name: T['name']);
}
declare function serial(): PgSerialBuilderInitial<''>;
declare function serial<TName extends string>(name: TName): PgSerialBuilderInitial<TName>;
type PgSmallIntBuilderInitial<TName extends string> = PgSmallIntBuilder<{
    name: TName;
    dataType: 'number';
    columnType: 'PgSmallInt';
    data: number;
    driverParam: number | string;
    enumValues: undefined;
}>;
declare class PgSmallIntBuilder<T extends ColumnBuilderBaseConfig<'number', 'PgSmallInt'>> extends PgIntColumnBaseBuilder<T> {
    static readonly [entityKind]: string;
    constructor(name: T['name']);
}
declare function smallint(): PgSmallIntBuilderInitial<''>;
declare function smallint<TName extends string>(name: TName): PgSmallIntBuilderInitial<TName>;
type PgSmallSerialBuilderInitial<TName extends string> = NotNull<HasDefault<PgSmallSerialBuilder<{
    name: TName;
    dataType: 'number';
    columnType: 'PgSmallSerial';
    data: number;
    driverParam: number;
    enumValues: undefined;
}>>>;
declare class PgSmallSerialBuilder<T extends ColumnBuilderBaseConfig<'number', 'PgSmallSerial'>> extends PgColumnBuilder<T> {
    static readonly [entityKind]: string;
    constructor(name: T['name']);
}
declare function smallserial(): PgSmallSerialBuilderInitial<''>;
declare function smallserial<TName extends string>(name: TName): PgSmallSerialBuilderInitial<TName>;
type PgTextBuilderInitial<TName extends string, TEnum extends [
    string,
    ...string[]
]> = PgTextBuilder<{
    name: TName;
    dataType: 'string';
    columnType: 'PgText';
    data: TEnum[number];
    enumValues: TEnum;
    driverParam: string;
}>;
declare class PgTextBuilder<T extends ColumnBuilderBaseConfig<'string', 'PgText'>> extends PgColumnBuilder<T, {
    enumValues: T['enumValues'];
}> {
    static readonly [entityKind]: string;
    constructor(name: T['name'], config: PgTextConfig<T['enumValues']>);
}
interface PgTextConfig<TEnum extends readonly string[] | string[] | undefined = readonly string[] | string[] | undefined> {
    enum?: TEnum;
}
declare function text(): PgTextBuilderInitial<'', [
    string,
    ...string[]
]>;
declare function text<U extends string, T extends Readonly<[
    U,
    ...U[]
]>>(config?: PgTextConfig<T | Writable<T>>): PgTextBuilderInitial<'', Writable<T>>;
declare function text<TName extends string, U extends string, T extends Readonly<[
    U,
    ...U[]
]>>(name: TName, config?: PgTextConfig<T | Writable<T>>): PgTextBuilderInitial<TName, Writable<T>>;
type PgTimeBuilderInitial<TName extends string> = PgTimeBuilder<{
    name: TName;
    dataType: 'string';
    columnType: 'PgTime';
    data: string;
    driverParam: string;
    enumValues: undefined;
}>;
declare class PgTimeBuilder<T extends ColumnBuilderBaseConfig<'string', 'PgTime'>> extends PgDateColumnBaseBuilder<T, {
    withTimezone: boolean;
    precision: number | undefined;
}> {
    readonly withTimezone: boolean;
    readonly precision: number | undefined;
    static readonly [entityKind]: string;
    constructor(name: T['name'], withTimezone: boolean, precision: number | undefined);
}
interface TimeConfig {
    precision?: Precision;
    withTimezone?: boolean;
}
declare function time(): PgTimeBuilderInitial<''>;
declare function time(config?: TimeConfig): PgTimeBuilderInitial<''>;
declare function time<TName extends string>(name: TName, config?: TimeConfig): PgTimeBuilderInitial<TName>;
type PgUUIDBuilderInitial<TName extends string> = PgUUIDBuilder<{
    name: TName;
    dataType: 'string';
    columnType: 'PgUUID';
    data: string;
    driverParam: string;
    enumValues: undefined;
}>;
declare class PgUUIDBuilder<T extends ColumnBuilderBaseConfig<'string', 'PgUUID'>> extends PgColumnBuilder<T> {
    static readonly [entityKind]: string;
    constructor(name: T['name']);
    defaultRandom(): ReturnType<this['default']>;
}
declare function uuid(): PgUUIDBuilderInitial<''>;
declare function uuid<TName extends string>(name: TName): PgUUIDBuilderInitial<TName>;
type PgVarcharBuilderInitial<TName extends string, TEnum extends [
    string,
    ...string[]
], TLength extends number | undefined> = PgVarcharBuilder<{
    name: TName;
    dataType: 'string';
    columnType: 'PgVarchar';
    data: TEnum[number];
    driverParam: string;
    enumValues: TEnum;
    length: TLength;
}>;
declare class PgVarcharBuilder<T extends ColumnBuilderBaseConfig<'string', 'PgVarchar'> & {
    length?: number | undefined;
}> extends PgColumnBuilder<T, {
    length: T['length'];
    enumValues: T['enumValues'];
}, {
    length: T['length'];
}> {
    static readonly [entityKind]: string;
    constructor(name: T['name'], config: PgVarcharConfig<T['enumValues'], T['length']>);
}
interface PgVarcharConfig<TEnum extends readonly string[] | string[] | undefined = readonly string[] | string[] | undefined, TLength extends number | undefined = number | undefined> {
    enum?: TEnum;
    length?: TLength;
}
declare function varchar(): PgVarcharBuilderInitial<'', [
    string,
    ...string[]
], undefined>;
declare function varchar<U extends string, T extends Readonly<[
    U,
    ...U[]
]>, L extends number | undefined>(config?: PgVarcharConfig<T | Writable<T>, L>): PgVarcharBuilderInitial<'', Writable<T>, L>;
declare function varchar<TName extends string, U extends string, T extends Readonly<[
    U,
    ...U[]
]>, L extends number | undefined>(name: TName, config?: PgVarcharConfig<T | Writable<T>, L>): PgVarcharBuilderInitial<TName, Writable<T>, L>;
type PgBinaryVectorBuilderInitial<TName extends string, TDimensions extends number> = PgBinaryVectorBuilder<{
    name: TName;
    dataType: 'string';
    columnType: 'PgBinaryVector';
    data: string;
    driverParam: string;
    enumValues: undefined;
    dimensions: TDimensions;
}>;
declare class PgBinaryVectorBuilder<T extends ColumnBuilderBaseConfig<'string', 'PgBinaryVector'> & {
    dimensions: number;
}> extends PgColumnBuilder<T, {
    dimensions: T['dimensions'];
}> {
    static readonly [entityKind]: string;
    constructor(name: string, config: PgBinaryVectorConfig<T['dimensions']>);
}
interface PgBinaryVectorConfig<TDimensions extends number = number> {
    dimensions: TDimensions;
}
declare function bit<D extends number>(config: PgBinaryVectorConfig<D>): PgBinaryVectorBuilderInitial<'', D>;
declare function bit<TName extends string, D extends number>(name: TName, config: PgBinaryVectorConfig<D>): PgBinaryVectorBuilderInitial<TName, D>;
type PgHalfVectorBuilderInitial<TName extends string, TDimensions extends number> = PgHalfVectorBuilder<{
    name: TName;
    dataType: 'array';
    columnType: 'PgHalfVector';
    data: number[];
    driverParam: string;
    enumValues: undefined;
    dimensions: TDimensions;
}>;
declare class PgHalfVectorBuilder<T extends ColumnBuilderBaseConfig<'array', 'PgHalfVector'> & {
    dimensions: number;
}> extends PgColumnBuilder<T, {
    dimensions: T['dimensions'];
}, {
    dimensions: T['dimensions'];
}> {
    static readonly [entityKind]: string;
    constructor(name: string, config: PgHalfVectorConfig<T['dimensions']>);
}
interface PgHalfVectorConfig<TDimensions extends number = number> {
    dimensions: TDimensions;
}
declare function halfvec<D extends number>(config: PgHalfVectorConfig<D>): PgHalfVectorBuilderInitial<'', D>;
declare function halfvec<TName extends string, D extends number>(name: TName, config: PgHalfVectorConfig): PgHalfVectorBuilderInitial<TName, D>;
type PgSparseVectorBuilderInitial<TName extends string> = PgSparseVectorBuilder<{
    name: TName;
    dataType: 'string';
    columnType: 'PgSparseVector';
    data: string;
    driverParam: string;
    enumValues: undefined;
}>;
declare class PgSparseVectorBuilder<T extends ColumnBuilderBaseConfig<'string', 'PgSparseVector'>> extends PgColumnBuilder<T, {
    dimensions: number | undefined;
}> {
    static readonly [entityKind]: string;
    constructor(name: string, config: PgSparseVectorConfig);
}
interface PgSparseVectorConfig {
    dimensions: number;
}
declare function sparsevec(config: PgSparseVectorConfig): PgSparseVectorBuilderInitial<''>;
declare function sparsevec<TName extends string>(name: TName, config: PgSparseVectorConfig): PgSparseVectorBuilderInitial<TName>;
type PgVectorBuilderInitial<TName extends string, TDimensions extends number> = PgVectorBuilder<{
    name: TName;
    dataType: 'array';
    columnType: 'PgVector';
    data: number[];
    driverParam: string;
    enumValues: undefined;
    dimensions: TDimensions;
}>;
declare class PgVectorBuilder<T extends ColumnBuilderBaseConfig<'array', 'PgVector'> & {
    dimensions: number;
}> extends PgColumnBuilder<T, {
    dimensions: T['dimensions'];
}, {
    dimensions: T['dimensions'];
}> {
    static readonly [entityKind]: string;
    constructor(name: string, config: PgVectorConfig<T['dimensions']>);
}
interface PgVectorConfig<TDimensions extends number = number> {
    dimensions: TDimensions;
}
declare function vector<D extends number>(config: PgVectorConfig<D>): PgVectorBuilderInitial<'', D>;
declare function vector<TName extends string, D extends number>(name: TName, config: PgVectorConfig<D>): PgVectorBuilderInitial<TName, D>;
declare function getPgColumnBuilders(): {
    bigint: typeof bigint;
    bigserial: typeof bigserial;
    boolean: typeof boolean;
    char: typeof char;
    cidr: typeof cidr;
    customType: typeof customType;
    date: typeof date;
    doublePrecision: typeof doublePrecision;
    inet: typeof inet;
    integer: typeof integer;
    interval: typeof interval;
    json: typeof json;
    jsonb: typeof jsonb;
    line: typeof line;
    macaddr: typeof macaddr;
    macaddr8: typeof macaddr8;
    numeric: typeof numeric;
    point: typeof point;
    geometry: typeof geometry;
    real: typeof real;
    serial: typeof serial;
    smallint: typeof smallint;
    smallserial: typeof smallserial;
    text: typeof text;
    time: typeof time;
    timestamp: typeof timestamp;
    uuid: typeof uuid;
    varchar: typeof varchar;
    bit: typeof bit;
    halfvec: typeof halfvec;
    sparsevec: typeof sparsevec;
    vector: typeof vector;
};
type PgColumnsBuilders = ReturnType<typeof getPgColumnBuilders>;
interface IndexConfig {
    name?: string;
    columns: Partial<IndexedColumn | SQL>[];
    unique: boolean;
    concurrently?: boolean;
    only: boolean;
    where?: SQL;
    with?: Record<string, any>;
    method?: 'btree' | string;
}
type IndexColumn = PgColumn;
type PgIndexMethod = 'btree' | 'hash' | 'gist' | 'spgist' | 'gin' | 'brin' | 'hnsw' | 'ivfflat' | (string & {});
type PgIndexOpClass = 'abstime_ops' | 'access_method' | 'anyarray_eq' | 'anyarray_ge' | 'anyarray_gt' | 'anyarray_le' | 'anyarray_lt' | 'anyarray_ne' | 'bigint_ops' | 'bit_ops' | 'bool_ops' | 'box_ops' | 'bpchar_ops' | 'char_ops' | 'cidr_ops' | 'cstring_ops' | 'date_ops' | 'float_ops' | 'int2_ops' | 'int4_ops' | 'int8_ops' | 'interval_ops' | 'jsonb_ops' | 'macaddr_ops' | 'name_ops' | 'numeric_ops' | 'oid_ops' | 'oidint4_ops' | 'oidint8_ops' | 'oidname_ops' | 'oidvector_ops' | 'point_ops' | 'polygon_ops' | 'range_ops' | 'record_eq' | 'record_ge' | 'record_gt' | 'record_le' | 'record_lt' | 'record_ne' | 'text_ops' | 'time_ops' | 'timestamp_ops' | 'timestamptz_ops' | 'timetz_ops' | 'uuid_ops' | 'varbit_ops' | 'varchar_ops' | 'xml_ops' | 'vector_l2_ops' | 'vector_ip_ops' | 'vector_cosine_ops' | 'vector_l1_ops' | 'bit_hamming_ops' | 'bit_jaccard_ops' | 'halfvec_l2_ops' | 'sparsevec_l2_op' | (string & {});
declare class IndexBuilderOn {
    private unique;
    private name?;
    static readonly [entityKind]: string;
    constructor(unique: boolean, name?: string | undefined);
    on(...columns: [
        Partial<ExtraConfigColumn> | SQL,
        ...Partial<ExtraConfigColumn | SQL>[]
    ]): IndexBuilder;
    onOnly(...columns: [
        Partial<ExtraConfigColumn | SQL>,
        ...Partial<ExtraConfigColumn | SQL>[]
    ]): IndexBuilder;
    using(method: PgIndexMethod, ...columns: [
        Partial<ExtraConfigColumn | SQL>,
        ...Partial<ExtraConfigColumn | SQL>[]
    ]): IndexBuilder;
}
interface AnyIndexBuilder {
    build(table: PgTable): Index;
}
interface IndexBuilder extends AnyIndexBuilder {
}
declare class IndexBuilder implements AnyIndexBuilder {
    static readonly [entityKind]: string;
    constructor(columns: Partial<IndexedColumn | SQL>[], unique: boolean, only: boolean, name?: string, method?: string);
    concurrently(): this;
    with(obj: Record<string, any>): this;
    where(condition: SQL): this;
}
declare class Index {
    static readonly [entityKind]: string;
    readonly config: IndexConfig & {
        table: PgTable;
    };
    constructor(config: IndexConfig, table: PgTable);
}
declare function index(name?: string): IndexBuilderOn;
interface PgRoleConfig {
    createDb?: boolean;
    createRole?: boolean;
    inherit?: boolean;
}
declare class PgRole implements PgRoleConfig {
    readonly name: string;
    static readonly [entityKind]: string;
    constructor(name: string, config?: PgRoleConfig);
    existing(): this;
}
type PgPolicyToOption = 'public' | 'current_role' | 'current_user' | 'session_user' | (string & {}) | PgPolicyToOption[] | PgRole;
interface PgPolicyConfig {
    as?: 'permissive' | 'restrictive';
    for?: 'all' | 'select' | 'insert' | 'update' | 'delete';
    to?: PgPolicyToOption;
    using?: SQL;
    withCheck?: SQL;
}
declare class PgPolicy implements PgPolicyConfig {
    readonly name: string;
    static readonly [entityKind]: string;
    readonly as: PgPolicyConfig['as'];
    readonly for: PgPolicyConfig['for'];
    readonly to: PgPolicyConfig['to'];
    readonly using: PgPolicyConfig['using'];
    readonly withCheck: PgPolicyConfig['withCheck'];
    constructor(name: string, config?: PgPolicyConfig);
    link(table: PgTable): this;
}
declare function primaryKey<TTableName extends string, TColumn extends AnyPgColumn<{
    tableName: TTableName;
}>, TColumns extends AnyPgColumn<{
    tableName: TTableName;
}>[]>(config: {
    name?: string;
    columns: [
        TColumn,
        ...TColumns
    ];
}): PrimaryKeyBuilder;
declare function primaryKey<TTableName extends string, TColumns extends AnyPgColumn<{
    tableName: TTableName;
}>[]>(...columns: TColumns): PrimaryKeyBuilder;
declare class PrimaryKeyBuilder {
    static readonly [entityKind]: string;
    constructor(columns: PgColumn[], name?: string);
}
declare class UniqueConstraintBuilder {
    private name?;
    static readonly [entityKind]: string;
    constructor(columns: PgColumn[], name?: string | undefined);
    nullsNotDistinct(): this;
}
type PgTableExtraConfigValue = AnyIndexBuilder | CheckBuilder | ForeignKeyBuilder | PrimaryKeyBuilder | UniqueConstraintBuilder | PgPolicy;
type PgTableExtraConfig = Record<string, PgTableExtraConfigValue>;
type TableConfig$2 = TableConfig$5<PgColumn>;
declare class PgTable<T extends TableConfig$2 = TableConfig$2> extends Table<T> {
    static readonly [entityKind]: string;
}
type PgTableWithColumns<T extends TableConfig$2> = PgTable<T> & {
    [Key in keyof T['columns']]: T['columns'][Key];
} & {
    enableRLS: () => Omit<PgTableWithColumns<T>, 'enableRLS'>;
};
interface PgTableFn<TSchema extends string | undefined = undefined> {
    <TTableName extends string, TColumnsMap extends Record<string, PgColumnBuilderBase>>(name: TTableName, columns: TColumnsMap, extraConfig?: (self: BuildExtraConfigColumns<TTableName, TColumnsMap, 'pg'>) => PgTableExtraConfigValue[]): PgTableWithColumns<{
        name: TTableName;
        schema: TSchema;
        columns: BuildColumns<TTableName, TColumnsMap, 'pg'>;
        dialect: 'pg';
    }>;
    <TTableName extends string, TColumnsMap extends Record<string, PgColumnBuilderBase>>(name: TTableName, columns: (columnTypes: PgColumnsBuilders) => TColumnsMap, extraConfig?: (self: BuildExtraConfigColumns<TTableName, TColumnsMap, 'pg'>) => PgTableExtraConfigValue[]): PgTableWithColumns<{
        name: TTableName;
        schema: TSchema;
        columns: BuildColumns<TTableName, TColumnsMap, 'pg'>;
        dialect: 'pg';
    }>;
    <TTableName extends string, TColumnsMap extends Record<string, PgColumnBuilderBase>>(name: TTableName, columns: TColumnsMap, extraConfig: (self: BuildExtraConfigColumns<TTableName, TColumnsMap, 'pg'>) => PgTableExtraConfig): PgTableWithColumns<{
        name: TTableName;
        schema: TSchema;
        columns: BuildColumns<TTableName, TColumnsMap, 'pg'>;
        dialect: 'pg';
    }>;
    <TTableName extends string, TColumnsMap extends Record<string, PgColumnBuilderBase>>(name: TTableName, columns: (columnTypes: PgColumnsBuilders) => TColumnsMap, extraConfig: (self: BuildExtraConfigColumns<TTableName, TColumnsMap, 'pg'>) => PgTableExtraConfig): PgTableWithColumns<{
        name: TTableName;
        schema: TSchema;
        columns: BuildColumns<TTableName, TColumnsMap, 'pg'>;
        dialect: 'pg';
    }>;
}
declare const pgTable: PgTableFn;
type UpdateDeleteAction = 'cascade' | 'restrict' | 'no action' | 'set null' | 'set default';
declare class ForeignKeyBuilder {
    static readonly [entityKind]: string;
    constructor(config: () => {
        name?: string;
        columns: PgColumn[];
        foreignColumns: PgColumn[];
    }, actions?: {
        onUpdate?: UpdateDeleteAction;
        onDelete?: UpdateDeleteAction;
    } | undefined);
    onUpdate(action: UpdateDeleteAction): this;
    onDelete(action: UpdateDeleteAction): this;
}
interface ReferenceConfig {
    ref: () => PgColumn;
    actions: {
        onUpdate?: UpdateDeleteAction;
        onDelete?: UpdateDeleteAction;
    };
}
interface PgColumnBuilderBase<T extends ColumnBuilderBaseConfig<ColumnDataType, string> = ColumnBuilderBaseConfig<ColumnDataType, string>, TTypeConfig extends object = object> extends ColumnBuilderBase<T, TTypeConfig & {
    dialect: 'pg';
}> {
}
declare abstract class PgColumnBuilder<T extends ColumnBuilderBaseConfig<ColumnDataType, string> = ColumnBuilderBaseConfig<ColumnDataType, string>, TRuntimeConfig extends object = object, TTypeConfig extends object = object, TExtraConfig extends ColumnBuilderExtraConfig = ColumnBuilderExtraConfig> extends ColumnBuilder<T, TRuntimeConfig, TTypeConfig & {
    dialect: 'pg';
}, TExtraConfig> implements PgColumnBuilderBase<T, TTypeConfig> {
    private foreignKeyConfigs;
    static readonly [entityKind]: string;
    array<TSize extends number | undefined = undefined>(size?: TSize): PgArrayBuilder<{
        name: T['name'];
        dataType: 'array';
        columnType: 'PgArray';
        data: T['data'][];
        driverParam: T['driverParam'][] | string;
        enumValues: T['enumValues'];
        size: TSize;
        baseBuilder: T;
    } & (T extends {
        notNull: true;
    } ? {
        notNull: true;
    } : {}) & (T extends {
        hasDefault: true;
    } ? {
        hasDefault: true;
    } : {}), T>;
    references(ref: ReferenceConfig['ref'], actions?: ReferenceConfig['actions']): this;
    unique(name?: string, config?: {
        nulls: 'distinct' | 'not distinct';
    }): this;
    generatedAlwaysAs(as: SQL | T['data'] | (() => SQL)): HasGenerated<this, {
        type: 'always';
    }>;
}
declare abstract class PgColumn<T extends ColumnBaseConfig<ColumnDataType, string> = ColumnBaseConfig<ColumnDataType, string>, TRuntimeConfig extends object = {}, TTypeConfig extends object = {}> extends Column<T, TRuntimeConfig, TTypeConfig & {
    dialect: 'pg';
}> {
    readonly table: PgTable;
    static readonly [entityKind]: string;
    constructor(table: PgTable, config: ColumnBuilderRuntimeConfig<T['data'], TRuntimeConfig>);
}
type IndexedExtraConfigType = {
    order?: 'asc' | 'desc';
    nulls?: 'first' | 'last';
    opClass?: string;
};
declare class ExtraConfigColumn<T extends ColumnBaseConfig<ColumnDataType, string> = ColumnBaseConfig<ColumnDataType, string>> extends PgColumn<T, IndexedExtraConfigType> {
    static readonly [entityKind]: string;
    getSQLType(): string;
    indexConfig: IndexedExtraConfigType;
    defaultConfig: IndexedExtraConfigType;
    asc(): Omit<this, 'asc' | 'desc'>;
    desc(): Omit<this, 'asc' | 'desc'>;
    nullsFirst(): Omit<this, 'nullsFirst' | 'nullsLast'>;
    nullsLast(): Omit<this, 'nullsFirst' | 'nullsLast'>;
    op(opClass: PgIndexOpClass): Omit<this, 'op'>;
}
declare class IndexedColumn {
    static readonly [entityKind]: string;
    constructor(name: string | undefined, keyAsName: boolean, type: string, indexConfig: IndexedExtraConfigType);
    name: string | undefined;
    keyAsName: boolean;
    type: string;
    indexConfig: IndexedExtraConfigType;
}
type AnyPgColumn<TPartial extends Partial<ColumnBaseConfig<ColumnDataType, string>> = {}> = PgColumn<Required<Update<ColumnBaseConfig<ColumnDataType, string>, TPartial>>>;
type PgArrayColumnBuilderBaseConfig = ColumnBuilderBaseConfig<'array', 'PgArray'> & {
    size: number | undefined;
    baseBuilder: ColumnBuilderBaseConfig<ColumnDataType, string>;
};
declare class PgArrayBuilder<T extends PgArrayColumnBuilderBaseConfig, TBase extends ColumnBuilderBaseConfig<ColumnDataType, string> | PgArrayColumnBuilderBaseConfig> extends PgColumnBuilder<T, {
    baseBuilder: TBase extends PgArrayColumnBuilderBaseConfig ? PgArrayBuilder<TBase, TBase extends {
        baseBuilder: infer TBaseBuilder extends ColumnBuilderBaseConfig<any, any>;
    } ? TBaseBuilder : never> : PgColumnBuilder<TBase, {}, Simplify<Omit<TBase, keyof ColumnBuilderBaseConfig<any, any>>>>;
    size: T['size'];
}, {
    baseBuilder: TBase extends PgArrayColumnBuilderBaseConfig ? PgArrayBuilder<TBase, TBase extends {
        baseBuilder: infer TBaseBuilder extends ColumnBuilderBaseConfig<any, any>;
    } ? TBaseBuilder : never> : PgColumnBuilder<TBase, {}, Simplify<Omit<TBase, keyof ColumnBuilderBaseConfig<any, any>>>>;
    size: T['size'];
}> {
    static readonly [entityKind] = "PgArrayBuilder";
    constructor(name: string, baseBuilder: PgArrayBuilder<T, TBase>['config']['baseBuilder'], size: T['size']);
}
type PgBigInt53BuilderInitial<TName extends string> = PgBigInt53Builder<{
    name: TName;
    dataType: 'number';
    columnType: 'PgBigInt53';
    data: number;
    driverParam: number | string;
    enumValues: undefined;
}>;
declare class PgBigInt53Builder<T extends ColumnBuilderBaseConfig<'number', 'PgBigInt53'>> extends PgIntColumnBaseBuilder<T> {
    static readonly [entityKind]: string;
    constructor(name: T['name']);
}
type PgBigInt64BuilderInitial<TName extends string> = PgBigInt64Builder<{
    name: TName;
    dataType: 'bigint';
    columnType: 'PgBigInt64';
    data: bigint;
    driverParam: string;
    enumValues: undefined;
}>;
declare class PgBigInt64Builder<T extends ColumnBuilderBaseConfig<'bigint', 'PgBigInt64'>> extends PgIntColumnBaseBuilder<T> {
    static readonly [entityKind]: string;
    constructor(name: T['name']);
}
interface PgBigIntConfig<T extends 'number' | 'bigint' = 'number' | 'bigint'> {
    mode: T;
}
declare function bigint<TMode extends PgBigIntConfig['mode']>(config: PgBigIntConfig<TMode>): TMode extends 'number' ? PgBigInt53BuilderInitial<''> : PgBigInt64BuilderInitial<''>;
declare function bigint<TName extends string, TMode extends PgBigIntConfig['mode']>(name: TName, config: PgBigIntConfig<TMode>): TMode extends 'number' ? PgBigInt53BuilderInitial<TName> : PgBigInt64BuilderInitial<TName>;
type PgEnumObjectColumnBuilderInitial<TName extends string, TValues extends object> = PgEnumObjectColumnBuilder<{
    name: TName;
    dataType: 'string';
    columnType: 'PgEnumObjectColumn';
    data: TValues[keyof TValues];
    enumValues: string[];
    driverParam: string;
}>;
interface PgEnumObject<TValues extends object> {
    (): PgEnumObjectColumnBuilderInitial<'', TValues>;
    <TName extends string>(name: TName): PgEnumObjectColumnBuilderInitial<TName, TValues>;
    <TName extends string>(name?: TName): PgEnumObjectColumnBuilderInitial<TName, TValues>;
    readonly enumName: string;
    readonly enumValues: string[];
    readonly schema: string | undefined;
}
declare class PgEnumObjectColumnBuilder<T extends ColumnBuilderBaseConfig<'string', 'PgEnumObjectColumn'> & {
    enumValues: string[];
}> extends PgColumnBuilder<T, {
    enum: PgEnumObject<any>;
}> {
    static readonly [entityKind]: string;
    constructor(name: T['name'], enumInstance: PgEnumObject<any>);
}
type PgEnumColumnBuilderInitial<TName extends string, TValues extends [
    string,
    ...string[]
]> = PgEnumColumnBuilder<{
    name: TName;
    dataType: 'string';
    columnType: 'PgEnumColumn';
    data: TValues[number];
    enumValues: TValues;
    driverParam: string;
}>;
interface PgEnum<TValues extends [
    string,
    ...string[]
]> {
    (): PgEnumColumnBuilderInitial<'', TValues>;
    <TName extends string>(name: TName): PgEnumColumnBuilderInitial<TName, TValues>;
    <TName extends string>(name?: TName): PgEnumColumnBuilderInitial<TName, TValues>;
    readonly enumName: string;
    readonly enumValues: TValues;
    readonly schema: string | undefined;
}
declare class PgEnumColumnBuilder<T extends ColumnBuilderBaseConfig<'string', 'PgEnumColumn'> & {
    enumValues: [
        string,
        ...string[]
    ];
}> extends PgColumnBuilder<T, {
    enum: PgEnum<T['enumValues']>;
}> {
    static readonly [entityKind]: string;
    constructor(name: T['name'], enumInstance: PgEnum<T['enumValues']>);
}
declare function pgEnum<U extends string, T extends Readonly<[
    U,
    ...U[]
]>>(enumName: string, values: T | Writable<T>): PgEnum<Writable<T>>;
declare function pgEnum<E extends Record<string, string>>(enumName: string, enumObj: NonArray<E>): PgEnumObject<E>;
declare abstract class PgViewBase<TName extends string = string, TExisting extends boolean = boolean, TSelectedFields extends ColumnsSelection = ColumnsSelection> extends View<TName, TExisting, TSelectedFields> {
    static readonly [entityKind]: string;
    readonly _: View<TName, TExisting, TSelectedFields>['_'] & {
        readonly viewBrand: 'PgViewBase';
    };
}
declare class PgCountBuilder<TSession extends PgSession<any, any, any>> extends SQL<number> implements Promise<number>, SQLWrapper {
    readonly params: {
        source: PgTable | SQL | SQLWrapper;
        filters?: SQL<unknown>;
        session: TSession;
    };
    private sql;
    private token?;
    static readonly [entityKind] = "PgCountBuilder";
    [Symbol.toStringTag]: string;
    private session;
    private static buildEmbeddedCount;
    private static buildCount;
    constructor(params: {
        source: PgTable | SQL | SQLWrapper;
        filters?: SQL<unknown>;
        session: TSession;
    });
    setToken(token?: NeonAuthToken): this;
    then<TResult1 = number, TResult2 = never>(onfulfilled?: ((value: number) => TResult1 | PromiseLike<TResult1>) | null | undefined, onrejected?: ((reason: any) => TResult2 | PromiseLike<TResult2>) | null | undefined): Promise<TResult1 | TResult2>;
    catch(onRejected?: ((reason: any) => any) | null | undefined): Promise<number>;
    finally(onFinally?: (() => void) | null | undefined): Promise<number>;
}
declare class RelationalQueryBuilder<TSchema extends TablesRelationalConfig, TFields extends TableRelationalConfig> {
    private fullSchema;
    private schema;
    private tableNamesMap;
    private table;
    private tableConfig;
    private dialect;
    private session;
    static readonly [entityKind]: string;
    constructor(fullSchema: Record<string, unknown>, schema: TSchema, tableNamesMap: Record<string, string>, table: PgTable, tableConfig: TableRelationalConfig, dialect: PgDialect, session: PgSession);
    findMany<TConfig extends DBQueryConfig<'many', true, TSchema, TFields>>(config?: KnownKeysOnly<TConfig, DBQueryConfig<'many', true, TSchema, TFields>>): PgRelationalQuery<BuildQueryResult<TSchema, TFields, TConfig>[]>;
    findFirst<TSelection extends Omit<DBQueryConfig<'many', true, TSchema, TFields>, 'limit'>>(config?: KnownKeysOnly<TSelection, Omit<DBQueryConfig<'many', true, TSchema, TFields>, 'limit'>>): PgRelationalQuery<BuildQueryResult<TSchema, TFields, TSelection> | undefined>;
}
declare class PgRelationalQuery<TResult> extends QueryPromise<TResult> implements RunnableQuery<TResult, 'pg'>, SQLWrapper {
    private fullSchema;
    private schema;
    private tableNamesMap;
    private table;
    private tableConfig;
    private dialect;
    private session;
    private config;
    private mode;
    static readonly [entityKind]: string;
    readonly _: {
        readonly dialect: 'pg';
        readonly result: TResult;
    };
    constructor(fullSchema: Record<string, unknown>, schema: TablesRelationalConfig, tableNamesMap: Record<string, string>, table: PgTable, tableConfig: TableRelationalConfig, dialect: PgDialect, session: PgSession, config: DBQueryConfig<'many', true> | true, mode: 'many' | 'first');
    prepare(name: string): PgPreparedQuery<PreparedQueryConfig & {
        execute: TResult;
    }>;
    private _getQuery;
    private _toSQL;
    toSQL(): Query;
    private authToken?;
    execute(): Promise<TResult>;
}
interface PgRaw<TResult> extends QueryPromise<TResult>, RunnableQuery<TResult, 'pg'>, SQLWrapper {
}
declare class PgRaw<TResult> extends QueryPromise<TResult> implements RunnableQuery<TResult, 'pg'>, SQLWrapper, PreparedQuery {
    execute: () => Promise<TResult>;
    private sql;
    private query;
    private mapBatchResult;
    static readonly [entityKind]: string;
    readonly _: {
        readonly dialect: 'pg';
        readonly result: TResult;
    };
    constructor(execute: () => Promise<TResult>, sql: SQL, query: Query, mapBatchResult: (result: unknown) => unknown);
    getQuery(): Query;
    mapResult(result: unknown, isFromBatch?: boolean): unknown;
    _prepare(): PreparedQuery;
}
interface PgRefreshMaterializedView<TQueryResult extends PgQueryResultHKT> extends QueryPromise<PgQueryResultKind<TQueryResult, never>>, RunnableQuery<PgQueryResultKind<TQueryResult, never>, 'pg'>, SQLWrapper {
    readonly _: {
        readonly dialect: 'pg';
        readonly result: PgQueryResultKind<TQueryResult, never>;
    };
}
declare class PgRefreshMaterializedView<TQueryResult extends PgQueryResultHKT> extends QueryPromise<PgQueryResultKind<TQueryResult, never>> implements RunnableQuery<PgQueryResultKind<TQueryResult, never>, 'pg'>, SQLWrapper {
    private session;
    private dialect;
    static readonly [entityKind]: string;
    private config;
    constructor(view: PgMaterializedView, session: PgSession, dialect: PgDialect);
    concurrently(): this;
    withNoData(): this;
    toSQL(): Query;
    prepare(name: string): PgPreparedQuery<PreparedQueryConfig & {
        execute: PgQueryResultKind<TQueryResult, never>;
    }>;
    private authToken?;
    execute: ReturnType<this['prepare']>['execute'];
}
type SubqueryWithSelection<TSelection extends ColumnsSelection, TAlias extends string> = Subquery<TAlias, AddAliasToSelection<TSelection, TAlias, 'pg'>> & AddAliasToSelection<TSelection, TAlias, 'pg'>;
type WithSubqueryWithSelection<TSelection extends ColumnsSelection, TAlias extends string> = WithSubquery<TAlias, AddAliasToSelection<TSelection, TAlias, 'pg'>> & AddAliasToSelection<TSelection, TAlias, 'pg'>;
interface WithBuilder {
    <TAlias extends string>(alias: TAlias): {
        as: {
            <TSelection extends ColumnsSelection>(qb: TypedQueryBuilder<TSelection> | ((qb: QueryBuilder) => TypedQueryBuilder<TSelection>)): WithSubqueryWithSelection<TSelection, TAlias>;
            (qb: TypedQueryBuilder<undefined> | ((qb: QueryBuilder) => TypedQueryBuilder<undefined>)): WithSubqueryWithoutSelection<TAlias>;
        };
    };
    <TAlias extends string, TSelection extends ColumnsSelection>(alias: TAlias, selection: TSelection): {
        as: (qb: SQL | ((qb: QueryBuilder) => SQL)) => WithSubqueryWithSelection<TSelection, TAlias>;
    };
}
declare class PgDatabase<TQueryResult extends PgQueryResultHKT, TFullSchema extends Record<string, unknown> = Record<string, never>, TSchema extends TablesRelationalConfig = ExtractTablesWithRelations<TFullSchema>> {
    static readonly [entityKind]: string;
    readonly _: {
        readonly schema: TSchema | undefined;
        readonly fullSchema: TFullSchema;
        readonly tableNamesMap: Record<string, string>;
        readonly session: PgSession<TQueryResult, TFullSchema, TSchema>;
    };
    query: TFullSchema extends Record<string, never> ? DrizzleTypeError<'Seems like the schema generic is missing - did you forget to add it to your DB type?'> : {
        [K in keyof TSchema]: RelationalQueryBuilder<TSchema, TSchema[K]>;
    };
    constructor(dialect: PgDialect, session: PgSession<any, any, any>, schema: RelationalSchemaConfig<TSchema> | undefined);
    $with: WithBuilder;
    $count(source: PgTable | PgViewBase | SQL | SQLWrapper, filters?: SQL<unknown>): PgCountBuilder<PgSession<any, any, any>>;
    $cache: {
        invalidate: Cache['onMutate'];
    };
    with(...queries: WithSubquery[]): {
        select: {
            (): PgSelectBuilder<undefined>;
            <TSelection extends SelectedFields>(fields: TSelection): PgSelectBuilder<TSelection>;
        };
        selectDistinct: {
            (): PgSelectBuilder<undefined>;
            <TSelection extends SelectedFields>(fields: TSelection): PgSelectBuilder<TSelection>;
        };
        selectDistinctOn: {
            (on: (PgColumn | SQLWrapper)[]): PgSelectBuilder<undefined>;
            <TSelection extends SelectedFields>(on: (PgColumn | SQLWrapper)[], fields: TSelection): PgSelectBuilder<TSelection>;
        };
        update: <TTable extends PgTable>(table: TTable) => PgUpdateBuilder<TTable, TQueryResult>;
        insert: <TTable extends PgTable>(table: TTable) => PgInsertBuilder<TTable, TQueryResult>;
        delete: <TTable extends PgTable>(table: TTable) => PgDeleteBase<TTable, TQueryResult>;
    };
    select(): PgSelectBuilder<undefined>;
    select<TSelection extends SelectedFields>(fields: TSelection): PgSelectBuilder<TSelection>;
    selectDistinct(): PgSelectBuilder<undefined>;
    selectDistinct<TSelection extends SelectedFields>(fields: TSelection): PgSelectBuilder<TSelection>;
    selectDistinctOn(on: (PgColumn | SQLWrapper)[]): PgSelectBuilder<undefined>;
    selectDistinctOn<TSelection extends SelectedFields>(on: (PgColumn | SQLWrapper)[], fields: TSelection): PgSelectBuilder<TSelection>;
    update<TTable extends PgTable>(table: TTable): PgUpdateBuilder<TTable, TQueryResult>;
    insert<TTable extends PgTable>(table: TTable): PgInsertBuilder<TTable, TQueryResult>;
    delete<TTable extends PgTable>(table: TTable): PgDeleteBase<TTable, TQueryResult>;
    refreshMaterializedView<TView extends PgMaterializedView>(view: TView): PgRefreshMaterializedView<TQueryResult>;
    protected authToken?: NeonAuthToken;
    execute<TRow extends Record<string, unknown> = Record<string, unknown>>(query: SQLWrapper | string): PgRaw<PgQueryResultKind<TQueryResult, TRow>>;
    transaction<T>(transaction: (tx: PgTransaction<TQueryResult, TFullSchema, TSchema>) => Promise<T>, config?: PgTransactionConfig): Promise<T>;
}
interface PreparedQueryConfig {
    execute: unknown;
    all: unknown;
    values: unknown;
}
declare abstract class PgPreparedQuery<T extends PreparedQueryConfig> implements PreparedQuery {
    protected query: Query;
    private cache;
    private queryMetadata;
    private cacheConfig?;
    constructor(query: Query, cache: Cache | undefined, queryMetadata: {
        type: 'select' | 'update' | 'delete' | 'insert';
        tables: string[];
    } | undefined, cacheConfig?: WithCacheConfig | undefined);
    protected authToken?: NeonAuthToken;
    getQuery(): Query;
    mapResult(response: unknown, _isFromBatch?: boolean): unknown;
    static readonly [entityKind]: string;
    abstract execute(placeholderValues?: Record<string, unknown>): Promise<T['execute']>;
}
interface PgTransactionConfig {
    isolationLevel?: 'read uncommitted' | 'read committed' | 'repeatable read' | 'serializable';
    accessMode?: 'read only' | 'read write';
    deferrable?: boolean;
}
declare abstract class PgSession<TQueryResult extends PgQueryResultHKT = PgQueryResultHKT, TFullSchema extends Record<string, unknown> = Record<string, never>, TSchema extends TablesRelationalConfig = Record<string, never>> {
    protected dialect: PgDialect;
    static readonly [entityKind]: string;
    constructor(dialect: PgDialect);
    abstract prepareQuery<T extends PreparedQueryConfig = PreparedQueryConfig>(query: Query, fields: SelectedFieldsOrdered | undefined, name: string | undefined, isResponseInArrayMode: boolean, customResultMapper?: (rows: unknown[][], mapColumnValue?: (value: unknown) => unknown) => T['execute'], queryMetadata?: {
        type: 'select' | 'update' | 'delete' | 'insert';
        tables: string[];
    }, cacheConfig?: WithCacheConfig): PgPreparedQuery<T>;
    execute<T>(query: SQL): Promise<T>;
    all<T = unknown>(query: SQL): Promise<T[]>;
    count(sql: SQL): Promise<number>;
    abstract transaction<T>(transaction: (tx: PgTransaction<TQueryResult, TFullSchema, TSchema>) => Promise<T>, config?: PgTransactionConfig): Promise<T>;
}
declare abstract class PgTransaction<TQueryResult extends PgQueryResultHKT, TFullSchema extends Record<string, unknown> = Record<string, never>, TSchema extends TablesRelationalConfig = Record<string, never>> extends PgDatabase<TQueryResult, TFullSchema, TSchema> {
    protected schema: {
        fullSchema: Record<string, unknown>;
        schema: TSchema;
        tableNamesMap: Record<string, string>;
    } | undefined;
    protected readonly nestedIndex: number;
    static readonly [entityKind]: string;
    constructor(dialect: PgDialect, session: PgSession<any, any, any>, schema: {
        fullSchema: Record<string, unknown>;
        schema: TSchema;
        tableNamesMap: Record<string, string>;
    } | undefined, nestedIndex?: number);
    rollback(): never;
    setTransaction(config: PgTransactionConfig): Promise<void>;
    abstract transaction<T>(transaction: (tx: PgTransaction<TQueryResult, TFullSchema, TSchema>) => Promise<T>): Promise<T>;
}
interface PgQueryResultHKT {
    readonly $brand: 'PgQueryResultHKT';
    readonly row: unknown;
    readonly type: unknown;
}
type PgQueryResultKind<TKind extends PgQueryResultHKT, TRow> = (TKind & {
    readonly row: TRow;
})['type'];
type PgDeleteWithout<T extends AnyPgDeleteBase, TDynamic extends boolean, K extends keyof T & string> = TDynamic extends true ? T : Omit<PgDeleteBase<T['_']['table'], T['_']['queryResult'], T['_']['selectedFields'], T['_']['returning'], TDynamic, T['_']['excludedMethods'] | K>, T['_']['excludedMethods'] | K>;
type PgDelete<TTable extends PgTable = PgTable, TQueryResult extends PgQueryResultHKT = PgQueryResultHKT, TSelectedFields extends ColumnsSelection | undefined = undefined, TReturning extends Record<string, unknown> | undefined = Record<string, unknown> | undefined> = PgDeleteBase<TTable, TQueryResult, TSelectedFields, TReturning, true, never>;
interface PgDeleteConfig {
    where?: SQL | undefined;
    table: PgTable;
    returningFields?: SelectedFieldsFlat;
    returning?: SelectedFieldsOrdered;
    withList?: Subquery[];
}
type PgDeleteReturningAll<T extends AnyPgDeleteBase, TDynamic extends boolean> = PgDeleteWithout<PgDeleteBase<T['_']['table'], T['_']['queryResult'], T['_']['table']['_']['columns'], T['_']['table']['$inferSelect'], TDynamic, T['_']['excludedMethods']>, TDynamic, 'returning'>;
type PgDeleteReturning<T extends AnyPgDeleteBase, TDynamic extends boolean, TSelectedFields extends SelectedFieldsFlat> = PgDeleteWithout<PgDeleteBase<T['_']['table'], T['_']['queryResult'], TSelectedFields, SelectResultFields<TSelectedFields>, TDynamic, T['_']['excludedMethods']>, TDynamic, 'returning'>;
type PgDeletePrepare<T extends AnyPgDeleteBase> = PgPreparedQuery<PreparedQueryConfig & {
    execute: T['_']['returning'] extends undefined ? PgQueryResultKind<T['_']['queryResult'], never> : T['_']['returning'][];
}>;
type PgDeleteDynamic<T extends AnyPgDeleteBase> = PgDelete<T['_']['table'], T['_']['queryResult'], T['_']['selectedFields'], T['_']['returning']>;
type AnyPgDeleteBase = PgDeleteBase<any, any, any, any, any, any>;
interface PgDeleteBase<TTable extends PgTable, TQueryResult extends PgQueryResultHKT, TSelectedFields extends ColumnsSelection | undefined = undefined, TReturning extends Record<string, unknown> | undefined = undefined, TDynamic extends boolean = false, TExcludedMethods extends string = never> extends TypedQueryBuilder<TSelectedFields, TReturning extends undefined ? PgQueryResultKind<TQueryResult, never> : TReturning[]>, QueryPromise<TReturning extends undefined ? PgQueryResultKind<TQueryResult, never> : TReturning[]>, RunnableQuery<TReturning extends undefined ? PgQueryResultKind<TQueryResult, never> : TReturning[], 'pg'>, SQLWrapper {
    readonly _: {
        readonly dialect: 'pg';
        readonly table: TTable;
        readonly queryResult: TQueryResult;
        readonly selectedFields: TSelectedFields;
        readonly returning: TReturning;
        readonly dynamic: TDynamic;
        readonly excludedMethods: TExcludedMethods;
        readonly result: TReturning extends undefined ? PgQueryResultKind<TQueryResult, never> : TReturning[];
    };
}
declare class PgDeleteBase<TTable extends PgTable, TQueryResult extends PgQueryResultHKT, TSelectedFields extends ColumnsSelection | undefined = undefined, TReturning extends Record<string, unknown> | undefined = undefined, TDynamic extends boolean = false, TExcludedMethods extends string = never> extends QueryPromise<TReturning extends undefined ? PgQueryResultKind<TQueryResult, never> : TReturning[]> implements TypedQueryBuilder<TSelectedFields, TReturning extends undefined ? PgQueryResultKind<TQueryResult, never> : TReturning[]>, RunnableQuery<TReturning extends undefined ? PgQueryResultKind<TQueryResult, never> : TReturning[], 'pg'>, SQLWrapper {
    private session;
    private dialect;
    static readonly [entityKind]: string;
    private config;
    protected cacheConfig?: WithCacheConfig;
    constructor(table: TTable, session: PgSession, dialect: PgDialect, withList?: Subquery[]);
    where(where: SQL | undefined): PgDeleteWithout<this, TDynamic, 'where'>;
    returning(): PgDeleteReturningAll<this, TDynamic>;
    returning<TSelectedFields extends SelectedFieldsFlat>(fields: TSelectedFields): PgDeleteReturning<this, TDynamic, TSelectedFields>;
    toSQL(): Query;
    prepare(name: string): PgDeletePrepare<this>;
    private authToken?;
    execute: ReturnType<this['prepare']>['execute'];
    $dynamic(): PgDeleteDynamic<this>;
}
interface PgUpdateConfig {
    where?: SQL | undefined;
    set: UpdateSet;
    table: PgTable;
    from?: PgTable | Subquery | PgViewBase | SQL;
    joins: PgSelectJoinConfig[];
    returningFields?: SelectedFields;
    returning?: SelectedFieldsOrdered;
    withList?: Subquery[];
}
type PgUpdateSetSource<TTable extends PgTable> = {
    [Key in keyof TTable['$inferInsert']]?: GetColumnData<TTable['_']['columns'][Key]> | SQL | PgColumn | undefined;
} & {};
declare class PgUpdateBuilder<TTable extends PgTable, TQueryResult extends PgQueryResultHKT> {
    private table;
    private session;
    private dialect;
    private withList?;
    static readonly [entityKind]: string;
    readonly _: {
        readonly table: TTable;
    };
    constructor(table: TTable, session: PgSession, dialect: PgDialect, withList?: Subquery[] | undefined);
    private authToken?;
    setToken(token: NeonAuthToken): this;
    set(values: PgUpdateSetSource<TTable>): PgUpdateWithout<PgUpdateBase<TTable, TQueryResult>, false, 'leftJoin' | 'rightJoin' | 'innerJoin' | 'fullJoin'>;
}
type PgUpdateWithout<T extends AnyPgUpdate, TDynamic extends boolean, K extends keyof T & string> = TDynamic extends true ? T : Omit<PgUpdateBase<T['_']['table'], T['_']['queryResult'], T['_']['from'], T['_']['selectedFields'], T['_']['returning'], T['_']['nullabilityMap'], T['_']['joins'], TDynamic, T['_']['excludedMethods'] | K>, T['_']['excludedMethods'] | K>;
type PgUpdateWithJoins<T extends AnyPgUpdate, TDynamic extends boolean, TFrom extends PgTable | Subquery | PgViewBase | SQL> = TDynamic extends true ? T : Omit<PgUpdateBase<T['_']['table'], T['_']['queryResult'], TFrom, T['_']['selectedFields'], T['_']['returning'], AppendToNullabilityMap<T['_']['nullabilityMap'], GetSelectTableName<TFrom>, 'inner'>, [
    ...T['_']['joins'],
    {
        name: GetSelectTableName<TFrom>;
        joinType: 'inner';
        table: TFrom;
    }
], TDynamic, Exclude<T['_']['excludedMethods'] | 'from', 'leftJoin' | 'rightJoin' | 'innerJoin' | 'fullJoin'>>, Exclude<T['_']['excludedMethods'] | 'from', 'leftJoin' | 'rightJoin' | 'innerJoin' | 'fullJoin'>>;
type PgUpdateJoinFn<T extends AnyPgUpdate, TDynamic extends boolean, TJoinType extends JoinType> = <TJoinedTable extends PgTable | Subquery | PgViewBase | SQL>(table: TableLikeHasEmptySelection<TJoinedTable> extends true ? DrizzleTypeError<"Cannot reference a data-modifying statement subquery if it doesn't contain a `returning` clause"> : TJoinedTable, on: ((updateTable: T['_']['table']['_']['columns'], from: T['_']['from'] extends PgTable ? T['_']['from']['_']['columns'] : T['_']['from'] extends Subquery | PgViewBase ? T['_']['from']['_']['selectedFields'] : never) => SQL | undefined) | SQL | undefined) => PgUpdateJoin<T, TDynamic, TJoinType, TJoinedTable>;
type PgUpdateJoin<T extends AnyPgUpdate, TDynamic extends boolean, TJoinType extends JoinType, TJoinedTable extends PgTable | Subquery | PgViewBase | SQL> = TDynamic extends true ? T : PgUpdateBase<T['_']['table'], T['_']['queryResult'], T['_']['from'], T['_']['selectedFields'], T['_']['returning'], AppendToNullabilityMap<T['_']['nullabilityMap'], GetSelectTableName<TJoinedTable>, TJoinType>, [
    ...T['_']['joins'],
    {
        name: GetSelectTableName<TJoinedTable>;
        joinType: TJoinType;
        table: TJoinedTable;
    }
], TDynamic, T['_']['excludedMethods']>;
type Join = {
    name: string | undefined;
    joinType: JoinType;
    table: PgTable | Subquery | PgViewBase | SQL;
};
type AccumulateToResult<T extends AnyPgUpdate, TSelectMode extends SelectMode, TJoins extends Join[], TSelectedFields extends ColumnsSelection> = TJoins extends [
    infer TJoin extends Join,
    ...infer TRest extends Join[]
] ? AccumulateToResult<T, TSelectMode extends 'partial' ? TSelectMode : 'multiple', TRest, AppendToResult<T['_']['table']['_']['name'], TSelectedFields, TJoin['name'], TJoin['table'] extends Table ? TJoin['table']['_']['columns'] : TJoin['table'] extends Subquery ? Assume<TJoin['table']['_']['selectedFields'], SelectedFields> : never, TSelectMode extends 'partial' ? TSelectMode : 'multiple'>> : TSelectedFields;
type PgUpdateReturningAll<T extends AnyPgUpdate, TDynamic extends boolean> = PgUpdateWithout<PgUpdateBase<T['_']['table'], T['_']['queryResult'], T['_']['from'], Equal<T['_']['joins'], [
]> extends true ? T['_']['table']['_']['columns'] : Simplify<Record<T['_']['table']['_']['name'], T['_']['table']['_']['columns']> & {
    [K in keyof T['_']['joins'] as T['_']['joins'][K]['table']['_']['name']]: T['_']['joins'][K]['table']['_']['columns'];
}>, SelectResult<AccumulateToResult<T, 'single', T['_']['joins'], GetSelectTableSelection<T['_']['table']>>, 'partial', T['_']['nullabilityMap']>, T['_']['nullabilityMap'], T['_']['joins'], TDynamic, T['_']['excludedMethods']>, TDynamic, 'returning'>;
type PgUpdateReturning<T extends AnyPgUpdate, TDynamic extends boolean, TSelectedFields extends SelectedFields> = PgUpdateWithout<PgUpdateBase<T['_']['table'], T['_']['queryResult'], T['_']['from'], TSelectedFields, SelectResult<AccumulateToResult<T, 'partial', T['_']['joins'], TSelectedFields>, 'partial', T['_']['nullabilityMap']>, T['_']['nullabilityMap'], T['_']['joins'], TDynamic, T['_']['excludedMethods']>, TDynamic, 'returning'>;
type PgUpdatePrepare<T extends AnyPgUpdate> = PgPreparedQuery<PreparedQueryConfig & {
    execute: T['_']['returning'] extends undefined ? PgQueryResultKind<T['_']['queryResult'], never> : T['_']['returning'][];
}>;
type PgUpdateDynamic<T extends AnyPgUpdate> = PgUpdate<T['_']['table'], T['_']['queryResult'], T['_']['from'], T['_']['returning'], T['_']['nullabilityMap']>;
type PgUpdate<TTable extends PgTable = PgTable, TQueryResult extends PgQueryResultHKT = PgQueryResultHKT, TFrom extends PgTable | Subquery | PgViewBase | SQL | undefined = undefined, TSelectedFields extends ColumnsSelection | undefined = undefined, TReturning extends Record<string, unknown> | undefined = Record<string, unknown> | undefined, TNullabilityMap extends Record<string, JoinNullability> = Record<TTable['_']['name'], 'not-null'>, TJoins extends Join[] = [
]> = PgUpdateBase<TTable, TQueryResult, TFrom, TSelectedFields, TReturning, TNullabilityMap, TJoins, true, never>;
type AnyPgUpdate = PgUpdateBase<any, any, any, any, any, any, any, any, any>;
interface PgUpdateBase<TTable extends PgTable, TQueryResult extends PgQueryResultHKT, TFrom extends PgTable | Subquery | PgViewBase | SQL | undefined = undefined, TSelectedFields extends ColumnsSelection | undefined = undefined, TReturning extends Record<string, unknown> | undefined = undefined, TNullabilityMap extends Record<string, JoinNullability> = Record<TTable['_']['name'], 'not-null'>, TJoins extends Join[] = [
], TDynamic extends boolean = false, TExcludedMethods extends string = never> extends TypedQueryBuilder<TSelectedFields, TReturning extends undefined ? PgQueryResultKind<TQueryResult, never> : TReturning[]>, QueryPromise<TReturning extends undefined ? PgQueryResultKind<TQueryResult, never> : TReturning[]>, RunnableQuery<TReturning extends undefined ? PgQueryResultKind<TQueryResult, never> : TReturning[], 'pg'>, SQLWrapper {
    readonly _: {
        readonly dialect: 'pg';
        readonly table: TTable;
        readonly joins: TJoins;
        readonly nullabilityMap: TNullabilityMap;
        readonly queryResult: TQueryResult;
        readonly from: TFrom;
        readonly selectedFields: TSelectedFields;
        readonly returning: TReturning;
        readonly dynamic: TDynamic;
        readonly excludedMethods: TExcludedMethods;
        readonly result: TReturning extends undefined ? PgQueryResultKind<TQueryResult, never> : TReturning[];
    };
}
declare class PgUpdateBase<TTable extends PgTable, TQueryResult extends PgQueryResultHKT, TFrom extends PgTable | Subquery | PgViewBase | SQL | undefined = undefined, TSelectedFields extends ColumnsSelection | undefined = undefined, TReturning extends Record<string, unknown> | undefined = undefined, TNullabilityMap extends Record<string, JoinNullability> = Record<TTable['_']['name'], 'not-null'>, TJoins extends Join[] = [
], TDynamic extends boolean = false, TExcludedMethods extends string = never> extends QueryPromise<TReturning extends undefined ? PgQueryResultKind<TQueryResult, never> : TReturning[]> implements RunnableQuery<TReturning extends undefined ? PgQueryResultKind<TQueryResult, never> : TReturning[], 'pg'>, SQLWrapper {
    private session;
    private dialect;
    static readonly [entityKind]: string;
    private config;
    private tableName;
    private joinsNotNullableMap;
    protected cacheConfig?: WithCacheConfig;
    constructor(table: TTable, set: UpdateSet, session: PgSession, dialect: PgDialect, withList?: Subquery[]);
    from<TFrom extends PgTable | Subquery | PgViewBase | SQL>(source: TableLikeHasEmptySelection<TFrom> extends true ? DrizzleTypeError<"Cannot reference a data-modifying statement subquery if it doesn't contain a `returning` clause"> : TFrom): PgUpdateWithJoins<this, TDynamic, TFrom>;
    private getTableLikeFields;
    private createJoin;
    leftJoin: PgUpdateJoinFn<this, TDynamic, "left">;
    rightJoin: PgUpdateJoinFn<this, TDynamic, "right">;
    innerJoin: PgUpdateJoinFn<this, TDynamic, "inner">;
    fullJoin: PgUpdateJoinFn<this, TDynamic, "full">;
    where(where: SQL | undefined): PgUpdateWithout<this, TDynamic, 'where'>;
    returning(): PgUpdateReturningAll<this, TDynamic>;
    returning<TSelectedFields extends SelectedFields>(fields: TSelectedFields): PgUpdateReturning<this, TDynamic, TSelectedFields>;
    toSQL(): Query;
    prepare(name: string): PgUpdatePrepare<this>;
    private authToken?;
    execute: ReturnType<this['prepare']>['execute'];
    $dynamic(): PgUpdateDynamic<this>;
}
interface PgInsertConfig<TTable extends PgTable = PgTable> {
    table: TTable;
    values: Record<string, Param | SQL>[] | PgInsertSelectQueryBuilder<TTable> | SQL;
    withList?: Subquery[];
    onConflict?: SQL;
    returningFields?: SelectedFieldsFlat;
    returning?: SelectedFieldsOrdered;
    select?: boolean;
    overridingSystemValue_?: boolean;
}
type PgInsertValue<TTable extends PgTable<TableConfig$2>, OverrideT extends boolean = false> = {
    [Key in keyof InferInsertModel<TTable, {
        dbColumnNames: false;
        override: OverrideT;
    }>]: InferInsertModel<TTable, {
        dbColumnNames: false;
        override: OverrideT;
    }>[Key] | SQL | Placeholder;
} & {};
type PgInsertSelectQueryBuilder<TTable extends PgTable> = TypedQueryBuilder<{
    [K in keyof TTable['$inferInsert']]: AnyPgColumn | SQL | SQL.Aliased | TTable['$inferInsert'][K];
}>;
declare class PgInsertBuilder<TTable extends PgTable, TQueryResult extends PgQueryResultHKT, OverrideT extends boolean = false> {
    private table;
    private session;
    private dialect;
    private withList?;
    private overridingSystemValue_?;
    static readonly [entityKind]: string;
    constructor(table: TTable, session: PgSession, dialect: PgDialect, withList?: Subquery[] | undefined, overridingSystemValue_?: boolean | undefined);
    private authToken?;
    overridingSystemValue(): Omit<PgInsertBuilder<TTable, TQueryResult, true>, 'overridingSystemValue'>;
    values(value: PgInsertValue<TTable, OverrideT>): PgInsertBase<TTable, TQueryResult>;
    values(values: PgInsertValue<TTable, OverrideT>[]): PgInsertBase<TTable, TQueryResult>;
    select(selectQuery: (qb: QueryBuilder) => PgInsertSelectQueryBuilder<TTable>): PgInsertBase<TTable, TQueryResult>;
    select(selectQuery: (qb: QueryBuilder) => SQL): PgInsertBase<TTable, TQueryResult>;
    select(selectQuery: SQL): PgInsertBase<TTable, TQueryResult>;
    select(selectQuery: PgInsertSelectQueryBuilder<TTable>): PgInsertBase<TTable, TQueryResult>;
}
type PgInsertWithout<T extends AnyPgInsert, TDynamic extends boolean, K extends keyof T & string> = TDynamic extends true ? T : Omit<PgInsertBase<T['_']['table'], T['_']['queryResult'], T['_']['selectedFields'], T['_']['returning'], TDynamic, T['_']['excludedMethods'] | K>, T['_']['excludedMethods'] | K>;
type PgInsertReturning<T extends AnyPgInsert, TDynamic extends boolean, TSelectedFields extends SelectedFieldsFlat> = PgInsertBase<T['_']['table'], T['_']['queryResult'], TSelectedFields, SelectResultFields<TSelectedFields>, TDynamic, T['_']['excludedMethods']>;
type PgInsertReturningAll<T extends AnyPgInsert, TDynamic extends boolean> = PgInsertBase<T['_']['table'], T['_']['queryResult'], T['_']['table']['_']['columns'], T['_']['table']['$inferSelect'], TDynamic, T['_']['excludedMethods']>;
interface PgInsertOnConflictDoUpdateConfig<T extends AnyPgInsert> {
    target: IndexColumn | IndexColumn[];
    where?: SQL;
    targetWhere?: SQL;
    setWhere?: SQL;
    set: PgUpdateSetSource<T['_']['table']>;
}
type PgInsertPrepare<T extends AnyPgInsert> = PgPreparedQuery<PreparedQueryConfig & {
    execute: T['_']['returning'] extends undefined ? PgQueryResultKind<T['_']['queryResult'], never> : T['_']['returning'][];
}>;
type PgInsertDynamic<T extends AnyPgInsert> = PgInsert<T['_']['table'], T['_']['queryResult'], T['_']['returning']>;
type AnyPgInsert = PgInsertBase<any, any, any, any, any, any>;
type PgInsert<TTable extends PgTable = PgTable, TQueryResult extends PgQueryResultHKT = PgQueryResultHKT, TSelectedFields extends ColumnsSelection | undefined = ColumnsSelection | undefined, TReturning extends Record<string, unknown> | undefined = Record<string, unknown> | undefined> = PgInsertBase<TTable, TQueryResult, TSelectedFields, TReturning, true, never>;
interface PgInsertBase<TTable extends PgTable, TQueryResult extends PgQueryResultHKT, TSelectedFields extends ColumnsSelection | undefined = undefined, TReturning extends Record<string, unknown> | undefined = undefined, TDynamic extends boolean = false, TExcludedMethods extends string = never> extends TypedQueryBuilder<TSelectedFields, TReturning extends undefined ? PgQueryResultKind<TQueryResult, never> : TReturning[]>, QueryPromise<TReturning extends undefined ? PgQueryResultKind<TQueryResult, never> : TReturning[]>, RunnableQuery<TReturning extends undefined ? PgQueryResultKind<TQueryResult, never> : TReturning[], 'pg'>, SQLWrapper {
    readonly _: {
        readonly dialect: 'pg';
        readonly table: TTable;
        readonly queryResult: TQueryResult;
        readonly selectedFields: TSelectedFields;
        readonly returning: TReturning;
        readonly dynamic: TDynamic;
        readonly excludedMethods: TExcludedMethods;
        readonly result: TReturning extends undefined ? PgQueryResultKind<TQueryResult, never> : TReturning[];
    };
}
declare class PgInsertBase<TTable extends PgTable, TQueryResult extends PgQueryResultHKT, TSelectedFields extends ColumnsSelection | undefined = undefined, TReturning extends Record<string, unknown> | undefined = undefined, TDynamic extends boolean = false, TExcludedMethods extends string = never> extends QueryPromise<TReturning extends undefined ? PgQueryResultKind<TQueryResult, never> : TReturning[]> implements TypedQueryBuilder<TSelectedFields, TReturning extends undefined ? PgQueryResultKind<TQueryResult, never> : TReturning[]>, RunnableQuery<TReturning extends undefined ? PgQueryResultKind<TQueryResult, never> : TReturning[], 'pg'>, SQLWrapper {
    private session;
    private dialect;
    static readonly [entityKind]: string;
    private config;
    protected cacheConfig?: WithCacheConfig;
    constructor(table: TTable, values: PgInsertConfig['values'], session: PgSession, dialect: PgDialect, withList?: Subquery[], select?: boolean, overridingSystemValue_?: boolean);
    returning(): PgInsertWithout<PgInsertReturningAll<this, TDynamic>, TDynamic, 'returning'>;
    returning<TSelectedFields extends SelectedFieldsFlat>(fields: TSelectedFields): PgInsertWithout<PgInsertReturning<this, TDynamic, TSelectedFields>, TDynamic, 'returning'>;
    onConflictDoNothing(config?: {
        target?: IndexColumn | IndexColumn[];
        where?: SQL;
    }): PgInsertWithout<this, TDynamic, 'onConflictDoNothing' | 'onConflictDoUpdate'>;
    onConflictDoUpdate(config: PgInsertOnConflictDoUpdateConfig<this>): PgInsertWithout<this, TDynamic, 'onConflictDoNothing' | 'onConflictDoUpdate'>;
    toSQL(): Query;
    prepare(name: string): PgInsertPrepare<this>;
    private authToken?;
    execute: ReturnType<this['prepare']>['execute'];
    $dynamic(): PgInsertDynamic<this>;
}
declare class PgSelectBuilder<TSelection extends SelectedFields | undefined, TBuilderMode extends 'db' | 'qb' = 'db'> {
    static readonly [entityKind]: string;
    private fields;
    private session;
    private dialect;
    private withList;
    private distinct;
    constructor(config: {
        fields: TSelection;
        session: PgSession | undefined;
        dialect: PgDialect;
        withList?: Subquery[];
        distinct?: boolean | {
            on: (PgColumn | SQLWrapper)[];
        };
    });
    private authToken?;
    from<TFrom extends PgTable | Subquery | PgViewBase | SQL>(source: TableLikeHasEmptySelection<TFrom> extends true ? DrizzleTypeError<"Cannot reference a data-modifying statement subquery if it doesn't contain a `returning` clause"> : TFrom): CreatePgSelectFromBuilderMode<TBuilderMode, GetSelectTableName<TFrom>, TSelection extends undefined ? GetSelectTableSelection<TFrom> : TSelection, TSelection extends undefined ? 'single' : 'partial'>;
}
declare abstract class PgSelectQueryBuilderBase<THKT extends PgSelectHKTBase, TTableName extends string | undefined, TSelection extends ColumnsSelection, TSelectMode extends SelectMode, TNullabilityMap extends Record<string, JoinNullability> = TTableName extends string ? Record<TTableName, 'not-null'> : {}, TDynamic extends boolean = false, TExcludedMethods extends string = never, TResult extends any[] = SelectResult<TSelection, TSelectMode, TNullabilityMap>[], TSelectedFields extends ColumnsSelection = BuildSubquerySelection<TSelection, TNullabilityMap>> extends TypedQueryBuilder<TSelectedFields, TResult> {
    static readonly [entityKind]: string;
    readonly _: {
        readonly dialect: 'pg';
        readonly hkt: THKT;
        readonly tableName: TTableName;
        readonly selection: TSelection;
        readonly selectMode: TSelectMode;
        readonly nullabilityMap: TNullabilityMap;
        readonly dynamic: TDynamic;
        readonly excludedMethods: TExcludedMethods;
        readonly result: TResult;
        readonly selectedFields: TSelectedFields;
        readonly config: PgSelectConfig;
    };
    protected config: PgSelectConfig;
    protected joinsNotNullableMap: Record<string, boolean>;
    protected tableName: string | undefined;
    private isPartialSelect;
    protected session: PgSession | undefined;
    protected dialect: PgDialect;
    protected cacheConfig?: WithCacheConfig;
    protected usedTables: Set<string>;
    constructor({ table, fields, isPartialSelect, session, dialect, withList, distinct }: {
        table: PgSelectConfig['table'];
        fields: PgSelectConfig['fields'];
        isPartialSelect: boolean;
        session: PgSession | undefined;
        dialect: PgDialect;
        withList: Subquery[];
        distinct: boolean | {
            on: (PgColumn | SQLWrapper)[];
        } | undefined;
    });
    private createJoin;
    leftJoin: PgSelectJoinFn<this, TDynamic, "left", false>;
    leftJoinLateral: PgSelectJoinFn<this, TDynamic, "left", true>;
    rightJoin: PgSelectJoinFn<this, TDynamic, "right", false>;
    innerJoin: PgSelectJoinFn<this, TDynamic, "inner", false>;
    innerJoinLateral: PgSelectJoinFn<this, TDynamic, "inner", true>;
    fullJoin: PgSelectJoinFn<this, TDynamic, "full", false>;
    crossJoin: PgSelectCrossJoinFn<this, TDynamic, false>;
    crossJoinLateral: PgSelectCrossJoinFn<this, TDynamic, true>;
    private createSetOperator;
    union: <TValue extends PgSetOperatorWithResult<TResult>>(rightSelection: ((setOperators: GetPgSetOperators) => SetOperatorRightSelect<TValue, TResult>) | SetOperatorRightSelect<TValue, TResult>) => PgSelectWithout<this, TDynamic, PgSetOperatorExcludedMethods, true>;
    unionAll: <TValue extends PgSetOperatorWithResult<TResult>>(rightSelection: ((setOperators: GetPgSetOperators) => SetOperatorRightSelect<TValue, TResult>) | SetOperatorRightSelect<TValue, TResult>) => PgSelectWithout<this, TDynamic, PgSetOperatorExcludedMethods, true>;
    intersect: <TValue extends PgSetOperatorWithResult<TResult>>(rightSelection: ((setOperators: GetPgSetOperators) => SetOperatorRightSelect<TValue, TResult>) | SetOperatorRightSelect<TValue, TResult>) => PgSelectWithout<this, TDynamic, PgSetOperatorExcludedMethods, true>;
    intersectAll: <TValue extends PgSetOperatorWithResult<TResult>>(rightSelection: ((setOperators: GetPgSetOperators) => SetOperatorRightSelect<TValue, TResult>) | SetOperatorRightSelect<TValue, TResult>) => PgSelectWithout<this, TDynamic, PgSetOperatorExcludedMethods, true>;
    except: <TValue extends PgSetOperatorWithResult<TResult>>(rightSelection: ((setOperators: GetPgSetOperators) => SetOperatorRightSelect<TValue, TResult>) | SetOperatorRightSelect<TValue, TResult>) => PgSelectWithout<this, TDynamic, PgSetOperatorExcludedMethods, true>;
    exceptAll: <TValue extends PgSetOperatorWithResult<TResult>>(rightSelection: ((setOperators: GetPgSetOperators) => SetOperatorRightSelect<TValue, TResult>) | SetOperatorRightSelect<TValue, TResult>) => PgSelectWithout<this, TDynamic, PgSetOperatorExcludedMethods, true>;
    where(where: ((aliases: this['_']['selection']) => SQL | undefined) | SQL | undefined): PgSelectWithout<this, TDynamic, 'where'>;
    having(having: ((aliases: this['_']['selection']) => SQL | undefined) | SQL | undefined): PgSelectWithout<this, TDynamic, 'having'>;
    groupBy(builder: (aliases: this['_']['selection']) => ValueOrArray<PgColumn | SQL | SQL.Aliased>): PgSelectWithout<this, TDynamic, 'groupBy'>;
    groupBy(...columns: (PgColumn | SQL | SQL.Aliased)[]): PgSelectWithout<this, TDynamic, 'groupBy'>;
    orderBy(builder: (aliases: this['_']['selection']) => ValueOrArray<PgColumn | SQL | SQL.Aliased>): PgSelectWithout<this, TDynamic, 'orderBy'>;
    orderBy(...columns: (PgColumn | SQL | SQL.Aliased)[]): PgSelectWithout<this, TDynamic, 'orderBy'>;
    limit(limit: number | Placeholder): PgSelectWithout<this, TDynamic, 'limit'>;
    offset(offset: number | Placeholder): PgSelectWithout<this, TDynamic, 'offset'>;
    for(strength: LockStrength, config?: LockConfig): PgSelectWithout<this, TDynamic, 'for'>;
    toSQL(): Query;
    as<TAlias extends string>(alias: TAlias): SubqueryWithSelection<this['_']['selectedFields'], TAlias>;
    $dynamic(): PgSelectDynamic<this>;
    $withCache(config?: {
        config?: CacheConfig;
        tag?: string;
        autoInvalidate?: boolean;
    } | false): this;
}
interface PgSelectBase<TTableName extends string | undefined, TSelection extends ColumnsSelection, TSelectMode extends SelectMode, TNullabilityMap extends Record<string, JoinNullability> = TTableName extends string ? Record<TTableName, 'not-null'> : {}, TDynamic extends boolean = false, TExcludedMethods extends string = never, TResult extends any[] = SelectResult<TSelection, TSelectMode, TNullabilityMap>[], TSelectedFields extends ColumnsSelection = BuildSubquerySelection<TSelection, TNullabilityMap>> extends PgSelectQueryBuilderBase<PgSelectHKT, TTableName, TSelection, TSelectMode, TNullabilityMap, TDynamic, TExcludedMethods, TResult, TSelectedFields>, QueryPromise<TResult>, SQLWrapper {
}
declare class PgSelectBase<TTableName extends string | undefined, TSelection extends ColumnsSelection, TSelectMode extends SelectMode, TNullabilityMap extends Record<string, JoinNullability> = TTableName extends string ? Record<TTableName, 'not-null'> : {}, TDynamic extends boolean = false, TExcludedMethods extends string = never, TResult = SelectResult<TSelection, TSelectMode, TNullabilityMap>[], TSelectedFields = BuildSubquerySelection<TSelection, TNullabilityMap>> extends PgSelectQueryBuilderBase<PgSelectHKT, TTableName, TSelection, TSelectMode, TNullabilityMap, TDynamic, TExcludedMethods, TResult, TSelectedFields> implements RunnableQuery<TResult, 'pg'>, SQLWrapper {
    static readonly [entityKind]: string;
    prepare(name: string): PgSelectPrepare<this>;
    private authToken?;
    execute: ReturnType<this['prepare']>['execute'];
}
interface PgDialectConfig {
    casing?: Casing;
}
declare class PgDialect {
    static readonly [entityKind]: string;
    constructor(config?: PgDialectConfig);
    migrate(migrations: MigrationMeta[], session: PgSession, config: string | MigrationConfig): Promise<void>;
    escapeName(name: string): string;
    escapeParam(num: number): string;
    escapeString(str: string): string;
    private buildWithCTE;
    buildDeleteQuery({ table, where, returning, withList }: PgDeleteConfig): SQL;
    buildUpdateSet(table: PgTable, set: UpdateSet): SQL;
    buildUpdateQuery({ table, set, where, returning, withList, from, joins }: PgUpdateConfig): SQL;
    private buildSelection;
    private buildJoins;
    private buildFromTable;
    buildSelectQuery({ withList, fields, fieldsFlat, where, having, table, joins, orderBy, groupBy, limit, offset, lockingClause, distinct, setOperators, }: PgSelectConfig): SQL;
    buildSetOperations(leftSelect: SQL, setOperators: PgSelectConfig['setOperators']): SQL;
    buildSetOperationQuery({ leftSelect, setOperator: { type, isAll, rightSelect, limit, orderBy, offset }, }: {
        leftSelect: SQL;
        setOperator: PgSelectConfig['setOperators'][number];
    }): SQL;
    buildInsertQuery({ table, values: valuesOrSelect, onConflict, returning, withList, select, overridingSystemValue_ }: PgInsertConfig): SQL;
    buildRefreshMaterializedViewQuery({ view, concurrently, withNoData }: {
        view: PgMaterializedView;
        concurrently?: boolean;
        withNoData?: boolean;
    }): SQL;
    prepareTyping(encoder: DriverValueEncoder<unknown, unknown>): QueryTypingsValue;
    sqlToQuery(sql: SQL, invokeSource?: 'indexes' | undefined): QueryWithTypings;
    buildRelationalQueryWithoutPK({ fullSchema, schema, tableNamesMap, table, tableConfig, queryConfig: config, tableAlias, nestedQueryRelation, joinOn, }: {
        fullSchema: Record<string, unknown>;
        schema: TablesRelationalConfig;
        tableNamesMap: Record<string, string>;
        table: PgTable;
        tableConfig: TableRelationalConfig;
        queryConfig: true | DBQueryConfig<'many', true>;
        tableAlias: string;
        nestedQueryRelation?: Relation;
        joinOn?: SQL;
    }): BuildRelationalQueryResult<PgTable, PgColumn>;
}
declare class QueryBuilder {
    static readonly [entityKind]: string;
    private dialect;
    private dialectConfig;
    constructor(dialect?: PgDialect | PgDialectConfig);
    $with: WithBuilder;
    with(...queries: WithSubquery[]): {
        select: {
            (): PgSelectBuilder<undefined, "qb">;
            <TSelection extends SelectedFields>(fields: TSelection): PgSelectBuilder<TSelection, "qb">;
        };
        selectDistinct: {
            (): PgSelectBuilder<undefined, "qb">;
            <TSelection extends SelectedFields>(fields: TSelection): PgSelectBuilder<TSelection, "qb">;
        };
        selectDistinctOn: {
            (on: (PgColumn | SQLWrapper)[]): PgSelectBuilder<undefined, "qb">;
            <TSelection extends SelectedFields>(on: (PgColumn | SQLWrapper)[], fields: TSelection): PgSelectBuilder<TSelection, "qb">;
        };
    };
    select(): PgSelectBuilder<undefined, 'qb'>;
    select<TSelection extends SelectedFields>(fields: TSelection): PgSelectBuilder<TSelection, 'qb'>;
    selectDistinct(): PgSelectBuilder<undefined>;
    selectDistinct<TSelection extends SelectedFields>(fields: TSelection): PgSelectBuilder<TSelection>;
    selectDistinctOn(on: (PgColumn | SQLWrapper)[]): PgSelectBuilder<undefined>;
    selectDistinctOn<TSelection extends SelectedFields>(on: (PgColumn | SQLWrapper)[], fields: TSelection): PgSelectBuilder<TSelection>;
    private getDialect;
}
declare const PgViewConfig: unique symbol;
type ViewWithConfig = RequireAtLeastOne<{
    checkOption: 'local' | 'cascaded';
    securityBarrier: boolean;
    securityInvoker: boolean;
}>;
type PgMaterializedViewWithConfig = RequireAtLeastOne<{
    fillfactor: number;
    toastTupleTarget: number;
    parallelWorkers: number;
    autovacuumEnabled: boolean;
    vacuumIndexCleanup: 'auto' | 'off' | 'on';
    vacuumTruncate: boolean;
    autovacuumVacuumThreshold: number;
    autovacuumVacuumScaleFactor: number;
    autovacuumVacuumCostDelay: number;
    autovacuumVacuumCostLimit: number;
    autovacuumFreezeMinAge: number;
    autovacuumFreezeMaxAge: number;
    autovacuumFreezeTableAge: number;
    autovacuumMultixactFreezeMinAge: number;
    autovacuumMultixactFreezeMaxAge: number;
    autovacuumMultixactFreezeTableAge: number;
    logAutovacuumMinDuration: number;
    userCatalogTable: boolean;
}>;
declare class PgView<TName extends string = string, TExisting extends boolean = boolean, TSelectedFields extends ColumnsSelection = ColumnsSelection> extends PgViewBase<TName, TExisting, TSelectedFields> {
    static readonly [entityKind]: string;
    [PgViewConfig]: {
        with?: ViewWithConfig;
    } | undefined;
    constructor({ pgConfig, config }: {
        pgConfig: {
            with?: ViewWithConfig;
        } | undefined;
        config: {
            name: TName;
            schema: string | undefined;
            selectedFields: ColumnsSelection;
            query: SQL | undefined;
        };
    });
}
type PgViewWithSelection<TName extends string = string, TExisting extends boolean = boolean, TSelectedFields extends ColumnsSelection = ColumnsSelection> = PgView<TName, TExisting, TSelectedFields> & TSelectedFields;
declare const PgMaterializedViewConfig: unique symbol;
declare class PgMaterializedView<TName extends string = string, TExisting extends boolean = boolean, TSelectedFields extends ColumnsSelection = ColumnsSelection> extends PgViewBase<TName, TExisting, TSelectedFields> {
    static readonly [entityKind]: string;
    readonly [PgMaterializedViewConfig]: {
        readonly with?: PgMaterializedViewWithConfig;
        readonly using?: string;
        readonly tablespace?: string;
        readonly withNoData?: boolean;
    } | undefined;
    constructor({ pgConfig, config }: {
        pgConfig: {
            with: PgMaterializedViewWithConfig | undefined;
            using: string | undefined;
            tablespace: string | undefined;
            withNoData: boolean | undefined;
        } | undefined;
        config: {
            name: TName;
            schema: string | undefined;
            selectedFields: ColumnsSelection;
            query: SQL | undefined;
        };
    });
}
interface PgSelectJoinConfig {
    on: SQL | undefined;
    table: PgTable | Subquery | PgViewBase | SQL;
    alias: string | undefined;
    joinType: JoinType;
    lateral?: boolean;
}
type BuildAliasTable<TTable extends PgTable | View, TAlias extends string> = TTable extends Table ? PgTableWithColumns<UpdateTableConfig<TTable['_']['config'], {
    name: TAlias;
    columns: MapColumnsToTableAlias<TTable['_']['columns'], TAlias, 'pg'>;
}>> : TTable extends View ? PgViewWithSelection<TAlias, TTable['_']['existing'], MapColumnsToTableAlias<TTable['_']['selectedFields'], TAlias, 'pg'>> : never;
interface PgSelectConfig {
    withList?: Subquery[];
    fields: Record<string, unknown>;
    fieldsFlat?: SelectedFieldsOrdered;
    where?: SQL;
    having?: SQL;
    table: PgTable | Subquery | PgViewBase | SQL;
    limit?: number | Placeholder;
    offset?: number | Placeholder;
    joins?: PgSelectJoinConfig[];
    orderBy?: (PgColumn | SQL | SQL.Aliased)[];
    groupBy?: (PgColumn | SQL | SQL.Aliased)[];
    lockingClause?: {
        strength: LockStrength;
        config: LockConfig;
    };
    distinct?: boolean | {
        on: (PgColumn | SQLWrapper)[];
    };
    setOperators: {
        rightSelect: TypedQueryBuilder<any, any>;
        type: SetOperator;
        isAll: boolean;
        orderBy?: (PgColumn | SQL | SQL.Aliased)[];
        limit?: number | Placeholder;
        offset?: number | Placeholder;
    }[];
}
type TableLikeHasEmptySelection<T extends PgTable | Subquery | PgViewBase | SQL> = T extends Subquery ? Equal<T['_']['selectedFields'], {}> extends true ? true : false : false;
type PgSelectJoin<T extends AnyPgSelectQueryBuilder, TDynamic extends boolean, TJoinType extends JoinType, TJoinedTable extends PgTable | Subquery | PgViewBase | SQL, TJoinedName extends GetSelectTableName<TJoinedTable> = GetSelectTableName<TJoinedTable>> = T extends any ? PgSelectWithout<PgSelectKind<T['_']['hkt'], T['_']['tableName'], AppendToResult<T['_']['tableName'], T['_']['selection'], TJoinedName, TJoinedTable extends Table ? TJoinedTable['_']['columns'] : TJoinedTable extends Subquery | View ? Assume<TJoinedTable['_']['selectedFields'], SelectedFields> : never, T['_']['selectMode']>, T['_']['selectMode'] extends 'partial' ? T['_']['selectMode'] : 'multiple', AppendToNullabilityMap<T['_']['nullabilityMap'], TJoinedName, TJoinType>, T['_']['dynamic'], T['_']['excludedMethods']>, TDynamic, T['_']['excludedMethods']> : never;
type PgSelectJoinFn<T extends AnyPgSelectQueryBuilder, TDynamic extends boolean, TJoinType extends JoinType, TIsLateral extends boolean> = <TJoinedTable extends (TIsLateral extends true ? Subquery | SQL : PgTable | Subquery | PgViewBase | SQL), TJoinedName extends GetSelectTableName<TJoinedTable> = GetSelectTableName<TJoinedTable>>(table: TableLikeHasEmptySelection<TJoinedTable> extends true ? DrizzleTypeError<"Cannot reference a data-modifying statement subquery if it doesn't contain a `returning` clause"> : TJoinedTable, on: ((aliases: T['_']['selection']) => SQL | undefined) | SQL | undefined) => PgSelectJoin<T, TDynamic, TJoinType, TJoinedTable, TJoinedName>;
type PgSelectCrossJoinFn<T extends AnyPgSelectQueryBuilder, TDynamic extends boolean, TIsLateral extends boolean> = <TJoinedTable extends (TIsLateral extends true ? Subquery | SQL : PgTable | Subquery | PgViewBase | SQL), TJoinedName extends GetSelectTableName<TJoinedTable> = GetSelectTableName<TJoinedTable>>(table: TableLikeHasEmptySelection<TJoinedTable> extends true ? DrizzleTypeError<"Cannot reference a data-modifying statement subquery if it doesn't contain a `returning` clause"> : TJoinedTable) => PgSelectJoin<T, TDynamic, 'cross', TJoinedTable, TJoinedName>;
type SelectedFieldsFlat = SelectedFieldsFlat$1<PgColumn>;
type SelectedFields = SelectedFields$1<PgColumn, PgTable>;
type SelectedFieldsOrdered = SelectedFieldsOrdered$1<PgColumn>;
type LockStrength = 'update' | 'no key update' | 'share' | 'key share';
type LockConfig = {
    of?: ValueOrArray<PgTable>;
} & ({
    noWait: true;
    skipLocked?: undefined;
} | {
    noWait?: undefined;
    skipLocked: true;
} | {
    noWait?: undefined;
    skipLocked?: undefined;
});
interface PgSelectHKTBase {
    tableName: string | undefined;
    selection: unknown;
    selectMode: SelectMode;
    nullabilityMap: unknown;
    dynamic: boolean;
    excludedMethods: string;
    result: unknown;
    selectedFields: unknown;
    _type: unknown;
}
type PgSelectKind<T extends PgSelectHKTBase, TTableName extends string | undefined, TSelection extends ColumnsSelection, TSelectMode extends SelectMode, TNullabilityMap extends Record<string, JoinNullability>, TDynamic extends boolean, TExcludedMethods extends string, TResult = SelectResult<TSelection, TSelectMode, TNullabilityMap>[], TSelectedFields = BuildSubquerySelection<TSelection, TNullabilityMap>> = (T & {
    tableName: TTableName;
    selection: TSelection;
    selectMode: TSelectMode;
    nullabilityMap: TNullabilityMap;
    dynamic: TDynamic;
    excludedMethods: TExcludedMethods;
    result: TResult;
    selectedFields: TSelectedFields;
})['_type'];
interface PgSelectQueryBuilderHKT extends PgSelectHKTBase {
    _type: PgSelectQueryBuilderBase<PgSelectQueryBuilderHKT, this['tableName'], Assume<this['selection'], ColumnsSelection>, this['selectMode'], Assume<this['nullabilityMap'], Record<string, JoinNullability>>, this['dynamic'], this['excludedMethods'], Assume<this['result'], any[]>, Assume<this['selectedFields'], ColumnsSelection>>;
}
interface PgSelectHKT extends PgSelectHKTBase {
    _type: PgSelectBase<this['tableName'], Assume<this['selection'], ColumnsSelection>, this['selectMode'], Assume<this['nullabilityMap'], Record<string, JoinNullability>>, this['dynamic'], this['excludedMethods'], Assume<this['result'], any[]>, Assume<this['selectedFields'], ColumnsSelection>>;
}
type CreatePgSelectFromBuilderMode<TBuilderMode extends 'db' | 'qb', TTableName extends string | undefined, TSelection extends ColumnsSelection, TSelectMode extends SelectMode> = TBuilderMode extends 'db' ? PgSelectBase<TTableName, TSelection, TSelectMode> : PgSelectQueryBuilderBase<PgSelectQueryBuilderHKT, TTableName, TSelection, TSelectMode>;
type PgSetOperatorExcludedMethods = 'leftJoin' | 'rightJoin' | 'innerJoin' | 'fullJoin' | 'where' | 'having' | 'groupBy' | 'for';
type PgSelectWithout<T extends AnyPgSelectQueryBuilder, TDynamic extends boolean, K extends keyof T & string, TResetExcluded extends boolean = false> = TDynamic extends true ? T : Omit<PgSelectKind<T['_']['hkt'], T['_']['tableName'], T['_']['selection'], T['_']['selectMode'], T['_']['nullabilityMap'], TDynamic, TResetExcluded extends true ? K : T['_']['excludedMethods'] | K, T['_']['result'], T['_']['selectedFields']>, TResetExcluded extends true ? K : T['_']['excludedMethods'] | K>;
type PgSelectPrepare<T extends AnyPgSelect> = PgPreparedQuery<PreparedQueryConfig & {
    execute: T['_']['result'];
}>;
type PgSelectDynamic<T extends AnyPgSelectQueryBuilder> = PgSelectKind<T['_']['hkt'], T['_']['tableName'], T['_']['selection'], T['_']['selectMode'], T['_']['nullabilityMap'], true, never, T['_']['result'], T['_']['selectedFields']>;
type AnyPgSelectQueryBuilder = PgSelectQueryBuilderBase<any, any, any, any, any, any, any, any, any>;
type AnyPgSetOperatorInterface = PgSetOperatorInterface<any, any, any, any, any, any, any, any>;
interface PgSetOperatorInterface<TTableName extends string | undefined, TSelection extends ColumnsSelection, TSelectMode extends SelectMode, TNullabilityMap extends Record<string, JoinNullability> = TTableName extends string ? Record<TTableName, 'not-null'> : {}, TDynamic extends boolean = false, TExcludedMethods extends string = never, TResult extends any[] = SelectResult<TSelection, TSelectMode, TNullabilityMap>[], TSelectedFields extends ColumnsSelection = BuildSubquerySelection<TSelection, TNullabilityMap>> {
    _: {
        readonly hkt: PgSelectHKT;
        readonly tableName: TTableName;
        readonly selection: TSelection;
        readonly selectMode: TSelectMode;
        readonly nullabilityMap: TNullabilityMap;
        readonly dynamic: TDynamic;
        readonly excludedMethods: TExcludedMethods;
        readonly result: TResult;
        readonly selectedFields: TSelectedFields;
    };
}
type PgSetOperatorWithResult<TResult extends any[]> = PgSetOperatorInterface<any, any, any, any, any, any, TResult, any>;
type AnyPgSelect = PgSelectBase<any, any, any, any, any, any, any, any>;
type SetOperatorRightSelect<TValue extends PgSetOperatorWithResult<TResult>, TResult extends any[]> = TValue extends PgSetOperatorInterface<any, any, any, any, any, any, infer TValueResult, any> ? ValidateShape<TValueResult[number], TResult[number], TypedQueryBuilder<any, TValueResult>> : TValue;
type SetOperatorRestSelect<TValue extends readonly PgSetOperatorWithResult<TResult>[], TResult extends any[]> = TValue extends [
    infer First,
    ...infer Rest
] ? First extends PgSetOperatorInterface<any, any, any, any, any, any, infer TValueResult, any> ? Rest extends AnyPgSetOperatorInterface[] ? [
    ValidateShape<TValueResult[number], TResult[number], TypedQueryBuilder<any, TValueResult>>,
    ...SetOperatorRestSelect<Rest, TResult>
] : ValidateShape<TValueResult[number], TResult[number], TypedQueryBuilder<any, TValueResult>[]> : never : TValue;
type PgCreateSetOperatorFn = <TTableName extends string | undefined, TSelection extends ColumnsSelection, TSelectMode extends SelectMode, TValue extends PgSetOperatorWithResult<TResult>, TRest extends PgSetOperatorWithResult<TResult>[], TNullabilityMap extends Record<string, JoinNullability> = TTableName extends string ? Record<TTableName, 'not-null'> : {}, TDynamic extends boolean = false, TExcludedMethods extends string = never, TResult extends any[] = SelectResult<TSelection, TSelectMode, TNullabilityMap>[], TSelectedFields extends ColumnsSelection = BuildSubquerySelection<TSelection, TNullabilityMap>>(leftSelect: PgSetOperatorInterface<TTableName, TSelection, TSelectMode, TNullabilityMap, TDynamic, TExcludedMethods, TResult, TSelectedFields>, rightSelect: SetOperatorRightSelect<TValue, TResult>, ...restSelects: SetOperatorRestSelect<TRest, TResult>) => PgSelectWithout<PgSelectBase<TTableName, TSelection, TSelectMode, TNullabilityMap, TDynamic, TExcludedMethods, TResult, TSelectedFields>, false, PgSetOperatorExcludedMethods, true>;
type GetPgSetOperators = {
    union: PgCreateSetOperatorFn;
    intersect: PgCreateSetOperatorFn;
    except: PgCreateSetOperatorFn;
    unionAll: PgCreateSetOperatorFn;
    intersectAll: PgCreateSetOperatorFn;
    exceptAll: PgCreateSetOperatorFn;
};
declare function alias<TTable extends PgTable | PgViewBase, TAlias extends string>(table: TTable, alias: TAlias): BuildAliasTable<TTable, TAlias>;
type TableConfig$1 = TableConfig$5<SingleStoreColumn>;
declare class SingleStoreTable<T extends TableConfig$1 = TableConfig$1> extends Table<T> {
    static readonly [entityKind]: string;
    protected $columns: T['columns'];
}
declare abstract class SingleStoreColumn<T extends ColumnBaseConfig<ColumnDataType, string> = ColumnBaseConfig<ColumnDataType, string>, TRuntimeConfig extends object = {}, TTypeConfig extends object = {}> extends Column<T, TRuntimeConfig, TTypeConfig & {
    dialect: 'singlestore';
}> {
    readonly table: SingleStoreTable;
    static readonly [entityKind]: string;
    constructor(table: SingleStoreTable, config: ColumnBuilderRuntimeConfig<T['data'], TRuntimeConfig>);
}
type TableConfig = TableConfig$5<SQLiteColumn<any>>;
declare class SQLiteTable<T extends TableConfig = TableConfig> extends Table<T> {
    static readonly [entityKind]: string;
}
declare abstract class SQLiteColumn<T extends ColumnBaseConfig<ColumnDataType, string> = ColumnBaseConfig<ColumnDataType, string>, TRuntimeConfig extends object = {}, TTypeConfig extends object = {}> extends Column<T, TRuntimeConfig, TTypeConfig & {
    dialect: 'sqlite';
}> {
    readonly table: SQLiteTable;
    static readonly [entityKind]: string;
    constructor(table: SQLiteTable, config: ColumnBuilderRuntimeConfig<T['data'], TRuntimeConfig>);
}
type ColumnDataType = 'string' | 'number' | 'boolean' | 'array' | 'json' | 'date' | 'bigint' | 'custom' | 'buffer' | 'dateDuration' | 'duration' | 'relDuration' | 'localTime' | 'localDate' | 'localDateTime';
type Dialect = 'pg' | 'mysql' | 'sqlite' | 'singlestore' | 'common' | 'gel';
type GeneratedStorageMode = 'virtual' | 'stored';
type GeneratedType = 'always' | 'byDefault';
type GeneratedColumnConfig<TDataType> = {
    as: TDataType | SQL | (() => SQL);
    type?: GeneratedType;
    mode?: GeneratedStorageMode;
};
type GeneratedIdentityConfig = {
    sequenceName?: string;
    sequenceOptions?: PgSequenceOptions;
    type: 'always' | 'byDefault';
};
interface ColumnBuilderBaseConfig<TDataType extends ColumnDataType, TColumnType extends string> {
    name: string;
    dataType: TDataType;
    columnType: TColumnType;
    data: unknown;
    driverParam: unknown;
    enumValues: string[] | undefined;
}
type MakeColumnConfig<T extends ColumnBuilderBaseConfig<ColumnDataType, string>, TTableName extends string, TData = T extends {
    $type: infer U;
} ? U : T['data']> = {
    name: T['name'];
    tableName: TTableName;
    dataType: T['dataType'];
    columnType: T['columnType'];
    data: TData;
    driverParam: T['driverParam'];
    notNull: T extends {
        notNull: true;
    } ? true : false;
    hasDefault: T extends {
        hasDefault: true;
    } ? true : false;
    isPrimaryKey: T extends {
        isPrimaryKey: true;
    } ? true : false;
    isAutoincrement: T extends {
        isAutoincrement: true;
    } ? true : false;
    hasRuntimeDefault: T extends {
        hasRuntimeDefault: true;
    } ? true : false;
    enumValues: T['enumValues'];
    baseColumn: T extends {
        baseBuilder: infer U extends ColumnBuilderBase;
    } ? BuildColumn<TTableName, U, 'common'> : never;
    identity: T extends {
        identity: 'always';
    } ? 'always' : T extends {
        identity: 'byDefault';
    } ? 'byDefault' : undefined;
    generated: T extends {
        generated: infer G;
    } ? unknown extends G ? undefined : G extends undefined ? undefined : G : undefined;
} & {};
type ColumnBuilderTypeConfig<T extends ColumnBuilderBaseConfig<ColumnDataType, string>, TTypeConfig extends object = object> = Simplify<{
    brand: 'ColumnBuilder';
    name: T['name'];
    dataType: T['dataType'];
    columnType: T['columnType'];
    data: T['data'];
    driverParam: T['driverParam'];
    notNull: T extends {
        notNull: infer U;
    } ? U : boolean;
    hasDefault: T extends {
        hasDefault: infer U;
    } ? U : boolean;
    enumValues: T['enumValues'];
    identity: T extends {
        identity: infer U;
    } ? U : unknown;
    generated: T extends {
        generated: infer G;
    } ? G extends undefined ? unknown : G : unknown;
} & TTypeConfig>;
type ColumnBuilderRuntimeConfig<TData, TRuntimeConfig extends object = object> = {
    name: string;
    keyAsName: boolean;
    notNull: boolean;
    default: TData | SQL | undefined;
    defaultFn: (() => TData | SQL) | undefined;
    onUpdateFn: (() => TData | SQL) | undefined;
    hasDefault: boolean;
    primaryKey: boolean;
    isUnique: boolean;
    uniqueName: string | undefined;
    uniqueType: string | undefined;
    dataType: string;
    columnType: string;
    generated: GeneratedColumnConfig<TData> | undefined;
    generatedIdentity: GeneratedIdentityConfig | undefined;
} & TRuntimeConfig;
interface ColumnBuilderExtraConfig {
    primaryKeyHasDefault?: boolean;
}
type NotNull<T extends ColumnBuilderBase> = T & {
    _: {
        notNull: true;
    };
};
type HasDefault<T extends ColumnBuilderBase> = T & {
    _: {
        hasDefault: true;
    };
};
type IsPrimaryKey<T extends ColumnBuilderBase> = T & {
    _: {
        isPrimaryKey: true;
    };
};
type HasRuntimeDefault<T extends ColumnBuilderBase> = T & {
    _: {
        hasRuntimeDefault: true;
    };
};
type $Type<T extends ColumnBuilderBase, TType> = T & {
    _: {
        $type: TType;
    };
};
type HasGenerated<T extends ColumnBuilderBase, TGenerated extends {} = {}> = T & {
    _: {
        hasDefault: true;
        generated: TGenerated;
    };
};
type IsIdentity<T extends ColumnBuilderBase, TType extends 'always' | 'byDefault'> = T & {
    _: {
        notNull: true;
        hasDefault: true;
        identity: TType;
    };
};
interface ColumnBuilderBase<T extends ColumnBuilderBaseConfig<ColumnDataType, string> = ColumnBuilderBaseConfig<ColumnDataType, string>, TTypeConfig extends object = object> {
    _: ColumnBuilderTypeConfig<T, TTypeConfig>;
}
declare abstract class ColumnBuilder<T extends ColumnBuilderBaseConfig<ColumnDataType, string> = ColumnBuilderBaseConfig<ColumnDataType, string>, TRuntimeConfig extends object = object, TTypeConfig extends object = object, TExtraConfig extends ColumnBuilderExtraConfig = ColumnBuilderExtraConfig> implements ColumnBuilderBase<T, TTypeConfig> {
    static readonly [entityKind]: string;
    _: ColumnBuilderTypeConfig<T, TTypeConfig>;
    protected config: ColumnBuilderRuntimeConfig<T['data'], TRuntimeConfig>;
    constructor(name: T['name'], dataType: T['dataType'], columnType: T['columnType']);
    $type<TType>(): $Type<this, TType>;
    notNull(): NotNull<this>;
    default(value: (this['_'] extends {
        $type: infer U;
    } ? U : this['_']['data']) | SQL): HasDefault<this>;
    $defaultFn(fn: () => (this['_'] extends {
        $type: infer U;
    } ? U : this['_']['data']) | SQL): HasRuntimeDefault<HasDefault<this>>;
    $default: (fn: () => (this["_"] extends {
        $type: infer U;
    } ? U : this["_"]["data"]) | SQL) => HasRuntimeDefault<HasDefault<this>>;
    $onUpdateFn(fn: () => (this['_'] extends {
        $type: infer U;
    } ? U : this['_']['data']) | SQL): HasDefault<this>;
    $onUpdate: (fn: () => (this["_"] extends {
        $type: infer U;
    } ? U : this["_"]["data"]) | SQL) => HasDefault<this>;
    primaryKey(): TExtraConfig['primaryKeyHasDefault'] extends true ? IsPrimaryKey<HasDefault<NotNull<this>>> : IsPrimaryKey<NotNull<this>>;
    abstract generatedAlwaysAs(as: SQL | T['data'] | (() => SQL), config?: Partial<GeneratedColumnConfig<unknown>>): HasGenerated<this, {
        type: 'always';
    }>;
}
type BuildColumn<TTableName extends string, TBuilder extends ColumnBuilderBase, TDialect extends Dialect> = TDialect extends 'pg' ? PgColumn<MakeColumnConfig<TBuilder['_'], TTableName>, {}, Simplify<Omit<TBuilder['_'], keyof MakeColumnConfig<TBuilder['_'], TTableName> | 'brand' | 'dialect'>>> : TDialect extends 'mysql' ? MySqlColumn<MakeColumnConfig<TBuilder['_'], TTableName>, {}, Simplify<Omit<TBuilder['_'], keyof MakeColumnConfig<TBuilder['_'], TTableName> | 'brand' | 'dialect' | 'primaryKeyHasDefault' | 'mysqlColumnBuilderBrand'>>> : TDialect extends 'sqlite' ? SQLiteColumn<MakeColumnConfig<TBuilder['_'], TTableName>, {}, Simplify<Omit<TBuilder['_'], keyof MakeColumnConfig<TBuilder['_'], TTableName> | 'brand' | 'dialect'>>> : TDialect extends 'common' ? Column<MakeColumnConfig<TBuilder['_'], TTableName>, {}, Simplify<Omit<TBuilder['_'], keyof MakeColumnConfig<TBuilder['_'], TTableName> | 'brand' | 'dialect'>>> : TDialect extends 'singlestore' ? SingleStoreColumn<MakeColumnConfig<TBuilder['_'], TTableName>, {}, Simplify<Omit<TBuilder['_'], keyof MakeColumnConfig<TBuilder['_'], TTableName> | 'brand' | 'dialect' | 'primaryKeyHasDefault' | 'singlestoreColumnBuilderBrand'>>> : TDialect extends 'gel' ? GelColumn<MakeColumnConfig<TBuilder['_'], TTableName>, {}, Simplify<Omit<TBuilder['_'], keyof MakeColumnConfig<TBuilder['_'], TTableName> | 'brand' | 'dialect'>>> : never;
type BuildIndexColumn<TDialect extends Dialect> = TDialect extends 'pg' ? ExtraConfigColumn : TDialect extends 'gel' ? GelExtraConfigColumn : never;
type BuildColumns<TTableName extends string, TConfigMap extends Record<string, ColumnBuilderBase>, TDialect extends Dialect> = {
    [Key in keyof TConfigMap]: BuildColumn<TTableName, {
        _: Omit<TConfigMap[Key]['_'], 'name'> & {
            name: TConfigMap[Key]['_']['name'] extends '' ? Assume<Key, string> : TConfigMap[Key]['_']['name'];
        };
    }, TDialect>;
} & {};
type BuildExtraConfigColumns<_TTableName extends string, TConfigMap extends Record<string, ColumnBuilderBase>, TDialect extends Dialect> = {
    [Key in keyof TConfigMap]: BuildIndexColumn<TDialect>;
} & {};
type ChangeColumnTableName<TColumn extends Column, TAlias extends string, TDialect extends Dialect> = TDialect extends 'pg' ? PgColumn<MakeColumnConfig<TColumn['_'], TAlias>> : TDialect extends 'mysql' ? MySqlColumn<MakeColumnConfig<TColumn['_'], TAlias>> : TDialect extends 'singlestore' ? SingleStoreColumn<MakeColumnConfig<TColumn['_'], TAlias>> : TDialect extends 'sqlite' ? SQLiteColumn<MakeColumnConfig<TColumn['_'], TAlias>> : TDialect extends 'gel' ? GelColumn<MakeColumnConfig<TColumn['_'], TAlias>> : never;
interface ColumnBaseConfig<TDataType extends ColumnDataType, TColumnType extends string> extends ColumnBuilderBaseConfig<TDataType, TColumnType> {
    tableName: string;
    notNull: boolean;
    hasDefault: boolean;
    isPrimaryKey: boolean;
    isAutoincrement: boolean;
    hasRuntimeDefault: boolean;
}
type ColumnTypeConfig<T extends ColumnBaseConfig<ColumnDataType, string>, TTypeConfig extends object> = T & {
    brand: 'Column';
    tableName: T['tableName'];
    name: T['name'];
    dataType: T['dataType'];
    columnType: T['columnType'];
    data: T['data'];
    driverParam: T['driverParam'];
    notNull: T['notNull'];
    hasDefault: T['hasDefault'];
    isPrimaryKey: T['isPrimaryKey'];
    isAutoincrement: T['isAutoincrement'];
    hasRuntimeDefault: T['hasRuntimeDefault'];
    enumValues: T['enumValues'];
    baseColumn: T extends {
        baseColumn: infer U;
    } ? U : unknown;
    generated: GeneratedColumnConfig<T['data']> | undefined;
    identity: undefined | 'always' | 'byDefault';
} & TTypeConfig;
type ColumnRuntimeConfig<TData, TRuntimeConfig extends object> = ColumnBuilderRuntimeConfig<TData, TRuntimeConfig>;
interface Column<T extends ColumnBaseConfig<ColumnDataType, string> = ColumnBaseConfig<ColumnDataType, string>, TRuntimeConfig extends object = object, TTypeConfig extends object = object> extends DriverValueMapper<T['data'], T['driverParam']>, SQLWrapper {
}
declare abstract class Column<T extends ColumnBaseConfig<ColumnDataType, string> = ColumnBaseConfig<ColumnDataType, string>, TRuntimeConfig extends object = object, TTypeConfig extends object = object> implements DriverValueMapper<T['data'], T['driverParam']>, SQLWrapper {
    readonly table: Table;
    static readonly [entityKind]: string;
    readonly _: ColumnTypeConfig<T, TTypeConfig>;
    readonly name: string;
    readonly keyAsName: boolean;
    readonly primary: boolean;
    readonly notNull: boolean;
    readonly default: T['data'] | SQL | undefined;
    readonly defaultFn: (() => T['data'] | SQL) | undefined;
    readonly onUpdateFn: (() => T['data'] | SQL) | undefined;
    readonly hasDefault: boolean;
    readonly isUnique: boolean;
    readonly uniqueName: string | undefined;
    readonly uniqueType: string | undefined;
    readonly dataType: T['dataType'];
    readonly columnType: T['columnType'];
    readonly enumValues: T['enumValues'];
    readonly generated: GeneratedColumnConfig<T['data']> | undefined;
    readonly generatedIdentity: GeneratedIdentityConfig | undefined;
    protected config: ColumnRuntimeConfig<T['data'], TRuntimeConfig>;
    constructor(table: Table, config: ColumnRuntimeConfig<T['data'], TRuntimeConfig>);
    abstract getSQLType(): string;
    mapFromDriverValue(value: unknown): unknown;
    mapToDriverValue(value: unknown): unknown;
}
type UpdateColConfig<T extends ColumnBaseConfig<ColumnDataType, string>, TUpdate extends Partial<ColumnBaseConfig<ColumnDataType, string>>> = Update<T, TUpdate>;
type AnyColumn<TPartial extends Partial<ColumnBaseConfig<ColumnDataType, string>> = {}> = Column<Required<Update<ColumnBaseConfig<ColumnDataType, string>, TPartial>>>;
type GetColumnData<TColumn extends Column, TInferMode extends 'query' | 'raw' = 'query'> = TInferMode extends 'raw' ? TColumn['_']['data'] : TColumn['_']['notNull'] extends true ? TColumn['_']['data'] : TColumn['_']['data'] | null;
type Literal = Static<typeof literalSchema>;
type Json = Literal | {
    [key: string]: any;
} | any[];
interface JsonSchema extends TSchema {
    [Kind]: 'Union';
    static: Json;
    anyOf: Json;
}
interface BufferSchema extends TSchema {
    [Kind]: 'Buffer';
    static: Buffer;
    type: 'buffer';
}
type IsNever<T> = [
    T
] extends [
    never
] ? true : false;
type IsEnumDefined<TEnum extends string[] | undefined> = [
    string,
    ...string[]
] extends TEnum ? false : undefined extends TEnum ? false : true;
type ColumnIsGeneratedAlwaysAs<TColumn> = TColumn extends Column ? TColumn['_']['identity'] extends 'always' ? true : TColumn['_']['generated'] extends {
    type: 'byDefault';
} | undefined ? false : true : false;
type GetSelection<T extends SelectedFieldsFlat$1<Column> | Table | View> = T extends Table ? T['_']['columns'] : T extends View ? T['_']['selectedFields'] : T;
declare const literalSchema: t.TUnion<[
    t.TString,
    t.TNumber,
    t.TBoolean,
    t.TNull
]>;
type HasBaseColumn<TColumn> = TColumn extends {
    _: {
        baseColumn: Column | undefined;
    };
} ? IsNever<TColumn['_']['baseColumn']> extends false ? true : false : false;
type EnumValuesToEnum<TEnumValues extends [
    string,
    ...string[]
]> = {
    [K in TEnumValues[number]]: K;
};
interface GenericSchema<T> extends t.TSchema {
    static: T;
}
type GetTypeboxType<TColumn extends Column> = TColumn['_']['columnType'] extends 'MySqlTinyInt' | 'SingleStoreTinyInt' | 'PgSmallInt' | 'PgSmallSerial' | 'MySqlSmallInt' | 'MySqlMediumInt' | 'SingleStoreSmallInt' | 'SingleStoreMediumInt' | 'PgInteger' | 'PgSerial' | 'MySqlInt' | 'SingleStoreInt' | 'PgBigInt53' | 'PgBigSerial53' | 'MySqlBigInt53' | 'MySqlSerial' | 'SingleStoreBigInt53' | 'SingleStoreSerial' | 'SQLiteInteger' | 'MySqlYear' | 'SingleStoreYear' ? t.TInteger : TColumn['_']['columnType'] extends 'PgBinaryVector' ? t.TRegExp : HasBaseColumn<TColumn> extends true ? t.TArray<GetTypeboxType<Assume<TColumn['_']['baseColumn'], Column>>> : IsEnumDefined<TColumn['_']['enumValues']> extends true ? t.TEnum<{
    [K in Assume<TColumn['_']['enumValues'], string[]>[number]]: K;
}> : TColumn['_']['columnType'] extends 'PgGeometry' | 'PgPointTuple' ? t.TTuple<[
    t.TNumber,
    t.TNumber
]> : TColumn['_']['columnType'] extends 'PgLine' ? t.TTuple<[
    t.TNumber,
    t.TNumber,
    t.TNumber
]> : TColumn['_']['data'] extends Date ? t.TDate : TColumn['_']['data'] extends Buffer ? BufferSchema : TColumn['_']['dataType'] extends 'array' ? t.TArray<GetTypeboxPrimitiveType<Assume<TColumn['_']['data'], any[]>[number]>> : TColumn['_']['data'] extends Record<string, any> ? TColumn['_']['columnType'] extends 'PgJson' | 'PgJsonb' | 'MySqlJson' | 'SingleStoreJson' | 'SQLiteTextJson' | 'SQLiteBlobJson' ? GenericSchema<TColumn['_']['data']> : t.TObject<{
    [K in keyof TColumn['_']['data']]: GetTypeboxPrimitiveType<TColumn['_']['data'][K]>;
}> : TColumn['_']['dataType'] extends 'json' ? JsonSchema : GetTypeboxPrimitiveType<TColumn['_']['data']>;
type GetTypeboxPrimitiveType<TData> = TData extends number ? t.TNumber : TData extends bigint ? t.TBigInt : TData extends boolean ? t.TBoolean : TData extends string ? t.TString : t.TAny;
type HandleSelectColumn<TSchema extends t.TSchema, TColumn extends Column> = TColumn['_']['notNull'] extends true ? TSchema : t.Union<[
    TSchema,
    t.TNull
]>;
type HandleInsertColumn<TSchema extends t.TSchema, TColumn extends Column> = TColumn['_']['notNull'] extends true ? TColumn['_']['hasDefault'] extends true ? t.TOptional<TSchema> : TSchema : t.TOptional<t.Union<[
    TSchema,
    t.TNull
]>>;
type HandleUpdateColumn<TSchema extends t.TSchema, TColumn extends Column> = TColumn['_']['notNull'] extends true ? t.TOptional<TSchema> : t.TOptional<t.Union<[
    TSchema,
    t.TNull
]>>;
type HandleColumn<TType extends 'select' | 'insert' | 'update', TColumn extends Column> = TType extends 'select' ? HandleSelectColumn<GetTypeboxType<TColumn>, TColumn> : TType extends 'insert' ? HandleInsertColumn<GetTypeboxType<TColumn>, TColumn> : TType extends 'update' ? HandleUpdateColumn<GetTypeboxType<TColumn>, TColumn> : GetTypeboxType<TColumn>;
type BuildRefineField<T> = T extends t.TSchema ? ((schema: T) => t.TSchema) | t.TSchema : never;
type BuildRefine<TColumns extends Record<string, any>> = {
    [K in keyof TColumns as TColumns[K] extends Column | SelectedFieldsFlat$1<Column> | Table | View ? K : never]?: TColumns[K] extends Column ? BuildRefineField<GetTypeboxType<TColumns[K]>> : BuildRefine<GetSelection<TColumns[K]>>;
};
type HandleRefinement<TType extends 'select' | 'insert' | 'update', TRefinement, TColumn extends Column> = TRefinement extends (schema: any) => t.TSchema ? (TColumn['_']['notNull'] extends true ? ReturnType<TRefinement> : t.TUnion<[
    ReturnType<TRefinement>,
    t.TNull
]>) extends infer TSchema ? TType extends 'update' ? t.TOptional<Assume<TSchema, t.TSchema>> : TSchema : t.TSchema : TRefinement;
type IsRefinementDefined<TRefinements extends Record<string | symbol | number, any> | undefined, TKey extends string | symbol | number> = TRefinements extends object ? TRefinements[TKey] extends t.TSchema | ((schema: any) => any) ? true : false : false;
type BuildSchema<TType extends 'select' | 'insert' | 'update', TColumns extends Record<string, any>, TRefinements extends Record<string, any> | undefined> = t.TObject<Simplify<{
    [K in keyof TColumns as ColumnIsGeneratedAlwaysAs<TColumns[K]> extends true ? never : K]: TColumns[K] extends infer TColumn extends Column ? IsRefinementDefined<TRefinements, K> extends true ? Assume<HandleRefinement<TType, TRefinements[K & keyof TRefinements], TColumn>, t.TSchema> : HandleColumn<TType, TColumn> : TColumns[K] extends infer TObject extends SelectedFieldsFlat$1<Column> | Table | View ? BuildSchema<TType, GetSelection<TObject>, TRefinements extends object ? TRefinements[K & keyof TRefinements] : undefined> : t.TAny;
}>>;
type NoUnknownKeys<TRefinement extends Record<string, any>, TCompare extends Record<string, any>> = {
    [K in keyof TRefinement]: K extends keyof TCompare ? TRefinement[K] extends t.TSchema ? TRefinement[K] : TRefinement[K] extends Record<string, t.TSchema> ? NoUnknownKeys<TRefinement[K], TCompare[K]> : TRefinement[K] : DrizzleTypeError<`Found unknown key in refinement: "${K & string}"`>;
};
interface CreateSelectSchema {
    <TTable extends Table>(table: TTable): BuildSchema<'select', TTable['_']['columns'], undefined>;
    <TTable extends Table, TRefine extends BuildRefine<TTable['_']['columns']>>(table: TTable, refine?: NoUnknownKeys<TRefine, TTable['$inferSelect']>): BuildSchema<'select', TTable['_']['columns'], TRefine>;
    <TView extends View>(view: TView): BuildSchema<'select', TView['_']['selectedFields'], undefined>;
    <TView extends View, TRefine extends BuildRefine<TView['_']['selectedFields']>>(view: TView, refine: NoUnknownKeys<TRefine, TView['$inferSelect']>): BuildSchema<'select', TView['_']['selectedFields'], TRefine>;
    <TEnum extends PgEnum<any>>(enum_: TEnum): t.TEnum<EnumValuesToEnum<TEnum['enumValues']>>;
}
interface CreateInsertSchema {
    <TTable extends Table>(table: TTable): BuildSchema<'insert', TTable['_']['columns'], undefined>;
    <TTable extends Table, TRefine extends BuildRefine<Pick<TTable['_']['columns'], keyof TTable['$inferInsert']>>>(table: TTable, refine?: NoUnknownKeys<TRefine, TTable['$inferInsert']>): BuildSchema<'insert', TTable['_']['columns'], TRefine>;
}
interface CreateUpdateSchema {
    <TTable extends Table>(table: TTable): BuildSchema<'update', TTable['_']['columns'], undefined>;
    <TTable extends Table, TRefine extends BuildRefine<Pick<TTable['_']['columns'], keyof TTable['$inferInsert']>>>(table: TTable, refine?: TRefine): BuildSchema<'update', TTable['_']['columns'], TRefine>;
}
declare const createSelectSchema: CreateSelectSchema;
declare const createInsertSchema: CreateInsertSchema;
declare const createUpdateSchema: CreateUpdateSchema;
export { PgDatabase, alias, and, asc, boolean, count, createInsertSchema, createSelectSchema, createUpdateSchema, desc, eq, getTableColumns, gte, inArray, index, integer, jsonb, numeric, or, pgEnum, pgTable, primaryKey, relations, sql, sum, text, timestamp, uuid };
export type { PgQueryResultHKT };
