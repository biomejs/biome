import type {
	AnyJsAstNode,
	JsIdentifierAssignment,
	JsIdentifierBinding,
	JsReferenceIdentifier,
	JsxReferenceIdentifier,
	TsIdentifierBinding,
	TsLiteralEnumMemberName,
	TsTypeParameterName,
} from "./js_ast";

/** Identifier nodes that refer to a binding or an external name. */
export type AnyJsIdentifierReference =
	| JsReferenceIdentifier
	| JsIdentifierAssignment
	| JsxReferenceIdentifier;

/** Identifier nodes that introduce bindings. */
export type AnyJsIdentifierBinding =
	| JsIdentifierBinding
	| TsIdentifierBinding
	| TsTypeParameterName
	| TsLiteralEnumMemberName;

/** The semantic classification of a binding's declaration. */
export type JsDeclarationKind =
	/** Class binding, including named class expressions. */
	| "class"
	/** TypeScript enum declaration or member. */
	| "enum"
	/** Function declaration or overload signature. */
	| "function"
	/** Generic type parameter. */
	| "generic"
	/** Hoisted value, such as `var` or a named function expression. */
	| "hoistedValue"
	/** Import binding, including TypeScript `import =`. */
	| "import"
	/** Type-only import binding. */
	| "importType"
	/** TypeScript interface declaration. */
	| "interface"
	/** TypeScript module declaration. */
	| "module"
	/** TypeScript namespace declaration. */
	| "namespace"
	/** Type alias declaration. */
	| "type"
	/** Declaration not classified by the model. */
	| "unknown"
	/** Explicit resource-management binding. */
	| "using"
	/** Value binding, such as `let`, `const`, or a parameter. */
	| "value";

/**
 * A range in the analyzed source, measured in UTF-8 bytes.
 * The range starts at `start` and stops before `end`.
 * For example, `{ start: 2, end: 5 }` covers bytes 2, 3, and 4; byte 5 is not included.
 */
export interface TextRange {
	/** Starting byte offset, included in the range. */
	readonly start: number;
	/** Byte offset immediately after the range. */
	readonly end: number;
}

/**
 * Bindings and references in one analyzed source snapshot.
 * Access it through `context.model` in a rule using `semantic(...)`.
 *
 * Node arguments must belong to this model's source. Saved models and handles
 * retain their original source; prepared code fixes do not update the model.
 * Collection methods return fresh arrays whose contents can be inspected with
 * normal JavaScript iteration and array methods.
 */
