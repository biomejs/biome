import type { JsAstNode } from "./js_ast";

/** A high-level classification performed by the typed service. */
export type TypeClassification =
	| "promise"
	| "thenable"
	| "arrayOfPromises"
	| "promiseReturningFunction";

/** The outcome of a {@link TypeClassification}. */
export type TypeClassificationResult = "match" | "noMatch" | "indeterminate";

/**
 * Type information available through `context.types` in a rule that requests
 * the `types` capability.
 *
 * The service throws if the analyzed file wasn't indexed with type inference.
 * Enable the `types` domain in the Biome configuration, for example with
 * `linter.domains.types: "all"`.
 */
export interface TypeService {
	/** Returns the inferred type of an expression, if it is known. */
	typeOf(expression: JsAstNode): TypeInfo | undefined;

	/** Returns the inferred type of the binding `name` visible at `node`. */
	typeOfNamed(node: JsAstNode, name: string): TypeInfo | undefined;

	/** Returns the inferred return type of a function-like node. */
	returnTypeOf(functionNode: JsAstNode): TypeInfo | undefined;

	/**
	 * Returns the inferred return type of a class or object member.
	 *
	 * When the member is not a function, the member's own type is returned.
	 */
	returnTypeOfMember(member: JsAstNode, name: string): TypeInfo | undefined;

	/**
	 * Returns the expected callable type of the argument at `index` for a call
	 * or `new` expression, such as a callback parameter.
	 */
	expectedArgumentType(call: JsAstNode, index: number): TypeInfo | undefined;

	/** Classifies an expression, such as whether it is a promise. */
	classifiesAs(
		expression: JsAstNode,
		classification: TypeClassification,
	): TypeClassificationResult;
}

/**
 * An inferred type.
 *
 * A type is bound to the analyzed file; use it during the current `run()` call.
 * Predicates that support an undecided result return `undefined` instead of
 * `false`. Predicates for primitive-like types report `false` when traversal
 * cannot prove that every variant matches.
 */
export interface TypeInfo {
	/** Renders the type as TypeScript-like source, such as `Promise<string>`. */
	toString(): string;

	/** Returns a short category name, such as `string`, `union`, or `class`. */
	kind(): string;

	isAny(): boolean;
	isUnknown(): boolean;
	isNever(): boolean;
	isPrimitive(): boolean;
	isObjectLike(): boolean;
	isArray(): boolean;
	isTuple(): boolean;
	isFunction(): boolean;
	isCallable(): boolean | undefined;
	isPromise(): boolean | undefined;
	isArrayOfPromise(): boolean | undefined;
	isNullish(): boolean;
	isAlwaysTruthy(): boolean;
	isAlwaysFalsy(): boolean;
	isStringLike(): boolean;
	isNumberLike(): boolean;
	isBooleanLike(): boolean;
	isBigIntLike(): boolean;
}
