/* should generate diagnostics */
node.kind === "JS_CALL_EXPRESSION" || node.kind === "JS_NEW_EXPRESSION";
"JS_CALL_EXPRESSION" === node.kind || node.kind == "JS_NEW_EXPRESSION";
token.kind !== "EQ2" && token.kind !== "NEQ";
node.parent.kind === "JS_CALL_EXPRESSION" || node.parent.kind === "JS_NEW_EXPRESSION" || node.parent.kind === "JS_IMPORT_CALL_EXPRESSION";
node?.kind === "JS_CALL_EXPRESSION" || node?.kind === "JS_NEW_EXPRESSION";
(node.kind === "JS_CALL_EXPRESSION" || node.kind === "JS_NEW_EXPRESSION") || node.kind === "JS_IMPORT_CALL_EXPRESSION";
node.kind === "JS_CALL_EXPRESSION" || isSpecial(node) || node.kind === "JS_NEW_EXPRESSION";
left.kind === "JS_CALL_EXPRESSION" || right.kind === "IDENT" || left.kind === "JS_NEW_EXPRESSION" || right.kind === "COMMA";
token.kind === "JS_NUMBER_LITERAL" || token.kind === "JS_STRING_LITERAL";
node.kind === `JS_CALL_EXPRESSION` || node.kind === "JS_NEW_EXPRESSION";

["JS_CALL_EXPRESSION", "JS_NEW_EXPRESSION"].includes(node.kind);
!["EQ2", "NEQ"].includes(token.kind);
new Set(["JS_CALL_EXPRESSION", "JS_NEW_EXPRESSION"]).has(node.kind);

const CALL_KINDS = ["JS_CALL_EXPRESSION", "JS_NEW_EXPRESSION"];
CALL_KINDS.includes(node.kind);
ast(...CALL_KINDS);
const CALL_KIND_SET = new Set(CALL_KINDS);
CALL_KIND_SET.has(node.kind);
const EQUALITY_TOKENS = new Set(["EQ2", "NEQ"]);
EQUALITY_TOKENS.has(token.kind);

const CALL = "JS_CALL_EXPRESSION";
const NEW = "JS_NEW_EXPRESSION";
node.kind === CALL || node.kind === NEW;
node.kind === CALL || node.kind === "JS_IMPORT_CALL_EXPRESSION";
[CALL, NEW].includes(node.kind);