export interface SemanticModel {
	/**
	 * Resolves an identifier use to the binding it refers to, respecting shadowing.
	 * Returns `undefined` for configured globals and unresolved references.
	 *
	 * In a rule querying `semantic("JS_REFERENCE_IDENTIFIER")`, highlight the
	 * declaration of the referenced name:
	 * ```ts
	 * const binding = context.model.binding(node);
	 * if (binding) {
	 *   registerDiagnostic(binding.syntax(), "information", "Declared here.");
	 * }
	 * ```
	 */
	binding(reference: AnyJsIdentifierReference): Binding | undefined;
	/**
	 * Looks up the binding introduced by a declaration's identifier node.
	 * Returns `undefined` when the model does not track that declaration.
	 *
	 * In a rule querying `semantic("JS_IDENTIFIER_BINDING")`, report how many
	 * references resolve to the declaration:
	 * ```ts
	 * const binding = context.model.asBinding(node);
	 * if (binding) {
	 *   const count = binding.allReferences().length;
	 *   registerDiagnostic(node, "information", `${count} references.`);
	 * }
	 * ```
	 */
	asBinding(node: AnyJsIdentifierBinding): Binding | undefined;
	/**
	 * Returns all tracked bindings in the source, including nested scopes.
	 * Declarations with the same spelling can introduce separate bindings.
	 *
	 * In a semantic rule querying the root, highlight each declaration:
	 * ```ts
	 * for (const binding of context.model.allBindings()) {
	 *   registerDiagnostic(binding.syntax(), "information", "Declaration.");
	 * }
	 * ```
	 */
	allBindings(): readonly Binding[];
	/**
	 * Returns the bindings marked as exported by the model, with each binding
	 * appearing once even if it has multiple export sites. Order is unspecified.
	 * Re-exports that introduce no local binding are not included.
	 *
	 * In a semantic rule querying the root, highlight exported declarations:
	 * ```ts
	 * for (const binding of context.model.allExportedBindings()) {
	 *   registerDiagnostic(binding.syntax(), "information", "Exported declaration.");
	 * }
	 * ```
	 */
	allExportedBindings(): readonly Binding[];
	/**
	 * Returns whether the model tracks at least one exported binding.
	 * This is not a check for export syntax: an empty export, a re-export without
	 * a local binding, or an exported literal can leave this value `false`.
	 *
	 * In a semantic rule querying the root, check for exported bindings:
	 * ```ts
	 * if (context.model.hasExports()) {
	 *   registerDiagnostic(node, "information", "This source exports bindings.");
	 * }
	 * ```
	 */
	hasExports(): boolean;
	/**
	 * Returns all scopes in this source, including the global scope and nested scopes.
	 *
	 * In a semantic rule querying the root, inspect each scope's declarations:
	 * ```ts
	 * for (const scope of context.model.scopes()) {
	 *   for (const binding of scope.bindings()) {
	 *     registerDiagnostic(binding.syntax(), "information", "Declared in this scope.");
	 *   }
	 * }
	 * ```
	 */
	scopes(): readonly Scope[];
	/**
	 * Returns the outermost scope of this source, including for an empty file.
	 * This scope has no parent. It is distinct from the configured global names
	 * returned by `allGlobalReferences()`.
	 *
	 * In a semantic rule, inspect bindings in the outermost scope:
	 * ```ts
	 * const scope = context.model.globalScope();
	 * const names = scope.bindings().map(binding => binding.syntax().text);
	 * ```
	 */
	globalScope(): Scope;
	/**
	 * Returns the most specific scope containing the node's trimmed source range.
	 * Returns `undefined` for an empty node. Use `globalScope()` to obtain a scope
	 * for an empty file.
	 *
	 * In a semantic rule, inspect the scope surrounding the matched node:
	 * ```ts
	 * const scope = context.model.scope(node);
	 * if (scope) {
	 *   registerDiagnostic(scope.syntax(), "information", "Containing scope.");
	 * }
	 * ```
	 */
	scope(node: AnyJsAstNode): Scope | undefined;
	/**
	 * Returns the scope a declaration's identifier was hoisted to, if the model
	 * records one. This can differ from the scope containing its source text.
	 *
	 * In a rule querying `semantic("JS_IDENTIFIER_BINDING")`, inspect hoisting:
	 * ```ts
	 * const scope = context.model.scopeHoistedTo(node);
	 * if (scope) {
	 *   registerDiagnostic(scope.syntax(), "information", "Declaration is hoisted here.");
	 * }
	 * ```
	 */
	scopeHoistedTo(node: AnyJsAstNode): Scope | undefined;
	/**
	 * Returns references resolved to globals configured in the supplied model.
	 * Locally shadowed names resolve to local bindings instead.
	 *
	 * In a semantic rule querying the root, highlight uses of configured globals:
	 * ```ts
	 * for (const reference of context.model.allGlobalReferences()) {
	 *   registerDiagnostic(reference.syntax(), "information", "Uses a global.");
	 * }
	 * ```
	 */
	allGlobalReferences(): readonly GlobalReference[];
	/**
	 * Returns references that resolve to neither a local binding nor a global
	 * configured in the supplied model.
	 *
	 * In a semantic rule querying the root, highlight unresolved names:
	 * ```ts
	 * for (const reference of context.model.allUnresolvedReferences()) {
	 *   registerDiagnostic(reference.syntax(), "warning", "Unresolved name.");
	 * }
	 * ```
	 */
	allUnresolvedReferences(): readonly UnresolvedReference[];
	/**
	 * Returns whether an identifier use resolves to a configured global.
	 * Returns `false` for local bindings and unresolved references.
	 *
	 * In a rule querying `semantic("JS_REFERENCE_IDENTIFIER")`, distinguish
	 * configured globals from other identifier uses:
	 * ```ts
	 * if (context.model.isGlobalReference(node)) {
	 *   registerDiagnostic(node, "information", "Uses a configured global.");
	 * }
	 * ```
	 */
	isGlobalReference(reference: AnyJsIdentifierReference): boolean;
	/**
	 * Returns whether an identifier use has no local binding or configured global.
	 * A reference to a declaration later in the source can still resolve.
	 *
	 * In a rule querying `semantic("JS_REFERENCE_IDENTIFIER")`, report names the
	 * model cannot resolve:
	 * ```ts
	 * if (context.model.isUnresolvedReference(node)) {
	 *   registerDiagnostic(node, "warning", "This name could not be resolved.");
	 * }
	 * ```
	 */
	isUnresolvedReference(reference: AnyJsIdentifierReference): boolean;
	/**
	 * Returns whether a declaration is imported, or a reference resolves to an
	 * imported declaration. Returns `false` when a reference has no local binding.
	 *
	 * In a rule querying `semantic("JS_REFERENCE_IDENTIFIER")`, highlight uses
	 * of imported names:
	 * ```ts
	 * if (context.model.isImported(node)) {
	 *   registerDiagnostic(node, "information", "Uses an imported binding.");
	 * }
	 * ```
	 */
	isImported(node: AnyJsIdentifierBinding | AnyJsIdentifierReference): boolean;
	/**
	 * Returns whether a declaration is exported, or a reference resolves to an
	 * exported declaration. Returns `false` when a reference has no local binding.
	 *
	 * In a rule querying `semantic("JS_IDENTIFIER_BINDING")`, highlight exported
	 * declarations:
	 * ```ts
	 * if (context.model.isExported(node)) {
	 *   registerDiagnostic(node, "information", "This binding is exported.");
	 * }
	 * ```
	 */
	isExported(node: AnyJsIdentifierBinding | AnyJsIdentifierReference): boolean;
}

