interface KyselyTypeError<E extends string> {
    readonly __kyselyTypeError__: E;
}
type OperationNodeKind = 'IdentifierNode' | 'SchemableIdentifierNode' | 'RawNode' | 'SelectQueryNode' | 'SelectionNode' | 'ReferenceNode' | 'ColumnNode' | 'TableNode' | 'AliasNode' | 'FromNode' | 'SelectAllNode' | 'AndNode' | 'OrNode' | 'ParensNode' | 'ValueNode' | 'ValueListNode' | 'PrimitiveValueListNode' | 'JoinNode' | 'OperatorNode' | 'WhereNode' | 'InsertQueryNode' | 'DeleteQueryNode' | 'ReturningNode' | 'CreateTableNode' | 'ColumnDefinitionNode' | 'AddColumnNode' | 'DropTableNode' | 'DataTypeNode' | 'OrderByNode' | 'OrderByItemNode' | 'GroupByNode' | 'GroupByItemNode' | 'UpdateQueryNode' | 'ColumnUpdateNode' | 'LimitNode' | 'OffsetNode' | 'OnConflictNode' | 'OnDuplicateKeyNode' | 'CreateIndexNode' | 'DropIndexNode' | 'ListNode' | 'ReferencesNode' | 'PrimaryKeyConstraintNode' | 'UniqueConstraintNode' | 'CheckConstraintNode' | 'ForeignKeyConstraintNode' | 'WithNode' | 'CommonTableExpressionNode' | 'HavingNode' | 'CreateSchemaNode' | 'DropSchemaNode' | 'AlterTableNode' | 'ModifyColumnNode' | 'DropColumnNode' | 'RenameColumnNode' | 'AlterColumnNode' | 'AddConstraintNode' | 'DropConstraintNode' | 'CreateViewNode' | 'RefreshMaterializedViewNode' | 'DropViewNode' | 'GeneratedNode' | 'DefaultValueNode' | 'OnNode' | 'ValuesNode' | 'CommonTableExpressionNameNode' | 'SelectModifierNode' | 'CreateTypeNode' | 'DropTypeNode' | 'ExplainNode' | 'DefaultInsertValueNode' | 'AggregateFunctionNode' | 'OverNode' | 'PartitionByNode' | 'PartitionByItemNode' | 'SetOperationNode' | 'BinaryOperationNode' | 'UnaryOperationNode' | 'UsingNode' | 'FunctionNode' | 'CaseNode' | 'WhenNode' | 'JSONReferenceNode' | 'JSONPathNode' | 'JSONPathLegNode' | 'JSONOperatorChainNode' | 'TupleNode' | 'MergeQueryNode' | 'MatchedNode' | 'AddIndexNode' | 'CastNode' | 'FetchNode' | 'TopNode' | 'OutputNode' | 'OrActionNode' | 'CollateNode' | 'RenameConstraintNode' | 'AlterTypeNode' | 'AddValueNode' | 'RenameValueNode';
interface OperationNode {
    readonly kind: OperationNodeKind;
}
type IdentifierNodeFactory = Readonly<{
    is(node: OperationNode): node is IdentifierNode;
    create(name: string): Readonly<IdentifierNode>;
}>;
interface IdentifierNode extends OperationNode {
    readonly kind: 'IdentifierNode';
    readonly name: string;
}
declare const IdentifierNode: IdentifierNodeFactory;
type CheckConstraintNodeFactory = Readonly<{
    is(node: OperationNode): node is CheckConstraintNode;
    create(expression: OperationNode, constraintName?: string): Readonly<CheckConstraintNode>;
}>;
interface CheckConstraintNode extends OperationNode {
    readonly kind: 'CheckConstraintNode';
    readonly expression: OperationNode;
    readonly name?: IdentifierNode;
}
declare const CheckConstraintNode: CheckConstraintNodeFactory;
type ColumnNodeFactory = Readonly<{
    is(node: OperationNode): node is ColumnNode;
    create(column: string): Readonly<ColumnNode>;
}>;
interface ColumnNode extends OperationNode {
    readonly kind: 'ColumnNode';
    readonly column: IdentifierNode;
}
declare const ColumnNode: ColumnNodeFactory;
type DefaultValueNodeFactory = Readonly<{
    is(node: OperationNode): node is DefaultValueNode;
    create(defaultValue: OperationNode): Readonly<DefaultValueNode>;
}>;
interface DefaultValueNode extends OperationNode {
    readonly kind: 'DefaultValueNode';
    readonly defaultValue: OperationNode;
}
declare const DefaultValueNode: DefaultValueNodeFactory;
type GeneratedNodeParams = Omit<GeneratedNode, 'kind' | 'expression'>;
type GeneratedNodeFactory = Readonly<{
    is(node: OperationNode): node is GeneratedNode;
    create(params: GeneratedNodeParams): Readonly<GeneratedNode>;
    createWithExpression(expression: OperationNode): Readonly<GeneratedNode>;
    cloneWith(node: GeneratedNode, params: GeneratedNodeParams): Readonly<GeneratedNode>;
}>;
interface GeneratedNode extends OperationNode {
    readonly kind: 'GeneratedNode';
    readonly byDefault?: boolean;
    readonly always?: boolean;
    readonly identity?: boolean;
    readonly stored?: boolean;
    readonly expression?: OperationNode;
}
declare const GeneratedNode: GeneratedNodeFactory;
type SchemableIdentifierNodeFactory = Readonly<{
    is(node: OperationNode): node is SchemableIdentifierNode;
    create(identifier: string): Readonly<SchemableIdentifierNode>;
    createWithSchema(schema: string, identifier: string): Readonly<SchemableIdentifierNode>;
}>;
interface SchemableIdentifierNode extends OperationNode {
    readonly kind: 'SchemableIdentifierNode';
    readonly schema?: IdentifierNode;
    readonly identifier: IdentifierNode;
}
declare const SchemableIdentifierNode: SchemableIdentifierNodeFactory;
type TableNodeFactory = Readonly<{
    is(node: OperationNode): node is TableNode;
    create(table: string): Readonly<TableNode>;
    createWithSchema(schema: string, table: string): Readonly<TableNode>;
}>;
interface TableNode extends OperationNode {
    readonly kind: 'TableNode';
    readonly table: SchemableIdentifierNode;
}
declare const TableNode: TableNodeFactory;
type OnModifyForeignAction = 'cascade' | 'no action' | 'restrict' | 'set default' | 'set null';
type ReferencesNodeFactory = Readonly<{
    is(node: OperationNode): node is ReferencesNode;
    create(table: TableNode, columns: ReadonlyArray<ColumnNode>): Readonly<ReferencesNode>;
    cloneWithOnDelete(references: ReferencesNode, onDelete: OnModifyForeignAction): Readonly<ReferencesNode>;
    cloneWithOnUpdate(references: ReferencesNode, onUpdate: OnModifyForeignAction): Readonly<ReferencesNode>;
}>;
interface ReferencesNode extends OperationNode {
    readonly kind: 'ReferencesNode';
    readonly table: TableNode;
    readonly columns: ReadonlyArray<ColumnNode>;
    readonly onDelete?: OnModifyForeignAction;
    readonly onUpdate?: OnModifyForeignAction;
}
declare const ReferencesNode: ReferencesNodeFactory;
type ColumnDefinitionNodeProps = Omit<Partial<ColumnDefinitionNode>, 'kind' | 'dataType'>;
type ColumnDefinitionNodeFactory = Readonly<{
    is(node: OperationNode): node is ColumnDefinitionNode;
    create(column: string, dataType: OperationNode): Readonly<ColumnDefinitionNode>;
    cloneWithFrontModifier(node: ColumnDefinitionNode, modifier: OperationNode): Readonly<ColumnDefinitionNode>;
    cloneWithEndModifier(node: ColumnDefinitionNode, modifier: OperationNode): Readonly<ColumnDefinitionNode>;
    cloneWith(node: ColumnDefinitionNode, props: ColumnDefinitionNodeProps): Readonly<ColumnDefinitionNode>;
}>;
interface ColumnDefinitionNode extends OperationNode {
    readonly kind: 'ColumnDefinitionNode';
    readonly column: ColumnNode;
    readonly dataType: OperationNode;
    readonly references?: ReferencesNode;
    readonly primaryKey?: boolean;
    readonly autoIncrement?: boolean;
    readonly unique?: boolean;
    readonly notNull?: boolean;
    readonly defaultTo?: DefaultValueNode;
    readonly check?: CheckConstraintNode;
    readonly generated?: GeneratedNode;
    readonly unsigned?: boolean;
    readonly frontModifiers?: ReadonlyArray<OperationNode>;
    readonly endModifiers?: ReadonlyArray<OperationNode>;
    readonly nullsNotDistinct?: boolean;
    readonly identity?: boolean;
    readonly ifNotExists?: boolean;
}
declare const ColumnDefinitionNode: ColumnDefinitionNodeFactory;
type AddColumnNodeFactory = Readonly<{
    is(node: OperationNode): node is AddColumnNode;
    create(column: ColumnDefinitionNode): Readonly<AddColumnNode>;
}>;
interface AddColumnNode extends OperationNode {
    readonly kind: 'AddColumnNode';
    readonly column: ColumnDefinitionNode;
}
declare const AddColumnNode: AddColumnNodeFactory;
type DropColumnNodeProps = Omit<DropColumnNode, 'kind' | 'column'>;
type DropColumnNodeFactory = Readonly<{
    is(node: OperationNode): node is DropColumnNode;
    create(column: string): Readonly<DropColumnNode>;
    cloneWith(node: DropColumnNode, props: DropColumnNodeProps): Readonly<DropColumnNode>;
}>;
interface DropColumnNode extends OperationNode {
    readonly kind: 'DropColumnNode';
    readonly column: ColumnNode;
    readonly ifExists?: boolean;
}
declare const DropColumnNode: DropColumnNodeFactory;
type RenameColumnNodeFactory = Readonly<{
    is(node: OperationNode): node is RenameColumnNode;
    create(column: string, newColumn: string): Readonly<RenameColumnNode>;
}>;
interface RenameColumnNode extends OperationNode {
    readonly kind: 'RenameColumnNode';
    readonly column: ColumnNode;
    readonly renameTo: ColumnNode;
}
declare const RenameColumnNode: RenameColumnNodeFactory;
type RawNodeFactory = Readonly<{
    is(node: OperationNode): node is RawNode;
    create(sqlFragments: ReadonlyArray<string>, parameters: ReadonlyArray<OperationNode>): Readonly<RawNode>;
    createWithSql(sql: string): Readonly<RawNode>;
    createWithChild(child: OperationNode): Readonly<RawNode>;
    createWithChildren(children: ReadonlyArray<OperationNode>): Readonly<RawNode>;
}>;
interface RawNode extends OperationNode {
    readonly kind: 'RawNode';
    readonly sqlFragments: ReadonlyArray<string>;
    readonly parameters: ReadonlyArray<OperationNode>;
}
declare const RawNode: RawNodeFactory;
type AlterColumnNodeProps = Omit<AlterColumnNode, 'kind' | 'column'>;
type AlterColumnNodeFactory = Readonly<{
    is(node: OperationNode): node is AlterColumnNode;
    create<T extends keyof AlterColumnNodeProps>(column: string, prop: T, value: Required<AlterColumnNodeProps>[T]): Readonly<AlterColumnNode>;
}>;
interface AlterColumnNode extends OperationNode {
    readonly kind: 'AlterColumnNode';
    readonly column: ColumnNode;
    readonly dataType?: OperationNode;
    readonly dataTypeExpression?: RawNode;
    readonly setDefault?: OperationNode;
    readonly dropDefault?: true;
    readonly setNotNull?: true;
    readonly dropNotNull?: true;
}
declare const AlterColumnNode: AlterColumnNodeFactory;
type ForeignKeyConstraintNodeProps = Omit<ForeignKeyConstraintNode, 'kind' | 'columns' | 'references'>;
type ForeignKeyConstraintNodeFactory = Readonly<{
    is(node: OperationNode): node is ForeignKeyConstraintNode;
    create(sourceColumns: ReadonlyArray<ColumnNode>, targetTable: TableNode, targetColumns: ReadonlyArray<ColumnNode>, constraintName?: string): Readonly<ForeignKeyConstraintNode>;
    cloneWith(node: ForeignKeyConstraintNode, props: ForeignKeyConstraintNodeProps): Readonly<ForeignKeyConstraintNode>;
}>;
interface ForeignKeyConstraintNode extends OperationNode {
    readonly kind: 'ForeignKeyConstraintNode';
    readonly columns: ReadonlyArray<ColumnNode>;
    readonly references: ReferencesNode;
    readonly onDelete?: OnModifyForeignAction;
    readonly onUpdate?: OnModifyForeignAction;
    readonly name?: IdentifierNode;
    readonly deferrable?: boolean;
    readonly initiallyDeferred?: boolean;
}
declare const ForeignKeyConstraintNode: ForeignKeyConstraintNodeFactory;
type PrimaryKeyConstraintNodeProps = Omit<Partial<PrimaryKeyConstraintNode>, 'kind'>;
type PrimaryKeyConstraintNodeFactory = Readonly<{
    is(node: OperationNode): node is PrimaryKeyConstraintNode;
    create(columns: string[], constraintName?: string): Readonly<PrimaryKeyConstraintNode>;
    cloneWith(node: PrimaryKeyConstraintNode, props: PrimaryKeyConstraintNodeProps): Readonly<PrimaryKeyConstraintNode>;
}>;
interface PrimaryKeyConstraintNode extends OperationNode {
    readonly kind: 'PrimaryKeyConstraintNode';
    readonly columns: ReadonlyArray<ColumnNode>;
    readonly name?: IdentifierNode;
    readonly deferrable?: boolean;
    readonly initiallyDeferred?: boolean;
}
declare const PrimaryKeyConstraintNode: PrimaryKeyConstraintNodeFactory;
type UniqueConstraintNodeProps = Omit<Partial<UniqueConstraintNode>, 'kind'>;
type UniqueConstraintNodeFactory = Readonly<{
    is(node: OperationNode): node is UniqueConstraintNode;
    create(columns: OperationNode[], constraintName?: string, nullsNotDistinct?: boolean): UniqueConstraintNode;
    create(columns: string[], constraintName?: string, nullsNotDistinct?: boolean): UniqueConstraintNode;
    cloneWith(node: UniqueConstraintNode, props: UniqueConstraintNodeProps): UniqueConstraintNode;
}>;
interface UniqueConstraintNode extends OperationNode {
    readonly kind: 'UniqueConstraintNode';
    readonly columns: ReadonlyArray<OperationNode>;
    readonly name?: IdentifierNode;
    readonly nullsNotDistinct?: boolean;
    readonly deferrable?: boolean;
    readonly initiallyDeferred?: boolean;
}
declare const UniqueConstraintNode: UniqueConstraintNodeFactory;
type ConstraintNode = PrimaryKeyConstraintNode | UniqueConstraintNode | CheckConstraintNode | ForeignKeyConstraintNode;
type AddConstraintNodeFactory = Readonly<{
    is(node: OperationNode): node is AddConstraintNode;
    create(constraint: ConstraintNode): Readonly<AddConstraintNode>;
}>;
interface AddConstraintNode extends OperationNode {
    readonly kind: 'AddConstraintNode';
    readonly constraint: ConstraintNode;
}
declare const AddConstraintNode: AddConstraintNodeFactory;
type DropConstraintNodeProps = Omit<DropConstraintNode, 'kind' | 'constraintName'>;
type DropConstraintNodeFactory = Readonly<{
    is(node: OperationNode): node is DropConstraintNode;
    create(constraintName: string, params?: DropConstraintNodeProps): Readonly<DropConstraintNode>;
    cloneWith(dropConstraint: DropConstraintNode, props: DropConstraintNodeProps): Readonly<DropConstraintNode>;
}>;
interface DropConstraintNode extends OperationNode {
    readonly kind: 'DropConstraintNode';
    readonly constraintName: IdentifierNode;
    readonly ifExists?: boolean;
    readonly modifier?: 'cascade' | 'restrict';
}
declare const DropConstraintNode: DropConstraintNodeFactory;
type ModifyColumnNodeFactory = Readonly<{
    is(node: OperationNode): node is ModifyColumnNode;
    create(column: ColumnDefinitionNode): Readonly<ModifyColumnNode>;
}>;
interface ModifyColumnNode extends OperationNode {
    readonly kind: 'ModifyColumnNode';
    readonly column: ColumnDefinitionNode;
}
declare const ModifyColumnNode: ModifyColumnNodeFactory;
type DropIndexNodeProps = Omit<DropIndexNode, 'kind' | 'name'>;
type DropIndexNodeFactory = Readonly<{
    is(node: OperationNode): node is DropIndexNode;
    create(name: string, params?: DropIndexNodeProps): Readonly<DropIndexNode>;
    cloneWith(dropIndex: DropIndexNode, props: DropIndexNodeProps): Readonly<DropIndexNode>;
}>;
interface DropIndexNode extends OperationNode {
    readonly kind: 'DropIndexNode';
    readonly name: SchemableIdentifierNode;
    readonly table?: TableNode;
    readonly ifExists?: boolean;
    readonly cascade?: boolean;
}
declare const DropIndexNode: DropIndexNodeFactory;
type AddIndexNodeFactory = Readonly<{
    is(node: OperationNode): node is AddIndexNode;
    create(name: string): Readonly<AddIndexNode>;
    cloneWith(node: AddIndexNode, props: Required<Pick<AddIndexNode, 'ifNotExists'>>): Readonly<AddIndexNode>;
    cloneWith(node: AddIndexNode, props: Omit<AddIndexNode, 'kind' | 'name' | 'ifNotExists'>): Readonly<AddIndexNode>;
    cloneWithColumns(node: AddIndexNode, columns: OperationNode[]): Readonly<AddIndexNode>;
}>;
interface AddIndexNode extends OperationNode {
    readonly kind: 'AddIndexNode';
    readonly name: IdentifierNode;
    readonly columns?: OperationNode[];
    readonly unique?: boolean;
    readonly using?: RawNode;
    readonly ifNotExists?: boolean;
}
declare const AddIndexNode: AddIndexNodeFactory;
type RenameConstraintNodeFactory = Readonly<{
    is(node: OperationNode): node is RenameConstraintNode;
    create(oldName: string, newName: string): Readonly<RenameConstraintNode>;
}>;
interface RenameConstraintNode extends OperationNode {
    readonly kind: 'RenameConstraintNode';
    readonly oldName: IdentifierNode;
    readonly newName: IdentifierNode;
}
declare const RenameConstraintNode: RenameConstraintNodeFactory;
type AlterTableNodeTableProps = Pick<AlterTableNode, 'renameTo' | 'setSchema' | 'addConstraint' | 'dropConstraint' | 'addIndex' | 'dropIndex' | 'renameConstraint'>;
type AlterTableColumnAlterationNode = RenameColumnNode | AddColumnNode | DropColumnNode | AlterColumnNode | ModifyColumnNode;
type AlterTableNodeFactory = Readonly<{
    is(node: OperationNode): node is AlterTableNode;
    create(table: TableNode): Readonly<AlterTableNode>;
    cloneWithTableProps(node: AlterTableNode, props: AlterTableNodeTableProps): Readonly<AlterTableNode>;
    cloneWithColumnAlteration(node: AlterTableNode, columnAlteration: AlterTableColumnAlterationNode): Readonly<AlterTableNode>;
}>;
interface AlterTableNode extends OperationNode {
    readonly kind: 'AlterTableNode';
    readonly table: TableNode;
    readonly renameTo?: TableNode;
    readonly setSchema?: IdentifierNode;
    readonly columnAlterations?: ReadonlyArray<AlterTableColumnAlterationNode>;
    readonly addConstraint?: AddConstraintNode;
    readonly dropConstraint?: DropConstraintNode;
    readonly renameConstraint?: RenameConstraintNode;
    readonly addIndex?: AddIndexNode;
    readonly dropIndex?: DropIndexNode;
}
declare const AlterTableNode: AlterTableNodeFactory;
type ValueNodeFactory = Readonly<{
    is(node: OperationNode): node is ValueNode;
    create(value: unknown): Readonly<ValueNode>;
    createImmediate(value: unknown): Readonly<ValueNode>;
}>;
interface ValueNode extends OperationNode {
    readonly kind: 'ValueNode';
    readonly value: unknown;
    readonly immediate?: boolean;
}
declare const ValueNode: ValueNodeFactory;
type AddValueNodeProps = Omit<AddValueNode, 'kind' | 'value'>;
type AddValueNodeFactory = Readonly<{
    is(node: OperationNode): node is AddValueNode;
    create(value: ValueNode): Readonly<AddValueNode>;
    cloneWith(node: AddValueNode, props: AddValueNodeProps): Readonly<AddValueNode>;
}>;
interface AddValueNode extends OperationNode {
    readonly kind: 'AddValueNode';
    readonly value: ValueNode;
    readonly ifNotExists?: boolean;
    readonly neighborValue?: ValueNode;
    readonly isBefore?: boolean;
}
declare const AddValueNode: AddValueNodeFactory;
type RenameValueNodeFactory = Readonly<{
    is(node: OperationNode): node is RenameValueNode;
    create(existingEnumValue: ValueNode, newEnumValue: ValueNode): Readonly<RenameValueNode>;
}>;
interface RenameValueNode extends OperationNode {
    readonly kind: 'RenameValueNode';
    readonly oldValue: ValueNode;
    readonly newValue: ValueNode;
}
declare const RenameValueNode: RenameValueNodeFactory;
type AlterTypeNodeProps = Omit<AlterTypeNode, 'kind' | 'name'>;
type AlterTypeNodeFactory = Readonly<{
    is(node: OperationNode): node is AlterTypeNode;
    create(name: SchemableIdentifierNode): Readonly<AlterTypeNode>;
    cloneWith(node: AlterTypeNode, props: AlterTypeNodeProps): Readonly<AlterTypeNode>;
}>;
interface AlterTypeNode extends OperationNode {
    readonly kind: 'AlterTypeNode';
    readonly name: SchemableIdentifierNode;
    readonly addValue?: AddValueNode;
    readonly renameTo?: IdentifierNode;
    readonly renameValue?: RenameValueNode;
    readonly setSchema?: IdentifierNode;
}
declare const AlterTypeNode: AlterTypeNodeFactory;
type WhereNodeFactory = Readonly<{
    is(node: OperationNode): node is WhereNode;
    create(filter: OperationNode): Readonly<WhereNode>;
    cloneWithOperation(whereNode: WhereNode, operator: 'And' | 'Or', operation: OperationNode): Readonly<WhereNode>;
}>;
interface WhereNode extends OperationNode {
    readonly kind: 'WhereNode';
    readonly where: OperationNode;
}
declare const WhereNode: WhereNodeFactory;
type CreateIndexNodeProps = Omit<CreateIndexNode, 'kind' | 'name'>;
type IndexType = 'btree' | 'hash' | 'gist' | 'gin';
type CreateIndexNodeFactory = Readonly<{
    is(node: OperationNode): node is CreateIndexNode;
    create(name: string): Readonly<CreateIndexNode>;
    cloneWith(node: CreateIndexNode, props: CreateIndexNodeProps): Readonly<CreateIndexNode>;
    cloneWithColumns(node: CreateIndexNode, columns: OperationNode[]): Readonly<CreateIndexNode>;
}>;
interface CreateIndexNode extends OperationNode {
    readonly kind: 'CreateIndexNode';
    readonly name: IdentifierNode;
    readonly table?: TableNode;
    readonly columns?: OperationNode[];
    readonly unique?: boolean;
    readonly using?: RawNode;
    readonly ifNotExists?: boolean;
    readonly where?: WhereNode;
    readonly nullsNotDistinct?: boolean;
}
declare const CreateIndexNode: CreateIndexNodeFactory;
type CreateSchemaNodeParams = Omit<Partial<CreateSchemaNode>, 'kind' | 'schema'>;
type CreateSchemaNodeFactory = Readonly<{
    is(node: OperationNode): node is CreateSchemaNode;
    create(schema: string, params?: CreateSchemaNodeParams): Readonly<CreateSchemaNode>;
    cloneWith(createSchema: CreateSchemaNode, params: CreateSchemaNodeParams): Readonly<CreateSchemaNode>;
}>;
interface CreateSchemaNode extends OperationNode {
    readonly kind: 'CreateSchemaNode';
    readonly schema: IdentifierNode;
    readonly ifNotExists?: boolean;
}
declare const CreateSchemaNode: CreateSchemaNodeFactory;
declare class DeleteResult {
    readonly numDeletedRows: bigint;
    constructor(numDeletedRows: bigint);
}
declare class InsertResult {
    readonly insertId: bigint | undefined;
    readonly numInsertedOrUpdatedRows: bigint | undefined;
    constructor(insertId: bigint | undefined, numInsertedOrUpdatedRows: bigint | undefined);
}
declare class MergeResult {
    readonly numChangedRows: bigint | undefined;
    constructor(numChangedRows: bigint | undefined);
}
declare class UpdateResult {
    readonly numUpdatedRows: bigint;
    readonly numChangedRows?: bigint;
    constructor(numUpdatedRows: bigint, numChangedRows: bigint | undefined);
}
type AnyColumn<DB, TB extends keyof DB> = {
    [T in TB]: keyof DB[T];
}[TB] & string;
type ExtractColumnType<DB, TB extends keyof DB, C> = {
    [T in TB]: C extends keyof DB[T] ? DB[T][C] : never;
}[TB];
type AnyColumnWithTable<DB, TB extends keyof DB> = {
    [T in TB]: `${T & string}.${keyof DB[T] & string}`;
}[TB];
type AnyAliasedColumn<DB, TB extends keyof DB> = `${AnyColumn<DB, TB>} as ${string}`;
type AnyAliasedColumnWithTable<DB, TB extends keyof DB> = `${AnyColumnWithTable<DB, TB>} as ${string}`;
type ArrayItemType<T> = T extends ReadonlyArray<infer I> ? I : never;
type SimplifySingleResult<O> = O extends InsertResult | UpdateResult | DeleteResult | MergeResult ? O : Simplify<O> | undefined;
type SimplifyResult<O> = O extends InsertResult | UpdateResult | DeleteResult | MergeResult ? O : Simplify<O>;
type Simplify<T> = DrainOuterGeneric<{
    [K in keyof T]: T[K];
} & {}>;
type UnknownRow = Record<string, unknown>;
type Nullable<T> = {
    [P in keyof T]: T[P] | null;
};
type IsNever<T> = [
    T
] extends [
    never
] ? true : false;
type IsNullable<T> = [
    T
] extends [
    NonNullable<T>
] ? false : true;
type NarrowPartial<O, T> = T extends object ? DrainOuterGeneric<{
    [K in keyof O & string]: K extends keyof T ? T[K] extends NotNull$1 ? Exclude<O[K], null> : T[K] extends O[K] ? T[K] : T[K] extends object ? SimplifyDeep<O[K] & NarrowPartial<O[K], T[K]>> : KyselyTypeError<`$narrowType() call failed: passed type does not exist in '${K}'s type union`> : O[K];
}> : never;
type SimplifyDeep<T> = T extends object ? T extends Date | RegExp | Map<any, any> | Set<any> ? T : DrainOuterGeneric<{
    [K in keyof T]: SimplifyDeep<T[K]>;
} & {}> : T;
type NotNull$1 = {
    readonly __notNull__: unique symbol;
};
type SqlBool = boolean | 0 | 1;
type DrainOuterGeneric<T> = [
    T
] extends [
    unknown
] ? T : never;
type ShallowRecord<K extends keyof any, T> = DrainOuterGeneric<{
    [P in K]: T;
}>;
type ShallowDehydrateObject<O> = {
    [K in keyof O]: ShallowDehydrateValue<O[K]>;
};
type ShallowDehydrateValue<T> = T extends null | undefined ? T : '__kysely_dehydrate__' extends keyof T & {} ? T : T & {} extends (infer U)[] ? Array<ShallowDehydrateValue<U>> | Extract<T, null | undefined> : Exclude<T, StringsWhenDataTypeNotAvailable | NumbersWhenDataTypeNotAvailable> | (IsNever<Extract<T, NumbersWhenDataTypeNotAvailable>> extends true ? never : number) | (IsNever<Extract<T, StringsWhenDataTypeNotAvailable>> extends true ? never : string);
type StringsWhenDataTypeNotAvailable = Date | Uint8Array;
type NumbersWhenDataTypeNotAvailable = bigint | NumericString;
type NumericString = `${number}`;
declare const ON_COMMIT_ACTIONS: string[];
type OnCommitAction = ArrayItemType<typeof ON_COMMIT_ACTIONS>;
type CreateTableNodeParams = Omit<CreateTableNode, 'kind' | 'table' | 'columns' | 'constraints' | 'indexes' | 'frontModifiers' | 'endModifiers'>;
type CreateTableNodeFactory = Readonly<{
    is(node: OperationNode): node is CreateTableNode;
    create(table: TableNode): Readonly<CreateTableNode>;
    cloneWithColumn(node: CreateTableNode, column: ColumnDefinitionNode): Readonly<CreateTableNode>;
    cloneWithConstraint(node: CreateTableNode, constraint: ConstraintNode): Readonly<CreateTableNode>;
    cloneWithIndex(node: CreateTableNode, index: AddIndexNode): Readonly<CreateTableNode>;
    cloneWithFrontModifier(node: CreateTableNode, modifier: OperationNode): Readonly<CreateTableNode>;
    cloneWithEndModifier(node: CreateTableNode, modifier: OperationNode): Readonly<CreateTableNode>;
    cloneWith(node: CreateTableNode, params: CreateTableNodeParams): Readonly<CreateTableNode>;
}>;
interface CreateTableNode extends OperationNode {
    readonly kind: 'CreateTableNode';
    readonly table: TableNode;
    readonly columns: ReadonlyArray<ColumnDefinitionNode>;
    readonly constraints?: ReadonlyArray<ConstraintNode>;
    readonly indexes?: ReadonlyArray<AddIndexNode>;
    readonly temporary?: boolean;
    readonly ifNotExists?: boolean;
    readonly onCommit?: OnCommitAction;
    readonly frontModifiers?: ReadonlyArray<OperationNode>;
    readonly endModifiers?: ReadonlyArray<OperationNode>;
    readonly selectQuery?: OperationNode;
}
declare const CreateTableNode: CreateTableNodeFactory;
type ValueListNodeFactory = Readonly<{
    is(node: OperationNode): node is ValueListNode;
    create(values: ReadonlyArray<OperationNode>): Readonly<ValueListNode>;
}>;
interface ValueListNode extends OperationNode {
    readonly kind: 'ValueListNode';
    readonly values: ReadonlyArray<OperationNode>;
}
declare const ValueListNode: ValueListNodeFactory;
type CreateTypeNodeFactory = Readonly<{
    is(node: OperationNode): node is CreateTypeNode;
    create(name: SchemableIdentifierNode): Readonly<CreateTypeNode>;
    cloneWithEnum(createType: CreateTypeNode, values: readonly string[]): Readonly<CreateTypeNode>;
}>;
interface CreateTypeNode extends OperationNode {
    readonly kind: 'CreateTypeNode';
    readonly name: SchemableIdentifierNode;
    readonly enum?: ValueListNode;
}
declare const CreateTypeNode: CreateTypeNodeFactory;
type FromNodeFactory = Readonly<{
    is(node: OperationNode): node is FromNode;
    create(froms: ReadonlyArray<OperationNode>): Readonly<FromNode>;
    cloneWithFroms(from: FromNode, froms: ReadonlyArray<OperationNode>): Readonly<FromNode>;
}>;
interface FromNode extends OperationNode {
    readonly kind: 'FromNode';
    readonly froms: ReadonlyArray<OperationNode>;
}
declare const FromNode: FromNodeFactory;
type GroupByItemNodeFactory = Readonly<{
    is(node: OperationNode): node is GroupByItemNode;
    create(groupBy: OperationNode): Readonly<GroupByItemNode>;
}>;
interface GroupByItemNode extends OperationNode {
    readonly kind: 'GroupByItemNode';
    readonly groupBy: OperationNode;
}
declare const GroupByItemNode: GroupByItemNodeFactory;
type GroupByNodeFactory = Readonly<{
    is(node: OperationNode): node is GroupByNode;
    create(items: ReadonlyArray<GroupByItemNode>): Readonly<GroupByNode>;
    cloneWithItems(groupBy: GroupByNode, items: ReadonlyArray<GroupByItemNode>): Readonly<GroupByNode>;
}>;
interface GroupByNode extends OperationNode {
    readonly kind: 'GroupByNode';
    readonly items: ReadonlyArray<GroupByItemNode>;
}
declare const GroupByNode: GroupByNodeFactory;
type HavingNodeFactory = Readonly<{
    is(node: OperationNode): node is HavingNode;
    create(filter: OperationNode): Readonly<HavingNode>;
    cloneWithOperation(havingNode: HavingNode, operator: 'And' | 'Or', operation: OperationNode): Readonly<HavingNode>;
}>;
interface HavingNode extends OperationNode {
    readonly kind: 'HavingNode';
    readonly having: OperationNode;
}
declare const HavingNode: HavingNodeFactory;
type OnNodeFactory = Readonly<{
    is(node: OperationNode): node is OnNode;
    create(filter: OperationNode): Readonly<OnNode>;
    cloneWithOperation(onNode: OnNode, operator: 'And' | 'Or', operation: OperationNode): Readonly<OnNode>;
}>;
interface OnNode extends OperationNode {
    readonly kind: 'OnNode';
    readonly on: OperationNode;
}
declare const OnNode: OnNodeFactory;
type JoinType = 'InnerJoin' | 'LeftJoin' | 'RightJoin' | 'FullJoin' | 'CrossJoin' | 'LateralInnerJoin' | 'LateralLeftJoin' | 'LateralCrossJoin' | 'Using' | 'OuterApply' | 'CrossApply';
type JoinNodeFactory = Readonly<{
    is(node: OperationNode): node is JoinNode;
    create(joinType: JoinType, table: OperationNode): Readonly<JoinNode>;
    createWithOn(joinType: JoinType, table: OperationNode, on: OperationNode): Readonly<JoinNode>;
    cloneWithOn(joinNode: JoinNode, operation: OperationNode): Readonly<JoinNode>;
}>;
interface JoinNode extends OperationNode {
    readonly kind: 'JoinNode';
    readonly joinType: JoinType;
    readonly table: OperationNode;
    readonly on?: OnNode;
}
declare const JoinNode: JoinNodeFactory;
type LimitNodeFactory = Readonly<{
    is(node: OperationNode): node is LimitNode;
    create(limit: OperationNode): Readonly<LimitNode>;
}>;
interface LimitNode extends OperationNode {
    readonly kind: 'LimitNode';
    readonly limit: OperationNode;
}
declare const LimitNode: LimitNodeFactory;
type OffsetNodeFactory = Readonly<{
    is(node: OperationNode): node is OffsetNode;
    create(offset: OperationNode): Readonly<OffsetNode>;
}>;
interface OffsetNode extends OperationNode {
    readonly kind: 'OffsetNode';
    readonly offset: OperationNode;
}
declare const OffsetNode: OffsetNodeFactory;
type CollateNodeFactory = Readonly<{
    is(node: OperationNode): node is CollateNode;
    create(collation: string): Readonly<CollateNode>;
}>;
interface CollateNode extends OperationNode {
    readonly kind: 'CollateNode';
    readonly collation: IdentifierNode;
}
declare const CollateNode: CollateNodeFactory;
type OrderByItemNodeProps = Omit<OrderByItemNode, 'kind' | 'orderBy'>;
type OrderByItemNodeFactory = Readonly<{
    is(node: OperationNode): node is OrderByItemNode;
    create(orderBy: OperationNode, direction?: OperationNode): Readonly<OrderByItemNode>;
    cloneWith(node: OrderByItemNode, props: OrderByItemNodeProps): Readonly<OrderByItemNode>;
}>;
interface OrderByItemNode extends OperationNode {
    readonly kind: 'OrderByItemNode';
    readonly orderBy: OperationNode;
    readonly direction?: OperationNode;
    readonly nulls?: 'first' | 'last';
    readonly collation?: CollateNode;
}
declare const OrderByItemNode: OrderByItemNodeFactory;
type OrderByNodeFactory = Readonly<{
    is(node: OperationNode): node is OrderByNode;
    create(items: ReadonlyArray<OrderByItemNode>): Readonly<OrderByNode>;
    cloneWithItems(orderBy: OrderByNode, items: ReadonlyArray<OrderByItemNode>): Readonly<OrderByNode>;
}>;
interface OrderByNode extends OperationNode {
    readonly kind: 'OrderByNode';
    readonly items: ReadonlyArray<OrderByItemNode>;
}
declare const OrderByNode: OrderByNodeFactory;
type AliasNodeFactory = Readonly<{
    is(node: OperationNode): node is AliasNode;
    create(node: OperationNode, alias: OperationNode): Readonly<AliasNode>;
}>;
interface AliasNode extends OperationNode {
    readonly kind: 'AliasNode';
    readonly node: OperationNode;
    readonly alias: OperationNode;
}
declare const AliasNode: AliasNodeFactory;
type SelectAllNodeFactory = Readonly<{
    is(node: OperationNode): node is SelectAllNode;
    create(): Readonly<SelectAllNode>;
}>;
interface SelectAllNode extends OperationNode {
    readonly kind: 'SelectAllNode';
}
declare const SelectAllNode: SelectAllNodeFactory;
type ReferenceNodeFactory = Readonly<{
    is(node: OperationNode): node is ReferenceNode;
    create(column: ColumnNode, table?: TableNode): Readonly<ReferenceNode>;
    createSelectAll(table: TableNode): Readonly<ReferenceNode>;
}>;
interface ReferenceNode extends OperationNode {
    readonly kind: 'ReferenceNode';
    readonly column: ColumnNode | SelectAllNode;
    readonly table?: TableNode;
}
declare const ReferenceNode: ReferenceNodeFactory;
type SimpleReferenceExpressionNode = ColumnNode | ReferenceNode;
type SelectionNodeChild = SimpleReferenceExpressionNode | AliasNode | SelectAllNode;
type SelectionNodeFactory = Readonly<{
    is(node: OperationNode): node is SelectionNode;
    create(selection: SelectionNodeChild): Readonly<SelectionNode>;
    createSelectAll(): Readonly<SelectionNode>;
    createSelectAllFromTable(table: TableNode): Readonly<SelectionNode>;
}>;
interface SelectionNode extends OperationNode {
    readonly kind: 'SelectionNode';
    readonly selection: SelectionNodeChild;
}
declare const SelectionNode: SelectionNodeFactory;
type CommonTableExpressionNameNodeFactory = Readonly<{
    is(node: OperationNode): node is CommonTableExpressionNameNode;
    create(tableName: string, columnNames?: ReadonlyArray<string>): Readonly<CommonTableExpressionNameNode>;
}>;
interface CommonTableExpressionNameNode extends OperationNode {
    readonly kind: 'CommonTableExpressionNameNode';
    readonly table: TableNode;
    readonly columns?: ReadonlyArray<ColumnNode>;
}
declare const CommonTableExpressionNameNode: CommonTableExpressionNameNodeFactory;
type CommonTableExpressionNodeProps = Pick<CommonTableExpressionNode, 'materialized'>;
type CommonTableExpressionNodeFactory = Readonly<{
    is(node: OperationNode): node is CommonTableExpressionNode;
    create(name: CommonTableExpressionNameNode, expression: OperationNode): Readonly<CommonTableExpressionNode>;
    cloneWith(node: CommonTableExpressionNode, props: CommonTableExpressionNodeProps): Readonly<CommonTableExpressionNode>;
}>;
interface CommonTableExpressionNode extends OperationNode {
    readonly kind: 'CommonTableExpressionNode';
    readonly name: CommonTableExpressionNameNode;
    readonly materialized?: boolean;
    readonly expression: OperationNode;
}
declare const CommonTableExpressionNode: CommonTableExpressionNodeFactory;
type WithNodeParams = Omit<WithNode, 'kind' | 'expressions'>;
type WithNodeFactory = Readonly<{
    is(node: OperationNode): node is WithNode;
    create(expression: CommonTableExpressionNode, params?: WithNodeParams): Readonly<WithNode>;
    cloneWithExpression(withNode: WithNode, expression: CommonTableExpressionNode): Readonly<WithNode>;
}>;
interface WithNode extends OperationNode {
    readonly kind: 'WithNode';
    readonly expressions: ReadonlyArray<CommonTableExpressionNode>;
    readonly recursive?: boolean;
}
declare const WithNode: WithNodeFactory;
type SelectModifier = 'ForUpdate' | 'ForNoKeyUpdate' | 'ForShare' | 'ForKeyShare' | 'NoWait' | 'SkipLocked' | 'Distinct';
type SelectModifierNodeFactory = Readonly<{
    is(node: OperationNode): node is SelectModifierNode;
    create(modifier: SelectModifier, of?: ReadonlyArray<OperationNode>): Readonly<SelectModifierNode>;
    createWithExpression(modifier: OperationNode): Readonly<SelectModifierNode>;
}>;
interface SelectModifierNode extends OperationNode {
    readonly kind: 'SelectModifierNode';
    readonly modifier?: SelectModifier;
    readonly rawModifier?: OperationNode;
    readonly of?: ReadonlyArray<OperationNode>;
}
declare const SelectModifierNode: SelectModifierNodeFactory;
interface OperationNodeSource {
    toOperationNode(): OperationNode;
}
interface Expression<out T> extends OperationNodeSource {
    get expressionType(): T | undefined;
    toOperationNode(): OperationNode;
}
interface AliasableExpression<out T> extends Expression<T> {
    as<A extends string>(alias: A): AliasedExpression<T, A>;
    as<A extends string>(alias: Expression<any>): AliasedExpression<T, A>;
}
interface AliasedExpression<out T, out A extends string> extends OperationNodeSource {
    get expression(): Expression<T>;
    get alias(): A | Expression<unknown>;
    toOperationNode(): AliasNode;
}
type ExplainFormat = 'text' | 'xml' | 'json' | 'yaml' | 'traditional' | 'tree';
interface Explainable {
    explain<O extends Record<string, any> = Record<string, any>>(format?: ExplainFormat, options?: Expression<any>): Promise<O[]>;
}
type ExplainNodeFactory = Readonly<{
    is(node: OperationNode): node is ExplainNode;
    create(format?: ExplainFormat, options?: OperationNode): Readonly<ExplainNode>;
}>;
interface ExplainNode extends OperationNode {
    readonly kind: 'ExplainNode';
    readonly format?: ExplainFormat;
    readonly options?: OperationNode;
}
declare const ExplainNode: ExplainNodeFactory;
type SetOperator = 'union' | 'intersect' | 'except';
type SetOperationNodeFactory = Readonly<{
    is(node: OperationNode): node is SetOperationNode;
    create(operator: SetOperator, expression: OperationNode, all: boolean): Readonly<SetOperationNode>;
}>;
interface SetOperationNode extends OperationNode {
    kind: 'SetOperationNode';
    operator: SetOperator;
    expression: OperationNode;
    all: boolean;
}
declare const SetOperationNode: SetOperationNodeFactory;
type FetchModifier = 'only' | 'with ties';
type FetchNodeFactory = Readonly<{
    is(node: OperationNode): node is FetchNode;
    create(rowCount: number | bigint, modifier: FetchModifier): Readonly<FetchNode>;
}>;
interface FetchNode extends OperationNode {
    readonly kind: 'FetchNode';
    readonly rowCount: ValueNode;
    readonly modifier: FetchModifier;
}
declare const FetchNode: FetchNodeFactory;
type TopModifier = 'percent' | 'with ties' | 'percent with ties';
type TopNodeFactory = Readonly<{
    is(node: OperationNode): node is TopNode;
    create(expression: number | bigint, modifiers?: TopModifier): Readonly<TopNode>;
}>;
interface TopNode extends OperationNode {
    readonly kind: 'TopNode';
    readonly expression: number | bigint;
    readonly modifiers?: TopModifier;
}
declare const TopNode: TopNodeFactory;
type SelectQueryNodeFactory = Readonly<{
    is(node: OperationNode): node is SelectQueryNode;
    create(withNode?: WithNode): Readonly<SelectQueryNode>;
    createFrom(fromItems: ReadonlyArray<OperationNode>, withNode?: WithNode): Readonly<SelectQueryNode>;
    cloneWithSelections(select: SelectQueryNode, selections: ReadonlyArray<SelectionNode>): Readonly<SelectQueryNode>;
    cloneWithDistinctOn(select: SelectQueryNode, expressions: ReadonlyArray<OperationNode>): Readonly<SelectQueryNode>;
    cloneWithFrontModifier(select: SelectQueryNode, modifier: SelectModifierNode): Readonly<SelectQueryNode>;
    cloneWithOrderByItems(node: SelectQueryNode, items: ReadonlyArray<OrderByItemNode>): Readonly<SelectQueryNode>;
    cloneWithGroupByItems(selectNode: SelectQueryNode, items: ReadonlyArray<GroupByItemNode>): Readonly<SelectQueryNode>;
    cloneWithLimit(selectNode: SelectQueryNode, limit: LimitNode): Readonly<SelectQueryNode>;
    cloneWithOffset(selectNode: SelectQueryNode, offset: OffsetNode): Readonly<SelectQueryNode>;
    cloneWithFetch(selectNode: SelectQueryNode, fetch: FetchNode): Readonly<SelectQueryNode>;
    cloneWithHaving(selectNode: SelectQueryNode, operation: OperationNode): Readonly<SelectQueryNode>;
    cloneWithSetOperations(selectNode: SelectQueryNode, setOperations: ReadonlyArray<SetOperationNode>): Readonly<SelectQueryNode>;
    cloneWithoutSelections(select: SelectQueryNode): Readonly<SelectQueryNode>;
    cloneWithoutLimit(select: SelectQueryNode): Readonly<SelectQueryNode>;
    cloneWithoutOffset(select: SelectQueryNode): Readonly<SelectQueryNode>;
    cloneWithoutOrderBy(node: SelectQueryNode): Readonly<SelectQueryNode>;
    cloneWithoutGroupBy(select: SelectQueryNode): Readonly<SelectQueryNode>;
}>;
interface SelectQueryNode extends OperationNode {
    readonly kind: 'SelectQueryNode';
    readonly from?: FromNode;
    readonly selections?: ReadonlyArray<SelectionNode>;
    readonly distinctOn?: ReadonlyArray<OperationNode>;
    readonly joins?: ReadonlyArray<JoinNode>;
    readonly groupBy?: GroupByNode;
    readonly orderBy?: OrderByNode;
    readonly where?: WhereNode;
    readonly frontModifiers?: ReadonlyArray<SelectModifierNode>;
    readonly endModifiers?: ReadonlyArray<SelectModifierNode>;
    readonly limit?: LimitNode;
    readonly offset?: OffsetNode;
    readonly with?: WithNode;
    readonly having?: HavingNode;
    readonly explain?: ExplainNode;
    readonly setOperations?: ReadonlyArray<SetOperationNode>;
    readonly fetch?: FetchNode;
    readonly top?: TopNode;
}
declare const SelectQueryNode: SelectQueryNodeFactory;
type CreateViewNodeParams = Omit<Partial<CreateViewNode>, 'kind' | 'name'>;
type CreateViewNodeFactory = Readonly<{
    is(node: OperationNode): node is CreateViewNode;
    create(name: string): Readonly<CreateViewNode>;
    cloneWith(createView: CreateViewNode, params: CreateViewNodeParams): Readonly<CreateViewNode>;
}>;
interface CreateViewNode extends OperationNode {
    readonly kind: 'CreateViewNode';
    readonly name: SchemableIdentifierNode;
    readonly temporary?: boolean;
    readonly materialized?: boolean;
    readonly orReplace?: boolean;
    readonly ifNotExists?: boolean;
    readonly columns?: ReadonlyArray<ColumnNode>;
    readonly as?: SelectQueryNode | RawNode;
}
declare const CreateViewNode: CreateViewNodeFactory;
type DropSchemaNodeParams = Omit<Partial<DropSchemaNode>, 'kind' | 'schema'>;
type DropSchemaNodeFactory = Readonly<{
    is(node: OperationNode): node is DropSchemaNode;
    create(schema: string, params?: DropSchemaNodeParams): Readonly<DropSchemaNode>;
    cloneWith(dropSchema: DropSchemaNode, params: DropSchemaNodeParams): Readonly<DropSchemaNode>;
}>;
interface DropSchemaNode extends OperationNode {
    readonly kind: 'DropSchemaNode';
    readonly schema: IdentifierNode;
    readonly ifExists?: boolean;
    readonly cascade?: boolean;
}
declare const DropSchemaNode: DropSchemaNodeFactory;
type DropTableNodeParams = Omit<Partial<DropTableNode>, 'kind' | 'table'>;
type DropTableNodeFactory = Readonly<{
    is(node: OperationNode): node is DropTableNode;
    create(table: TableNode, params?: DropTableNodeParams): Readonly<DropTableNode>;
    cloneWith(dropIndex: DropTableNode, params: DropTableNodeParams): Readonly<DropTableNode>;
}>;
interface DropTableNode extends OperationNode {
    readonly kind: 'DropTableNode';
    readonly table: TableNode;
    readonly ifExists?: boolean;
    readonly cascade?: boolean;
    readonly temporary?: boolean;
}
declare const DropTableNode: DropTableNodeFactory;
type DropTypeNodeParams = Omit<Partial<DropTypeNode>, 'kind' | 'name' | 'additionalNames'>;
type DropTypeNodeFactory = Readonly<{
    is(node: OperationNode): node is DropTypeNode;
    create(names: SchemableIdentifierNode | SchemableIdentifierNode[]): Readonly<DropTypeNode>;
    cloneWith(dropType: DropTypeNode, params: DropTypeNodeParams): Readonly<DropTypeNode>;
}>;
interface DropTypeNode extends OperationNode {
    readonly kind: 'DropTypeNode';
    readonly name: SchemableIdentifierNode;
    readonly additionalNames?: SchemableIdentifierNode[];
    readonly ifExists?: boolean;
    readonly cascade?: boolean;
}
declare const DropTypeNode: DropTypeNodeFactory;
type DropViewNodeParams = Omit<Partial<DropViewNode>, 'kind' | 'name'>;
type DropViewNodeFactory = Readonly<{
    is(node: OperationNode): node is DropViewNode;
    create(name: string): Readonly<DropViewNode>;
    cloneWith(dropView: DropViewNode, params: DropViewNodeParams): Readonly<DropViewNode>;
}>;
interface DropViewNode extends OperationNode {
    readonly kind: 'DropViewNode';
    readonly name: SchemableIdentifierNode;
    readonly ifExists?: boolean;
    readonly materialized?: boolean;
    readonly cascade?: boolean;
}
declare const DropViewNode: DropViewNodeFactory;
type ColumnUpdateNodeFactory = Readonly<{
    is(node: OperationNode): node is ColumnUpdateNode;
    create(column: OperationNode, value: OperationNode): Readonly<ColumnUpdateNode>;
}>;
interface ColumnUpdateNode extends OperationNode {
    readonly kind: 'ColumnUpdateNode';
    readonly column: OperationNode;
    readonly value: OperationNode;
}
declare const ColumnUpdateNode: ColumnUpdateNodeFactory;
type OnConflictNodeProps = Omit<OnConflictNode, 'kind' | 'indexWhere' | 'updateWhere'>;
type OnConflictNodeFactory = Readonly<{
    is(node: OperationNode): node is OnConflictNode;
    create(): Readonly<OnConflictNode>;
    cloneWith(node: OnConflictNode, props: OnConflictNodeProps): Readonly<OnConflictNode>;
    cloneWithIndexWhere(node: OnConflictNode, operation: OperationNode): Readonly<OnConflictNode>;
    cloneWithIndexOrWhere(node: OnConflictNode, operation: OperationNode): Readonly<OnConflictNode>;
    cloneWithUpdateWhere(node: OnConflictNode, operation: OperationNode): Readonly<OnConflictNode>;
    cloneWithUpdateOrWhere(node: OnConflictNode, operation: OperationNode): Readonly<OnConflictNode>;
    cloneWithoutIndexWhere(node: OnConflictNode): Readonly<OnConflictNode>;
    cloneWithoutUpdateWhere(node: OnConflictNode): Readonly<OnConflictNode>;
}>;
interface OnConflictNode extends OperationNode {
    readonly kind: 'OnConflictNode';
    readonly columns?: ReadonlyArray<ColumnNode>;
    readonly constraint?: IdentifierNode;
    readonly indexExpression?: OperationNode;
    readonly indexWhere?: WhereNode;
    readonly updates?: ReadonlyArray<ColumnUpdateNode>;
    readonly updateWhere?: WhereNode;
    readonly doNothing?: boolean;
}
declare const OnConflictNode: OnConflictNodeFactory;
type OnDuplicateKeyNodeFactory = Readonly<{
    is(node: OperationNode): node is OnDuplicateKeyNode;
    create(updates: ReadonlyArray<ColumnUpdateNode>): Readonly<OnDuplicateKeyNode>;
}>;
interface OnDuplicateKeyNode extends OperationNode {
    readonly kind: 'OnDuplicateKeyNode';
    readonly updates: ReadonlyArray<ColumnUpdateNode>;
}
declare const OnDuplicateKeyNode: OnDuplicateKeyNodeFactory;
type OrActionNodeFactory = Readonly<{
    is(node: OperationNode): node is OrActionNode;
    create(action: string): Readonly<OrActionNode>;
}>;
interface OrActionNode extends OperationNode {
    readonly kind: 'OrActionNode';
    readonly action: string;
}
declare const OrActionNode: OrActionNodeFactory;
type OutputNodeFactory = Readonly<{
    is(node: OperationNode): node is OutputNode;
    create(selections: ReadonlyArray<OperationNode>): Readonly<OutputNode>;
    cloneWithSelections(output: OutputNode, selections: ReadonlyArray<OperationNode>): Readonly<OutputNode>;
}>;
interface OutputNode extends OperationNode {
    readonly kind: 'OutputNode';
    readonly selections: ReadonlyArray<OperationNode>;
}
declare const OutputNode: OutputNodeFactory;
type ReturningNodeFactory = Readonly<{
    is(node: OperationNode): node is ReturningNode;
    create(selections: ReadonlyArray<SelectionNode>): Readonly<ReturningNode>;
    cloneWithSelections(returning: ReturningNode, selections: ReadonlyArray<SelectionNode>): Readonly<ReturningNode>;
}>;
interface ReturningNode extends OperationNode {
    readonly kind: 'ReturningNode';
    readonly selections: ReadonlyArray<SelectionNode>;
}
declare const ReturningNode: ReturningNodeFactory;
type InsertQueryNodeProps = Omit<InsertQueryNode, 'kind' | 'into'>;
type InsertQueryNodeFactory = Readonly<{
    is(node: OperationNode): node is InsertQueryNode;
    create(into: TableNode, withNode?: WithNode, replace?: boolean): Readonly<InsertQueryNode>;
    createWithoutInto(): Readonly<InsertQueryNode>;
    cloneWith(insertQuery: InsertQueryNode, props: InsertQueryNodeProps): Readonly<InsertQueryNode>;
}>;
interface InsertQueryNode extends OperationNode {
    readonly kind: 'InsertQueryNode';
    readonly into?: TableNode;
    readonly columns?: ReadonlyArray<ColumnNode>;
    readonly values?: OperationNode;
    readonly returning?: ReturningNode;
    readonly onConflict?: OnConflictNode;
    readonly onDuplicateKey?: OnDuplicateKeyNode;
    readonly with?: WithNode;
    readonly orAction?: OrActionNode;
    readonly replace?: boolean;
    readonly explain?: ExplainNode;
    readonly defaultValues?: boolean;
    readonly endModifiers?: ReadonlyArray<OperationNode>;
    readonly top?: TopNode;
    readonly output?: OutputNode;
}
declare const InsertQueryNode: InsertQueryNodeFactory;
type UpdateQueryNodeFactory = Readonly<{
    is(node: OperationNode): node is UpdateQueryNode;
    create(tables: ReadonlyArray<OperationNode>, withNode?: WithNode): Readonly<UpdateQueryNode>;
    createWithoutTable(): Readonly<UpdateQueryNode>;
    cloneWithFromItems(updateQuery: UpdateQueryNode, fromItems: ReadonlyArray<OperationNode>): Readonly<UpdateQueryNode>;
    cloneWithUpdates(updateQuery: UpdateQueryNode, updates: ReadonlyArray<ColumnUpdateNode>): Readonly<UpdateQueryNode>;
    cloneWithLimit(updateQuery: UpdateQueryNode, limit: LimitNode): Readonly<UpdateQueryNode>;
}>;
interface UpdateQueryNode extends OperationNode {
    readonly kind: 'UpdateQueryNode';
    readonly table?: OperationNode;
    readonly from?: FromNode;
    readonly joins?: ReadonlyArray<JoinNode>;
    readonly where?: WhereNode;
    readonly updates?: ReadonlyArray<ColumnUpdateNode>;
    readonly returning?: ReturningNode;
    readonly with?: WithNode;
    readonly explain?: ExplainNode;
    readonly endModifiers?: ReadonlyArray<OperationNode>;
    readonly limit?: LimitNode;
    readonly top?: TopNode;
    readonly output?: OutputNode;
    readonly orderBy?: OrderByNode;
}
declare const UpdateQueryNode: UpdateQueryNodeFactory;
type UsingNodeFactory = Readonly<{
    is(node: OperationNode): node is UsingNode;
    create(tables: ReadonlyArray<OperationNode>): Readonly<UsingNode>;
    cloneWithTables(using: UsingNode, tables: ReadonlyArray<OperationNode>): Readonly<UsingNode>;
}>;
interface UsingNode extends OperationNode {
    readonly kind: 'UsingNode';
    readonly tables: ReadonlyArray<OperationNode>;
}
declare const UsingNode: UsingNodeFactory;
type DeleteQueryNodeFactory = Readonly<{
    is(node: OperationNode): node is DeleteQueryNode;
    create(fromItems: OperationNode[], withNode?: WithNode): Readonly<DeleteQueryNode>;
    cloneWithOrderByItems(node: DeleteQueryNode, items: ReadonlyArray<OrderByItemNode>): Readonly<DeleteQueryNode>;
    cloneWithoutOrderBy(node: DeleteQueryNode): Readonly<DeleteQueryNode>;
    cloneWithLimit(deleteNode: DeleteQueryNode, limit: LimitNode): Readonly<DeleteQueryNode>;
    cloneWithoutLimit(deleteNode: DeleteQueryNode): Readonly<DeleteQueryNode>;
    cloneWithUsing(deleteNode: DeleteQueryNode, tables: OperationNode[]): Readonly<DeleteQueryNode>;
}>;
interface DeleteQueryNode extends OperationNode {
    readonly kind: 'DeleteQueryNode';
    readonly from: FromNode;
    readonly using?: UsingNode;
    readonly joins?: ReadonlyArray<JoinNode>;
    readonly where?: WhereNode;
    readonly returning?: ReturningNode;
    readonly with?: WithNode;
    readonly orderBy?: OrderByNode;
    readonly limit?: LimitNode;
    readonly explain?: ExplainNode;
    readonly endModifiers?: ReadonlyArray<OperationNode>;
    readonly top?: TopNode;
    readonly output?: OutputNode;
}
declare const DeleteQueryNode: DeleteQueryNodeFactory;
type WhenNodeFactory = Readonly<{
    is(node: OperationNode): node is WhenNode;
    create(condition: OperationNode): Readonly<WhenNode>;
    cloneWithResult(whenNode: WhenNode, result: OperationNode): Readonly<WhenNode>;
}>;
interface WhenNode extends OperationNode {
    readonly kind: 'WhenNode';
    readonly condition: OperationNode;
    readonly result?: OperationNode;
}
declare const WhenNode: WhenNodeFactory;
type MergeQueryNodeFactory = Readonly<{
    is(node: OperationNode): node is MergeQueryNode;
    create(into: TableNode | AliasNode, withNode?: WithNode): Readonly<MergeQueryNode>;
    cloneWithUsing(mergeNode: MergeQueryNode, using: JoinNode): Readonly<MergeQueryNode>;
    cloneWithWhen(mergeNode: MergeQueryNode, when: WhenNode): Readonly<MergeQueryNode>;
    cloneWithThen(mergeNode: MergeQueryNode, then: OperationNode): Readonly<MergeQueryNode>;
}>;
interface MergeQueryNode extends OperationNode {
    readonly kind: 'MergeQueryNode';
    readonly into: TableNode | AliasNode;
    readonly using?: JoinNode;
    readonly whens?: ReadonlyArray<WhenNode>;
    readonly with?: WithNode;
    readonly top?: TopNode;
    readonly returning?: ReturningNode;
    readonly output?: OutputNode;
    readonly endModifiers?: ReadonlyArray<OperationNode>;
}
declare const MergeQueryNode: MergeQueryNodeFactory;
type HasJoins = {
    joins?: ReadonlyArray<JoinNode>;
};
type HasWhere = {
    where?: WhereNode;
};
type HasReturning = {
    returning?: ReturningNode;
};
type HasExplain = {
    explain?: ExplainNode;
};
type HasTop = {
    top?: TopNode;
};
type HasOutput = {
    output?: OutputNode;
};
type HasEndModifiers = {
    endModifiers?: ReadonlyArray<OperationNode>;
};
type HasOrderBy = {
    orderBy?: OrderByNode;
};
type QueryNodeFactory = Readonly<{
    is(node: OperationNode): node is QueryNode;
    cloneWithEndModifier<T extends HasEndModifiers>(node: T, modifier: OperationNode): Readonly<T>;
    cloneWithWhere<T extends HasWhere>(node: T, operation: OperationNode): Readonly<T>;
    cloneWithJoin<T extends HasJoins>(node: T, join: JoinNode): Readonly<T>;
    cloneWithReturning<T extends HasReturning>(node: T, selections: ReadonlyArray<SelectionNode>): Readonly<T>;
    cloneWithoutReturning<T extends HasReturning>(node: T): Readonly<T>;
    cloneWithoutWhere<T extends HasWhere>(node: T): Readonly<T>;
    cloneWithExplain<T extends HasExplain>(node: T, format: ExplainFormat | undefined, options: Expression<any> | undefined): Readonly<T>;
    cloneWithTop<T extends HasTop>(node: T, top: TopNode): Readonly<T>;
    cloneWithOutput<T extends HasOutput>(node: T, selections: ReadonlyArray<SelectionNode>): Readonly<T>;
    cloneWithOrderByItems<T extends HasOrderBy>(node: T, items: ReadonlyArray<OrderByItemNode>): Readonly<T>;
    cloneWithoutOrderBy<T extends HasOrderBy>(node: T): Readonly<T>;
}>;
type QueryNode = SelectQueryNode | InsertQueryNode | UpdateQueryNode | DeleteQueryNode | MergeQueryNode;
declare const QueryNode: QueryNodeFactory;
type RefreshMaterializedViewNodeParams = Omit<Partial<RefreshMaterializedViewNode>, 'kind' | 'name'>;
type RefreshMaterializedViewNodeFactory = Readonly<{
    is(node: OperationNode): node is RefreshMaterializedViewNode;
    create(name: string): Readonly<RefreshMaterializedViewNode>;
    cloneWith(createView: RefreshMaterializedViewNode, params: RefreshMaterializedViewNodeParams): Readonly<RefreshMaterializedViewNode>;
}>;
interface RefreshMaterializedViewNode extends OperationNode {
    readonly kind: 'RefreshMaterializedViewNode';
    readonly name: SchemableIdentifierNode;
    readonly concurrently?: boolean;
    readonly withNoData?: boolean;
}
declare const RefreshMaterializedViewNode: RefreshMaterializedViewNodeFactory;
type RootOperationNode = QueryNode | CreateTableNode | CreateIndexNode | CreateSchemaNode | CreateViewNode | RefreshMaterializedViewNode | DropTableNode | DropIndexNode | DropSchemaNode | DropViewNode | AlterTableNode | RawNode | CreateTypeNode | DropTypeNode | AlterTypeNode;
interface QueryId {
    readonly queryId: string;
}
type CompiledQueryFactory = Readonly<{
    raw(sql: string, parameters?: unknown[]): Readonly<CompiledQuery>;
}>;
interface CompiledQuery<O = unknown> {
    readonly query: RootOperationNode;
    readonly queryId: QueryId;
    readonly sql: string;
    readonly parameters: ReadonlyArray<unknown>;
}
declare const CompiledQuery: CompiledQueryFactory;
interface QueryCompiler {
    compileQuery(node: RootOperationNode, queryId: QueryId): CompiledQuery;
}
interface AbortableOperationOptions {
    readonly signal?: AbortSignal | undefined;
}
interface AbortableQueryOptions extends AbortableOperationOptions {
    readonly inflightQueryAbortStrategy?: InflightQueryAbortStrategy | undefined;
}
type InflightQueryAbortStrategy = 'ignore query' | 'cancel query' | 'kill session';
interface DatabaseConnection {
    cancelQuery?(controlConnectionProvider: ControlConnectionProvider): Promise<void>;
    collectSessionInfo?(): Promise<void>;
    executeQuery<R>(compiledQuery: CompiledQuery, options?: AbortableOperationOptions): Promise<QueryResult<R>>;
    killSession?(controlConnectionProvider: ControlConnectionProvider): Promise<void>;
    streamQuery<R>(compiledQuery: CompiledQuery, chunkSize: number, options?: AbortableOperationOptions): AsyncIterableIterator<QueryResult<R>>;
}
type ControlConnectionProvider = (consumer: (connection: DatabaseConnection) => Promise<void>) => Promise<void>;
interface QueryResult<O> {
    readonly numAffectedRows?: bigint;
    readonly numChangedRows?: bigint;
    readonly insertId?: bigint;
    readonly rows: O[];
}
interface Driver {
    init(options?: AbortableOperationOptions): Promise<void>;
    acquireConnection(options?: AbortableOperationOptions): Promise<DatabaseConnection>;
    beginTransaction(connection: DatabaseConnection, settings: TransactionSettings): Promise<void>;
    commitTransaction(connection: DatabaseConnection): Promise<void>;
    rollbackTransaction(connection: DatabaseConnection): Promise<void>;
    savepoint?(connection: DatabaseConnection, savepointName: string, compileQuery: QueryCompiler['compileQuery']): Promise<void>;
    rollbackToSavepoint?(connection: DatabaseConnection, savepointName: string, compileQuery: QueryCompiler['compileQuery']): Promise<void>;
    releaseSavepoint?(connection: DatabaseConnection, savepointName: string, compileQuery: QueryCompiler['compileQuery']): Promise<void>;
    releaseConnection(connection: DatabaseConnection, options?: AbortableOperationOptions): Promise<void>;
    destroy(options?: AbortableOperationOptions): Promise<void>;
}
interface TransactionSettings {
    readonly accessMode?: AccessMode;
    readonly isolationLevel?: IsolationLevel;
}
declare const TRANSACTION_ACCESS_MODES: readonly [
    'read only',
    'read write'
];
type AccessMode = ArrayItemType<typeof TRANSACTION_ACCESS_MODES>;
declare const TRANSACTION_ISOLATION_LEVELS: readonly [
    'read uncommitted',
    'read committed',
    'repeatable read',
    'serializable',
    'snapshot'
];
type IsolationLevel = ArrayItemType<typeof TRANSACTION_ISOLATION_LEVELS>;
interface DatabaseIntrospector {
    getSchemas(): Promise<SchemaMetadata[]>;
    getTables(options?: DatabaseMetadataOptions): Promise<TableMetadata[]>;
}
interface DatabaseMetadataOptions {
    withInternalKyselyTables: boolean;
}
interface SchemaMetadata {
    readonly name: string;
}
interface TableMetadata {
    readonly name: string;
    readonly isView: boolean;
    readonly isForeign: boolean;
    readonly columns: ColumnMetadata[];
    readonly schema?: string;
}
interface ColumnMetadata {
    readonly name: string;
    readonly dataType: string;
    readonly dataTypeSchema?: string;
    readonly isAutoIncrementing: boolean;
    readonly isNullable: boolean;
    readonly hasDefaultValue: boolean;
    readonly comment?: string;
}
interface DialectAdapter {
    readonly supportsCreateIfNotExists?: boolean;
    readonly supportsMultipleConnections?: boolean;
    readonly supportsTransactionalDdl?: boolean;
    readonly supportsReturning?: boolean;
    readonly supportsOutput?: boolean;
    acquireMigrationLock(db: Kysely<any>, options: MigrationLockOptions): Promise<void>;
    releaseMigrationLock(db: Kysely<any>, options: MigrationLockOptions): Promise<void>;
}
interface MigrationLockOptions {
    readonly lockTable: string;
    readonly lockRowId: string;
    readonly lockTableSchema?: string;
}
interface Dialect {
    createDriver(): Driver;
    createQueryCompiler(): QueryCompiler;
    createAdapter(): DialectAdapter;
    createIntrospector(db: Kysely<any>): DatabaseIntrospector;
}
interface ConnectionProvider {
    provideConnection<T>(consumer: (connection: DatabaseConnection) => Promise<T>, options?: AbortableOperationOptions): Promise<T>;
}
interface KyselyPlugin {
    transformQuery(args: PluginTransformQueryArgs): RootOperationNode;
    transformResult(args: PluginTransformResultArgs): Promise<QueryResult<UnknownRow>>;
}
interface PluginTransformQueryArgs {
    readonly queryId: QueryId;
    readonly node: RootOperationNode;
}
interface PluginTransformResultArgs extends AbortableOperationOptions {
    readonly queryId: QueryId;
    readonly result: QueryResult<UnknownRow>;
}
interface QueryExecutor extends ConnectionProvider {
    get adapter(): DialectAdapter;
    get plugins(): ReadonlyArray<KyselyPlugin>;
    transformQuery<T extends RootOperationNode>(node: T, queryId: QueryId): T;
    compileQuery<R = unknown>(node: RootOperationNode, queryId: QueryId): CompiledQuery<R>;
    executeQuery<R>(compiledQuery: CompiledQuery<R>, options?: AbortableQueryOptions): Promise<QueryResult<R>>;
    stream<R>(compiledQuery: CompiledQuery<R>, chunkSize: number, options?: AbortableOperationOptions): AsyncIterableIterator<QueryResult<R>>;
    withConnectionProvider(connectionProvider: ConnectionProvider): QueryExecutor;
    withPlugin(plugin: KyselyPlugin): QueryExecutor;
    withPlugins(plugin: ReadonlyArray<KyselyPlugin>): QueryExecutor;
    withPluginAtFront(plugin: KyselyPlugin): QueryExecutor;
    withoutPlugins(): QueryExecutor;
}
interface Compilable<O = unknown> {
    compile(): CompiledQuery<O>;
}
type DefaultValueExpression = unknown | Expression<unknown>;
declare class ColumnDefinitionBuilder implements OperationNodeSource {
    #private;
    constructor(node: ColumnDefinitionNode);
    autoIncrement(): ColumnDefinitionBuilder;
    identity(): ColumnDefinitionBuilder;
    primaryKey(): ColumnDefinitionBuilder;
    references(ref: string): ColumnDefinitionBuilder;
    onDelete(onDelete: OnModifyForeignAction): ColumnDefinitionBuilder;
    onUpdate(onUpdate: OnModifyForeignAction): ColumnDefinitionBuilder;
    unique(): ColumnDefinitionBuilder;
    notNull(): ColumnDefinitionBuilder;
    unsigned(): ColumnDefinitionBuilder;
    defaultTo(value: DefaultValueExpression): ColumnDefinitionBuilder;
    check(expression: Expression<any>): ColumnDefinitionBuilder;
    generatedAlwaysAs(expression: Expression<any>): ColumnDefinitionBuilder;
    generatedAlwaysAsIdentity(): ColumnDefinitionBuilder;
    generatedByDefaultAsIdentity(): ColumnDefinitionBuilder;
    stored(): ColumnDefinitionBuilder;
    modifyFront(modifier: Expression<any>): ColumnDefinitionBuilder;
    nullsNotDistinct(): ColumnDefinitionBuilder;
    ifNotExists(): ColumnDefinitionBuilder;
    modifyEnd(modifier: Expression<any>): ColumnDefinitionBuilder;
    $call<T>(func: (qb: this) => T): T;
    toOperationNode(): ColumnDefinitionNode;
}
type ColumnDefinitionBuilderCallback = (builder: ColumnDefinitionBuilder) => ColumnDefinitionBuilder;
type SimpleColumnDataType = 'varchar' | 'char' | 'text' | 'integer' | 'int2' | 'int4' | 'int8' | 'smallint' | 'bigint' | 'boolean' | 'real' | 'double precision' | 'float4' | 'float8' | 'decimal' | 'numeric' | 'binary' | 'bytea' | 'date' | 'datetime' | 'time' | 'timetz' | 'timestamp' | 'timestamptz' | 'serial' | 'bigserial' | 'uuid' | 'json' | 'jsonb' | 'datetime2' | 'blob' | 'varbinary' | 'int4range' | 'int4multirange' | 'int8range' | 'int8multirange' | 'numrange' | 'nummultirange' | 'tsrange' | 'tsmultirange' | 'tstzrange' | 'tstzmultirange' | 'daterange' | 'datemultirange';
type ColumnDataType = SimpleColumnDataType | `varchar(${number})` | `char(${number})` | `decimal(${number}, ${number})` | `numeric(${number}, ${number})` | `binary(${number})` | `datetime(${number})` | `datetime2(${number})` | `time(${number})` | `timetz(${number})` | `timestamp(${number})` | `timestamptz(${number})` | `varbinary(${number})`;
type DataTypeExpression = ColumnDataType | Expression<any>;
interface ForeignKeyConstraintBuilderInterface<R> {
    onDelete(onDelete: OnModifyForeignAction): R;
    onUpdate(onUpdate: OnModifyForeignAction): R;
    deferrable(): R;
    notDeferrable(): R;
    initiallyDeferred(): R;
    initiallyImmediate(): R;
}
declare class ForeignKeyConstraintBuilder implements ForeignKeyConstraintBuilderInterface<ForeignKeyConstraintBuilder>, OperationNodeSource {
    #private;
    constructor(node: ForeignKeyConstraintNode);
    onDelete(onDelete: OnModifyForeignAction): ForeignKeyConstraintBuilder;
    onUpdate(onUpdate: OnModifyForeignAction): ForeignKeyConstraintBuilder;
    deferrable(): ForeignKeyConstraintBuilder;
    notDeferrable(): ForeignKeyConstraintBuilder;
    initiallyDeferred(): ForeignKeyConstraintBuilder;
    initiallyImmediate(): ForeignKeyConstraintBuilder;
    $call<T>(func: (qb: this) => T): T;
    toOperationNode(): ForeignKeyConstraintNode;
}
type ForeignKeyConstraintBuilderCallback = (builder: ForeignKeyConstraintBuilder) => ForeignKeyConstraintBuilder;
declare class AlterColumnBuilder {
    #private;
    constructor(column: string);
    setDataType(dataType: DataTypeExpression): AlteredColumnBuilder;
    setDefault(value: DefaultValueExpression): AlteredColumnBuilder;
    dropDefault(): AlteredColumnBuilder;
    setNotNull(): AlteredColumnBuilder;
    dropNotNull(): AlteredColumnBuilder;
    $call<T>(func: (qb: this) => T): T;
}
declare class AlteredColumnBuilder implements OperationNodeSource {
    #private;
    constructor(alterColumnNode: AlterColumnNode);
    toOperationNode(): AlterColumnNode;
}
type AlterColumnBuilderCallback = (builder: AlterColumnBuilder) => AlteredColumnBuilder;
declare class AlterTableExecutor implements OperationNodeSource, Compilable {
    #private;
    constructor(props: AlterTableExecutorProps);
    toOperationNode(): AlterTableNode;
    compile(): CompiledQuery;
    execute(options?: AbortableQueryOptions): Promise<void>;
}
interface AlterTableExecutorProps {
    readonly queryId: QueryId;
    readonly executor: QueryExecutor;
    readonly node: AlterTableNode;
}
declare class AlterTableAddForeignKeyConstraintBuilder implements ForeignKeyConstraintBuilderInterface<AlterTableAddForeignKeyConstraintBuilder>, OperationNodeSource, Compilable {
    #private;
    constructor(props: AlterTableAddForeignKeyConstraintBuilderProps);
    onDelete(onDelete: OnModifyForeignAction): AlterTableAddForeignKeyConstraintBuilder;
    onUpdate(onUpdate: OnModifyForeignAction): AlterTableAddForeignKeyConstraintBuilder;
    deferrable(): AlterTableAddForeignKeyConstraintBuilder;
    notDeferrable(): AlterTableAddForeignKeyConstraintBuilder;
    initiallyDeferred(): AlterTableAddForeignKeyConstraintBuilder;
    initiallyImmediate(): AlterTableAddForeignKeyConstraintBuilder;
    $call<T>(func: (qb: this) => T): T;
    toOperationNode(): AlterTableNode;
    compile(): CompiledQuery;
    execute(options?: AbortableQueryOptions): Promise<void>;
}
interface AlterTableAddForeignKeyConstraintBuilderProps {
    readonly queryId: QueryId;
    readonly executor: QueryExecutor;
    readonly node: AlterTableNode;
    readonly constraintBuilder: ForeignKeyConstraintBuilder;
}
declare class AlterTableDropConstraintBuilder implements OperationNodeSource, Compilable {
    #private;
    constructor(props: AlterTableDropConstraintBuilderProps);
    ifExists(): AlterTableDropConstraintBuilder;
    cascade(): AlterTableDropConstraintBuilder;
    restrict(): AlterTableDropConstraintBuilder;
    $call<T>(func: (qb: this) => T): T;
    toOperationNode(): AlterTableNode;
    compile(): CompiledQuery;
    execute(options?: AbortableQueryOptions): Promise<void>;
}
interface AlterTableDropConstraintBuilderProps {
    readonly queryId: QueryId;
    readonly executor: QueryExecutor;
    readonly node: AlterTableNode;
}
interface SelectQueryBuilderExpression<out O> extends AliasableExpression<O> {
    get isSelectQueryBuilder(): true;
    toOperationNode(): SelectQueryNode;
}
type ComparisonOperator = '=' | '==' | '!=' | '<>' | '>' | '>=' | '<' | '<=' | 'in' | 'not in' | 'is' | 'is not' | 'like' | 'not like' | 'match' | 'ilike' | 'not ilike' | '@>' | '<@' | '^@' | '&&' | '?' | '?&' | '?|' | '!<' | '!>' | '<=>' | '!~' | '~' | '~*' | '!~*' | '@@' | '@@@' | '!!' | '<->' | 'regexp' | 'is distinct from' | 'is not distinct from';
type ArithmeticOperator = '+' | '-' | '*' | '/' | '%' | '^' | '&' | '|' | '#' | '<<' | '>>';
type JSONOperator = '->' | '->>';
type JSONOperatorWith$ = JSONOperator | `${JSONOperator}$`;
type BinaryOperator = ComparisonOperator | ArithmeticOperator | '||';
type UnaryFilterOperator = 'exists' | 'not exists';
type UnaryOperator = 'not' | '-' | UnaryFilterOperator;
type Operator = BinaryOperator | JSONOperator | UnaryOperator | 'between' | 'between symmetric';
type OperatorNodeFactory = Readonly<{
    is(node: OperationNode): node is OperatorNode;
    create(operator: Operator): Readonly<OperatorNode>;
}>;
interface OperatorNode extends OperationNode {
    readonly kind: 'OperatorNode';
    readonly operator: Operator;
}
declare const OperatorNode: OperatorNodeFactory;
type ValueExpression<DB, TB extends keyof DB, V> = V | ExpressionOrFactory<DB, TB, V>;
type ValueExpressionOrList<DB, TB extends keyof DB, V> = ValueExpression<DB, TB, V> | ReadonlyArray<ValueExpression<DB, TB, V>>;
type ExtractTypeFromValueExpression<VE> = VE extends SelectQueryBuilderExpression<Record<string, infer SV>> ? SV : VE extends Expression<infer V> ? V : VE;
type ColumnType<SelectType, InsertType = SelectType, UpdateType = SelectType> = {
    readonly __select__: SelectType;
    readonly __insert__: InsertType;
    readonly __update__: UpdateType;
};
type Generated<S> = ColumnType<S, S | undefined, S>;
type GeneratedAlways<S> = ColumnType<S, never, never>;
type JSONColumnType<SelectType extends object | null, InsertType = string, UpdateType = string> = ColumnType<SelectType, InsertType, UpdateType>;
type IfNullable<T, K> = IsNullable<T> extends true ? K : never;
type IfNotNullable<T, K> = IsNullable<T> extends true ? never : IfNotNever<T, K>;
type IfNotNever<T, K> = IsNever<T> extends true ? never : K;
type SelectType<T> = T extends ColumnType<infer S, any, any> ? S : T;
type InsertType<T> = T extends ColumnType<any, infer I, any> ? I : T;
type UpdateType<T> = T extends ColumnType<any, any, infer U> ? U : T;
type NullableInsertKeys<R> = {
    [K in keyof R]: IfNullable<InsertType<R[K]>, K>;
}[keyof R];
type NonNullableInsertKeys<R> = {
    [K in keyof R]: IfNotNullable<InsertType<R[K]>, K>;
}[keyof R];
type NonNeverSelectKeys<R> = {
    [K in keyof R]: IfNotNever<SelectType<R[K]>, K>;
}[keyof R];
type UpdateKeys<R> = {
    [K in keyof R]: IfNotNever<UpdateType<R[K]>, K>;
}[keyof R];
type Selectable<R> = DrainOuterGeneric<{
    [K in NonNeverSelectKeys<R>]: SelectType<R[K]>;
}>;
type Insertable<R> = DrainOuterGeneric<object & {
    [K in NonNullableInsertKeys<R>]: InsertType<R[K]>;
} & {
    [K in NullableInsertKeys<R>]?: InsertType<R[K]>;
}>;
type Updateable<R> = DrainOuterGeneric<{
    [K in UpdateKeys<R>]?: UpdateType<R[K]> | undefined;
}>;
type OperandValueExpression<DB, TB extends keyof DB, RE> = ValueExpression<DB, TB, ExtractTypeFromReferenceExpression<DB, TB, RE>>;
type OperandValueExpressionOrList<DB, TB extends keyof DB, RE> = ValueExpressionOrList<DB, TB, ExtractTypeFromReferenceExpression<DB, TB, RE> | null>;
type BinaryOperatorExpression = BinaryOperator | Expression<unknown>;
type ComparisonOperatorExpression = ComparisonOperator | Expression<unknown>;
type FilterObject<DB, TB extends keyof DB> = IsNever<TB> extends true ? KyselyTypeError<'there are no tables in query context, so a filter object cannot be defined. try passing an array instead.'> : {
    [R in StringReference<DB, TB>]?: ValueExpressionOrList<DB, TB, SelectType<ExtractTypeFromStringReference<DB, TB, R>>>;
};
declare class JoinBuilder<DB, TB extends keyof DB> implements OperationNodeSource {
    #private;
    constructor(props: JoinBuilderProps);
    on<RE extends ReferenceExpression<DB, TB>>(lhs: RE, op: ComparisonOperatorExpression, rhs: OperandValueExpressionOrList<DB, TB, RE>): JoinBuilder<DB, TB>;
    on<E extends ExpressionOrFactory<DB, TB, SqlBool>>(expression: E): JoinBuilder<DB, TB>;
    onRef<LRE extends ReferenceExpression<DB, TB>, RRE extends ReferenceExpression<DB, TB>>(lhs: LRE, op: ComparisonOperatorExpression, rhs: RRE): JoinBuilder<DB, TB>;
    onTrue(): JoinBuilder<DB, TB>;
    $call<T>(func: (qb: this) => T): T;
    toOperationNode(): JoinNode;
}
interface JoinBuilderProps {
    readonly joinNode: JoinNode;
}
declare class DynamicTableBuilder<T extends string> {
    #private;
    get table(): T;
    constructor(table: T);
    as<A extends string>(alias: A): AliasedDynamicTableBuilder<T, A>;
}
declare class AliasedDynamicTableBuilder<T extends string, A extends string> implements OperationNodeSource {
    #private;
    get table(): T;
    get alias(): A;
    constructor(table: T, alias: A);
    toOperationNode(): AliasNode;
}
type TableExpression<DB, TB extends keyof DB> = AnyAliasedTable<DB> | AnyTable<DB> | AliasedExpressionOrFactory<DB, TB> | AliasedDynamicTableBuilder<any, any>;
type TableExpressionOrList<DB, TB extends keyof DB> = TableExpression<DB, TB> | ReadonlyArray<TableExpression<DB, TB>>;
type SimpleTableReference<DB> = AnyAliasedTable<DB> | AnyTable<DB>;
type AnyAliasedTable<DB> = `${AnyTable<DB>} as ${string}`;
type AnyTable<DB> = keyof DB & string;
type From<DB, TE> = DrainOuterGeneric<{
    [C in keyof DB | ExtractAliasFromTableExpression<DB, TE>]: C extends ExtractAliasFromTableExpression<DB, TE> ? ExtractRowTypeFromTableExpression<DB, TE, C> : C extends keyof DB ? DB[C] : never;
}>;
type FromTables<DB, TB extends keyof DB, TE> = DrainOuterGeneric<TB | ExtractAliasFromTableExpression<DB, TE>>;
type ExtractTableAlias<DB, TE> = TE extends `${string} as ${infer TA}` ? TA extends keyof DB ? TA : never : TE extends keyof DB ? TE : never;
type ExtractAliasFromTableExpression<DB, TE> = TE extends string ? TE extends `${string} as ${infer TA}` ? TA : TE extends keyof DB ? TE : never : TE extends AliasedExpression<any, infer QA> ? QA : TE extends (qb: any) => AliasedExpression<any, infer QA> ? QA : TE extends AliasedDynamicTableBuilder<any, infer DA> ? DA : never;
type ExtractRowTypeFromTableExpression<DB, TE, A extends keyof any> = TE extends `${infer T} as ${infer TA}` ? TA extends A ? T extends keyof DB ? DB[T] : never : never : TE extends A ? TE extends keyof DB ? DB[TE] : never : TE extends AliasedExpression<infer O, infer QA> ? QA extends A ? O : never : TE extends (qb: any) => AliasedExpression<infer O, infer QA> ? QA extends A ? O : never : TE extends AliasedDynamicTableBuilder<infer T, infer DA> ? DA extends A ? T extends keyof DB ? DB[T] : never : never : never;
type JoinReferenceExpression<DB, TB extends keyof DB, TE> = DrainOuterGeneric<AnyJoinColumn<DB, TB, TE> | AnyJoinColumnWithTable<DB, TB, TE>>;
type JoinCallbackExpression<DB, TB extends keyof DB, TE> = (join: JoinBuilder<From<DB, TE>, FromTables<DB, TB, TE>>) => JoinBuilder<any, any>;
type AnyJoinColumn<DB, TB extends keyof DB, TE> = AnyColumn<From<DB, TE>, FromTables<DB, TB, TE>>;
type AnyJoinColumnWithTable<DB, TB extends keyof DB, TE> = AnyColumnWithTable<From<DB, TE>, FromTables<DB, TB, TE>>;
declare class DynamicReferenceBuilder<R extends string = never> implements OperationNodeSource {
    #private;
    get dynamicReference(): string;
    protected get refType(): R;
    constructor(reference: string);
    toOperationNode(): SimpleReferenceExpressionNode;
}
type SelectExpression<DB, TB extends keyof DB> = AnyAliasedColumnWithTable<DB, TB> | AnyAliasedColumn<DB, TB> | AnyColumnWithTable<DB, TB> | AnyColumn<DB, TB> | DynamicReferenceBuilder<any> | AliasedExpressionOrFactory<DB, TB>;
type SelectCallback<DB, TB extends keyof DB> = (eb: ExpressionBuilder<DB, TB>) => ReadonlyArray<SelectExpression<DB, TB>>;
type Selection<DB, TB extends keyof DB, SE> = [
    DB
] extends [
    unknown
] ? {
    [E in FlattenSelectExpression<SE> as ExtractAliasFromSelectExpression<E>]: SelectType<ExtractTypeFromSelectExpression<DB, TB, E>>;
} : {};
type CallbackSelection<DB, TB extends keyof DB, CB> = CB extends (eb: any) => ReadonlyArray<infer SE> ? Selection<DB, TB, SE> : never;
type FlattenSelectExpression<SE> = SE extends DynamicReferenceBuilder<infer RA> ? {
    [R in RA]: DynamicReferenceBuilder<R>;
}[RA] : SE;
type ExtractAliasFromSelectExpression<SE> = SE extends string ? ExtractAliasFromStringSelectExpression<SE> : SE extends AliasedExpression<any, infer EA> ? EA : SE extends (qb: any) => AliasedExpression<any, infer EA> ? EA : SE extends DynamicReferenceBuilder<infer RA> ? ExtractAliasFromStringSelectExpression<RA> : never;
type ExtractAliasFromStringSelectExpression<SE extends string> = SE extends `${string}.${string}.${string} as ${infer A}` ? A : SE extends `${string}.${string} as ${infer A}` ? A : SE extends `${string} as ${infer A}` ? A : SE extends `${string}.${string}.${infer C}` ? C : SE extends `${string}.${infer C}` ? C : SE;
type ExtractTypeFromSelectExpression<DB, TB extends keyof DB, SE> = SE extends string ? ExtractTypeFromStringSelectExpression<DB, TB, SE> : SE extends AliasedSelectQueryBuilder<infer O, any> ? O[keyof O] | null : SE extends (eb: any) => AliasedSelectQueryBuilder<infer O, any> ? O[keyof O] | null : SE extends AliasedExpression<infer O, any> ? O : SE extends (eb: any) => AliasedExpression<infer O, any> ? O : SE extends DynamicReferenceBuilder<infer RA> ? ExtractTypeFromStringSelectExpression<DB, TB, RA> | undefined : never;
type ExtractTypeFromStringSelectExpression<DB, TB extends keyof DB, SE extends string> = SE extends `${infer SC}.${infer T}.${infer C} as ${string}` ? `${SC}.${T}` extends TB ? C extends keyof DB[`${SC}.${T}`] ? DB[`${SC}.${T}`][C] : never : never : SE extends `${infer T}.${infer C} as ${string}` ? T extends TB ? C extends keyof DB[T] ? DB[T][C] : never : never : SE extends `${infer C} as ${string}` ? C extends AnyColumn<DB, TB> ? ExtractColumnType<DB, TB, C> : never : SE extends `${infer SC}.${infer T}.${infer C}` ? `${SC}.${T}` extends TB ? C extends keyof DB[`${SC}.${T}`] ? DB[`${SC}.${T}`][C] : never : never : SE extends `${infer T}.${infer C}` ? T extends TB ? C extends keyof DB[T] ? DB[T][C] : never : never : SE extends AnyColumn<DB, TB> ? ExtractColumnType<DB, TB, SE> : never;
type AllSelection<DB, TB extends keyof DB> = DrainOuterGeneric<{
    [C in AnyColumn<DB, TB>]: {
        [T in TB]: SelectType<C extends keyof DB[T] ? DB[T][C] : never>;
    }[TB];
}>;
type Collation = 'nocase' | 'binary' | 'rtrim' | (string & {});
declare class OrderByItemBuilder implements OperationNodeSource {
    #private;
    constructor(props: OrderByItemBuilderProps);
    desc(): OrderByItemBuilder;
    asc(): OrderByItemBuilder;
    nullsLast(): OrderByItemBuilder;
    nullsFirst(): OrderByItemBuilder;
    collate(collation: Collation): OrderByItemBuilder;
    toOperationNode(): OrderByItemNode;
}
interface OrderByItemBuilderProps {
    readonly node: OrderByItemNode;
}
type OrderByExpression<DB, TB extends keyof DB, O> = StringReference<DB, TB> | (keyof O & string) | ExpressionOrFactory<DB, TB, any> | DynamicReferenceBuilder<any>;
type OrderByModifiers = OrderByDirection | OrderByModifiersCallbackExpression;
type OrderByDirection = 'asc' | 'desc';
type OrderByModifiersCallbackExpression = (builder: OrderByItemBuilder) => OrderByItemBuilder;
type DirectedOrderByStringReference<DB, TB extends keyof DB, O> = `${StringReference<DB, TB> | (keyof O & string)} ${OrderByDirection}`;
type GroupByExpression<DB, TB extends keyof DB, O> = ReferenceExpression<DB, TB> | (keyof O & string);
type GroupByArg<DB, TB extends keyof DB, O> = GroupByExpression<DB, TB, O> | ReadonlyArray<GroupByExpression<DB, TB, O>> | ((eb: ExpressionBuilder<DB, TB>) => ReadonlyArray<GroupByExpression<DB, TB, O>>);
interface WhereInterface<DB, TB extends keyof DB> {
    where<RE extends ReferenceExpression<DB, TB>, VE extends OperandValueExpressionOrList<DB, TB, RE>>(lhs: RE, op: ComparisonOperatorExpression, rhs: VE): WhereInterface<DB, TB>;
    where<E extends ExpressionOrFactory<DB, TB, SqlBool>>(expression: E): WhereInterface<DB, TB>;
    whereRef<LRE extends ReferenceExpression<DB, TB>, RRE extends ReferenceExpression<DB, TB>>(lhs: LRE, op: ComparisonOperatorExpression, rhs: RRE): WhereInterface<DB, TB>;
    clearWhere(): WhereInterface<DB, TB>;
}
interface HavingInterface<DB, TB extends keyof DB> {
    having<RE extends ReferenceExpression<DB, TB>, VE extends OperandValueExpressionOrList<DB, TB, RE>>(lhs: RE, op: ComparisonOperatorExpression, rhs: VE): HavingInterface<DB, TB>;
    having<E>(expression: E): HavingInterface<DB, TB>;
    havingRef<LRE extends ReferenceExpression<DB, TB>, RRE extends ReferenceExpression<DB, TB>>(lhs: LRE, op: ComparisonOperatorExpression, rhs: RRE): HavingInterface<DB, TB>;
}
type SetOperandExpression<DB, O> = Expression<O> | ReadonlyArray<Expression<O>> | ((eb: ExpressionBuilder<DB, never>) => Expression<O> | ReadonlyArray<Expression<O>>);
interface Streamable<O> {
    stream(chunkSizeOrOptions?: StreamOptions | StreamOptions['chunkSize']): AsyncIterableIterator<O>;
}
interface StreamOptions extends AbortableOperationOptions {
    chunkSize?: number;
}
type AndNodeFactory = Readonly<{
    is(node: OperationNode): node is AndNode;
    create(left: OperationNode, right: OperationNode): Readonly<AndNode>;
}>;
interface AndNode extends OperationNode {
    readonly kind: 'AndNode';
    readonly left: OperationNode;
    readonly right: OperationNode;
}
declare const AndNode: AndNodeFactory;
type OrNodeFactory = Readonly<{
    is(node: OperationNode): node is OrNode;
    create(left: OperationNode, right: OperationNode): Readonly<OrNode>;
}>;
interface OrNode extends OperationNode {
    readonly kind: 'OrNode';
    readonly left: OperationNode;
    readonly right: OperationNode;
}
declare const OrNode: OrNodeFactory;
type ParensNodeFactory = Readonly<{
    is(node: OperationNode): node is ParensNode;
    create(node: OperationNode): Readonly<ParensNode>;
}>;
interface ParensNode extends OperationNode {
    readonly kind: 'ParensNode';
    readonly node: OperationNode;
}
declare const ParensNode: ParensNodeFactory;
declare class ExpressionWrapper<DB, TB extends keyof DB, T> implements AliasableExpression<T> {
    #private;
    constructor(node: OperationNode);
    get expressionType(): T | undefined;
    as<A extends string>(alias: A): AliasedExpression<T, A>;
    as<A extends string>(alias: Expression<unknown>): AliasedExpression<T, A>;
    or<RE extends ReferenceExpression<DB, TB>, VE extends OperandValueExpressionOrList<DB, TB, RE>>(lhs: RE, op: ComparisonOperatorExpression, rhs: VE): T extends SqlBool ? OrWrapper<DB, TB, SqlBool> : KyselyTypeError<'or() method can only be called on boolean expressions'>;
    or<E extends OperandExpression<SqlBool>>(expression: E): T extends SqlBool ? OrWrapper<DB, TB, SqlBool> : KyselyTypeError<'or() method can only be called on boolean expressions'>;
    and<RE extends ReferenceExpression<DB, TB>, VE extends OperandValueExpressionOrList<DB, TB, RE>>(lhs: RE, op: ComparisonOperatorExpression, rhs: VE): T extends SqlBool ? AndWrapper<DB, TB, SqlBool> : KyselyTypeError<'and() method can only be called on boolean expressions'>;
    and<E extends OperandExpression<SqlBool>>(expression: E): T extends SqlBool ? AndWrapper<DB, TB, SqlBool> : KyselyTypeError<'and() method can only be called on boolean expressions'>;
    $castTo<C>(): ExpressionWrapper<DB, TB, C>;
    $notNull(): ExpressionWrapper<DB, TB, Exclude<T, null>>;
    toOperationNode(): OperationNode;
}
declare class OrWrapper<DB, TB extends keyof DB, T extends SqlBool> implements AliasableExpression<T> {
    #private;
    constructor(node: OrNode);
    get expressionType(): T | undefined;
    as<A extends string>(alias: A): AliasedExpression<T, A>;
    as<A extends string>(alias: Expression<unknown>): AliasedExpression<T, A>;
    or<RE extends ReferenceExpression<DB, TB>, VE extends OperandValueExpressionOrList<DB, TB, RE>>(lhs: RE, op: ComparisonOperatorExpression, rhs: VE): OrWrapper<DB, TB, T>;
    or<E extends OperandExpression<SqlBool>>(expression: E): OrWrapper<DB, TB, T>;
    $castTo<C extends SqlBool>(): OrWrapper<DB, TB, C>;
    toOperationNode(): ParensNode;
}
declare class AndWrapper<DB, TB extends keyof DB, T extends SqlBool> implements AliasableExpression<T> {
    #private;
    constructor(node: AndNode);
    get expressionType(): T | undefined;
    as<A extends string>(alias: A): AliasedExpression<T, A>;
    as<A extends string>(alias: Expression<unknown>): AliasedExpression<T, A>;
    and<RE extends ReferenceExpression<DB, TB>, VE extends OperandValueExpressionOrList<DB, TB, RE>>(lhs: RE, op: ComparisonOperatorExpression, rhs: VE): AndWrapper<DB, TB, T>;
    and<E extends OperandExpression<SqlBool>>(expression: E): AndWrapper<DB, TB, T>;
    $castTo<C extends SqlBool>(): AndWrapper<DB, TB, C>;
    toOperationNode(): ParensNode;
}
interface OrderByInterface<DB, TB extends keyof DB, O> {
    orderBy<OE extends OrderByExpression<DB, TB, O>>(expr: OE, modifiers?: OrderByModifiers): OrderByInterface<DB, TB, O>;
    orderBy<OE extends OrderByExpression<DB, TB, O> | DirectedOrderByStringReference<DB, TB, O>>(exprs: ReadonlyArray<OE>): OrderByInterface<DB, TB, O>;
    orderBy<OE extends DirectedOrderByStringReference<DB, TB, O>>(expr: OE): OrderByInterface<DB, TB, O>;
    orderBy<OE extends OrderByExpression<DB, TB, O>>(expr: OE, modifiers: Expression<any>): OrderByInterface<DB, TB, O>;
    clearOrderBy(): OrderByInterface<DB, TB, O>;
}
type NoResultErrorConstructor = new (node: QueryNode) => Error;
interface Executable<O> {
    execute(options?: AbortableQueryOptions): Promise<SimplifyResult<O>[]>;
    executeTakeFirst(options?: AbortableQueryOptions): Promise<SimplifySingleResult<O>>;
    executeTakeFirstOrThrow(options?: ExecuteTakeFirstOrThrowOptions | ExecuteTakeFirstOrThrowOptions['errorConstructor']): Promise<SimplifyResult<O>>;
}
interface ExecuteTakeFirstOrThrowOptions extends AbortableQueryOptions {
    errorConstructor?: NoResultErrorConstructor | ((node: QueryNode) => Error);
}
interface SelectQueryBuilder<DB, TB extends keyof DB, O> extends WhereInterface<DB, TB>, HavingInterface<DB, TB>, OrderByInterface<DB, TB, O>, SelectQueryBuilderExpression<O>, Compilable<O>, Executable<O>, Explainable, Streamable<O> {
    where<RE extends ReferenceExpression<DB, TB>, VE extends OperandValueExpressionOrList<DB, TB, RE>>(lhs: RE, op: ComparisonOperatorExpression, rhs: VE): SelectQueryBuilder<DB, TB, O>;
    where<E extends ExpressionOrFactory<DB, TB, SqlBool>>(expression: E): SelectQueryBuilder<DB, TB, O>;
    whereRef<LRE extends ReferenceExpression<DB, TB>, RRE extends ReferenceExpression<DB, TB>>(lhs: LRE, op: ComparisonOperatorExpression, rhs: RRE): SelectQueryBuilder<DB, TB, O>;
    having<RE extends ReferenceExpression<DB, TB>, VE extends OperandValueExpressionOrList<DB, TB, RE>>(lhs: RE, op: ComparisonOperatorExpression, rhs: VE): SelectQueryBuilder<DB, TB, O>;
    having<E extends ExpressionOrFactory<DB, TB, SqlBool>>(expression: E): SelectQueryBuilder<DB, TB, O>;
    havingRef<LRE extends ReferenceExpression<DB, TB>, RRE extends ReferenceExpression<DB, TB>>(lhs: LRE, op: ComparisonOperatorExpression, rhs: RRE): SelectQueryBuilder<DB, TB, O>;
    select<SE extends SelectExpression<DB, TB>>(selections: ReadonlyArray<SE>): SelectQueryBuilder<DB, TB, O & Selection<DB, TB, SE>>;
    select<const CB extends SelectCallback<DB, TB>>(callback: CB): SelectQueryBuilder<DB, TB, O & CallbackSelection<DB, TB, CB>>;
    select<SE extends SelectExpression<DB, TB>>(selection: SE): SelectQueryBuilder<DB, TB, O & Selection<DB, TB, SE>>;
    distinctOn<RE extends ReferenceExpression<DB, TB>>(selections: ReadonlyArray<RE>): SelectQueryBuilder<DB, TB, O>;
    distinctOn<RE extends ReferenceExpression<DB, TB>>(selection: RE): SelectQueryBuilder<DB, TB, O>;
    modifyFront(modifier: Expression<any>): SelectQueryBuilder<DB, TB, O>;
    modifyEnd(modifier: Expression<any>): SelectQueryBuilder<DB, TB, O>;
    distinct(): SelectQueryBuilder<DB, TB, O>;
    forUpdate(of?: TableOrList<TB>): SelectQueryBuilder<DB, TB, O>;
    forShare(of?: TableOrList<TB>): SelectQueryBuilder<DB, TB, O>;
    forKeyShare(of?: TableOrList<TB>): SelectQueryBuilder<DB, TB, O>;
    forNoKeyUpdate(of?: TableOrList<TB>): SelectQueryBuilder<DB, TB, O>;
    skipLocked(): SelectQueryBuilder<DB, TB, O>;
    noWait(): SelectQueryBuilder<DB, TB, O>;
    selectAll<T extends TB>(table: ReadonlyArray<T>): SelectQueryBuilder<DB, TB, O & AllSelection<DB, T>>;
    selectAll<T extends TB>(table: T): SelectQueryBuilder<DB, TB, O & Selectable<DB[T]>>;
    selectAll(): SelectQueryBuilder<DB, TB, O & AllSelection<DB, TB>>;
    innerJoin<TE extends TableExpression<DB, TB>, K1 extends JoinReferenceExpression<DB, TB, TE>, K2 extends JoinReferenceExpression<DB, TB, TE>>(table: TE, k1: K1, k2: K2): SelectQueryBuilderWithInnerJoin<DB, TB, O, TE>;
    innerJoin<TE extends TableExpression<DB, TB>, const FN extends JoinCallbackExpression<DB, TB, TE>>(table: TE, callback: FN): SelectQueryBuilderWithInnerJoin<DB, TB, O, TE>;
    leftJoin<TE extends TableExpression<DB, TB>, K1 extends JoinReferenceExpression<DB, TB, TE>, K2 extends JoinReferenceExpression<DB, TB, TE>>(table: TE, k1: K1, k2: K2): SelectQueryBuilderWithLeftJoin<DB, TB, O, TE>;
    leftJoin<TE extends TableExpression<DB, TB>, const FN extends JoinCallbackExpression<DB, TB, TE>>(table: TE, callback: FN): SelectQueryBuilderWithLeftJoin<DB, TB, O, TE>;
    rightJoin<TE extends TableExpression<DB, TB>, K1 extends JoinReferenceExpression<DB, TB, TE>, K2 extends JoinReferenceExpression<DB, TB, TE>>(table: TE, k1: K1, k2: K2): SelectQueryBuilderWithRightJoin<DB, TB, O, TE>;
    rightJoin<TE extends TableExpression<DB, TB>, const FN extends JoinCallbackExpression<DB, TB, TE>>(table: TE, callback: FN): SelectQueryBuilderWithRightJoin<DB, TB, O, TE>;
    fullJoin<TE extends TableExpression<DB, TB>, K1 extends JoinReferenceExpression<DB, TB, TE>, K2 extends JoinReferenceExpression<DB, TB, TE>>(table: TE, k1: K1, k2: K2): SelectQueryBuilderWithFullJoin<DB, TB, O, TE>;
    fullJoin<TE extends TableExpression<DB, TB>, const FN extends JoinCallbackExpression<DB, TB, TE>>(table: TE, callback: FN): SelectQueryBuilderWithFullJoin<DB, TB, O, TE>;
    crossJoin<TE extends TableExpression<DB, TB>>(table: TE): SelectQueryBuilderWithInnerJoin<DB, TB, O, TE>;
    innerJoinLateral<TE extends TableExpression<DB, TB>, K1 extends JoinReferenceExpression<DB, TB, TE>, K2 extends JoinReferenceExpression<DB, TB, TE>>(table: TE, k1: K1, k2: K2): SelectQueryBuilderWithInnerJoin<DB, TB, O, TE>;
    innerJoinLateral<TE extends TableExpression<DB, TB>, const FN extends JoinCallbackExpression<DB, TB, TE>>(table: TE, callback: FN): SelectQueryBuilderWithInnerJoin<DB, TB, O, TE>;
    leftJoinLateral<TE extends TableExpression<DB, TB>, K1 extends JoinReferenceExpression<DB, TB, TE>, K2 extends JoinReferenceExpression<DB, TB, TE>>(table: TE, k1: K1, k2: K2): SelectQueryBuilderWithLeftJoin<DB, TB, O, TE>;
    leftJoinLateral<TE extends TableExpression<DB, TB>, const FN extends JoinCallbackExpression<DB, TB, TE>>(table: TE, callback: FN): SelectQueryBuilderWithLeftJoin<DB, TB, O, TE>;
    crossJoinLateral<TE extends TableExpression<DB, TB>>(table: TE): SelectQueryBuilderWithInnerJoin<DB, TB, O, TE>;
    crossApply<TE extends TableExpression<DB, TB>>(table: TE): SelectQueryBuilderWithInnerJoin<DB, TB, O, TE>;
    outerApply<TE extends TableExpression<DB, TB>>(table: TE): SelectQueryBuilderWithLeftJoin<DB, TB, O, TE>;
    groupBy<GE extends GroupByArg<DB, TB, O>>(groupBy: GE): SelectQueryBuilder<DB, TB, O>;
    orderBy<OE extends OrderByExpression<DB, TB, O>>(expr: OE, modifiers?: OrderByModifiers): SelectQueryBuilder<DB, TB, O>;
    orderBy<OE extends OrderByExpression<DB, TB, O> | DirectedOrderByStringReference<DB, TB, O>>(exprs: ReadonlyArray<OE>): SelectQueryBuilder<DB, TB, O>;
    orderBy<OE extends DirectedOrderByStringReference<DB, TB, O>>(expr: OE): SelectQueryBuilder<DB, TB, O>;
    orderBy<OE extends OrderByExpression<DB, TB, O>>(expr: OE, modifiers: Expression<any>): SelectQueryBuilder<DB, TB, O>;
    limit<VE extends ValueExpression<DB, TB, number | bigint | null>>(limit: VE): SelectQueryBuilder<DB, TB, O>;
    offset<VE extends ValueExpression<DB, TB, number | bigint>>(offset: VE): SelectQueryBuilder<DB, TB, O>;
    fetch(rowCount: number | bigint, modifier?: FetchModifier): SelectQueryBuilder<DB, TB, O>;
    top(expression: number | bigint, modifiers?: TopModifier): SelectQueryBuilder<DB, TB, O>;
    union<E extends SetOperandExpression<DB, O>>(expression: E): SelectQueryBuilder<DB, TB, O>;
    unionAll<E extends SetOperandExpression<DB, O>>(expression: E): SelectQueryBuilder<DB, TB, O>;
    intersect<E extends SetOperandExpression<DB, O>>(expression: E): SelectQueryBuilder<DB, TB, O>;
    intersectAll<E extends SetOperandExpression<DB, O>>(expression: E): SelectQueryBuilder<DB, TB, O>;
    except<E extends SetOperandExpression<DB, O>>(expression: E): SelectQueryBuilder<DB, TB, O>;
    exceptAll<E extends SetOperandExpression<DB, O>>(expression: E): SelectQueryBuilder<DB, TB, O>;
    as<A extends string>(alias: A): AliasedSelectQueryBuilder<O, A>;
    clearSelect(): SelectQueryBuilder<DB, TB, {}>;
    clearWhere(): SelectQueryBuilder<DB, TB, O>;
    clearLimit(): SelectQueryBuilder<DB, TB, O>;
    clearOffset(): SelectQueryBuilder<DB, TB, O>;
    clearOrderBy(): SelectQueryBuilder<DB, TB, O>;
    clearGroupBy(): SelectQueryBuilder<DB, TB, O>;
    $call<T>(func: (qb: this) => T): T;
    $if<O2>(condition: boolean, func: (qb: this) => SelectQueryBuilder<any, any, O & O2>): SelectQueryBuilder<DB, TB, O & Partial<Omit<O2, keyof O>>>;
    $castTo<C>(): SelectQueryBuilder<DB, TB, C>;
    $asTuple<K1 extends keyof O, K2 extends Exclude<keyof O, K1>>(key1: K1, key2: K2): keyof O extends K1 | K2 ? ExpressionWrapper<DB, TB, [
        O[K1],
        O[K2]
    ]> : KyselyTypeError<'$asTuple() call failed: All selected columns must be provided as arguments'>;
    $asTuple<K1 extends keyof O, K2 extends Exclude<keyof O, K1>, K3 extends Exclude<keyof O, K1 | K2>>(key1: K1, key2: K2, key3: K3): keyof O extends K1 | K2 | K3 ? ExpressionWrapper<DB, TB, [
        O[K1],
        O[K2],
        O[K3]
    ]> : KyselyTypeError<'$asTuple() call failed: All selected columns must be provided as arguments'>;
    $asTuple<K1 extends keyof O, K2 extends Exclude<keyof O, K1>, K3 extends Exclude<keyof O, K1 | K2>, K4 extends Exclude<keyof O, K1 | K2 | K3>>(key1: K1, key2: K2, key3: K3, key4: K4): keyof O extends K1 | K2 | K3 | K4 ? ExpressionWrapper<DB, TB, [
        O[K1],
        O[K2],
        O[K3],
        O[K4]
    ]> : KyselyTypeError<'$asTuple() call failed: All selected columns must be provided as arguments'>;
    $asTuple<K1 extends keyof O, K2 extends Exclude<keyof O, K1>, K3 extends Exclude<keyof O, K1 | K2>, K4 extends Exclude<keyof O, K1 | K2 | K3>, K5 extends Exclude<keyof O, K1 | K2 | K3 | K4>>(key1: K1, key2: K2, key3: K3, key4: K4, key5: K5): keyof O extends K1 | K2 | K3 | K4 | K5 ? ExpressionWrapper<DB, TB, [
        O[K1],
        O[K2],
        O[K3],
        O[K4],
        O[K5]
    ]> : KyselyTypeError<'$asTuple() call failed: All selected columns must be provided as arguments'>;
    $asScalar<K extends keyof O = keyof O>(): ExpressionWrapper<DB, TB, O[K]>;
    $narrowType<T>(): SelectQueryBuilder<DB, TB, NarrowPartial<O, T>>;
    $assertType<T extends O>(): O extends T ? SelectQueryBuilder<DB, TB, T> : KyselyTypeError<`$assertType() call failed: The type passed in is not equal to the output type of the query.`>;
    withPlugin(plugin: KyselyPlugin): SelectQueryBuilder<DB, TB, O>;
    toOperationNode(): SelectQueryNode;
    compile(): CompiledQuery<Simplify<O>>;
    execute(options?: AbortableQueryOptions): Promise<NonNullable<SimplifyResult<O>>[]>;
    executeTakeFirst(options?: AbortableQueryOptions): Promise<SimplifySingleResult<O>>;
    executeTakeFirstOrThrow(options?: ExecuteTakeFirstOrThrowOptions | ExecuteTakeFirstOrThrowOptions['errorConstructor']): Promise<SimplifyResult<O>>;
    stream(chunkSizeOrOptions?: StreamOptions | StreamOptions['chunkSize']): AsyncIterableIterator<O>;
    explain<ER extends Record<string, any> = Record<string, any>>(format?: ExplainFormat, options?: Expression<any>): Promise<ER[]>;
}
interface AliasedSelectQueryBuilder<out O = undefined, out A extends string = never> extends AliasedExpression<O, A> {
    get isAliasedSelectQueryBuilder(): true;
}
type JoinResultForUnknownTable<O> = SelectQueryBuilder<any, any, O>;
type SelectQueryBuilderWithInnerJoin<DB, TB extends keyof DB, O, TE extends TableExpression<DB, TB>> = TableExpression<DB, TB> extends TE ? JoinResultForUnknownTable<O> : TE extends `${infer T} as ${infer A}` ? T extends keyof DB ? InnerJoinedBuilder$2<DB, TB, O, A, DB[T]> : never : TE extends keyof DB ? SelectQueryBuilder<DB, TB | TE, O> : TE extends AliasedExpression<infer QO, infer QA> ? InnerJoinedBuilder$2<DB, TB, O, QA, QO> : TE extends (qb: any) => AliasedExpression<infer QO, infer QA> ? InnerJoinedBuilder$2<DB, TB, O, QA, QO> : never;
type InnerJoinedBuilder$2<DB, TB extends keyof DB, O, A extends string, R> = A extends keyof DB ? SelectQueryBuilder<InnerJoinedDB$2<DB, A, R>, TB | A, O> : SelectQueryBuilder<DB & ShallowRecord<A, R>, TB | A, O>;
type InnerJoinedDB$2<DB, A extends string, R> = DrainOuterGeneric<{
    [C in keyof DB | A]: C extends A ? R : C extends keyof DB ? DB[C] : never;
}>;
type SelectQueryBuilderWithLeftJoin<DB, TB extends keyof DB, O, TE extends TableExpression<DB, TB>> = TableExpression<DB, TB> extends TE ? JoinResultForUnknownTable<O> : TE extends `${infer T} as ${infer A}` ? T extends keyof DB ? LeftJoinedBuilder$2<DB, TB, O, A, DB[T]> : never : TE extends keyof DB ? LeftJoinedBuilder$2<DB, TB, O, TE, DB[TE]> : TE extends AliasedExpression<infer QO, infer QA> ? LeftJoinedBuilder$2<DB, TB, O, QA, QO> : TE extends (qb: any) => AliasedExpression<infer QO, infer QA> ? LeftJoinedBuilder$2<DB, TB, O, QA, QO> : never;
type LeftJoinedBuilder$2<DB, TB extends keyof DB, O, A extends keyof any, R> = A extends keyof DB ? SelectQueryBuilder<LeftJoinedDB$2<DB, A, R>, TB | A, O> : SelectQueryBuilder<DB & ShallowRecord<A, Nullable<R>>, TB | A, O>;
type LeftJoinedDB$2<DB, A extends keyof any, R> = DrainOuterGeneric<{
    [C in keyof DB | A]: C extends A ? Nullable<R> : C extends keyof DB ? DB[C] : never;
}>;
type SelectQueryBuilderWithRightJoin<DB, TB extends keyof DB, O, TE extends TableExpression<DB, TB>> = TableExpression<DB, TB> extends TE ? JoinResultForUnknownTable<O> : TE extends `${infer T} as ${infer A}` ? T extends keyof DB ? RightJoinedBuilder$2<DB, TB, O, A, DB[T]> : never : TE extends keyof DB ? RightJoinedBuilder$2<DB, TB, O, TE, DB[TE]> : TE extends AliasedExpression<infer QO, infer QA> ? RightJoinedBuilder$2<DB, TB, O, QA, QO> : TE extends (qb: any) => AliasedExpression<infer QO, infer QA> ? RightJoinedBuilder$2<DB, TB, O, QA, QO> : never;
type RightJoinedBuilder$2<DB, TB extends keyof DB, O, A extends keyof any, R> = SelectQueryBuilder<RightJoinedDB$2<DB, TB, A, R>, TB | A, O>;
type RightJoinedDB$2<DB, TB extends keyof DB, A extends keyof any, R> = DrainOuterGeneric<{
    [C in keyof DB | A]: C extends A ? R : C extends TB ? Nullable<DB[C]> : C extends keyof DB ? DB[C] : never;
}>;
type SelectQueryBuilderWithFullJoin<DB, TB extends keyof DB, O, TE extends TableExpression<DB, TB>> = TableExpression<DB, TB> extends TE ? JoinResultForUnknownTable<O> : TE extends `${infer T} as ${infer A}` ? T extends keyof DB ? OuterJoinedBuilder$2<DB, TB, O, A, DB[T]> : never : TE extends keyof DB ? OuterJoinedBuilder$2<DB, TB, O, TE, DB[TE]> : TE extends AliasedExpression<infer QO, infer QA> ? OuterJoinedBuilder$2<DB, TB, O, QA, QO> : TE extends (qb: any) => AliasedExpression<infer QO, infer QA> ? OuterJoinedBuilder$2<DB, TB, O, QA, QO> : never;
type OuterJoinedBuilder$2<DB, TB extends keyof DB, O, A extends keyof any, R> = SelectQueryBuilder<OuterJoinedBuilderDB$2<DB, TB, A, R>, TB | A, O>;
type OuterJoinedBuilderDB$2<DB, TB extends keyof DB, A extends keyof any, R> = DrainOuterGeneric<{
    [C in keyof DB | A]: C extends A ? Nullable<R> : C extends TB ? Nullable<DB[C]> : C extends keyof DB ? DB[C] : never;
}>;
type TableOrList<TB extends keyof any> = (TB & string) | ReadonlyArray<TB & string>;
type ExtractTypeFromCoalesce1<DB, TB extends keyof DB, R1> = ExtractTypeFromReferenceExpression<DB, TB, R1>;
type ExtractTypeFromCoalesce2<DB, TB extends keyof DB, R1, R2> = ExtractTypeFromCoalesceValues2<ExtractTypeFromReferenceExpression<DB, TB, R1>, ExtractTypeFromReferenceExpression<DB, TB, R2>>;
type ExtractTypeFromCoalesceValues2<V1, V2> = null extends V1 ? null extends V2 ? V1 | V2 : NotNull<V1 | V2> : NotNull<V1>;
type ExtractTypeFromCoalesce3<DB, TB extends keyof DB, R1, R2, R3> = ExtractTypeFromCoalesceValues3<ExtractTypeFromReferenceExpression<DB, TB, R1>, ExtractTypeFromReferenceExpression<DB, TB, R2>, ExtractTypeFromReferenceExpression<DB, TB, R3>>;
type ExtractTypeFromCoalesceValues3<V1, V2, V3> = null extends V1 ? null extends V2 ? null extends V3 ? V1 | V2 | V3 : NotNull<V1 | V2 | V3> : NotNull<V1 | V2> : NotNull<V1>;
type ExtractTypeFromCoalesce4<DB, TB extends keyof DB, R1, R2, R3, R4> = ExtractTypeFromCoalesceValues4<ExtractTypeFromReferenceExpression<DB, TB, R1>, ExtractTypeFromReferenceExpression<DB, TB, R2>, ExtractTypeFromReferenceExpression<DB, TB, R3>, ExtractTypeFromReferenceExpression<DB, TB, R4>>;
type ExtractTypeFromCoalesceValues4<V1, V2, V3, V4> = null extends V1 ? null extends V2 ? null extends V3 ? null extends V4 ? V1 | V2 | V3 | V4 : NotNull<V1 | V2 | V3 | V4> : NotNull<V1 | V2 | V3> : NotNull<V1 | V2> : NotNull<V1>;
type ExtractTypeFromCoalesce5<DB, TB extends keyof DB, R1, R2, R3, R4, R5> = ExtractTypeFromCoalesceValues5<ExtractTypeFromReferenceExpression<DB, TB, R1>, ExtractTypeFromReferenceExpression<DB, TB, R2>, ExtractTypeFromReferenceExpression<DB, TB, R3>, ExtractTypeFromReferenceExpression<DB, TB, R4>, ExtractTypeFromReferenceExpression<DB, TB, R5>>;
type ExtractTypeFromCoalesceValues5<V1, V2, V3, V4, V5> = null extends V1 ? null extends V2 ? null extends V3 ? null extends V4 ? null extends V5 ? V1 | V2 | V3 | V4 | V5 : NotNull<V1 | V2 | V3 | V4 | V5> : NotNull<V1 | V2 | V3 | V4> : NotNull<V1 | V2 | V3> : NotNull<V1 | V2> : NotNull<V1>;
type NotNull<T> = Exclude<T, null>;
type PartitionByItemNodeFactory = Readonly<{
    is(node: OperationNode): node is PartitionByItemNode;
    create(partitionBy: SimpleReferenceExpressionNode): Readonly<PartitionByItemNode>;
}>;
interface PartitionByItemNode extends OperationNode {
    readonly kind: 'PartitionByItemNode';
    readonly partitionBy: SimpleReferenceExpressionNode;
}
declare const PartitionByItemNode: PartitionByItemNodeFactory;
type PartitionByNodeFactory = Readonly<{
    is(node: OperationNode): node is PartitionByNode;
    create(items: ReadonlyArray<PartitionByItemNode>): Readonly<PartitionByNode>;
    cloneWithItems(partitionBy: PartitionByNode, items: ReadonlyArray<PartitionByItemNode>): Readonly<PartitionByNode>;
}>;
interface PartitionByNode extends OperationNode {
    readonly kind: 'PartitionByNode';
    readonly items: ReadonlyArray<PartitionByItemNode>;
}
declare const PartitionByNode: PartitionByNodeFactory;
type OverNodeFactory = Readonly<{
    is(node: OperationNode): node is OverNode;
    create(): Readonly<OverNode>;
    cloneWithOrderByItems(overNode: OverNode, items: ReadonlyArray<OrderByItemNode>): Readonly<OverNode>;
    cloneWithPartitionByItems(overNode: OverNode, items: ReadonlyArray<PartitionByItemNode>): Readonly<OverNode>;
}>;
interface OverNode extends OperationNode {
    readonly kind: 'OverNode';
    readonly orderBy?: OrderByNode;
    readonly partitionBy?: PartitionByNode;
}
declare const OverNode: OverNodeFactory;
type AggregateFunctionNodeFactory = Readonly<{
    is(node: OperationNode): node is AggregateFunctionNode;
    create(aggregateFunction: string, aggregated?: readonly OperationNode[]): Readonly<AggregateFunctionNode>;
    cloneWithDistinct(aggregateFunctionNode: AggregateFunctionNode): Readonly<AggregateFunctionNode>;
    cloneWithOrderBy(aggregateFunctionNode: AggregateFunctionNode, orderItems: ReadonlyArray<OrderByItemNode>, withinGroup?: boolean): Readonly<AggregateFunctionNode>;
    cloneWithFilter(aggregateFunctionNode: AggregateFunctionNode, filter: OperationNode): Readonly<AggregateFunctionNode>;
    cloneWithOrFilter(aggregateFunctionNode: AggregateFunctionNode, filter: OperationNode): Readonly<AggregateFunctionNode>;
    cloneWithOver(aggregateFunctionNode: AggregateFunctionNode, over?: OverNode): Readonly<AggregateFunctionNode>;
}>;
interface AggregateFunctionNode extends OperationNode {
    readonly kind: 'AggregateFunctionNode';
    readonly func: string;
    readonly aggregated: readonly OperationNode[];
    readonly distinct?: boolean;
    readonly orderBy?: OrderByNode;
    readonly withinGroup?: OrderByNode;
    readonly filter?: WhereNode;
    readonly over?: OverNode;
}
declare const AggregateFunctionNode: AggregateFunctionNodeFactory;
type PartitionByExpression<DB, TB extends keyof DB> = StringReference<DB, TB> | DynamicReferenceBuilder<any>;
declare class OverBuilder<DB, TB extends keyof DB> implements OrderByInterface<DB, TB, {}>, OperationNodeSource {
    #private;
    constructor(props: OverBuilderProps);
    orderBy<OE extends OrderByExpression<DB, TB, {}>>(expr: OE, modifiers?: OrderByModifiers): OverBuilder<DB, TB>;
    orderBy<OE extends OrderByExpression<DB, TB, {}> | DirectedOrderByStringReference<DB, TB, {}>>(exprs: ReadonlyArray<OE>): OverBuilder<DB, TB>;
    orderBy<OE extends DirectedOrderByStringReference<DB, TB, {}>>(expr: OE): OverBuilder<DB, TB>;
    orderBy<OE extends OrderByExpression<DB, TB, {}>>(expr: OE, modifiers: Expression<any>): OverBuilder<DB, TB>;
    clearOrderBy(): OverBuilder<DB, TB>;
    partitionBy(partitionBy: ReadonlyArray<PartitionByExpression<DB, TB>>): OverBuilder<DB, TB>;
    partitionBy<PE extends PartitionByExpression<DB, TB>>(partitionBy: PE): OverBuilder<DB, TB>;
    $call<T>(func: (qb: this) => T): T;
    toOperationNode(): OverNode;
}
interface OverBuilderProps {
    readonly overNode: OverNode;
}
declare class AggregateFunctionBuilder<DB, TB extends keyof DB, O = unknown> implements OrderByInterface<DB, TB, {}>, AliasableExpression<O> {
    #private;
    constructor(props: AggregateFunctionBuilderProps);
    get expressionType(): O | undefined;
    as<A extends string>(alias: A): AliasedAggregateFunctionBuilder<DB, TB, O, A>;
    distinct(): AggregateFunctionBuilder<DB, TB, O>;
    orderBy<OE extends OrderByExpression<DB, TB, {}>>(expr: OE, modifiers?: OrderByModifiers): AggregateFunctionBuilder<DB, TB, O>;
    orderBy<OE extends OrderByExpression<DB, TB, {}> | DirectedOrderByStringReference<DB, TB, {}>>(exprs: ReadonlyArray<OE>): AggregateFunctionBuilder<DB, TB, O>;
    orderBy<OE extends DirectedOrderByStringReference<DB, TB, {}>>(expr: OE): AggregateFunctionBuilder<DB, TB, O>;
    orderBy<OE extends OrderByExpression<DB, TB, {}>>(expr: OE, modifiers: Expression<any>): AggregateFunctionBuilder<DB, TB, O>;
    clearOrderBy(): AggregateFunctionBuilder<DB, TB, O>;
    withinGroupOrderBy<OE extends OrderByExpression<DB, TB, {}>>(expr: OE, modifiers?: OrderByModifiers): AggregateFunctionBuilder<DB, TB, O>;
    withinGroupOrderBy<OE extends OrderByExpression<DB, TB, {}> | DirectedOrderByStringReference<DB, TB, {}>>(exprs: ReadonlyArray<OE>): AggregateFunctionBuilder<DB, TB, O>;
    withinGroupOrderBy<OE extends DirectedOrderByStringReference<DB, TB, {}>>(expr: OE): AggregateFunctionBuilder<DB, TB, O>;
    withinGroupOrderBy<OE extends OrderByExpression<DB, TB, {}>>(expr: OE, modifiers: Expression<any>): AggregateFunctionBuilder<DB, TB, O>;
    filterWhere<RE extends ReferenceExpression<DB, TB>, VE extends OperandValueExpressionOrList<DB, TB, RE>>(lhs: RE, op: ComparisonOperatorExpression, rhs: VE): AggregateFunctionBuilder<DB, TB, O>;
    filterWhere<E extends ExpressionOrFactory<DB, TB, SqlBool>>(expression: E): AggregateFunctionBuilder<DB, TB, O>;
    filterWhereRef<LRE extends ReferenceExpression<DB, TB>, RRE extends ReferenceExpression<DB, TB>>(lhs: LRE, op: ComparisonOperatorExpression, rhs: RRE): AggregateFunctionBuilder<DB, TB, O>;
    over(over?: OverBuilderCallback<DB, TB>): AggregateFunctionBuilder<DB, TB, O>;
    $call<T>(func: (qb: this) => T): T;
    $castTo<C>(): AggregateFunctionBuilder<DB, TB, C>;
    $notNull(): AggregateFunctionBuilder<DB, TB, Exclude<O, null>>;
    toOperationNode(): AggregateFunctionNode;
}
declare class AliasedAggregateFunctionBuilder<DB, TB extends keyof DB, O = unknown, A extends string = never> implements AliasedExpression<O, A> {
    #private;
    constructor(aggregateFunctionBuilder: AggregateFunctionBuilder<DB, TB, O>, alias: A);
    get expression(): Expression<O>;
    get alias(): A;
    toOperationNode(): AliasNode;
}
interface AggregateFunctionBuilderProps {
    aggregateFunctionNode: AggregateFunctionNode;
}
type OverBuilderCallback<DB, TB extends keyof DB> = (builder: OverBuilder<DB, TB>) => OverBuilder<any, any>;
interface FunctionModule<DB, TB extends keyof DB> {
    <O, RE extends ReferenceExpression<DB, TB> = ReferenceExpression<DB, TB>>(name: string, args?: ReadonlyArray<RE>): ExpressionWrapper<DB, TB, O>;
    agg<O, RE extends ReferenceExpression<DB, TB> = ReferenceExpression<DB, TB>>(name: string, args?: ReadonlyArray<RE>): AggregateFunctionBuilder<DB, TB, O>;
    avg<O extends number | string | null = number | string, RE extends ReferenceExpression<DB, TB> = ReferenceExpression<DB, TB>>(expr: RE): AggregateFunctionBuilder<DB, TB, O>;
    coalesce<V1 extends ReferenceExpression<DB, TB>>(v1: V1): ExpressionWrapper<DB, TB, ExtractTypeFromCoalesce1<DB, TB, V1>>;
    coalesce<V1 extends ReferenceExpression<DB, TB>, V2 extends ReferenceExpression<DB, TB>>(v1: V1, v2: V2): ExpressionWrapper<DB, TB, ExtractTypeFromCoalesce2<DB, TB, V1, V2>>;
    coalesce<V1 extends ReferenceExpression<DB, TB>, V2 extends ReferenceExpression<DB, TB>, V3 extends ReferenceExpression<DB, TB>>(v1: V1, v2: V2, v3: V3): ExpressionWrapper<DB, TB, ExtractTypeFromCoalesce3<DB, TB, V1, V2, V3>>;
    coalesce<V1 extends ReferenceExpression<DB, TB>, V2 extends ReferenceExpression<DB, TB>, V3 extends ReferenceExpression<DB, TB>, V4 extends ReferenceExpression<DB, TB>>(v1: V1, v2: V2, v3: V3, v4: V4): ExpressionWrapper<DB, TB, ExtractTypeFromCoalesce4<DB, TB, V1, V2, V3, V4>>;
    coalesce<V1 extends ReferenceExpression<DB, TB>, V2 extends ReferenceExpression<DB, TB>, V3 extends ReferenceExpression<DB, TB>, V4 extends ReferenceExpression<DB, TB>, V5 extends ReferenceExpression<DB, TB>>(v1: V1, v2: V2, v3: V3, v4: V4, v5: V5): ExpressionWrapper<DB, TB, ExtractTypeFromCoalesce5<DB, TB, V1, V2, V3, V4, V5>>;
    count<O extends number | string | bigint, RE extends ReferenceExpression<DB, TB> = ReferenceExpression<DB, TB>>(expr: RE): AggregateFunctionBuilder<DB, TB, O>;
    countAll<O extends number | string | bigint, T extends TB = TB>(table: T): AggregateFunctionBuilder<DB, TB, O>;
    countAll<O extends number | string | bigint>(): AggregateFunctionBuilder<DB, TB, O>;
    max<O extends number | string | Date | bigint | null = never, RE extends ReferenceExpression<DB, TB> = ReferenceExpression<DB, TB>>(expr: RE): AggregateFunctionBuilder<DB, TB, IsNever<O> extends true ? ExtractTypeFromReferenceExpression<DB, TB, RE, number | string | Date | bigint> : O>;
    min<O extends number | string | Date | bigint | null = never, RE extends ReferenceExpression<DB, TB> = ReferenceExpression<DB, TB>>(expr: RE): AggregateFunctionBuilder<DB, TB, IsNever<O> extends true ? ExtractTypeFromReferenceExpression<DB, TB, RE, number | string | Date | bigint> : O>;
    sum<O extends number | string | bigint | null = number | string | bigint, RE extends ReferenceExpression<DB, TB> = ReferenceExpression<DB, TB>>(expr: RE): AggregateFunctionBuilder<DB, TB, O>;
    any<RE extends StringReference<DB, TB>>(expr: RE): Exclude<ExtractTypeFromReferenceExpression<DB, TB, RE>, null> extends ReadonlyArray<infer I> ? ExpressionWrapper<DB, TB, I> : KyselyTypeError<'any(expr) call failed: expr must be an array'>;
    any<T>(subquery: SelectQueryBuilderExpression<Record<string, T>>): ExpressionWrapper<DB, TB, T>;
    any<T>(expr: Expression<ReadonlyArray<T>>): ExpressionWrapper<DB, TB, T>;
    jsonAgg<T extends (TB & string) | Expression<unknown>>(table: T): AggregateFunctionBuilder<DB, TB, T extends TB ? Simplify<ShallowDehydrateObject<Selectable<DB[T]>>>[] : T extends Expression<infer O> ? Simplify<ShallowDehydrateObject<O>>[] : never>;
    jsonAgg<RE extends StringReference<DB, TB>>(column: RE): AggregateFunctionBuilder<DB, TB, ShallowDehydrateValue<SelectType<ExtractTypeFromStringReference<DB, TB, RE>>>[] | null>;
    toJson<T extends (TB & string) | Expression<unknown>>(table: T): ExpressionWrapper<DB, TB, T extends TB ? Simplify<ShallowDehydrateObject<Selectable<DB[T]>>> : T extends Expression<infer O> ? Simplify<ShallowDehydrateObject<O>> : never>;
}
type CaseNodeFactory = Readonly<{
    is(node: OperationNode): node is CaseNode;
    create(value?: OperationNode): Readonly<CaseNode>;
    cloneWithWhen(caseNode: CaseNode, when: WhenNode): Readonly<CaseNode>;
    cloneWithThen(caseNode: CaseNode, then: OperationNode): Readonly<CaseNode>;
    cloneWith(caseNode: CaseNode, props: Partial<Pick<CaseNode, 'else' | 'isStatement'>>): Readonly<CaseNode>;
}>;
interface CaseNode extends OperationNode {
    readonly kind: 'CaseNode';
    readonly value?: OperationNode;
    readonly when?: ReadonlyArray<WhenNode>;
    readonly else?: OperationNode;
    readonly isStatement?: boolean;
}
declare const CaseNode: CaseNodeFactory;
declare class CaseBuilder<DB, TB extends keyof DB, W = unknown, O = never> implements Whenable<DB, TB, W, O> {
    #private;
    constructor(props: CaseBuilderProps);
    when<RE extends ReferenceExpression<DB, TB>, VE extends OperandValueExpressionOrList<DB, TB, RE>>(lhs: unknown extends W ? RE : KyselyTypeError<'when(lhs, op, rhs) is not supported when using case(value)'>, op: ComparisonOperatorExpression, rhs: VE): CaseThenBuilder<DB, TB, W, O>;
    when(expression: Expression<W>): CaseThenBuilder<DB, TB, W, O>;
    when(value: unknown extends W ? KyselyTypeError<'when(value) is only supported when using case(value)'> : W): CaseThenBuilder<DB, TB, W, O>;
    whenRef<RE extends ReferenceExpression<DB, TB>>(lhs: unknown extends W ? RE : KyselyTypeError<'whenRef(lhs, op, rhs) is not supported when using case(value)'>, op: ComparisonOperatorExpression, rhs: RE): CaseThenBuilder<DB, TB, W, O>;
}
interface CaseBuilderProps {
    readonly node: CaseNode;
}
declare class CaseThenBuilder<DB, TB extends keyof DB, W, O> {
    #private;
    constructor(props: CaseBuilderProps);
    then<E extends Expression<unknown>>(expression: E): CaseWhenBuilder<DB, TB, W, O | ExtractTypeFromValueExpression<E>>;
    then<V>(value: V): CaseWhenBuilder<DB, TB, W, O | V>;
    thenRef<RE extends ReferenceExpression<DB, TB>>(expression: RE): CaseWhenBuilder<DB, TB, W, O | ExtractTypeFromReferenceExpression<DB, TB, RE>>;
}
declare class CaseWhenBuilder<DB, TB extends keyof DB, W, O> implements Whenable<DB, TB, W, O>, Endable<DB, TB, O | null> {
    #private;
    constructor(props: CaseBuilderProps);
    when<RE extends ReferenceExpression<DB, TB>, VE extends OperandValueExpressionOrList<DB, TB, RE>>(lhs: unknown extends W ? RE : KyselyTypeError<'when(lhs, op, rhs) is not supported when using case(value)'>, op: ComparisonOperatorExpression, rhs: VE): CaseThenBuilder<DB, TB, W, O>;
    when(expression: Expression<W>): CaseThenBuilder<DB, TB, W, O>;
    when(value: unknown extends W ? KyselyTypeError<'when(value) is only supported when using case(value)'> : W): CaseThenBuilder<DB, TB, W, O>;
    whenRef<RE extends ReferenceExpression<DB, TB>>(lhs: unknown extends W ? RE : KyselyTypeError<'whenRef(lhs, op, rhs) is not supported when using case(value)'>, op: ComparisonOperatorExpression, rhs: RE): CaseThenBuilder<DB, TB, W, O>;
    else<E extends Expression<unknown>>(expression: E): CaseEndBuilder<DB, TB, O | ExtractTypeFromValueExpression<E>>;
    else<V>(value: V): CaseEndBuilder<DB, TB, O | V>;
    elseRef<RE extends ReferenceExpression<DB, TB>>(expression: RE): CaseEndBuilder<DB, TB, O | ExtractTypeFromReferenceExpression<DB, TB, RE>>;
    end(): ExpressionWrapper<DB, TB, O | null>;
    endCase(): ExpressionWrapper<DB, TB, O | null>;
}
declare class CaseEndBuilder<DB, TB extends keyof DB, O> implements Endable<DB, TB, O> {
    #private;
    constructor(props: CaseBuilderProps);
    end(): ExpressionWrapper<DB, TB, O>;
    endCase(): ExpressionWrapper<DB, TB, O>;
}
interface Whenable<DB, TB extends keyof DB, W, O> {
    when<RE extends ReferenceExpression<DB, TB>, VE extends OperandValueExpressionOrList<DB, TB, RE>>(lhs: unknown extends W ? RE : KyselyTypeError<'when(lhs, op, rhs) is not supported when using case(value)'>, op: ComparisonOperatorExpression, rhs: VE): CaseThenBuilder<DB, TB, W, O>;
    when(expression: Expression<W>): CaseThenBuilder<DB, TB, W, O>;
    when(value: unknown extends W ? KyselyTypeError<'when(value) is only supported when using case(value)'> : W): CaseThenBuilder<DB, TB, W, O>;
    whenRef<RE extends ReferenceExpression<DB, TB>>(lhs: unknown extends W ? RE : KyselyTypeError<'whenRef(lhs, op, rhs) is not supported when using case(value)'>, op: ComparisonOperatorExpression, rhs: RE): CaseThenBuilder<DB, TB, W, O>;
}
interface Endable<DB, TB extends keyof DB, O> {
    end(): ExpressionWrapper<DB, TB, O>;
    endCase(): ExpressionWrapper<DB, TB, O>;
}
type JSONPathLegType = 'Member' | 'ArrayLocation';
type JSONPathLegNodeFactory = Readonly<{
    is(node: OperationNode): node is JSONPathLegNode;
    create(type: JSONPathLegType, value: string | number): Readonly<JSONPathLegNode>;
}>;
interface JSONPathLegNode extends OperationNode {
    readonly kind: 'JSONPathLegNode';
    readonly type: JSONPathLegType;
    readonly value: string | number;
}
declare const JSONPathLegNode: JSONPathLegNodeFactory;
type JSONPathNodeFactory = Readonly<{
    is(node: OperationNode): node is JSONPathNode;
    create(inOperator?: OperatorNode): Readonly<JSONPathNode>;
    cloneWithLeg(jsonPathNode: JSONPathNode, pathLeg: JSONPathLegNode): Readonly<JSONPathNode>;
}>;
interface JSONPathNode extends OperationNode {
    readonly kind: 'JSONPathNode';
    readonly inOperator?: OperatorNode;
    readonly pathLegs: ReadonlyArray<JSONPathLegNode>;
}
declare const JSONPathNode: JSONPathNodeFactory;
type JSONOperatorChainNodeFactory = Readonly<{
    is(node: OperationNode): node is JSONOperatorChainNode;
    create(operator: OperatorNode): Readonly<JSONOperatorChainNode>;
    cloneWithValue(node: JSONOperatorChainNode, value: ValueNode): Readonly<JSONOperatorChainNode>;
}>;
interface JSONOperatorChainNode extends OperationNode {
    readonly kind: 'JSONOperatorChainNode';
    readonly operator: OperatorNode;
    readonly values: readonly ValueNode[];
}
declare const JSONOperatorChainNode: JSONOperatorChainNodeFactory;
type JSONReferenceNodeFactory = Readonly<{
    is(node: OperationNode): node is JSONReferenceNode;
    create(reference: ReferenceNode, traversal: JSONPathNode | JSONOperatorChainNode): Readonly<JSONReferenceNode>;
    cloneWithTraversal(node: JSONReferenceNode, traversal: JSONPathNode | JSONOperatorChainNode): Readonly<JSONReferenceNode>;
}>;
interface JSONReferenceNode extends OperationNode {
    readonly kind: 'JSONReferenceNode';
    readonly reference: ReferenceNode;
    readonly traversal: JSONPathNode | JSONOperatorChainNode;
}
declare const JSONReferenceNode: JSONReferenceNodeFactory;
declare class JSONPathBuilder<S, O = S> {
    #private;
    constructor(node: JSONReferenceNode | JSONPathNode);
    at<I extends (any[] extends O ? number | 'last' | `#-${number}` : never), O2 = null | NonNullable<NonNullable<O>[keyof NonNullable<O> & number]>>(index: `${I}` extends `${any}.${any}` | `#--${any}` ? never : I): TraversedJSONPathBuilder<S, O2>;
    key<K extends (any[] extends O ? never : O extends object ? keyof NonNullable<O> & string : never), O2 = undefined extends O ? null | NonNullable<NonNullable<O>[K]> : null extends O ? null | NonNullable<NonNullable<O>[K]> : string extends keyof NonNullable<O> ? null | NonNullable<NonNullable<O>[K]> : NonNullable<O>[K]>(key: K): TraversedJSONPathBuilder<S, O2>;
}
declare class TraversedJSONPathBuilder<S, O> extends JSONPathBuilder<S, O> implements AliasableExpression<O> {
    #private;
    constructor(node: JSONReferenceNode | JSONPathNode);
    get expressionType(): O | undefined;
    as<A extends string>(alias: A): AliasedExpression<O, A>;
    as<A extends string>(alias: Expression<unknown>): AliasedExpression<O, A>;
    $castTo<O2>(): TraversedJSONPathBuilder<S, O2>;
    $notNull(): TraversedJSONPathBuilder<S, Exclude<O, null>>;
    toOperationNode(): OperationNode;
}
type RefTuple2<DB, TB extends keyof DB, R1, R2> = DrainOuterGeneric<[
    ExtractTypeFromReferenceExpression<DB, TB, R1>,
    ExtractTypeFromReferenceExpression<DB, TB, R2>
]>;
type RefTuple3<DB, TB extends keyof DB, R1, R2, R3> = DrainOuterGeneric<[
    ExtractTypeFromReferenceExpression<DB, TB, R1>,
    ExtractTypeFromReferenceExpression<DB, TB, R2>,
    ExtractTypeFromReferenceExpression<DB, TB, R3>
]>;
type RefTuple4<DB, TB extends keyof DB, R1, R2, R3, R4> = DrainOuterGeneric<[
    ExtractTypeFromReferenceExpression<DB, TB, R1>,
    ExtractTypeFromReferenceExpression<DB, TB, R2>,
    ExtractTypeFromReferenceExpression<DB, TB, R3>,
    ExtractTypeFromReferenceExpression<DB, TB, R4>
]>;
type RefTuple5<DB, TB extends keyof DB, R1, R2, R3, R4, R5> = DrainOuterGeneric<[
    ExtractTypeFromReferenceExpression<DB, TB, R1>,
    ExtractTypeFromReferenceExpression<DB, TB, R2>,
    ExtractTypeFromReferenceExpression<DB, TB, R3>,
    ExtractTypeFromReferenceExpression<DB, TB, R4>,
    ExtractTypeFromReferenceExpression<DB, TB, R5>
]>;
type ValTuple2<V1, V2> = DrainOuterGeneric<[
    ExtractTypeFromValueExpression<V1>,
    ExtractTypeFromValueExpression<V2>
]>;
type ValTuple3<V1, V2, V3> = DrainOuterGeneric<[
    ExtractTypeFromValueExpression<V1>,
    ExtractTypeFromValueExpression<V2>,
    ExtractTypeFromValueExpression<V3>
]>;
type ValTuple4<V1, V2, V3, V4> = DrainOuterGeneric<[
    ExtractTypeFromValueExpression<V1>,
    ExtractTypeFromValueExpression<V2>,
    ExtractTypeFromValueExpression<V3>,
    ExtractTypeFromValueExpression<V4>
]>;
type ValTuple5<V1, V2, V3, V4, V5> = DrainOuterGeneric<[
    ExtractTypeFromValueExpression<V1>,
    ExtractTypeFromValueExpression<V2>,
    ExtractTypeFromValueExpression<V3>,
    ExtractTypeFromValueExpression<V4>,
    ExtractTypeFromValueExpression<V5>
]>;
type SelectFrom<DB, TB extends keyof DB, TE extends TableExpressionOrList<DB, TB>> = [
    TE
] extends [
    keyof DB
] ? SelectQueryBuilder<DB, TB | ExtractTableAlias<DB, TE>, {}> : [
    TE
] extends [
    `${infer T} as ${infer A}`
] ? T extends keyof DB ? SelectQueryBuilder<DB & ShallowRecord<A, DB[T]>, TB | A, {}> : never : TE extends ReadonlyArray<infer T> ? SelectQueryBuilder<From<DB, T>, FromTables<DB, TB, T>, {}> : SelectQueryBuilder<From<DB, TE>, FromTables<DB, TB, TE>, {}>;
interface ExpressionBuilder<DB, TB extends keyof DB> {
    <RE extends ReferenceExpression<DB, TB>, OP extends BinaryOperatorExpression, VE extends OperandValueExpressionOrList<DB, TB, RE>>(lhs: RE, op: OP, rhs: VE): ExpressionWrapper<DB, TB, OP extends ComparisonOperator ? SqlBool : OP extends Expression<infer T> ? unknown extends T ? SqlBool : T : ExtractTypeFromReferenceExpression<DB, TB, RE>>;
    get eb(): ExpressionBuilder<DB, TB>;
    get fn(): FunctionModule<DB, TB>;
    selectFrom<TE extends TableExpressionOrList<DB, TB>>(from: TE): SelectFrom<DB, TB, TE>;
    case(): CaseBuilder<DB, TB>;
    case<C extends SimpleReferenceExpression<DB, TB>>(column: C): CaseBuilder<DB, TB, ExtractTypeFromReferenceExpression<DB, TB, C>>;
    case<E extends Expression<any>>(expression: E): CaseBuilder<DB, TB, ExtractTypeFromValueExpression<E>>;
    ref<RE extends StringReference<DB, TB>>(reference: RE): ExpressionWrapper<DB, TB, ExtractTypeFromReferenceExpression<DB, TB, RE>>;
    ref<RE extends StringReference<DB, TB>>(reference: RE, op: JSONOperatorWith$): JSONPathBuilder<ExtractTypeFromReferenceExpression<DB, TB, RE>>;
    jsonPath<$ extends StringReference<DB, TB> = never>(): IsNever<$> extends true ? KyselyTypeError<"You must provide a column reference as this method's $ generic"> : JSONPathBuilder<ExtractTypeFromReferenceExpression<DB, TB, $>>;
    table<T extends TB & string>(table: T): ExpressionWrapper<DB, TB, Selectable<DB[T]>>;
    val<VE>(value: VE): ExpressionWrapper<DB, TB, ExtractTypeFromValueExpression<VE>>;
    refTuple<R1 extends ReferenceExpression<DB, TB>, R2 extends ReferenceExpression<DB, TB>>(value1: R1, value2: R2): ExpressionWrapper<DB, TB, RefTuple2<DB, TB, R1, R2>>;
    refTuple<R1 extends ReferenceExpression<DB, TB>, R2 extends ReferenceExpression<DB, TB>, R3 extends ReferenceExpression<DB, TB>>(value1: R1, value2: R2, value3: R3): ExpressionWrapper<DB, TB, RefTuple3<DB, TB, R1, R2, R3>>;
    refTuple<R1 extends ReferenceExpression<DB, TB>, R2 extends ReferenceExpression<DB, TB>, R3 extends ReferenceExpression<DB, TB>, R4 extends ReferenceExpression<DB, TB>>(value1: R1, value2: R2, value3: R3, value4: R4): ExpressionWrapper<DB, TB, RefTuple4<DB, TB, R1, R2, R3, R4>>;
    refTuple<R1 extends ReferenceExpression<DB, TB>, R2 extends ReferenceExpression<DB, TB>, R3 extends ReferenceExpression<DB, TB>, R4 extends ReferenceExpression<DB, TB>, R5 extends ReferenceExpression<DB, TB>>(value1: R1, value2: R2, value3: R3, value4: R4, value5: R5): ExpressionWrapper<DB, TB, RefTuple5<DB, TB, R1, R2, R3, R4, R5>>;
    tuple<V1, V2>(value1: V1, value2: V2): ExpressionWrapper<DB, TB, ValTuple2<V1, V2>>;
    tuple<V1, V2, V3>(value1: V1, value2: V2, value3: V3): ExpressionWrapper<DB, TB, ValTuple3<V1, V2, V3>>;
    tuple<V1, V2, V3, V4>(value1: V1, value2: V2, value3: V3, value4: V4): ExpressionWrapper<DB, TB, ValTuple4<V1, V2, V3, V4>>;
    tuple<V1, V2, V3, V4, V5>(value1: V1, value2: V2, value3: V3, value4: V4, value5: V5): ExpressionWrapper<DB, TB, ValTuple5<V1, V2, V3, V4, V5>>;
    lit<VE extends number | boolean | null>(literal: VE): ExpressionWrapper<DB, TB, VE>;
    unary<RE extends ReferenceExpression<DB, TB>>(op: UnaryOperator, expr: RE): ExpressionWrapper<DB, TB, ExtractTypeFromReferenceExpression<DB, TB, RE>>;
    not<RE extends ReferenceExpression<DB, TB>>(expr: RE): ExpressionWrapper<DB, TB, ExtractTypeFromReferenceExpression<DB, TB, RE>>;
    exists<RE extends ReferenceExpression<DB, TB>>(expr: RE): ExpressionWrapper<DB, TB, SqlBool>;
    neg<RE extends ReferenceExpression<DB, TB>>(expr: RE): ExpressionWrapper<DB, TB, ExtractTypeFromReferenceExpression<DB, TB, RE>>;
    between<RE extends ReferenceExpression<DB, TB>, SE extends OperandValueExpression<DB, TB, RE>, EE extends OperandValueExpression<DB, TB, RE>>(expr: RE, start: SE, end: EE): ExpressionWrapper<DB, TB, SqlBool>;
    betweenSymmetric<RE extends ReferenceExpression<DB, TB>, SE extends OperandValueExpression<DB, TB, RE>, EE extends OperandValueExpression<DB, TB, RE>>(expr: RE, start: SE, end: EE): ExpressionWrapper<DB, TB, SqlBool>;
    and<E extends OperandExpression<SqlBool>>(exprs: ReadonlyArray<E>): ExpressionWrapper<DB, TB, SqlBool>;
    and<E extends Readonly<FilterObject<DB, TB>>>(exprs: E): ExpressionWrapper<DB, TB, SqlBool>;
    or<E extends OperandExpression<SqlBool>>(exprs: ReadonlyArray<E>): ExpressionWrapper<DB, TB, SqlBool>;
    or<E extends Readonly<FilterObject<DB, TB>>>(exprs: E): ExpressionWrapper<DB, TB, SqlBool>;
    parens<RE extends ReferenceExpression<DB, TB>, OP extends BinaryOperatorExpression, VE extends OperandValueExpressionOrList<DB, TB, RE>>(lhs: RE, op: OP, rhs: VE): ExpressionWrapper<DB, TB, OP extends ComparisonOperator ? SqlBool : ExtractTypeFromReferenceExpression<DB, TB, RE>>;
    parens<T>(expr: Expression<T>): ExpressionWrapper<DB, TB, T>;
    cast<T, RE extends ReferenceExpression<DB, TB> = ReferenceExpression<DB, TB>>(expr: RE, dataType: DataTypeExpression): ExpressionWrapper<DB, TB, T>;
}
type OperandExpression<V> = Expression<V> | SelectQueryBuilderExpression<Record<string, V>>;
type ExpressionOrFactory<DB, TB extends keyof DB, V> = OperandExpression<V> | OperandExpressionFactory<DB, TB, V>;
type AliasedExpressionOrFactory<DB, TB extends keyof DB> = AliasedExpression<any, any> | AliasedExpressionFactory<DB, TB>;
type OperandExpressionFactory<DB, TB extends keyof DB, V> = (eb: ExpressionBuilder<DB, TB>) => OperandExpression<V>;
type AliasedExpressionFactory<DB, TB extends keyof DB> = (eb: ExpressionBuilder<DB, TB>) => AliasedExpression<any, any>;
type StringReference<DB, TB extends keyof DB> = AnyColumn<DB, TB> | AnyColumnWithTable<DB, TB>;
type SimpleReferenceExpression<DB, TB extends keyof DB> = StringReference<DB, TB> | DynamicReferenceBuilder<any>;
type ReferenceExpression<DB, TB extends keyof DB> = SimpleReferenceExpression<DB, TB> | ExpressionOrFactory<DB, TB, any>;
type ExtractTypeFromReferenceExpression<DB, TB extends keyof DB, RE, DV = unknown> = SelectType<ExtractRawTypeFromReferenceExpression<DB, TB, RE, DV>>;
type ExtractRawTypeFromReferenceExpression<DB, TB extends keyof DB, RE, DV = unknown> = RE extends string ? ExtractTypeFromStringReference<DB, TB, RE> : RE extends SelectQueryBuilderExpression<infer O> ? O[keyof O] | null : RE extends (qb: any) => SelectQueryBuilderExpression<infer O> ? O[keyof O] | null : RE extends Expression<infer O> ? O : RE extends (qb: any) => Expression<infer O> ? O : DV;
type ExtractTypeFromStringReference<DB, TB extends keyof DB, RE extends string, DV = unknown> = RE extends `${infer SC}.${infer T}.${infer C}` ? `${SC}.${T}` extends TB ? C extends keyof DB[`${SC}.${T}`] ? DB[`${SC}.${T}`][C] : never : never : RE extends `${infer T}.${infer C}` ? T extends TB ? C extends keyof DB[T] ? DB[T][C] : never : never : RE extends AnyColumn<DB, TB> ? ExtractColumnType<DB, TB, RE> : DV;
type OrderedColumnName<C extends string> = C extends `${string} ${infer O}` ? O extends OrderByDirection ? C : never : C;
type ExtractColumnNameFromOrderedColumnName<C extends string> = C extends `${infer CL} ${infer O}` ? O extends OrderByDirection ? CL : never : C;
declare class AlterTableAddIndexBuilder implements OperationNodeSource, Compilable {
    #private;
    constructor(props: AlterTableAddIndexBuilderProps);
    unique(): AlterTableAddIndexBuilder;
    column<CL extends string>(column: OrderedColumnName<CL>): AlterTableAddIndexBuilder;
    column(expression: Expression<any>): AlterTableAddIndexBuilder;
    columns<CL extends string>(columns: (OrderedColumnName<CL> | Expression<any>)[]): AlterTableAddIndexBuilder;
    expression(expression: Expression<any>): AlterTableAddIndexBuilder;
    using(indexType: IndexType): AlterTableAddIndexBuilder;
    using(indexType: string): AlterTableAddIndexBuilder;
    $call<T>(func: (qb: this) => T): T;
    toOperationNode(): AlterTableNode;
    compile(): CompiledQuery;
    execute(options?: AbortableQueryOptions): Promise<void>;
}
interface AlterTableAddIndexBuilderProps {
    readonly queryId: QueryId;
    readonly executor: QueryExecutor;
    readonly node: AlterTableNode;
}
declare class UniqueConstraintNodeBuilder implements OperationNodeSource {
    #private;
    constructor(node: UniqueConstraintNode);
    nullsNotDistinct(): UniqueConstraintNodeBuilder;
    deferrable(): UniqueConstraintNodeBuilder;
    notDeferrable(): UniqueConstraintNodeBuilder;
    initiallyDeferred(): UniqueConstraintNodeBuilder;
    initiallyImmediate(): UniqueConstraintNodeBuilder;
    $call<T>(func: (qb: this) => T): T;
    toOperationNode(): UniqueConstraintNode;
}
type UniqueConstraintNodeBuilderCallback = (builder: UniqueConstraintNodeBuilder) => UniqueConstraintNodeBuilder;
declare class PrimaryKeyConstraintBuilder implements OperationNodeSource {
    #private;
    constructor(node: PrimaryKeyConstraintNode);
    deferrable(): PrimaryKeyConstraintBuilder;
    notDeferrable(): PrimaryKeyConstraintBuilder;
    initiallyDeferred(): PrimaryKeyConstraintBuilder;
    initiallyImmediate(): PrimaryKeyConstraintBuilder;
    $call<T>(func: (qb: this) => T): T;
    toOperationNode(): PrimaryKeyConstraintNode;
}
type PrimaryKeyConstraintBuilderCallback = (builder: PrimaryKeyConstraintBuilder) => PrimaryKeyConstraintBuilder;
declare class CheckConstraintBuilder implements OperationNodeSource {
    #private;
    constructor(node: CheckConstraintNode);
    $call<T>(func: (qb: this) => T): T;
    toOperationNode(): CheckConstraintNode;
}
type CheckConstraintBuilderCallback = (builder: CheckConstraintBuilder) => CheckConstraintBuilder;
declare class DropColumnBuilder implements OperationNodeSource {
    #private;
    constructor(props: DropColumnBuilderProps);
    ifExists(): DropColumnBuilder;
    toOperationNode(): DropColumnNode;
}
interface DropColumnBuilderProps {
    readonly node: DropColumnNode;
}
type DropColumnBuilderCallback = (builder: DropColumnBuilder) => DropColumnBuilder;
declare class AlterTableBuilder implements ColumnAlteringInterface {
    #private;
    constructor(props: AlterTableBuilderProps);
    renameTo(newTableName: string): AlterTableExecutor;
    setSchema(newSchema: string): AlterTableExecutor;
    alterColumn(column: string, alteration: AlterColumnBuilderCallback): AlterTableColumnAlteringBuilder;
    dropColumn(column: string, build?: DropColumnBuilderCallback): AlterTableColumnAlteringBuilder;
    renameColumn(column: string, newColumn: string): AlterTableColumnAlteringBuilder;
    addColumn(columnName: string, dataType: DataTypeExpression, build?: ColumnDefinitionBuilderCallback): AlterTableColumnAlteringBuilder;
    modifyColumn(columnName: string, dataType: DataTypeExpression, build?: ColumnDefinitionBuilderCallback): AlterTableColumnAlteringBuilder;
    addUniqueConstraint(constraintName: string, columns: (string | ExpressionOrFactory<any, any, any>)[], build?: UniqueConstraintNodeBuilderCallback): AlterTableExecutor;
    addCheckConstraint(constraintName: string, checkExpression: Expression<any>, build?: CheckConstraintBuilderCallback): AlterTableExecutor;
    addForeignKeyConstraint(constraintName: string, columns: string[], targetTable: string, targetColumns: string[], build?: ForeignKeyConstraintBuilderCallback): AlterTableAddForeignKeyConstraintBuilder;
    addPrimaryKeyConstraint(constraintName: string, columns: string[], build?: PrimaryKeyConstraintBuilderCallback): AlterTableExecutor;
    dropConstraint(constraintName: string): AlterTableDropConstraintBuilder;
    renameConstraint(oldName: string, newName: string): AlterTableDropConstraintBuilder;
    addIndex(indexName: string): AlterTableAddIndexBuilder;
    dropIndex(indexName: string): AlterTableExecutor;
    $call<T>(func: (qb: this) => T): T;
}
interface AlterTableBuilderProps {
    readonly executor: QueryExecutor;
    readonly node: AlterTableNode;
    readonly queryId: QueryId;
}
interface ColumnAlteringInterface {
    alterColumn(column: string, alteration: AlterColumnBuilderCallback): ColumnAlteringInterface;
    dropColumn(column: string): ColumnAlteringInterface;
    renameColumn(column: string, newColumn: string): ColumnAlteringInterface;
    addColumn(columnName: string, dataType: DataTypeExpression, build?: ColumnDefinitionBuilderCallback): ColumnAlteringInterface;
    modifyColumn(columnName: string, dataType: DataTypeExpression, build: ColumnDefinitionBuilderCallback): ColumnAlteringInterface;
}
declare class AlterTableColumnAlteringBuilder implements ColumnAlteringInterface, OperationNodeSource, Compilable {
    #private;
    constructor(props: AlterTableColumnAlteringBuilderProps);
    alterColumn(column: string, alteration: AlterColumnBuilderCallback): AlterTableColumnAlteringBuilder;
    dropColumn(column: string, build?: DropColumnBuilderCallback): AlterTableColumnAlteringBuilder;
    renameColumn(column: string, newColumn: string): AlterTableColumnAlteringBuilder;
    addColumn(columnName: string, dataType: DataTypeExpression, build?: ColumnDefinitionBuilderCallback): AlterTableColumnAlteringBuilder;
    modifyColumn(columnName: string, dataType: DataTypeExpression, build?: ColumnDefinitionBuilderCallback): AlterTableColumnAlteringBuilder;
    toOperationNode(): AlterTableNode;
    compile(): CompiledQuery;
    execute(options?: AbortableQueryOptions): Promise<void>;
}
interface AlterTableColumnAlteringBuilderProps extends AlterTableBuilderProps {
}
declare class CreateIndexBuilder<C = never> implements OperationNodeSource, Compilable {
    #private;
    constructor(props: CreateIndexBuilderProps);
    ifNotExists(): CreateIndexBuilder<C>;
    unique(): CreateIndexBuilder<C>;
    nullsNotDistinct(): CreateIndexBuilder<C>;
    on(table: string): CreateIndexBuilder<C>;
    column<CL extends string>(column: OrderedColumnName<CL>): CreateIndexBuilder<C | ExtractColumnNameFromOrderedColumnName<CL>>;
    column<CL extends string = never>(expression: Expression<any>): CreateIndexBuilder<C | CL>;
    columns<CL extends string>(columns: (OrderedColumnName<CL> | Expression<any>)[]): CreateIndexBuilder<C | ExtractColumnNameFromOrderedColumnName<CL>>;
    expression(expression: Expression<any>): CreateIndexBuilder<C>;
    using(indexType: IndexType): CreateIndexBuilder<C>;
    using(indexType: string): CreateIndexBuilder<C>;
    where(lhs: C | Expression<any>, op: ComparisonOperatorExpression, rhs: unknown): CreateIndexBuilder<C>;
    where(factory: (qb: ExpressionBuilder<ShallowRecord<string, ShallowRecord<C & string, any>>, string>) => Expression<SqlBool>): CreateIndexBuilder<C>;
    where(expression: Expression<SqlBool>): CreateIndexBuilder<C>;
    $call<T>(func: (qb: this) => T): T;
    toOperationNode(): CreateIndexNode;
    compile(): CompiledQuery;
    execute(options?: AbortableQueryOptions): Promise<void>;
}
interface CreateIndexBuilderProps {
    readonly queryId: QueryId;
    readonly executor: QueryExecutor;
    readonly node: CreateIndexNode;
}
declare class CreateSchemaBuilder implements OperationNodeSource, Compilable {
    #private;
    constructor(props: CreateSchemaBuilderProps);
    ifNotExists(): CreateSchemaBuilder;
    $call<T>(func: (qb: this) => T): T;
    toOperationNode(): CreateSchemaNode;
    compile(): CompiledQuery;
    execute(options?: AbortableQueryOptions): Promise<void>;
}
interface CreateSchemaBuilderProps {
    readonly queryId: QueryId;
    readonly executor: QueryExecutor;
    readonly node: CreateSchemaNode;
}
declare class CreateTableAddIndexBuilder implements OperationNodeSource {
    #private;
    constructor(node: AddIndexNode);
    using(indexType: IndexType): CreateTableAddIndexBuilder;
    using(indexType: string): CreateTableAddIndexBuilder;
    $call<T>(func: (qb: this) => T): T;
    toOperationNode(): AddIndexNode;
}
type CreateTableAddIndexBuilderCallback = (builder: CreateTableAddIndexBuilder) => CreateTableAddIndexBuilder;
declare class CreateTableBuilder<TB extends string, C extends string = never> implements OperationNodeSource, Compilable {
    #private;
    constructor(props: CreateTableBuilderProps);
    temporary(): CreateTableBuilder<TB, C>;
    onCommit(onCommit: OnCommitAction): CreateTableBuilder<TB, C>;
    ifNotExists(): CreateTableBuilder<TB, C>;
    addColumn<CN extends string>(columnName: CN, dataType: DataTypeExpression, build?: ColumnBuilderCallback): CreateTableBuilder<TB, C | CN>;
    addPrimaryKeyConstraint(constraintName: string, columns: C[], build?: PrimaryKeyConstraintBuilderCallback): CreateTableBuilder<TB, C>;
    addUniqueConstraint(constraintName: string, columns: (C | ExpressionOrFactory<any, any, any>)[], build?: UniqueConstraintNodeBuilderCallback): CreateTableBuilder<TB, C>;
    addIndex(indexName: string, columns: (C | ExpressionOrFactory<any, any, any>)[], build?: CreateTableAddIndexBuilderCallback): CreateTableBuilder<TB, C>;
    addCheckConstraint(constraintName: string, checkExpression: Expression<any>, build?: CheckConstraintBuilderCallback): CreateTableBuilder<TB, C>;
    addForeignKeyConstraint(constraintName: string, columns: C[], targetTable: string, targetColumns: string[], build?: ForeignKeyConstraintBuilderCallback): CreateTableBuilder<TB, C>;
    modifyFront(modifier: Expression<any>): CreateTableBuilder<TB, C>;
    modifyEnd(modifier: Expression<any>): CreateTableBuilder<TB, C>;
    as(expression: Expression<unknown>): CreateTableBuilder<TB, C>;
    $call<T>(func: (qb: this) => T): T;
    toOperationNode(): CreateTableNode;
    compile(): CompiledQuery;
    execute(options?: AbortableQueryOptions): Promise<void>;
}
interface CreateTableBuilderProps {
    readonly queryId: QueryId;
    readonly executor: QueryExecutor;
    readonly node: CreateTableNode;
}
type ColumnBuilderCallback = (builder: ColumnDefinitionBuilder) => ColumnDefinitionBuilder;
declare class DropIndexBuilder implements OperationNodeSource, Compilable {
    #private;
    constructor(props: DropIndexBuilderProps);
    on(table: string): DropIndexBuilder;
    ifExists(): DropIndexBuilder;
    cascade(): DropIndexBuilder;
    $call<T>(func: (qb: this) => T): T;
    toOperationNode(): DropIndexNode;
    compile(): CompiledQuery;
    execute(options?: AbortableQueryOptions): Promise<void>;
}
interface DropIndexBuilderProps {
    readonly queryId: QueryId;
    readonly executor: QueryExecutor;
    readonly node: DropIndexNode;
}
declare class DropSchemaBuilder implements OperationNodeSource, Compilable {
    #private;
    constructor(props: DropSchemaBuilderProps);
    ifExists(): DropSchemaBuilder;
    cascade(): DropSchemaBuilder;
    $call<T>(func: (qb: this) => T): T;
    toOperationNode(): DropSchemaNode;
    compile(): CompiledQuery;
    execute(options?: AbortableQueryOptions): Promise<void>;
}
interface DropSchemaBuilderProps {
    readonly queryId: QueryId;
    readonly executor: QueryExecutor;
    readonly node: DropSchemaNode;
}
declare class DropTableBuilder implements OperationNodeSource, Compilable {
    #private;
    constructor(props: DropTableBuilderProps);
    temporary(): DropTableBuilder;
    ifExists(): DropTableBuilder;
    cascade(): DropTableBuilder;
    $call<T>(func: (qb: this) => T): T;
    toOperationNode(): DropTableNode;
    compile(): CompiledQuery;
    execute(options?: AbortableQueryOptions): Promise<void>;
}
interface DropTableBuilderProps {
    readonly queryId: QueryId;
    readonly executor: QueryExecutor;
    readonly node: DropTableNode;
}
interface QueryExecutorProvider {
    getExecutor(): QueryExecutor;
}
interface RawBuilder<O> extends AliasableExpression<O> {
    get isRawBuilder(): true;
    as<A extends string>(alias: A): AliasedRawBuilder<O, A>;
    as<A extends string>(alias: Expression<any>): AliasedRawBuilder<O, A>;
    $castTo<C>(): RawBuilder<C>;
    $notNull(): RawBuilder<Exclude<O, null>>;
    withPlugin(plugin: KyselyPlugin): RawBuilder<O>;
    compile(executorProvider: QueryExecutorProvider): CompiledQuery<O>;
    execute(executorProvider: QueryExecutorProvider, options?: AbortableQueryOptions): Promise<QueryResult<O>>;
    toOperationNode(): RawNode;
}
interface AliasedRawBuilder<out O = unknown, out A extends string = never> extends AliasedExpression<O, A> {
    get rawBuilder(): RawBuilder<O>;
}
declare class CreateViewBuilder implements OperationNodeSource, Compilable {
    #private;
    constructor(props: CreateViewBuilderProps);
    temporary(): CreateViewBuilder;
    materialized(): CreateViewBuilder;
    ifNotExists(): CreateViewBuilder;
    orReplace(): CreateViewBuilder;
    columns(columns: string[]): CreateViewBuilder;
    as(query: SelectQueryBuilder<any, any, any> | RawBuilder<any>): CreateViewBuilder;
    $call<T>(func: (qb: this) => T): T;
    toOperationNode(): CreateViewNode;
    compile(): CompiledQuery;
    execute(options?: AbortableQueryOptions): Promise<void>;
}
interface CreateViewBuilderProps {
    readonly queryId: QueryId;
    readonly executor: QueryExecutor;
    readonly node: CreateViewNode;
}
declare class DropViewBuilder implements OperationNodeSource, Compilable {
    #private;
    constructor(props: DropViewBuilderProps);
    materialized(): DropViewBuilder;
    ifExists(): DropViewBuilder;
    cascade(): DropViewBuilder;
    $call<T>(func: (qb: this) => T): T;
    toOperationNode(): DropViewNode;
    compile(): CompiledQuery;
    execute(options?: AbortableQueryOptions): Promise<void>;
}
interface DropViewBuilderProps {
    readonly queryId: QueryId;
    readonly executor: QueryExecutor;
    readonly node: DropViewNode;
}
declare class CreateTypeBuilder implements OperationNodeSource, Compilable {
    #private;
    constructor(props: CreateTypeBuilderProps);
    toOperationNode(): CreateTypeNode;
    asEnum(values: readonly string[]): CreateTypeBuilder;
    $call<T>(func: (qb: this) => T): T;
    compile(): CompiledQuery;
    execute(options?: AbortableQueryOptions): Promise<void>;
}
interface CreateTypeBuilderProps {
    readonly queryId: QueryId;
    readonly executor: QueryExecutor;
    readonly node: CreateTypeNode;
}
declare class DropTypeBuilder implements OperationNodeSource, Compilable {
    #private;
    constructor(props: DropTypeBuilderProps);
    ifExists(): DropTypeBuilder;
    cascade(): DropTypeBuilder;
    $call<T>(func: (qb: this) => T): T;
    toOperationNode(): DropTypeNode;
    compile(): CompiledQuery;
    execute(options?: AbortableQueryOptions): Promise<void>;
}
interface DropTypeBuilderProps {
    readonly queryId: QueryId;
    readonly executor: QueryExecutor;
    readonly node: DropTypeNode;
}
declare class RefreshMaterializedViewBuilder implements OperationNodeSource, Compilable {
    #private;
    constructor(props: RefreshMaterializedViewBuilderProps);
    concurrently(): RefreshMaterializedViewBuilder;
    withData(): RefreshMaterializedViewBuilder;
    withNoData(): RefreshMaterializedViewBuilder;
    $call<T>(func: (qb: this) => T): T;
    toOperationNode(): RefreshMaterializedViewNode;
    compile(): CompiledQuery;
    execute(options?: AbortableQueryOptions): Promise<void>;
}
interface RefreshMaterializedViewBuilderProps {
    readonly queryId: QueryId;
    readonly executor: QueryExecutor;
    readonly node: RefreshMaterializedViewNode;
}
declare class QueryFinalizer<N extends RootOperationNode, O = unknown> implements OperationNodeSource, Compilable {
    #private;
    constructor(props: QueryFinalizerProps<N>);
    toOperationNode(): N;
    compile(): CompiledQuery<O>;
    execute(options?: AbortableQueryOptions): Promise<QueryResult<O>>;
}
interface QueryFinalizerProps<N extends OperationNode> {
    readonly executor: QueryExecutor;
    readonly node: N;
    readonly queryId: QueryId;
}
declare class AlterTypeAddValueBuilder<const V extends string> extends QueryFinalizer<AlterTypeNode> {
    #private;
    constructor(props: AlterTypeAddValueBuilderProps);
    ifNotExists(): AlterTypeAddValueBuilder<V>;
    before<const NV extends string>(neighborValue: NV extends V ? never : NV): AlterTypeAddValueBuilder<V>;
    after<const NV extends string>(neighborValue: NV extends V ? never : NV): AlterTypeAddValueBuilder<V>;
}
interface AlterTypeAddValueBuilderProps {
    readonly executor: QueryExecutor;
    readonly node: AlterTypeNode;
    readonly queryId: QueryId;
}
declare class AlterTypeBuilder<const N extends string> {
    #private;
    constructor(props: AlterTypeBuilderProps);
    addValue<const V extends string>(value: V): AlterTypeAddValueBuilder<V>;
    renameTo<NN extends string>(newName: NN extends N ? never : NN): QueryFinalizer<AlterTypeNode>;
    renameValue<const OV extends string, const NV extends string>(oldValue: OV, newValue: NV extends OV ? never : NV): QueryFinalizer<AlterTypeNode>;
    setSchema<const NS extends string>(schema: NS extends (N extends `${infer S}.${string}` ? S : never) ? never : NS): QueryFinalizer<AlterTypeNode>;
}
interface AlterTypeBuilderProps {
    readonly executor: QueryExecutor;
    readonly node: AlterTypeNode;
    readonly queryId: QueryId;
}
declare class SchemaModule {
    #private;
    constructor(executor: QueryExecutor);
    createTable<TB extends string>(table: TB): CreateTableBuilder<TB, never>;
    dropTable(table: string): DropTableBuilder;
    createIndex(indexName: string): CreateIndexBuilder;
    dropIndex(indexName: string): DropIndexBuilder;
    createSchema(schema: string): CreateSchemaBuilder;
    dropSchema(schema: string): DropSchemaBuilder;
    alterTable(table: string): AlterTableBuilder;
    createView(viewName: string): CreateViewBuilder;
    refreshMaterializedView(viewName: string): RefreshMaterializedViewBuilder;
    dropView(viewName: string): DropViewBuilder;
    createType(typeName: string): CreateTypeBuilder;
    alterType<const N extends string>(name: N): AlterTypeBuilder<N>;
    dropType(typeName: string | string[]): DropTypeBuilder;
    withPlugin(plugin: KyselyPlugin): SchemaModule;
    withoutPlugins(): SchemaModule;
    withSchema(schema: string): SchemaModule;
}
declare class DynamicModule<DB> {
    ref<R extends string = never>(reference: string): DynamicReferenceBuilder<R>;
    table<T extends keyof DB & string>(table: T): DynamicTableBuilder<T>;
}
type InsertObject<DB, TB extends keyof DB> = {
    [C in NonNullableInsertKeys<DB[TB]>]: ValueExpression<DB, TB, InsertType<DB[TB][C]>>;
} & {
    [C in NullableInsertKeys<DB[TB]>]?: ValueExpression<DB, TB, InsertType<DB[TB][C]>> | undefined;
};
type InsertObjectOrList<DB, TB extends keyof DB> = InsertObject<DB, TB> | ReadonlyArray<InsertObject<DB, TB>>;
type InsertObjectOrListFactory<DB, TB extends keyof DB, UT extends keyof DB = never> = (eb: ExpressionBuilder<DB, TB | UT>) => InsertObjectOrList<DB, TB>;
type InsertExpression<DB, TB extends keyof DB, UT extends keyof DB = never> = InsertObjectOrList<DB, TB> | InsertObjectOrListFactory<DB, TB, UT>;
type UpdateObject<DB, TB extends keyof DB, UT extends keyof DB = TB> = DrainOuterGeneric<{
    [C in AnyColumn<DB, UT>]?: {
        [T in UT]: C extends keyof DB[T] ? ValueExpression<DB, TB, UpdateType<DB[T][C]>> | undefined : never;
    }[UT];
}>;
type UpdateObjectFactory<DB, TB extends keyof DB, UT extends keyof DB> = (eb: ExpressionBuilder<DB, TB>) => UpdateObject<DB, TB, UT>;
type UpdateObjectExpression<DB, TB extends keyof DB, UT extends keyof DB = TB> = UpdateObject<DB, TB, UT> | UpdateObjectFactory<DB, TB, UT>;
type ExtractUpdateTypeFromReferenceExpression<DB, TB extends keyof DB, RE, DV = unknown> = UpdateType<ExtractRawTypeFromReferenceExpression<DB, TB, RE, DV>>;
type ReturningRow<DB, TB extends keyof DB, O, SE> = O extends InsertResult | DeleteResult | UpdateResult | MergeResult ? Selection<DB, TB, SE> : O & Selection<DB, TB, SE>;
type ReturningCallbackRow<DB, TB extends keyof DB, O, CB> = O extends InsertResult | DeleteResult | UpdateResult | MergeResult ? CallbackSelection<DB, TB, CB> : O & CallbackSelection<DB, TB, CB>;
type ReturningAllRow<DB, TB extends keyof DB, O> = O extends InsertResult | DeleteResult | UpdateResult | MergeResult ? AllSelection<DB, TB> : O & AllSelection<DB, TB>;
interface ReturningInterface<DB, TB extends keyof DB, O> {
    returning<SE extends SelectExpression<DB, TB>>(selections: ReadonlyArray<SE>): ReturningInterface<DB, TB, ReturningRow<DB, TB, O, SE>>;
    returning<const CB extends SelectCallback<DB, TB>>(callback: CB): ReturningInterface<DB, TB, ReturningCallbackRow<DB, TB, O, CB>>;
    returning<SE extends SelectExpression<DB, TB>>(selection: SE): ReturningInterface<DB, TB, ReturningRow<DB, TB, O, SE>>;
    returningAll(): ReturningInterface<DB, TB, Selectable<DB[TB]>>;
}
interface MultiTableReturningInterface<DB, TB extends keyof DB, O> extends ReturningInterface<DB, TB, O> {
    returningAll<T extends TB>(tables: ReadonlyArray<T>): MultiTableReturningInterface<DB, TB, ReturningAllRow<DB, T, O>>;
    returningAll<T extends TB>(table: T): MultiTableReturningInterface<DB, TB, ReturningAllRow<DB, T, O>>;
    returningAll(): ReturningInterface<DB, TB, Selectable<DB[TB]>>;
}
declare class OnConflictBuilder<DB, TB extends keyof DB> implements WhereInterface<DB, TB> {
    #private;
    constructor(props: OnConflictBuilderProps);
    column(column: AnyColumn<DB, TB>): OnConflictBuilder<DB, TB>;
    columns(columns: ReadonlyArray<AnyColumn<DB, TB>>): OnConflictBuilder<DB, TB>;
    constraint(constraintName: string): OnConflictBuilder<DB, TB>;
    expression(expression: Expression<any>): OnConflictBuilder<DB, TB>;
    where<RE extends ReferenceExpression<DB, TB>, VE extends OperandValueExpressionOrList<DB, TB, RE>>(lhs: RE, op: ComparisonOperatorExpression, rhs: VE): OnConflictBuilder<DB, TB>;
    where<E extends ExpressionOrFactory<DB, TB, SqlBool>>(expression: E): OnConflictBuilder<DB, TB>;
    whereRef<LRE extends ReferenceExpression<DB, TB>, RRE extends ReferenceExpression<DB, TB>>(lhs: LRE, op: ComparisonOperatorExpression, rhs: RRE): OnConflictBuilder<DB, TB>;
    clearWhere(): OnConflictBuilder<DB, TB>;
    doNothing(): OnConflictDoNothingBuilder<DB, TB>;
    doUpdateSet(update: UpdateObjectExpression<OnConflictUpdateDatabase<DB, TB>, OnConflictTables<TB>, OnConflictTables<TB>>): OnConflictUpdateBuilder<OnConflictDatabase<DB, TB>, OnConflictTables<TB>>;
    $call<T>(func: (qb: this) => T): T;
}
interface OnConflictBuilderProps {
    readonly onConflictNode: OnConflictNode;
}
type OnConflictUpdateDatabase<DB, TB extends keyof DB> = {
    [K in keyof DB | 'excluded']: Updateable<K extends keyof DB ? DB[K] : DB[TB]>;
};
type OnConflictDatabase<DB, TB extends keyof DB> = {
    [K in keyof DB | 'excluded']: K extends keyof DB ? DB[K] : DB[TB];
};
type OnConflictTables<TB> = TB | 'excluded';
declare class OnConflictDoNothingBuilder<DB, _TB extends keyof DB> implements OperationNodeSource {
    #private;
    constructor(props: OnConflictBuilderProps);
    toOperationNode(): OnConflictNode;
}
declare class OnConflictUpdateBuilder<DB, TB extends keyof DB> implements WhereInterface<DB, TB>, OperationNodeSource {
    #private;
    constructor(props: OnConflictBuilderProps);
    where<RE extends ReferenceExpression<DB, TB>, VE extends OperandValueExpressionOrList<DB, TB, RE>>(lhs: RE, op: ComparisonOperatorExpression, rhs: VE): OnConflictUpdateBuilder<DB, TB>;
    where<E extends ExpressionOrFactory<DB, TB, SqlBool>>(expression: E): OnConflictUpdateBuilder<DB, TB>;
    whereRef<LRE extends ReferenceExpression<DB, TB>, RRE extends ReferenceExpression<DB, TB>>(lhs: LRE, op: ComparisonOperatorExpression, rhs: RRE): OnConflictUpdateBuilder<DB, TB>;
    clearWhere(): OnConflictUpdateBuilder<DB, TB>;
    $call<T>(func: (qb: this) => T): T;
    toOperationNode(): OnConflictNode;
}
interface OutputInterface<DB, TB extends keyof DB, O, OP extends OutputPrefix = OutputPrefix> {
    output<OE extends OutputExpression<DB, TB, OP>>(selections: ReadonlyArray<OE>): OutputInterface<DB, TB, ReturningRow<DB, TB, O, SelectExpressionFromOutputExpression<OE>>, OP>;
    output<const CB extends OutputCallback<DB, TB, OP>>(callback: CB): OutputInterface<DB, TB, ReturningRow<DB, TB, O, SelectExpressionFromOutputCallback<CB>>, OP>;
    output<OE extends OutputExpression<DB, TB, OP>>(selection: OE): OutputInterface<DB, TB, ReturningRow<DB, TB, O, SelectExpressionFromOutputExpression<OE>>, OP>;
    outputAll(table: OP): OutputInterface<DB, TB, ReturningAllRow<DB, TB, O>, OP>;
}
type OutputPrefix = 'deleted' | 'inserted';
type OutputDatabase<DB, TB extends keyof DB, OP extends OutputPrefix = OutputPrefix> = {
    [K in OP]: DB[TB];
};
type OutputExpression<DB, TB extends keyof DB, OP extends OutputPrefix = OutputPrefix, ODB = OutputDatabase<DB, TB, OP>, OTB extends keyof ODB = keyof ODB> = AnyAliasedColumnWithTable<ODB, OTB> | AnyColumnWithTable<ODB, OTB> | AliasedExpressionOrFactory<ODB, OTB>;
type OutputCallback<DB, TB extends keyof DB, OP extends OutputPrefix = OutputPrefix> = (eb: ExpressionBuilder<OutputDatabase<DB, TB, OP>, OP>) => ReadonlyArray<OutputExpression<DB, TB, OP>>;
type SelectExpressionFromOutputExpression<OE> = OE extends `${OutputPrefix}.${infer C}` ? C : OE;
type SelectExpressionFromOutputCallback<CB> = CB extends (eb: ExpressionBuilder<any, any>) => ReadonlyArray<infer OE> ? SelectExpressionFromOutputExpression<OE> : never;
declare class InsertQueryBuilder<DB, TB extends keyof DB, out O> implements ReturningInterface<DB, TB, O>, OutputInterface<DB, TB, O, 'inserted'>, OperationNodeSource, Compilable<O>, Executable<O>, Explainable, Streamable<O> {
    #private;
    constructor(props: InsertQueryBuilderProps);
    values(insert: InsertExpression<DB, TB>): InsertQueryBuilder<DB, TB, O>;
    columns(columns: ReadonlyArray<keyof DB[TB] & string>): InsertQueryBuilder<DB, TB, O>;
    expression(expression: ExpressionOrFactory<DB, TB, any>): InsertQueryBuilder<DB, TB, O>;
    defaultValues(): InsertQueryBuilder<DB, TB, O>;
    modifyEnd(modifier: Expression<any>): InsertQueryBuilder<DB, TB, O>;
    ignore(): InsertQueryBuilder<DB, TB, O>;
    orIgnore(): InsertQueryBuilder<DB, TB, O>;
    orAbort(): InsertQueryBuilder<DB, TB, O>;
    orFail(): InsertQueryBuilder<DB, TB, O>;
    orReplace(): InsertQueryBuilder<DB, TB, O>;
    orRollback(): InsertQueryBuilder<DB, TB, O>;
    top(expression: number | bigint, modifiers?: 'percent'): InsertQueryBuilder<DB, TB, O>;
    onConflict(callback: (builder: OnConflictBuilder<DB, TB>) => OnConflictUpdateBuilder<OnConflictDatabase<DB, TB>, OnConflictTables<TB>> | OnConflictDoNothingBuilder<DB, TB>): InsertQueryBuilder<DB, TB, O>;
    onDuplicateKeyUpdate(update: UpdateObjectExpression<DB, TB, TB>): InsertQueryBuilder<DB, TB, O>;
    returning<SE extends SelectExpression<DB, TB>>(selections: ReadonlyArray<SE>): InsertQueryBuilder<DB, TB, ReturningRow<DB, TB, O, SE>>;
    returning<const CB extends SelectCallback<DB, TB>>(callback: CB): InsertQueryBuilder<DB, TB, ReturningCallbackRow<DB, TB, O, CB>>;
    returning<SE extends SelectExpression<DB, TB>>(selection: SE): InsertQueryBuilder<DB, TB, ReturningRow<DB, TB, O, SE>>;
    returningAll(): InsertQueryBuilder<DB, TB, Selectable<DB[TB]>>;
    output<OE extends OutputExpression<DB, TB, 'inserted'>>(selections: readonly OE[]): InsertQueryBuilder<DB, TB, ReturningRow<DB, TB, O, SelectExpressionFromOutputExpression<OE>>>;
    output<const CB extends OutputCallback<DB, TB, 'inserted'>>(callback: CB): InsertQueryBuilder<DB, TB, ReturningRow<DB, TB, O, SelectExpressionFromOutputCallback<CB>>>;
    output<OE extends OutputExpression<DB, TB, 'inserted'>>(selection: OE): InsertQueryBuilder<DB, TB, ReturningRow<DB, TB, O, SelectExpressionFromOutputExpression<OE>>>;
    outputAll(table: 'inserted'): InsertQueryBuilder<DB, TB, ReturningAllRow<DB, TB, O>>;
    clearReturning(): InsertQueryBuilder<DB, TB, InsertResult>;
    $call<T>(func: (qb: this) => T): T;
    $if<O2>(condition: boolean, func: (qb: this) => InsertQueryBuilder<any, any, O2>): O2 extends InsertResult ? InsertQueryBuilder<DB, TB, InsertResult> : O2 extends O & infer E ? InsertQueryBuilder<DB, TB, O & Partial<E>> : InsertQueryBuilder<DB, TB, Partial<O2>>;
    $castTo<C>(): InsertQueryBuilder<DB, TB, C>;
    $narrowType<T>(): InsertQueryBuilder<DB, TB, NarrowPartial<O, T>>;
    $assertType<T extends O>(): O extends T ? InsertQueryBuilder<DB, TB, T> : KyselyTypeError<`$assertType() call failed: The type passed in is not equal to the output type of the query.`>;
    withPlugin(plugin: KyselyPlugin): InsertQueryBuilder<DB, TB, O>;
    toOperationNode(): InsertQueryNode;
    compile(): CompiledQuery<O>;
    execute(options?: AbortableQueryOptions): Promise<SimplifyResult<O>[]>;
    executeTakeFirst(options?: AbortableQueryOptions): Promise<SimplifySingleResult<O>>;
    executeTakeFirstOrThrow(errorConstructorOrOptions?: ExecuteTakeFirstOrThrowOptions | ExecuteTakeFirstOrThrowOptions['errorConstructor']): Promise<SimplifyResult<O>>;
    stream(chunkSizeOrOptions?: StreamOptions | StreamOptions['chunkSize']): AsyncIterableIterator<O>;
    explain<ER extends Record<string, any> = Record<string, any>>(format?: ExplainFormat, options?: Expression<any>): Promise<ER[]>;
}
interface InsertQueryBuilderProps {
    readonly queryId: QueryId;
    readonly queryNode: InsertQueryNode;
    readonly executor: QueryExecutor;
}
declare class UpdateQueryBuilder<DB, UT extends keyof DB, TB extends keyof DB, O> implements WhereInterface<DB, TB>, MultiTableReturningInterface<DB, TB, O>, OutputInterface<DB, TB, O>, OrderByInterface<DB, TB, never>, OperationNodeSource, Compilable<O>, Executable<O>, Explainable, Streamable<O> {
    #private;
    constructor(props: UpdateQueryBuilderProps);
    where<RE extends ReferenceExpression<DB, TB>, VE extends OperandValueExpressionOrList<DB, TB, RE>>(lhs: RE, op: ComparisonOperatorExpression, rhs: VE): UpdateQueryBuilder<DB, UT, TB, O>;
    where<E extends ExpressionOrFactory<DB, TB, SqlBool>>(expression: E): UpdateQueryBuilder<DB, UT, TB, O>;
    whereRef<LRE extends ReferenceExpression<DB, TB>, RRE extends ReferenceExpression<DB, TB>>(lhs: LRE, op: ComparisonOperatorExpression, rhs: RRE): UpdateQueryBuilder<DB, UT, TB, O>;
    clearWhere(): UpdateQueryBuilder<DB, UT, TB, O>;
    top(expression: number | bigint, modifiers?: 'percent'): UpdateQueryBuilder<DB, UT, TB, O>;
    from<TE extends TableExpression<DB, TB>>(table: TE): UpdateQueryBuilder<From<DB, TE>, UT, FromTables<DB, TB, TE>, O>;
    from<TE extends TableExpression<DB, TB>>(table: TE[]): UpdateQueryBuilder<From<DB, TE>, UT, FromTables<DB, TB, TE>, O>;
    innerJoin<TE extends TableExpression<DB, TB>, K1 extends JoinReferenceExpression<DB, TB, TE>, K2 extends JoinReferenceExpression<DB, TB, TE>>(table: TE, k1: K1, k2: K2): UpdateQueryBuilderWithInnerJoin<DB, UT, TB, O, TE>;
    innerJoin<TE extends TableExpression<DB, TB>, const FN extends JoinCallbackExpression<DB, TB, TE>>(table: TE, callback: FN): UpdateQueryBuilderWithInnerJoin<DB, UT, TB, O, TE>;
    leftJoin<TE extends TableExpression<DB, TB>, K1 extends JoinReferenceExpression<DB, TB, TE>, K2 extends JoinReferenceExpression<DB, TB, TE>>(table: TE, k1: K1, k2: K2): UpdateQueryBuilderWithLeftJoin<DB, UT, TB, O, TE>;
    leftJoin<TE extends TableExpression<DB, TB>, const FN extends JoinCallbackExpression<DB, TB, TE>>(table: TE, callback: FN): UpdateQueryBuilderWithLeftJoin<DB, UT, TB, O, TE>;
    rightJoin<TE extends TableExpression<DB, TB>, K1 extends JoinReferenceExpression<DB, TB, TE>, K2 extends JoinReferenceExpression<DB, TB, TE>>(table: TE, k1: K1, k2: K2): UpdateQueryBuilderWithRightJoin<DB, UT, TB, O, TE>;
    rightJoin<TE extends TableExpression<DB, TB>, const FN extends JoinCallbackExpression<DB, TB, TE>>(table: TE, callback: FN): UpdateQueryBuilderWithRightJoin<DB, UT, TB, O, TE>;
    fullJoin<TE extends TableExpression<DB, TB>, K1 extends JoinReferenceExpression<DB, TB, TE>, K2 extends JoinReferenceExpression<DB, TB, TE>>(table: TE, k1: K1, k2: K2): UpdateQueryBuilderWithFullJoin<DB, UT, TB, O, TE>;
    fullJoin<TE extends TableExpression<DB, TB>, const FN extends JoinCallbackExpression<DB, TB, TE>>(table: TE, callback: FN): UpdateQueryBuilderWithFullJoin<DB, UT, TB, O, TE>;
    orderBy<OE extends OrderByExpression<DB, TB, {}>>(expr: OE, modifiers?: OrderByModifiers): UpdateQueryBuilder<DB, UT, TB, O>;
    orderBy<OE extends OrderByExpression<DB, TB, {}> | DirectedOrderByStringReference<DB, TB, {}>>(exprs: ReadonlyArray<OE>): UpdateQueryBuilder<DB, UT, TB, O>;
    orderBy<OE extends DirectedOrderByStringReference<DB, TB, {}>>(expr: OE): UpdateQueryBuilder<DB, UT, TB, O>;
    orderBy<OE extends OrderByExpression<DB, TB, {}>>(expr: OE, modifiers: Expression<any>): UpdateQueryBuilder<DB, UT, TB, O>;
    clearOrderBy(): UpdateQueryBuilder<DB, UT, TB, O>;
    limit(limit: ValueExpression<DB, TB, number>): UpdateQueryBuilder<DB, UT, TB, O>;
    set(update: UpdateObjectExpression<DB, TB, UT>): UpdateQueryBuilder<DB, UT, TB, O>;
    set<RE extends ReferenceExpression<DB, UT>>(key: RE, value: ValueExpression<DB, TB, ExtractUpdateTypeFromReferenceExpression<DB, UT, RE>>): UpdateQueryBuilder<DB, UT, TB, O>;
    returning<SE extends SelectExpression<DB, TB>>(selections: ReadonlyArray<SE>): UpdateQueryBuilder<DB, UT, TB, ReturningRow<DB, TB, O, SE>>;
    returning<const CB extends SelectCallback<DB, TB>>(callback: CB): UpdateQueryBuilder<DB, UT, TB, ReturningCallbackRow<DB, TB, O, CB>>;
    returning<SE extends SelectExpression<DB, TB>>(selection: SE): UpdateQueryBuilder<DB, UT, TB, ReturningRow<DB, TB, O, SE>>;
    returningAll<T extends TB>(tables: ReadonlyArray<T>): UpdateQueryBuilder<DB, UT, TB, ReturningAllRow<DB, T, O>>;
    returningAll<T extends TB>(table: T): UpdateQueryBuilder<DB, UT, TB, ReturningAllRow<DB, T, O>>;
    returningAll(): UpdateQueryBuilder<DB, UT, TB, ReturningAllRow<DB, TB, O>>;
    output<OE extends OutputExpression<DB, UT>>(selections: readonly OE[]): UpdateQueryBuilder<DB, UT, TB, ReturningRow<DB, TB, O, SelectExpressionFromOutputExpression<OE>>>;
    output<const CB extends OutputCallback<DB, TB>>(callback: CB): UpdateQueryBuilder<DB, UT, TB, ReturningRow<DB, TB, O, SelectExpressionFromOutputCallback<CB>>>;
    output<OE extends OutputExpression<DB, TB>>(selection: OE): UpdateQueryBuilder<DB, UT, TB, ReturningRow<DB, TB, O, SelectExpressionFromOutputExpression<OE>>>;
    outputAll(table: OutputPrefix): UpdateQueryBuilder<DB, UT, TB, ReturningAllRow<DB, TB, O>>;
    modifyEnd(modifier: Expression<any>): UpdateQueryBuilder<DB, UT, TB, O>;
    clearReturning(): UpdateQueryBuilder<DB, UT, TB, UpdateResult>;
    $call<T>(func: (qb: this) => T): T;
    $if<O2>(condition: boolean, func: (qb: this) => UpdateQueryBuilder<any, any, any, O2>): unknown extends O2 ? UpdateQueryBuilder<any, any, any, O2> : O2 extends UpdateResult ? UpdateQueryBuilder<DB, UT, TB, UpdateResult> : O2 extends O & infer E ? UpdateQueryBuilder<DB, UT, TB, O & Partial<E>> : UpdateQueryBuilder<DB, UT, TB, Partial<O2>>;
    $castTo<C>(): UpdateQueryBuilder<DB, UT, TB, C>;
    $narrowType<T>(): UpdateQueryBuilder<DB, UT, TB, NarrowPartial<O, T>>;
    $assertType<T extends O>(): O extends T ? UpdateQueryBuilder<DB, UT, TB, T> : KyselyTypeError<`$assertType() call failed: The type passed in is not equal to the output type of the query.`>;
    withPlugin(plugin: KyselyPlugin): UpdateQueryBuilder<DB, UT, TB, O>;
    toOperationNode(): UpdateQueryNode;
    compile(): CompiledQuery<SimplifyResult<O>>;
    execute(options?: AbortableQueryOptions): Promise<SimplifyResult<O>[]>;
    executeTakeFirst(options?: AbortableQueryOptions): Promise<SimplifySingleResult<O>>;
    executeTakeFirstOrThrow(errorConstructorOrOptions?: ExecuteTakeFirstOrThrowOptions | ExecuteTakeFirstOrThrowOptions['errorConstructor']): Promise<SimplifyResult<O>>;
    stream(chunkSizeOrOptions?: StreamOptions | StreamOptions['chunkSize']): AsyncIterableIterator<O>;
    explain<ER extends Record<string, any> = Record<string, any>>(format?: ExplainFormat, options?: Expression<any>): Promise<ER[]>;
}
interface UpdateQueryBuilderProps {
    readonly queryId: QueryId;
    readonly queryNode: UpdateQueryNode;
    readonly executor: QueryExecutor;
}
type UpdateQueryBuilderWithInnerJoin<DB, UT extends keyof DB, TB extends keyof DB, O, TE extends TableExpression<DB, TB>> = TE extends `${infer T} as ${infer A}` ? T extends keyof DB ? InnerJoinedBuilder$1<DB, UT, TB, O, A, DB[T]> : never : TE extends keyof DB ? UpdateQueryBuilder<DB, UT, TB | TE, O> : TE extends AliasedExpression<infer QO, infer QA> ? InnerJoinedBuilder$1<DB, UT, TB, O, QA, QO> : TE extends (qb: any) => AliasedExpression<infer QO, infer QA> ? InnerJoinedBuilder$1<DB, UT, TB, O, QA, QO> : never;
type InnerJoinedBuilder$1<DB, UT extends keyof DB, TB extends keyof DB, O, A extends string, R> = A extends keyof DB ? UpdateQueryBuilder<InnerJoinedDB$1<DB, A, R>, UT, TB | A, O> : UpdateQueryBuilder<DB & ShallowRecord<A, R>, UT, TB | A, O>;
type InnerJoinedDB$1<DB, A extends string, R> = DrainOuterGeneric<{
    [C in keyof DB | A]: C extends A ? R : C extends keyof DB ? DB[C] : never;
}>;
type UpdateQueryBuilderWithLeftJoin<DB, UT extends keyof DB, TB extends keyof DB, O, TE extends TableExpression<DB, TB>> = TE extends `${infer T} as ${infer A}` ? T extends keyof DB ? LeftJoinedBuilder$1<DB, UT, TB, O, A, DB[T]> : never : TE extends keyof DB ? LeftJoinedBuilder$1<DB, UT, TB, O, TE, DB[TE]> : TE extends AliasedExpression<infer QO, infer QA> ? LeftJoinedBuilder$1<DB, UT, TB, O, QA, QO> : TE extends (qb: any) => AliasedExpression<infer QO, infer QA> ? LeftJoinedBuilder$1<DB, UT, TB, O, QA, QO> : never;
type LeftJoinedBuilder$1<DB, UT extends keyof DB, TB extends keyof DB, O, A extends keyof any, R> = A extends keyof DB ? UpdateQueryBuilder<LeftJoinedDB$1<DB, A, R>, UT, TB | A, O> : UpdateQueryBuilder<DB & ShallowRecord<A, Nullable<R>>, UT, TB | A, O>;
type LeftJoinedDB$1<DB, A extends keyof any, R> = DrainOuterGeneric<{
    [C in keyof DB | A]: C extends A ? Nullable<R> : C extends keyof DB ? DB[C] : never;
}>;
type UpdateQueryBuilderWithRightJoin<DB, UT extends keyof DB, TB extends keyof DB, O, TE extends TableExpression<DB, TB>> = TE extends `${infer T} as ${infer A}` ? T extends keyof DB ? RightJoinedBuilder$1<DB, UT, TB, O, A, DB[T]> : never : TE extends keyof DB ? RightJoinedBuilder$1<DB, UT, TB, O, TE, DB[TE]> : TE extends AliasedExpression<infer QO, infer QA> ? RightJoinedBuilder$1<DB, UT, TB, O, QA, QO> : TE extends (qb: any) => AliasedExpression<infer QO, infer QA> ? RightJoinedBuilder$1<DB, UT, TB, O, QA, QO> : never;
type RightJoinedBuilder$1<DB, UT extends keyof DB, TB extends keyof DB, O, A extends keyof any, R> = UpdateQueryBuilder<RightJoinedDB$1<DB, TB, A, R>, UT, TB | A, O>;
type RightJoinedDB$1<DB, TB extends keyof DB, A extends keyof any, R> = DrainOuterGeneric<{
    [C in keyof DB | A]: C extends A ? R : C extends TB ? Nullable<DB[C]> : C extends keyof DB ? DB[C] : never;
}>;
type UpdateQueryBuilderWithFullJoin<DB, UT extends keyof DB, TB extends keyof DB, O, TE extends TableExpression<DB, TB>> = TE extends `${infer T} as ${infer A}` ? T extends keyof DB ? OuterJoinedBuilder$1<DB, UT, TB, O, A, DB[T]> : never : TE extends keyof DB ? OuterJoinedBuilder$1<DB, UT, TB, O, TE, DB[TE]> : TE extends AliasedExpression<infer QO, infer QA> ? OuterJoinedBuilder$1<DB, UT, TB, O, QA, QO> : TE extends (qb: any) => AliasedExpression<infer QO, infer QA> ? OuterJoinedBuilder$1<DB, UT, TB, O, QA, QO> : never;
type OuterJoinedBuilder$1<DB, UT extends keyof DB, TB extends keyof DB, O, A extends keyof any, R> = UpdateQueryBuilder<OuterJoinedBuilderDB$1<DB, TB, A, R>, UT, TB | A, O>;
type OuterJoinedBuilderDB$1<DB, TB extends keyof DB, A extends keyof any, R> = DrainOuterGeneric<{
    [C in keyof DB | A]: C extends A ? Nullable<R> : C extends TB ? Nullable<DB[C]> : C extends keyof DB ? DB[C] : never;
}>;
declare class DeleteQueryBuilder<DB, TB extends keyof DB, O> implements WhereInterface<DB, TB>, MultiTableReturningInterface<DB, TB, O>, OutputInterface<DB, TB, O, 'deleted'>, OrderByInterface<DB, TB, {}>, OperationNodeSource, Compilable<O>, Executable<O>, Explainable, Streamable<O> {
    #private;
    constructor(props: DeleteQueryBuilderProps);
    where<RE extends ReferenceExpression<DB, TB>, VE extends OperandValueExpressionOrList<DB, TB, RE>>(lhs: RE, op: ComparisonOperatorExpression, rhs: VE): DeleteQueryBuilder<DB, TB, O>;
    where<E extends ExpressionOrFactory<DB, TB, SqlBool>>(expression: E): DeleteQueryBuilder<DB, TB, O>;
    whereRef<LRE extends ReferenceExpression<DB, TB>, RRE extends ReferenceExpression<DB, TB>>(lhs: LRE, op: ComparisonOperatorExpression, rhs: RRE): DeleteQueryBuilder<DB, TB, O>;
    clearWhere(): DeleteQueryBuilder<DB, TB, O>;
    top(expression: number | bigint, modifiers?: 'percent'): DeleteQueryBuilder<DB, TB, O>;
    using<TE extends TableExpression<DB, keyof DB>>(tables: TE[]): DeleteQueryBuilder<From<DB, TE>, FromTables<DB, TB, TE>, O>;
    using<TE extends TableExpression<DB, keyof DB>>(table: TE): DeleteQueryBuilder<From<DB, TE>, FromTables<DB, TB, TE>, O>;
    innerJoin<TE extends TableExpression<DB, TB>, K1 extends JoinReferenceExpression<DB, TB, TE>, K2 extends JoinReferenceExpression<DB, TB, TE>>(table: TE, k1: K1, k2: K2): DeleteQueryBuilderWithInnerJoin<DB, TB, O, TE>;
    innerJoin<TE extends TableExpression<DB, TB>, const FN extends JoinCallbackExpression<DB, TB, TE>>(table: TE, callback: FN): DeleteQueryBuilderWithInnerJoin<DB, TB, O, TE>;
    leftJoin<TE extends TableExpression<DB, TB>, K1 extends JoinReferenceExpression<DB, TB, TE>, K2 extends JoinReferenceExpression<DB, TB, TE>>(table: TE, k1: K1, k2: K2): DeleteQueryBuilderWithLeftJoin<DB, TB, O, TE>;
    leftJoin<TE extends TableExpression<DB, TB>, const FN extends JoinCallbackExpression<DB, TB, TE>>(table: TE, callback: FN): DeleteQueryBuilderWithLeftJoin<DB, TB, O, TE>;
    rightJoin<TE extends TableExpression<DB, TB>, K1 extends JoinReferenceExpression<DB, TB, TE>, K2 extends JoinReferenceExpression<DB, TB, TE>>(table: TE, k1: K1, k2: K2): DeleteQueryBuilderWithRightJoin<DB, TB, O, TE>;
    rightJoin<TE extends TableExpression<DB, TB>, const FN extends JoinCallbackExpression<DB, TB, TE>>(table: TE, callback: FN): DeleteQueryBuilderWithRightJoin<DB, TB, O, TE>;
    fullJoin<TE extends TableExpression<DB, TB>, K1 extends JoinReferenceExpression<DB, TB, TE>, K2 extends JoinReferenceExpression<DB, TB, TE>>(table: TE, k1: K1, k2: K2): DeleteQueryBuilderWithFullJoin<DB, TB, O, TE>;
    fullJoin<TE extends TableExpression<DB, TB>, const FN extends JoinCallbackExpression<DB, TB, TE>>(table: TE, callback: FN): DeleteQueryBuilderWithFullJoin<DB, TB, O, TE>;
    returning<SE extends SelectExpression<DB, TB>>(selections: ReadonlyArray<SE>): DeleteQueryBuilder<DB, TB, ReturningRow<DB, TB, O, SE>>;
    returning<const CB extends SelectCallback<DB, TB>>(callback: CB): DeleteQueryBuilder<DB, TB, ReturningCallbackRow<DB, TB, O, CB>>;
    returning<SE extends SelectExpression<DB, TB>>(selection: SE): DeleteQueryBuilder<DB, TB, ReturningRow<DB, TB, O, SE>>;
    returningAll<T extends TB>(tables: ReadonlyArray<T>): DeleteQueryBuilder<DB, TB, ReturningAllRow<DB, T, O>>;
    returningAll<T extends TB>(table: T): DeleteQueryBuilder<DB, TB, ReturningAllRow<DB, T, O>>;
    returningAll(): DeleteQueryBuilder<DB, TB, ReturningAllRow<DB, TB, O>>;
    output<OE extends OutputExpression<DB, TB, 'deleted'>>(selections: readonly OE[]): DeleteQueryBuilder<DB, TB, ReturningRow<DB, TB, O, SelectExpressionFromOutputExpression<OE>>>;
    output<const CB extends OutputCallback<DB, TB, 'deleted'>>(callback: CB): DeleteQueryBuilder<DB, TB, ReturningRow<DB, TB, O, SelectExpressionFromOutputCallback<CB>>>;
    output<OE extends OutputExpression<DB, TB, 'deleted'>>(selection: OE): DeleteQueryBuilder<DB, TB, ReturningRow<DB, TB, O, SelectExpressionFromOutputExpression<OE>>>;
    outputAll(table: 'deleted'): DeleteQueryBuilder<DB, TB, ReturningAllRow<DB, TB, O>>;
    clearReturning(): DeleteQueryBuilder<DB, TB, DeleteResult>;
    clearLimit(): DeleteQueryBuilder<DB, TB, O>;
    orderBy<OE extends OrderByExpression<DB, TB, {}>>(expr: OE, modifiers?: OrderByModifiers): DeleteQueryBuilder<DB, TB, O>;
    orderBy<OE extends OrderByExpression<DB, TB, {}> | DirectedOrderByStringReference<DB, TB, {}>>(exprs: ReadonlyArray<OE>): DeleteQueryBuilder<DB, TB, O>;
    orderBy<OE extends DirectedOrderByStringReference<DB, TB, {}>>(expr: OE): DeleteQueryBuilder<DB, TB, O>;
    orderBy<OE extends OrderByExpression<DB, TB, {}>>(expr: OE, modifiers: Expression<any>): DeleteQueryBuilder<DB, TB, O>;
    clearOrderBy(): DeleteQueryBuilder<DB, TB, O>;
    limit<VE extends ValueExpression<DB, TB, number>>(limit: VE): DeleteQueryBuilder<DB, TB, O>;
    modifyEnd(modifier: Expression<any>): DeleteQueryBuilder<DB, TB, O>;
    $call<T>(func: (qb: this) => T): T;
    $if<O2>(condition: boolean, func: (qb: this) => DeleteQueryBuilder<any, any, O2>): unknown extends O2 ? DeleteQueryBuilder<any, any, O2> : O2 extends DeleteResult ? DeleteQueryBuilder<DB, TB, DeleteResult> : O2 extends O & infer E ? DeleteQueryBuilder<DB, TB, O & Partial<E>> : DeleteQueryBuilder<DB, TB, Partial<O2>>;
    $castTo<C>(): DeleteQueryBuilder<DB, TB, C>;
    $narrowType<T>(): DeleteQueryBuilder<DB, TB, NarrowPartial<O, T>>;
    $assertType<T extends O>(): O extends T ? DeleteQueryBuilder<DB, TB, T> : KyselyTypeError<`$assertType() call failed: The type passed in is not equal to the output type of the query.`>;
    withPlugin(plugin: KyselyPlugin): DeleteQueryBuilder<DB, TB, O>;
    toOperationNode(): DeleteQueryNode;
    compile(): CompiledQuery<SimplifyResult<O>>;
    execute(options?: AbortableQueryOptions): Promise<SimplifyResult<O>[]>;
    executeTakeFirst(options?: AbortableQueryOptions): Promise<SimplifySingleResult<O>>;
    executeTakeFirstOrThrow(errorConstructorOrOptions?: ExecuteTakeFirstOrThrowOptions | ExecuteTakeFirstOrThrowOptions['errorConstructor']): Promise<SimplifyResult<O>>;
    stream(chunkSizeOrOptions?: StreamOptions | StreamOptions['chunkSize']): AsyncIterableIterator<O>;
    explain<ER extends Record<string, any> = Record<string, any>>(format?: ExplainFormat, options?: Expression<any>): Promise<ER[]>;
}
interface DeleteQueryBuilderProps {
    readonly queryId: QueryId;
    readonly queryNode: DeleteQueryNode;
    readonly executor: QueryExecutor;
}
type DeleteQueryBuilderWithInnerJoin<DB, TB extends keyof DB, O, TE extends TableExpression<DB, TB>> = TE extends `${infer T} as ${infer A}` ? T extends keyof DB ? InnerJoinedBuilder<DB, TB, O, A, DB[T]> : never : TE extends keyof DB ? DeleteQueryBuilder<DB, TB | TE, O> : TE extends AliasedExpression<infer QO, infer QA> ? InnerJoinedBuilder<DB, TB, O, QA, QO> : TE extends (qb: any) => AliasedExpression<infer QO, infer QA> ? InnerJoinedBuilder<DB, TB, O, QA, QO> : never;
type InnerJoinedBuilder<DB, TB extends keyof DB, O, A extends string, R> = A extends keyof DB ? DeleteQueryBuilder<InnerJoinedDB<DB, A, R>, TB | A, O> : DeleteQueryBuilder<DB & ShallowRecord<A, R>, TB | A, O>;
type InnerJoinedDB<DB, A extends string, R> = DrainOuterGeneric<{
    [C in keyof DB | A]: C extends A ? R : C extends keyof DB ? DB[C] : never;
}>;
type DeleteQueryBuilderWithLeftJoin<DB, TB extends keyof DB, O, TE extends TableExpression<DB, TB>> = TE extends `${infer T} as ${infer A}` ? T extends keyof DB ? LeftJoinedBuilder<DB, TB, O, A, DB[T]> : never : TE extends keyof DB ? LeftJoinedBuilder<DB, TB, O, TE, DB[TE]> : TE extends AliasedExpression<infer QO, infer QA> ? LeftJoinedBuilder<DB, TB, O, QA, QO> : TE extends (qb: any) => AliasedExpression<infer QO, infer QA> ? LeftJoinedBuilder<DB, TB, O, QA, QO> : never;
type LeftJoinedBuilder<DB, TB extends keyof DB, O, A extends keyof any, R> = A extends keyof DB ? DeleteQueryBuilder<LeftJoinedDB<DB, A, R>, TB | A, O> : DeleteQueryBuilder<DB & ShallowRecord<A, Nullable<R>>, TB | A, O>;
type LeftJoinedDB<DB, A extends keyof any, R> = DrainOuterGeneric<{
    [C in keyof DB | A]: C extends A ? Nullable<R> : C extends keyof DB ? DB[C] : never;
}>;
type DeleteQueryBuilderWithRightJoin<DB, TB extends keyof DB, O, TE extends TableExpression<DB, TB>> = TE extends `${infer T} as ${infer A}` ? T extends keyof DB ? RightJoinedBuilder<DB, TB, O, A, DB[T]> : never : TE extends keyof DB ? RightJoinedBuilder<DB, TB, O, TE, DB[TE]> : TE extends AliasedExpression<infer QO, infer QA> ? RightJoinedBuilder<DB, TB, O, QA, QO> : TE extends (qb: any) => AliasedExpression<infer QO, infer QA> ? RightJoinedBuilder<DB, TB, O, QA, QO> : never;
type RightJoinedBuilder<DB, TB extends keyof DB, O, A extends keyof any, R> = DeleteQueryBuilder<RightJoinedDB<DB, TB, A, R>, TB | A, O>;
type RightJoinedDB<DB, TB extends keyof DB, A extends keyof any, R> = DrainOuterGeneric<{
    [C in keyof DB | A]: C extends A ? R : C extends TB ? Nullable<DB[C]> : C extends keyof DB ? DB[C] : never;
}>;
type DeleteQueryBuilderWithFullJoin<DB, TB extends keyof DB, O, TE extends TableExpression<DB, TB>> = TE extends `${infer T} as ${infer A}` ? T extends keyof DB ? OuterJoinedBuilder<DB, TB, O, A, DB[T]> : never : TE extends keyof DB ? OuterJoinedBuilder<DB, TB, O, TE, DB[TE]> : TE extends AliasedExpression<infer QO, infer QA> ? OuterJoinedBuilder<DB, TB, O, QA, QO> : TE extends (qb: any) => AliasedExpression<infer QO, infer QA> ? OuterJoinedBuilder<DB, TB, O, QA, QO> : never;
type OuterJoinedBuilder<DB, TB extends keyof DB, O, A extends keyof any, R> = DeleteQueryBuilder<OuterJoinedBuilderDB<DB, TB, A, R>, TB | A, O>;
type OuterJoinedBuilderDB<DB, TB extends keyof DB, A extends keyof any, R> = DrainOuterGeneric<{
    [C in keyof DB | A]: C extends A ? Nullable<R> : C extends TB ? Nullable<DB[C]> : C extends keyof DB ? DB[C] : never;
}>;
declare class CTEBuilder<N extends string> implements OperationNodeSource {
    #private;
    constructor(props: CTEBuilderProps);
    materialized(): CTEBuilder<N>;
    notMaterialized(): CTEBuilder<N>;
    toOperationNode(): CommonTableExpressionNode;
}
interface CTEBuilderProps {
    readonly node: CommonTableExpressionNode;
}
type CTEBuilderCallback<N extends string> = (cte: <N2 extends string>(name: N2) => CTEBuilder<N2>) => CTEBuilder<N>;
type CommonTableExpression<DB, CN> = CommonTableExpressionOutput<DB, CN> | CommonTableExpressionFactory<DB, CN>;
type CommonTableExpressionFactory<DB, CN> = (creator: QueryCreator<DB>) => CommonTableExpressionOutput<DB, CN>;
type RecursiveCommonTableExpression<DB, CN extends string> = (creator: QueryCreator<DB & {
    [K in ExtractTableFromCommonTableExpressionName<CN>]: ExtractRowFromCommonTableExpressionName<CN>;
}>) => CommonTableExpressionOutput<DB, CN>;
type QueryCreatorWithCommonTableExpression<DB, CN extends string, CTE> = QueryCreator<DB & {
    [K in ExtractTableFromCommonTableExpressionName<CN>]: ExtractRowFromCommonTableExpression<CTE>;
}>;
type CommonTableExpressionOutput<DB, CN> = Expression<ExtractRowFromCommonTableExpressionName<CN>> | InsertQueryBuilder<DB, any, ExtractRowFromCommonTableExpressionName<CN>> | UpdateQueryBuilder<DB, any, any, ExtractRowFromCommonTableExpressionName<CN>> | DeleteQueryBuilder<DB, any, ExtractRowFromCommonTableExpressionName<CN>>;
type ExtractRowFromCommonTableExpression<CTE> = CTE extends Expression<infer O> | Compilable<infer O> ? O : CTE extends (creator: QueryCreator<any>) => infer Q ? Q extends Expression<infer O> | Compilable<infer O> ? O : never : never;
type ExtractTableFromCommonTableExpressionName<CN> = CN extends `${infer TB}(${string})` ? TB : CN;
type ExtractRowFromCommonTableExpressionName<CN> = CN extends `${string}(${infer CL})` ? {
    [C in ExtractColumnNamesFromColumnList<CL>]: any;
} : ShallowRecord<string, any>;
type ExtractColumnNamesFromColumnList<R> = R extends `${infer C}, ${infer RS}` ? C | ExtractColumnNamesFromColumnList<RS> : R;
type DeleteFrom<DB, TE extends TableExpressionOrList<DB, never>> = [
    TE
] extends [
    keyof DB
] ? DeleteQueryBuilder<DB, ExtractTableAlias<DB, TE>, DeleteResult> : [
    TE
] extends [
    `${infer T} as ${infer A}`
] ? T extends keyof DB ? DeleteQueryBuilder<DB & ShallowRecord<A, DB[T]>, A, DeleteResult> : never : TE extends ReadonlyArray<infer T> ? DeleteQueryBuilder<From<DB, T>, FromTables<DB, never, T>, DeleteResult> : DeleteQueryBuilder<From<DB, TE>, FromTables<DB, never, TE>, DeleteResult>;
type UpdateTable<DB, TE extends TableExpressionOrList<DB, never>> = [
    TE
] extends [
    keyof DB
] ? UpdateQueryBuilder<DB, ExtractTableAlias<DB, TE>, ExtractTableAlias<DB, TE>, UpdateResult> : [
    TE
] extends [
    `${infer T} as ${infer A}`
] ? T extends keyof DB ? UpdateQueryBuilder<DB & ShallowRecord<A, DB[T]>, A, A, UpdateResult> : never : TE extends ReadonlyArray<infer T> ? UpdateQueryBuilder<From<DB, T>, FromTables<DB, never, T>, FromTables<DB, never, T>, UpdateResult> : UpdateQueryBuilder<From<DB, TE>, FromTables<DB, never, TE>, FromTables<DB, never, TE>, UpdateResult>;
declare class MergeQueryBuilder<DB, TT extends keyof DB, O> implements MultiTableReturningInterface<DB, TT, O>, OutputInterface<DB, TT, O> {
    #private;
    constructor(props: MergeQueryBuilderProps);
    modifyEnd(modifier: Expression<any>): MergeQueryBuilder<DB, TT, O>;
    top(expression: number | bigint, modifiers?: 'percent'): MergeQueryBuilder<DB, TT, O>;
    using<TE extends TableExpression<DB, TT>, K1 extends JoinReferenceExpression<DB, TT, TE>, K2 extends JoinReferenceExpression<DB, TT, TE>>(sourceTable: TE, k1: K1, k2: K2): ExtractWheneableMergeQueryBuilder<DB, TT, TE, O>;
    using<TE extends TableExpression<DB, TT>, const FN extends JoinCallbackExpression<DB, TT, TE>>(sourceTable: TE, callback: FN): ExtractWheneableMergeQueryBuilder<DB, TT, TE, O>;
    returning<SE extends SelectExpression<DB, TT>>(selections: ReadonlyArray<SE>): MergeQueryBuilder<DB, TT, ReturningRow<DB, TT, O, SE>>;
    returning<const CB extends SelectCallback<DB, TT>>(callback: CB): MergeQueryBuilder<DB, TT, ReturningCallbackRow<DB, TT, O, CB>>;
    returning<SE extends SelectExpression<DB, TT>>(selection: SE): MergeQueryBuilder<DB, TT, ReturningRow<DB, TT, O, SE>>;
    returningAll<T extends TT>(table: T): MergeQueryBuilder<DB, TT, ReturningAllRow<DB, T, O>>;
    returningAll(): MergeQueryBuilder<DB, TT, ReturningAllRow<DB, TT, O>>;
    output<OE extends OutputExpression<DB, TT>>(selections: readonly OE[]): MergeQueryBuilder<DB, TT, ReturningRow<DB, TT, O, SelectExpressionFromOutputExpression<OE>>>;
    output<const CB extends OutputCallback<DB, TT>>(callback: CB): MergeQueryBuilder<DB, TT, ReturningRow<DB, TT, O, SelectExpressionFromOutputCallback<CB>>>;
    output<OE extends OutputExpression<DB, TT>>(selection: OE): MergeQueryBuilder<DB, TT, ReturningRow<DB, TT, O, SelectExpressionFromOutputExpression<OE>>>;
    outputAll(table: OutputPrefix): MergeQueryBuilder<DB, TT, ReturningAllRow<DB, TT, O>>;
}
interface MergeQueryBuilderProps {
    readonly queryId: QueryId;
    readonly queryNode: MergeQueryNode;
    readonly executor: QueryExecutor;
}
declare class WheneableMergeQueryBuilder<DB, TT extends keyof DB, ST extends keyof DB, O> implements MultiTableReturningInterface<DB, TT | ST, O>, OutputInterface<DB, TT, O>, OperationNodeSource, Compilable<O>, Executable<O> {
    #private;
    constructor(props: MergeQueryBuilderProps);
    modifyEnd(modifier: Expression<any>): WheneableMergeQueryBuilder<DB, TT, ST, O>;
    top(expression: number | bigint, modifiers?: 'percent'): WheneableMergeQueryBuilder<DB, TT, ST, O>;
    whenMatched(): MatchedThenableMergeQueryBuilder<DB, TT, ST, TT | ST, O>;
    whenMatchedAnd<RE extends ReferenceExpression<DB, TT | ST>, VE extends OperandValueExpressionOrList<DB, TT | ST, RE>>(lhs: RE, op: ComparisonOperatorExpression, rhs: VE): MatchedThenableMergeQueryBuilder<DB, TT, ST, TT | ST, O>;
    whenMatchedAnd<E extends ExpressionOrFactory<DB, TT | ST, SqlBool>>(expression: E): MatchedThenableMergeQueryBuilder<DB, TT, ST, TT | ST, O>;
    whenMatchedAndRef<LRE extends ReferenceExpression<DB, TT | ST>, RRE extends ReferenceExpression<DB, TT | ST>>(lhs: LRE, op: ComparisonOperatorExpression, rhs: RRE): MatchedThenableMergeQueryBuilder<DB, TT, ST, TT | ST, O>;
    whenNotMatched(): NotMatchedThenableMergeQueryBuilder<DB, TT, ST, O>;
    whenNotMatchedAnd<RE extends ReferenceExpression<DB, ST>, VE extends OperandValueExpressionOrList<DB, ST, RE>>(lhs: RE, op: ComparisonOperatorExpression, rhs: VE): NotMatchedThenableMergeQueryBuilder<DB, TT, ST, O>;
    whenNotMatchedAnd<E extends ExpressionOrFactory<DB, ST, SqlBool>>(expression: E): NotMatchedThenableMergeQueryBuilder<DB, TT, ST, O>;
    whenNotMatchedAndRef<LRE extends ReferenceExpression<DB, ST>, RRE extends ReferenceExpression<DB, ST>>(lhs: LRE, op: ComparisonOperatorExpression, rhs: RRE): NotMatchedThenableMergeQueryBuilder<DB, TT, ST, O>;
    whenNotMatchedBySource(): MatchedThenableMergeQueryBuilder<DB, TT, ST, TT, O>;
    whenNotMatchedBySourceAnd<RE extends ReferenceExpression<DB, TT>, VE extends OperandValueExpressionOrList<DB, TT, RE>>(lhs: RE, op: ComparisonOperatorExpression, rhs: VE): MatchedThenableMergeQueryBuilder<DB, TT, ST, TT, O>;
    whenNotMatchedBySourceAnd<E extends ExpressionOrFactory<DB, TT, SqlBool>>(expression: E): MatchedThenableMergeQueryBuilder<DB, TT, ST, TT, O>;
    whenNotMatchedBySourceAndRef<LRE extends ReferenceExpression<DB, TT>, RRE extends ReferenceExpression<DB, TT>>(lhs: LRE, op: ComparisonOperatorExpression, rhs: RRE): MatchedThenableMergeQueryBuilder<DB, TT, ST, TT, O>;
    returning<SE extends SelectExpression<DB, TT | ST>>(selections: ReadonlyArray<SE>): WheneableMergeQueryBuilder<DB, TT, ST, ReturningRow<DB, TT | ST, O, SE>>;
    returning<const CB extends SelectCallback<DB, TT | ST>>(callback: CB): WheneableMergeQueryBuilder<DB, TT, ST, ReturningCallbackRow<DB, TT | ST, O, CB>>;
    returning<SE extends SelectExpression<DB, TT | ST>>(selection: SE): WheneableMergeQueryBuilder<DB, TT, ST, ReturningRow<DB, TT | ST, O, SE>>;
    returningAll<T extends TT | ST>(table: T): WheneableMergeQueryBuilder<DB, TT, ST, ReturningAllRow<DB, T, O>>;
    returningAll(): WheneableMergeQueryBuilder<DB, TT, ST, ReturningAllRow<DB, TT | ST, O>>;
    output<OE extends OutputExpression<DB, TT>>(selections: readonly OE[]): WheneableMergeQueryBuilder<DB, TT, ST, ReturningRow<DB, TT, O, SelectExpressionFromOutputExpression<OE>>>;
    output<const CB extends OutputCallback<DB, TT>>(callback: CB): WheneableMergeQueryBuilder<DB, TT, ST, ReturningRow<DB, TT, O, SelectExpressionFromOutputCallback<CB>>>;
    output<OE extends OutputExpression<DB, TT>>(selection: OE): WheneableMergeQueryBuilder<DB, TT, ST, ReturningRow<DB, TT, O, SelectExpressionFromOutputExpression<OE>>>;
    outputAll(table: OutputPrefix): WheneableMergeQueryBuilder<DB, TT, ST, ReturningAllRow<DB, TT, O>>;
    $call<T>(func: (qb: this) => T): T;
    $if<O2>(condition: boolean, func: (qb: this) => WheneableMergeQueryBuilder<any, any, any, O2>): O2 extends MergeResult ? WheneableMergeQueryBuilder<DB, TT, ST, MergeResult> : O2 extends O & infer E ? WheneableMergeQueryBuilder<DB, TT, ST, O & Partial<E>> : WheneableMergeQueryBuilder<DB, TT, ST, Partial<O2>>;
    toOperationNode(): MergeQueryNode;
    compile(): CompiledQuery<O>;
    execute(options?: AbortableQueryOptions): Promise<SimplifyResult<O>[]>;
    executeTakeFirst(options?: AbortableQueryOptions): Promise<SimplifySingleResult<O>>;
    executeTakeFirstOrThrow(errorConstructorOrOptions?: ExecuteTakeFirstOrThrowOptions | ExecuteTakeFirstOrThrowOptions['errorConstructor']): Promise<SimplifyResult<O>>;
}
declare class MatchedThenableMergeQueryBuilder<DB, TT extends keyof DB, ST extends keyof DB, UT extends TT | ST, O> {
    #private;
    constructor(props: MergeQueryBuilderProps);
    thenDelete(): WheneableMergeQueryBuilder<DB, TT, ST, O>;
    thenDoNothing(): WheneableMergeQueryBuilder<DB, TT, ST, O>;
    thenUpdate<QB extends UpdateQueryBuilder<DB, TT, UT, never>>(set: (ub: QB) => QB): WheneableMergeQueryBuilder<DB, TT, ST, O>;
    thenUpdateSet<UO extends UpdateObject<DB, UT, TT>>(update: UO): WheneableMergeQueryBuilder<DB, TT, ST, O>;
    thenUpdateSet<U extends UpdateObjectFactory<DB, UT, TT>>(update: U): WheneableMergeQueryBuilder<DB, TT, ST, O>;
    thenUpdateSet<RE extends ReferenceExpression<DB, TT>, VE extends ValueExpression<DB, UT, ExtractUpdateTypeFromReferenceExpression<DB, TT, RE>>>(key: RE, value: VE): WheneableMergeQueryBuilder<DB, TT, ST, O>;
}
declare class NotMatchedThenableMergeQueryBuilder<DB, TT extends keyof DB, ST extends keyof DB, O> {
    #private;
    constructor(props: MergeQueryBuilderProps);
    thenDoNothing(): WheneableMergeQueryBuilder<DB, TT, ST, O>;
    thenInsertValues<I extends InsertObjectOrList<DB, TT>>(insert: I): WheneableMergeQueryBuilder<DB, TT, ST, O>;
    thenInsertValues<IO extends InsertObjectOrListFactory<DB, TT, ST>>(insert: IO): WheneableMergeQueryBuilder<DB, TT, ST, O>;
}
type ExtractWheneableMergeQueryBuilder<DB, TT extends keyof DB, TE extends TableExpression<DB, TT>, O> = TE extends `${infer T} as ${infer A}` ? T extends keyof DB ? UsingBuilder<DB, TT, A, DB[T], O> : never : TE extends keyof DB ? WheneableMergeQueryBuilder<DB, TT, TE, O> : TE extends AliasedExpression<infer QO, infer QA> ? UsingBuilder<DB, TT, QA, QO, O> : TE extends (qb: any) => AliasedExpression<infer QO, infer QA> ? UsingBuilder<DB, TT, QA, QO, O> : never;
type UsingBuilder<DB, TT extends keyof DB, A extends string, R, O> = A extends keyof DB ? WheneableMergeQueryBuilder<DB, TT, A, O> : WheneableMergeQueryBuilder<DB & ShallowRecord<A, R>, TT, A, O>;
type MergeInto<DB, TE extends SimpleTableReference<DB>> = [
    TE
] extends [
    keyof DB
] ? MergeQueryBuilder<DB, ExtractTableAlias<DB, TE>, MergeResult> : [
    TE
] extends [
    `${infer T} as ${infer A}`
] ? T extends keyof DB ? MergeQueryBuilder<DB & ShallowRecord<A, DB[T]>, A, MergeResult> : never : never;
declare class QueryCreator<DB> {
    #private;
    constructor(props: QueryCreatorProps);
    selectFrom<TE extends TableExpressionOrList<DB, never>>(from: TE): SelectFrom<DB, never, TE>;
    selectNoFrom<SE extends SelectExpression<DB, never>>(selections: ReadonlyArray<SE>): SelectQueryBuilder<DB, never, Selection<DB, never, SE>>;
    selectNoFrom<const CB extends SelectCallback<DB, never>>(callback: CB): SelectQueryBuilder<DB, never, CallbackSelection<DB, never, CB>>;
    selectNoFrom<SE extends SelectExpression<DB, never>>(selection: SE): SelectQueryBuilder<DB, never, Selection<DB, never, SE>>;
    insertInto<T extends keyof DB & string>(table: T): InsertQueryBuilder<DB, T, InsertResult>;
    replaceInto<T extends keyof DB & string>(table: T): InsertQueryBuilder<DB, T, InsertResult>;
    deleteFrom<TE extends TableExpressionOrList<DB, never>>(from: TE): DeleteFrom<DB, TE>;
    updateTable<TE extends TableExpressionOrList<DB, never>>(tables: TE): UpdateTable<DB, TE>;
    mergeInto<TR extends SimpleTableReference<DB>>(targetTable: TR): MergeInto<DB, TR>;
    with<N extends string, E extends CommonTableExpression<DB, N>>(nameOrBuilder: N | CTEBuilderCallback<N>, expression: E): QueryCreatorWithCommonTableExpression<DB, N, E>;
    withRecursive<N extends string, E extends RecursiveCommonTableExpression<DB, N>>(nameOrBuilder: N | CTEBuilderCallback<N>, expression: E): QueryCreatorWithCommonTableExpression<DB, N, E>;
    withPlugin(plugin: KyselyPlugin): QueryCreator<DB>;
    withoutPlugins(): QueryCreator<DB>;
    withSchema(schema: string): QueryCreator<DB>;
}
interface QueryCreatorProps {
    readonly executor: QueryExecutor;
    readonly withNode?: WithNode;
}
declare const logLevels: readonly [
    'query',
    'error'
];
declare const LOG_LEVELS: Readonly<typeof logLevels>;
type LogLevel = ArrayItemType<typeof LOG_LEVELS>;
interface QueryLogEvent {
    readonly level: 'query';
    readonly isStream?: boolean;
    readonly query: CompiledQuery;
    readonly queryDurationMillis: number;
}
interface ErrorLogEvent {
    readonly level: 'error';
    readonly error: unknown;
    readonly query: CompiledQuery;
    readonly queryDurationMillis: number;
}
type LogEvent = QueryLogEvent | ErrorLogEvent;
type Logger = (event: LogEvent) => void | Promise<void>;
type LogConfig = ReadonlyArray<LogLevel> | Logger;
type RollbackToSavepoint<S extends string[], SN extends S[number]> = S extends [
    ...infer L,
    infer R
] ? R extends SN ? S : RollbackToSavepoint<L extends string[] ? L : never, SN> : never;
type ReleaseSavepoint<S extends string[], SN extends S[number]> = S extends [
    ...infer L,
    infer R
] ? R extends SN ? L : ReleaseSavepoint<L extends string[] ? L : never, SN> : never;
interface ControlledConnection {
    readonly connection: DatabaseConnection;
    readonly release: () => void;
}
declare global {
    interface AsyncDisposable {
    }
    interface SymbolConstructor {
        readonly asyncDispose: unique symbol;
    }
}
declare class Kysely<DB> extends QueryCreator<DB> implements QueryExecutorProvider, AsyncDisposable {
    #private;
    constructor(args: KyselyConfig);
    constructor(args: KyselyProps);
    get schema(): SchemaModule;
    get dynamic(): DynamicModule<DB>;
    get introspection(): DatabaseIntrospector;
    case(): CaseBuilder<DB, keyof DB>;
    case<V>(value: Expression<V>): CaseBuilder<DB, keyof DB, V>;
    get fn(): FunctionModule<DB, keyof DB>;
    transaction(): TransactionBuilder<DB>;
    startTransaction(): ControlledTransactionBuilder<DB>;
    connection(): ConnectionBuilder<DB>;
    withPlugin(plugin: KyselyPlugin): Kysely<DB>;
    withoutPlugins(): Kysely<DB>;
    withSchema(schema: string): Kysely<DB>;
    $extendTables<T extends Record<string, Record<string, any>>>(): Kysely<DrainOuterGeneric<DB & T>>;
    $omitTables<T extends keyof DB>(): Kysely<DB extends object ? Omit<DB, T> : DB>;
    $pickTables<T extends keyof DB>(): Kysely<DB extends object ? Pick<DB, T> : DB>;
    withTables<T extends Record<string, Record<string, any>>>(): Kysely<DrainOuterGeneric<DB & T>>;
    destroy(): Promise<void>;
    get isTransaction(): boolean;
    getExecutor(): QueryExecutor;
    executeQuery<R>(query: CompiledQuery<R> | Compilable<R>, options?: AbortableQueryOptions): Promise<QueryResult<R>>;
    [Symbol.asyncDispose](): Promise<void>;
}
declare class Transaction<DB> extends Kysely<DB> {
    #private;
    constructor(props: KyselyProps);
    get isTransaction(): true;
    transaction(): never;
    startTransaction(): never;
    connection(): never;
    destroy(): never;
    withPlugin(plugin: KyselyPlugin): Transaction<DB>;
    withoutPlugins(): Transaction<DB>;
    withSchema(schema: string): Transaction<DB>;
    withTables<T extends Record<string, Record<string, any>>>(): Transaction<DrainOuterGeneric<DB & T>>;
    $extendTables<T extends Record<string, Record<string, any>>>(): Transaction<DrainOuterGeneric<DB & T>>;
    $omitTables<T extends keyof DB>(): Transaction<DB extends object ? Omit<DB, T> : DB>;
    $pickTables<T extends keyof DB>(): Transaction<DB extends object ? Pick<DB, T> : DB>;
}
interface KyselyProps {
    readonly config: KyselyConfig;
    readonly driver: Driver;
    readonly executor: QueryExecutor;
    readonly dialect: Dialect;
}
interface KyselyConfig {
    readonly dialect: Dialect;
    readonly plugins?: KyselyPlugin[];
    readonly log?: LogConfig;
}
declare class ConnectionBuilder<DB> {
    #private;
    constructor(props: ConnectionBuilderProps);
    execute<T>(callback: (db: Kysely<DB>) => Promise<T>, options?: AbortableOperationOptions): Promise<T>;
}
interface ConnectionBuilderProps extends KyselyProps {
}
declare class TransactionBuilder<DB> {
    #private;
    constructor(props: TransactionBuilderProps);
    setAccessMode(accessMode: AccessMode): TransactionBuilder<DB>;
    setIsolationLevel(isolationLevel: IsolationLevel): TransactionBuilder<DB>;
    execute<T>(callback: (trx: Transaction<DB>) => Promise<T>): Promise<T>;
}
interface TransactionBuilderProps extends KyselyProps {
    readonly accessMode?: AccessMode;
    readonly isolationLevel?: IsolationLevel;
}
declare class ControlledTransactionBuilder<DB> {
    #private;
    constructor(props: TransactionBuilderProps);
    setAccessMode(accessMode: AccessMode): ControlledTransactionBuilder<DB>;
    setIsolationLevel(isolationLevel: IsolationLevel): ControlledTransactionBuilder<DB>;
    execute(): Promise<ControlledTransaction<DB>>;
}
declare class ControlledTransaction<DB, S extends string[] = [
]> extends Transaction<DB> {
    #private;
    constructor(props: ControlledTransactionProps);
    get isCommitted(): boolean;
    get isRolledBack(): boolean;
    commit(): Command<void>;
    rollback(): Command<void>;
    savepoint<SN extends string>(savepointName: SN extends S ? never : SN): Command<ControlledTransaction<DB, [
        ...S,
        SN
    ]>>;
    rollbackToSavepoint<SN extends S[number]>(savepointName: SN): RollbackToSavepoint<S, SN> extends string[] ? Command<ControlledTransaction<DB, RollbackToSavepoint<S, SN>>> : never;
    releaseSavepoint<SN extends S[number]>(savepointName: SN): ReleaseSavepoint<S, SN> extends string[] ? Command<ControlledTransaction<DB, ReleaseSavepoint<S, SN>>> : never;
    withPlugin(plugin: KyselyPlugin): ControlledTransaction<DB, S>;
    withoutPlugins(): ControlledTransaction<DB, S>;
    withSchema(schema: string): ControlledTransaction<DB, S>;
    withTables<T extends Record<string, Record<string, any>>>(): ControlledTransaction<DrainOuterGeneric<DB & T>, S>;
    $extendTables<T extends Record<string, Record<string, any>>>(): ControlledTransaction<DrainOuterGeneric<DB & T>, S>;
    $omitTables<T extends keyof DB>(): ControlledTransaction<DB extends object ? Omit<DB, T> : DB, S>;
    $pickTables<T extends keyof DB>(): ControlledTransaction<DB extends object ? Pick<DB, T> : DB, S>;
}
interface ControlledTransactionProps extends KyselyProps {
    readonly connection: ControlledConnection;
}
declare class Command<T> {
    #private;
    constructor(cb: () => Promise<T>);
    execute(): Promise<T>;
}
interface Sql {
    <T = unknown>(sqlFragments: TemplateStringsArray, ...parameters: unknown[]): RawBuilder<T>;
    val<V>(value: V): RawBuilder<V>;
    ref<R = unknown>(columnReference: string): RawBuilder<R>;
    table<T = unknown>(tableReference: string): RawBuilder<T>;
    id<T = unknown>(...ids: readonly string[]): RawBuilder<T>;
    lit<V>(value: V): RawBuilder<V>;
    raw<R = unknown>(anySql: string): RawBuilder<R>;
    join<T = unknown>(array: readonly unknown[], separator?: RawBuilder<any>): RawBuilder<T>;
}
declare const sql: Sql;
declare function jsonArrayFrom<O>(expr: Expression<O>): RawBuilder<Simplify<ShallowDehydrateObject<O>>[]>;
declare function jsonObjectFrom<O>(expr: Expression<O>): RawBuilder<Simplify<ShallowDehydrateObject<O>> | null>;
declare function jsonBuildObject<O extends Record<string, Expression<unknown>>>(obj: O): RawBuilder<Simplify<{
    [K in keyof O]: O[K] extends Expression<infer V> ? ShallowDehydrateValue<V> : never;
}>>;
export { Kysely, Transaction, jsonArrayFrom, jsonBuildObject, jsonObjectFrom, sql };
export type { ColumnType, Expression, ExpressionBuilder, Generated, GeneratedAlways, Insertable, JSONColumnType, NotNull$1 as NotNull, Selectable, SqlBool, Updateable };
