import {
	type AnyJsAstNode,
	type JsAstElement,
	type JsAstToken,
	type JsCallExpression,
	type SemanticModel,
	defineRule,
	grit,
	registerDiagnostic,
} from "@biomejs/runtime/plugin";

declare const query: ReturnType<typeof grit>;
query.type satisfies "grit";
query.pattern satisfies string;
// @ts-expect-error Grit queries select nodes and tokens, not syntax kinds.
query.kinds;

const semanticQuery = grit("JsReferenceIdentifier()", { semantic: true });
semanticQuery.type satisfies "grit";

defineRule({
	query: grit("`console.log($args)`"),
	run(element) {
		element satisfies JsAstElement;
		// @ts-expect-error A Grit match may be a token.
		element satisfies AnyJsAstNode;
		if (element.kind === "JS_CALL_EXPRESSION") {
			element satisfies JsCallExpression;
			registerDiagnostic(element, "warning", "Unexpected console.log.");
		}
	},
});

defineRule({
	query: grit("JsReferenceIdentifier()", { semantic: true }),
	run(element, context) {
		element satisfies JsAstElement;
		context.model satisfies SemanticModel;
	},
});

defineRule({
	query: grit("JsReferenceIdentifier()"),
	run(element, context) {
		element satisfies JsAstElement;
		// @ts-expect-error Plain Grit rules do not receive a semantic model.
		context.model;
	},
});

defineRule({
	query: grit("`+`"),
	run(element) {
		element satisfies JsAstElement;
		if (element.kind === "PLUS") {
			element satisfies JsAstToken;
			element.text satisfies string;
			element.parent satisfies AnyJsAstNode | undefined;
			// @ts-expect-error Diagnostics require an AST node, not a token.
			registerDiagnostic(element, "warning", "Unexpected plus.");
		}
	},
});