/**
 * A declaration and the references that resolve to it.
 * The examples use a `binding` obtained from `context.model.binding(...)`
 * or `context.model.asBinding(...)`.
 */
export interface Binding {
	/**
	 * Returns the model's declaration classification, such as `"function"`,
	 * `"importType"`, or `"hoistedValue"`. This describes how the binding was
	 * declared, rather than the syntax kind of its identifier node.
	 *
	 * Highlight hoisted value bindings:
	 * ```ts
	 * if (binding.declarationKind() === "hoistedValue") {
	 *   registerDiagnostic(binding.syntax(), "information", "Hoisted value binding.");
	 * }
	 * ```
	 */
	declarationKind(): JsDeclarationKind;
	/**
	 * Returns the identifier nodes at this binding's tracked export sites.
	 * A directly exported declaration yields its identifier binding; an export
	 * specifier yields the local identifier reference, not its exported alias.
	 * Returns an empty array for a binding with no tracked export sites.
	 *
	 * Highlight the places that export this binding:
	 * ```ts
	 * for (const site of binding.exports()) {
	 *   registerDiagnostic(site, "information", "Exported here.");
	 * }
	 * ```
	 */
	exports(): readonly (AnyJsIdentifierBinding | AnyJsIdentifierReference)[];
	/**
	 * Returns the model's source ranges for this binding's export sites.
	 * Offsets count UTF-8 bytes, with an inclusive start and exclusive end;
	 * they are not JavaScript string indices. Returns an empty array when no
	 * export sites are tracked.
	 *
	 * Report the recorded byte offsets alongside the declaration:
	 * ```ts
	 * const locations = binding.exportRanges()
	 *   .map(range => `${range.start}..${range.end}`)
	 *   .join(", ");
	 * registerDiagnostic(binding.syntax(), "information", `Export byte ranges: ${locations}.`);
	 * ```
	 */
	exportRanges(): readonly TextRange[];
	/**
	 * Returns the identifier node introducing this binding, rather than the
	 * enclosing declaration statement. The node can be used in diagnostics and fixes.
	 *
	 * Highlight the declaration's name:
	 * ```ts
	 * const identifier = binding.syntax();
	 * registerDiagnostic(identifier, "information", `Declares ${identifier.text}.`);
	 * ```
	 */
	syntax(): AnyJsIdentifierBinding;
	/**
	 * Returns the scope containing the declaration's identifier in the source.
	 * For hoisted declarations, use `context.model.scopeHoistedTo(binding.syntax())`
	 * to inspect the destination scope instead.
	 *
	 * Inspect the declaration's surrounding scope:
	 * ```ts
	 * const scope = binding.scope();
	 * registerDiagnostic(scope.syntax(), "information", "Declaration appears here.");
	 * ```
	 */
	scope(): Scope;
	/**
	 * Returns all references that resolve to this binding, including reads and writes.
	 * References to a different binding with the same spelling are excluded.
	 *
	 * Highlight every reference to this declaration:
	 * ```ts
	 * for (const reference of binding.allReferences()) {
	 *   registerDiagnostic(reference.syntax(), "information", "References this binding.");
	 * }
	 * ```
	 */
	allReferences(): readonly Reference[];
	/**
	 * Returns references classified as reads by the semantic model.
	 *
	 * Highlight the places that read this binding:
	 * ```ts
	 * for (const reference of binding.allReads()) {
	 *   registerDiagnostic(reference.syntax(), "information", "Reads this binding.");
	 * }
	 * ```
	 */
	allReads(): readonly Reference[];
	/**
	 * Returns references classified as writes by the semantic model.
	 *
	 * Highlight the places that write to this binding:
	 * ```ts
	 * for (const reference of binding.allWrites()) {
	 *   registerDiagnostic(reference.syntax(), "information", "Writes to this binding.");
	 * }
	 * ```
	 */
	allWrites(): readonly Reference[];
	/**
	 * Returns whether this binding has an import declaration kind in the model.
	 *
	 * Highlight an imported declaration:
	 * ```ts
	 * if (binding.isImported()) {
	 *   registerDiagnostic(binding.syntax(), "information", "Imported binding.");
	 * }
	 * ```
	 */
	isImported(): boolean;
	/**
	 * Returns whether this binding is exported from the source.
	 *
	 * Highlight an exported declaration:
	 * ```ts
	 * if (binding.isExported()) {
	 *   registerDiagnostic(binding.syntax(), "information", "Exported binding.");
	 * }
	 * ```
	 */
	isExported(): boolean;
}

