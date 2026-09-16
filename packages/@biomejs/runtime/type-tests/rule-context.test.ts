import {
	ast,
	defineRule,
	type JsFileSource,
	type JsVariableStatement,
	type RuleContext,
} from "@biomejs/runtime/plugin";

defineRule({
	query: ast("JS_VARIABLE_STATEMENT"),
	run(node, context) {
		node satisfies JsVariableStatement;
		context satisfies RuleContext;
		context.filePath satisfies string;
		context.sourceType satisfies JsFileSource;
		if (context.sourceType.language.kind === "typescript") {
			context.sourceType.language.definitionFile satisfies boolean;
			// @ts-expect-error Source metadata is read-only.
			context.sourceType.language.definitionFile = true;
		} else {
			// @ts-expect-error JavaScript sources have no definition-file flag.
			context.sourceType.language.definitionFile;
		}
		const embedding = context.sourceType.embeddingKind;
		switch (embedding.kind) {
			case "astro":
				embedding.frontmatter satisfies boolean;
				embedding.isClassAttribute satisfies boolean;
				break;
			case "vue":
				embedding.setup satisfies boolean;
				embedding.isSource satisfies boolean;
				embedding.eventHandler satisfies boolean;
				break;
			case "svelte":
				embedding.fileKind satisfies "component" | "sourceModule";
				embedding.embeddingKind satisfies
					| "source"
					| "expression"
					| "snippetSignature"
					| "legacyConst"
					| "declaration";
				break;
			case "none":
				break;
			default:
				embedding satisfies never;
		}
		// @ts-expect-error File paths are read-only.
		context.filePath = "another.ts";
		// @ts-expect-error Source types are read-only.
		context.sourceType = context.sourceType;
		// @ts-expect-error Nested source metadata is read-only.
		context.sourceType.variant = "jsx";
		// @ts-expect-error Embedding tags are read-only.
		embedding.kind = "none";
	},
});
