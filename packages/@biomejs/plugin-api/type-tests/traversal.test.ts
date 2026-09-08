import type {
	AnyJsAstNode,
	AnyJsCallArgument,
	AstroImplicitFragment,
	JsAstNode,
	JsCallArgumentList,
	JsCallArgumentListNode,
	JsCallArguments,
	JsFunctionDeclaration,
	JsModule,
	JsModuleItemListNode,
	JsNodeByKind,
	JsVariableStatement,
	JsxChildList,
	JsxChildListNode,
	JsxElement,
	JsxFragment,
} from "@biomejs/plugin-api";

declare const base: JsAstNode;
declare const anyNode: AnyJsAstNode;
declare const mappedNode: JsNodeByKind[keyof JsNodeByKind];
declare const node: JsVariableStatement;

anyNode satisfies JsNodeByKind[keyof JsNodeByKind];
mappedNode satisfies AnyJsAstNode;
base.parent satisfies AnyJsAstNode | undefined;
base.ancestors() satisfies readonly AnyJsAstNode[];
base.children() satisfies readonly AnyJsAstNode[];
undefined satisfies typeof base.parent;

// @ts-expect-error Parent absence is represented by undefined, not null.
null satisfies typeof base.parent;
// @ts-expect-error The root has no parent, so a parent must be checked first.
base.parent satisfies AnyJsAstNode;
// @ts-expect-error Parent links cannot be reassigned.
base.parent = anyNode;
// @ts-expect-error Ancestors are not a mutable array.
base.ancestors() satisfies AnyJsAstNode[];
// @ts-expect-error Ancestors cannot be appended.
base.ancestors().push(anyNode);
// @ts-expect-error Ancestor entries cannot be replaced.
base.ancestors()[0] = anyNode;

// @ts-expect-error Child nodes are not a mutable array.
base.children() satisfies AnyJsAstNode[];
// @ts-expect-error Child nodes cannot be appended.
base.children().push(anyNode);
// @ts-expect-error Child entries cannot be replaced.
base.children()[0] = anyNode;

const parent = node.parent;
if (parent?.kind === "JS_MODULE_ITEM_LIST") {
	parent satisfies JsModuleItemListNode;
	parent.children() satisfies readonly AnyJsAstNode[];
	// @ts-expect-error A list wrapper is not the module containing it.
	parent.items;
	if (parent.parent?.kind === "JS_MODULE") {
		parent.parent satisfies JsModule;
		parent.parent.items;
	}
}

for (const ancestor of node.ancestors()) {
	ancestor satisfies AnyJsAstNode;
	if (ancestor.kind === "JS_FUNCTION_DECLARATION") {
		ancestor satisfies JsFunctionDeclaration;
		ancestor.body;
		// @ts-expect-error Kind narrowing excludes module roots.
		ancestor.items;
	} else if (ancestor.kind === "JS_MODULE_ITEM_LIST") {
		ancestor satisfies JsModuleItemListNode;
	} else if (ancestor.kind === "JS_MODULE") {
		ancestor satisfies JsModule;
	}
}

for (const child of base.children()) {
	child satisfies AnyJsAstNode;
	if (child.kind === "JS_VARIABLE_STATEMENT") {
		child satisfies JsVariableStatement;
		child.declaration;
		// @ts-expect-error Kind narrowing excludes module roots.
		child.items;
	} else if (child.kind === "JS_CALL_ARGUMENT_LIST") {
		child satisfies JsCallArgumentListNode;
		child.children() satisfies readonly AnyJsAstNode[];
		// @ts-expect-error A list wrapper is not a named-field array.
		child satisfies JsCallArgumentList;
	} else if (child.kind === "JSX_CHILD_LIST") {
		child satisfies JsxChildListNode;
	}
}

declare const args: JsCallArguments;
declare const list: JsCallArgumentListNode;
declare const mappedList: JsNodeByKind["JS_CALL_ARGUMENT_LIST"];

list satisfies JsAstNode;
list satisfies AnyJsAstNode;
list satisfies JsNodeByKind["JS_CALL_ARGUMENT_LIST"];
mappedList satisfies JsCallArgumentListNode;
list.kind satisfies "JS_CALL_ARGUMENT_LIST";
list.children() satisfies readonly AnyJsAstNode[];
args.args satisfies JsCallArgumentList;
args.args satisfies readonly AnyJsCallArgument[];
// @ts-expect-error Named-field arrays are not AST nodes.
args.args satisfies JsAstNode;
// @ts-expect-error Named-field arrays are not list wrappers.
args.args satisfies JsCallArgumentListNode;
// @ts-expect-error List wrappers are not arrays.
list satisfies JsCallArgumentList;
// @ts-expect-error Named-field arrays are readonly.
args.args.push(args.args[0]);

declare const jsx: JsxElement;
declare const fragment: JsxFragment;
declare const astro: AstroImplicitFragment;

jsx.elements satisfies JsxChildList;
fragment.elements satisfies JsxChildList;
astro.elements satisfies JsxChildList;
jsx.children() satisfies readonly AnyJsAstNode[];
fragment.children() satisfies readonly AnyJsAstNode[];
astro.children() satisfies readonly AnyJsAstNode[];
// @ts-expect-error JSX children is a traversal method, not a named-field array.
jsx.children satisfies JsxChildList;
// @ts-expect-error JSX named-field arrays are not list wrappers.
jsx.elements satisfies JsxChildListNode;
// @ts-expect-error JSX named-field arrays are readonly.
jsx.elements.push(jsx.elements[0]);