/**
 * A reference resolved to a local binding.
 * The examples use a `reference` obtained from a binding's reference arrays.
 */
export interface Reference {
	/**
	 * Returns the identifier node at the reference site. The node can be used in
	 * diagnostics and fixes.
	 *
	 * Highlight the identifier use rather than its declaration:
	 * ```ts
	 * registerDiagnostic(reference.syntax(), "information", "Referenced here.");
	 * ```
	 */
	syntax(): AnyJsIdentifierReference;
	/**
	 * Returns the binding this reference resolves to.
	 *
	 * Follow a reference back to its declaration:
	 * ```ts
	 * const declaration = reference.binding();
	 * if (declaration) {
	 *   registerDiagnostic(declaration.syntax(), "information", "Declared here.");
	 * }
	 * ```
	 */
	binding(): Binding | undefined;
	/**
	 * Returns the scope containing this identifier use, which can differ from the
	 * scope containing its declaration.
	 *
	 * Highlight the scope where the reference occurs:
	 * ```ts
	 * registerDiagnostic(reference.scope().syntax(), "information", "Reference scope.");
	 * ```
	 */
	scope(): Scope;
	/**
	 * Returns whether the model classifies this reference as a read.
	 *
	 * Report only read references while inspecting a binding's references:
	 * ```ts
	 * if (reference.isRead()) {
	 *   registerDiagnostic(reference.syntax(), "information", "Read access.");
	 * }
	 * ```
	 */
	isRead(): boolean;
	/**
	 * Returns whether the model classifies this reference as a write.
	 *
	 * Report only write references while inspecting a binding's references:
	 * ```ts
	 * if (reference.isWrite()) {
	 *   registerDiagnostic(reference.syntax(), "information", "Write access.");
	 * }
	 * ```
	 */
	isWrite(): boolean;
}

/**
 * A lexical scope in one source snapshot. Obtain scopes through the semantic
 * model, a binding, or a resolved reference. The examples use a `scope` returned
 * by one of those APIs. Collection methods return fresh arrays.
 */
