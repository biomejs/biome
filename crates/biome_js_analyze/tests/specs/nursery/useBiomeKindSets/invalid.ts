/* should generate diagnostics */
const CALL_KINDS = ["JS_CALL_EXPRESSION", "JS_NEW_EXPRESSION"] as const;
CALL_KINDS.includes(node.kind);
(node as AnyNode).kind === "JS_CALL_EXPRESSION" || (node as AnyNode).kind === "JS_NEW_EXPRESSION";
