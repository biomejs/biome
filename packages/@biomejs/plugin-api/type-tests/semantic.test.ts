import {
	ast,
	defineRule,
	type Binding,
	type JsIdentifierBinding,
	type JsDeclarationKind,
	type JsReferenceIdentifier,
	type Reference,
	type RuleContext,
	type Scope,
	type SemanticModel,
	type SemanticRuleContext,
	type TextRange,
	semantic,
} from "@biomejs/plugin-api";

defineRule({
	query: semantic("JS_REFERENCE_IDENTIFIER"),
	run(node, context) {
		node satisfies JsReferenceIdentifier;
		context satisfies SemanticRuleContext;
		context.model satisfies SemanticModel;
		context.model.binding(node) satisfies Binding | undefined;
		context.model.isImported(node) satisfies boolean;
		context.model.isExported(node) satisfies boolean;
		context.model.hasExports() satisfies boolean;
		context.model.allExportedBindings() satisfies readonly Binding[];
		context.model.isGlobalReference(node) satisfies boolean;
		context.model.isUnresolvedReference(node) satisfies boolean;
		context.model.allGlobalReferences().map(reference => reference.isRead());
		context.model.allUnresolvedReferences().map(reference => reference.syntax());
		context.model.scope(node) satisfies Scope | undefined;
		context.model.scopeHoistedTo(node) satisfies Scope | undefined;
		context.model.scopes() satisfies readonly Scope[];
		const scope = context.model.globalScope();
		scope satisfies Scope;
		scope.parent() satisfies Scope | undefined;
		scope.children() satisfies readonly Scope[];
		scope.ancestors() satisfies readonly Scope[];
		scope.bindings() satisfies readonly Binding[];
		scope.getBinding("value") satisfies Binding | undefined;
		scope.isGlobalScope() satisfies boolean;
		context.model.scope(scope.syntax());
		for (const binding of scope.bindings()) {
			binding.scope() satisfies Scope;
			binding.declarationKind() satisfies JsDeclarationKind;
			binding.exports().map(site => context.model.isExported(site));
			binding.exportRanges() satisfies readonly TextRange[];
			for (const range of binding.exportRanges()) {
				range.start satisfies number;
				range.end satisfies number;
				// @ts-expect-error Export range endpoints are read-only.
				range.start = 0;
			}
			for (const reference of binding.allReferences()) {
				reference.scope() satisfies Scope;
			}
		}
		// @ts-expect-error Scope collections are read-only.
		scope.children().push(scope);
		// @ts-expect-error Scope lookup requires a source node, not a scope handle.
		context.model.scope(scope);
		// @ts-expect-error Reference queries preserve the matched node type.
		node satisfies JsIdentifierBinding;
		// @ts-expect-error A reference does not introduce a binding.
		context.model.asBinding(node);
		// @ts-expect-error The supplied model is read-only.
		context.model = context.model;
	},
});

defineRule({
	query: semantic("JS_IDENTIFIER_BINDING", "JS_REFERENCE_IDENTIFIER"),
	run(node, context) {
		node satisfies JsIdentifierBinding | JsReferenceIdentifier;
		if (node.kind === "JS_IDENTIFIER_BINDING") {
			const binding = context.model.asBinding(node);
			binding?.allReferences() satisfies readonly Reference[] | undefined;
			binding?.allReads().map(reference => reference.binding());
			binding?.allWrites().map(reference => reference.isWrite());
		} else {
			context.model.binding(node);
		}
	},
});

defineRule({
	query: ast("JS_REFERENCE_IDENTIFIER"),
	run(node, context) {
		node satisfies JsReferenceIdentifier;
		context satisfies RuleContext;
		// @ts-expect-error Syntax queries do not expose the model.
		context.model;
	},
});

defineRule({
	query: ast("JS_REFERENCE_IDENTIFIER"),
	// @ts-expect-error A callback cannot request a model through its annotation.
	run(_node: JsReferenceIdentifier, _context: SemanticRuleContext) {},
});

defineRule({
	query: semantic("JS_REFERENCE_IDENTIFIER"),
	// @ts-expect-error A callback cannot override the queried node type.
	run(_node: JsIdentifierBinding, _context: SemanticRuleContext) {},
});

defineRule({
	query: semantic("JS_IDENTIFIER_BINDING"),
	run(node) {
		node satisfies JsIdentifierBinding;
	},
});

defineRule<JsReferenceIdentifier>({
	query: ast("JS_REFERENCE_IDENTIFIER"),
	run(node, context) {
		node satisfies JsReferenceIdentifier;
		context satisfies RuleContext;
	},
});