export interface Scope {
	/**
	 * Returns the syntax node associated with this scope, such as a module,
	 * function, or block. The node can be used in diagnostics and fixes.
	 *
	 * Highlight the code that defines this scope:
	 * ```ts
	 * registerDiagnostic(scope.syntax(), "information", "Scope starts here.");
	 * ```
	 */
	syntax(): AnyJsAstNode;
	/**
	 * Returns whether this is the source's outermost scope.
	 *
	 * Inspect nested scopes only:
	 * ```ts
	 * if (!scope.isGlobalScope()) {
	 *   registerDiagnostic(scope.syntax(), "information", "Nested scope.");
	 * }
	 * ```
	 */
	isGlobalScope(): boolean;
	/**
	 * Returns the immediately enclosing scope, or `undefined` for the global scope.
	 *
	 * Highlight the containing scope:
	 * ```ts
	 * const parent = scope.parent();
	 * if (parent) {
	 *   registerDiagnostic(parent.syntax(), "information", "Parent scope.");
	 * }
	 * ```
	 */
	parent(): Scope | undefined;
	/**
	 * Returns this scope's immediate child scopes, without their descendants.
	 *
	 * Inspect each directly nested scope:
	 * ```ts
	 * for (const child of scope.children()) {
	 *   registerDiagnostic(child.syntax(), "information", "Child scope.");
	 * }
	 * ```
	 */
	children(): readonly Scope[];
	/**
	 * Returns this scope followed by its parents, ending with the global scope.
	 * The first entry is the current scope itself.
	 *
	 * Search outward for the nearest binding named `value`:
	 * ```ts
	 * for (const ancestor of scope.ancestors()) {
	 *   const binding = ancestor.getBinding("value");
	 *   if (binding) {
	 *     registerDiagnostic(binding.syntax(), "information", "Nearest declaration.");
	 *     break;
	 *   }
	 * }
	 * ```
	 */
	ancestors(): readonly Scope[];
	/**
	 * Returns bindings declared in or hoisted to this scope. Bindings belonging
	 * to parent or child scopes are excluded.
	 *
	 * List this scope's binding names:
	 * ```ts
	 * const names = scope.bindings().map(binding => binding.syntax().text);
	 * ```
	 */
	bindings(): readonly Binding[];
	/**
	 * Looks up a binding in this scope only; parent scopes are not searched.
	 * Returns `undefined` when the name is absent. Unicode escapes are decoded.
	 * When a TypeScript name has both type and value bindings, the value binding
	 * is preferred, falling back to the type binding if no value exists.
	 *
	 * Inspect a binding declared in this scope:
	 * ```ts
	 * const binding = scope.getBinding("value");
	 * if (binding) {
	 *   registerDiagnostic(binding.syntax(), "information", "Local declaration.");
	 * }
	 * ```
	 */
	getBinding(name: string): Binding | undefined;
}

/**
 * A reference resolved to a global configured in the supplied model.
 * The examples use a `reference` from `context.model.allGlobalReferences()`.
 */
export interface GlobalReference {
	/**
	 * Returns the identifier node that uses the configured global.
	 *
	 * Highlight the global's name at its use site:
	 * ```ts
	 * const identifier = reference.syntax();
	 * registerDiagnostic(identifier, "information", `Uses global ${identifier.text}.`);
	 * ```
	 */
	syntax(): AnyJsIdentifierReference;
	/**
	 * Returns whether the model classifies this global reference as a read.
	 *
	 * Highlight reads of configured globals:
	 * ```ts
	 * if (reference.isRead()) {
	 *   registerDiagnostic(reference.syntax(), "information", "Reads a global.");
	 * }
	 * ```
	 */
	isRead(): boolean;
	/**
	 * Returns whether the model classifies this global reference as a write.
	 *
	 * Report assignments to configured globals:
	 * ```ts
	 * if (reference.isWrite()) {
	 *   registerDiagnostic(reference.syntax(), "warning", "Writes to a global.");
	 * }
	 * ```
	 */
	isWrite(): boolean;
}

/** A reference without a local binding or configured global. */
export interface UnresolvedReference {
	/**
	 * Returns the identifier node whose name could not be resolved.
	 *
	 * Highlight each unresolved name in a semantic rule:
	 * ```ts
	 * for (const reference of context.model.allUnresolvedReferences()) {
	 *   const identifier = reference.syntax();
	 *   registerDiagnostic(identifier, "warning", `Cannot resolve ${identifier.text}.`);
	 * }
	 * ```
	 */
	syntax(): AnyJsIdentifierReference;
}
