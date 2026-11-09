import {
	type JsCallExpression,
	type JsIdentifierBinding,
	type SemanticModel,
	type TypeInfo,
	type TypeService,
	ast,
	defineRule,
	grit,
	semantic,
} from "@biomejs/runtime/plugin";

defineRule({
	query: ast("JS_CALL_EXPRESSION", { types: true }),
	run(node, context) {
		node satisfies JsCallExpression;
		context.types satisfies TypeService;
		context.model satisfies SemanticModel;

		const ty = context.types.typeOf(node);
		ty satisfies TypeInfo | undefined;
		ty?.toString() satisfies string | undefined;
		ty?.kind() satisfies string | undefined;
		ty?.isPromise() satisfies boolean | undefined;
		ty?.isArray() satisfies boolean | undefined;
	},
});

const astSemantic = ast("JS_CALL_EXPRESSION", { semantic: true });
astSemantic.type satisfies "ast";

const semanticQuery = semantic("JS_IDENTIFIER_BINDING");
semanticQuery.type satisfies "semantic";

defineRule({
	query: ast("JS_CALL_EXPRESSION", { semantic: true }),
	run(node, context) {
		node satisfies JsCallExpression;
		context.model satisfies SemanticModel;
		// @ts-expect-error A semantic query does not provide type information.
		context.types;
	},
});

defineRule({
	query: ast("JS_IDENTIFIER_BINDING"),
	run(node, context) {
		node satisfies JsIdentifierBinding;
		context satisfies { readonly filePath: string };
		// @ts-expect-error A plain query does not provide type information.
		context.types;
		// @ts-expect-error A plain query does not provide a semantic model.
		context.model;
	},
});

const typedGrit = grit("`fetch($url)`", { types: true });
typedGrit.type satisfies "grit";

defineRule({
	query: typedGrit,
	run(element, context) {
		context.types satisfies TypeService;
		context.model satisfies SemanticModel;
	},
});

defineRule({
	query: grit("`fetch($url)`", { semantic: true }),
	run(element, context) {
		// @ts-expect-error A semantic Grit query does not provide type information.
		context.types;
	},
});

defineRule({
	query: grit("`fetch($url)`"),
	run(element, context) {
		// @ts-expect-error A plain Grit query does not provide type information.
		context.types;
	},
});
