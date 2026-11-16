import {
	type AnyJsAstNode,
	type JsAstNode,
	type JsAstToken,
	type TriviaKind,
	type TriviaPiece,
	type JsVariableStatement,
	factory,
	registerDiagnostic,
} from "@biomejs/runtime/plugin";

declare const base: JsAstNode;
declare const anyNode: AnyJsAstNode;
declare const statement: JsVariableStatement;
declare const token: JsAstToken;

base.leadingTrivia() satisfies readonly TriviaPiece[];
base.trailingTrivia() satisfies readonly TriviaPiece[];
anyNode.trailingTrivia() satisfies readonly TriviaPiece[];
statement.leadingTrivia() satisfies readonly TriviaPiece[];
token.leadingTrivia() satisfies readonly TriviaPiece[];
token.trailingTrivia() satisfies readonly TriviaPiece[];
token.leadingComments() satisfies readonly TriviaPiece[];
token.trailingComments() satisfies readonly TriviaPiece[];
factory.token("LET_KW").trailingComments() satisfies readonly TriviaPiece[];
factory.token("LET_KW").leadingTrivia() satisfies readonly TriviaPiece[];

const [piece] = base.leadingTrivia();
if (piece) {
	piece.kind satisfies TriviaKind;
	piece.text satisfies string;
	piece.token satisfies JsAstToken;
	piece.token.leadingTrivia() satisfies readonly TriviaPiece[];
	piece.isComment() satisfies boolean;
	if (piece.isComment()) {
		piece.kind satisfies "singleLineComment" | "multiLineComment";
		// @ts-expect-error A comment is not whitespace.
		piece.kind === "whitespace";
	}

	switch (piece.kind) {
		case "newline":
		case "whitespace":
		case "singleLineComment":
		case "multiLineComment":
			break;
		default:
			piece.kind satisfies never;
	}

	// @ts-expect-error Skipped text is not exposed.
	piece.kind === "skipped";
	// @ts-expect-error Trivia kinds are not syntax kinds.
	piece.kind === "COMMENT";
	// @ts-expect-error Pieces are read-only.
	piece.text = "";
	// @ts-expect-error Pieces are read-only.
	piece.token = token;
}

// @ts-expect-error Trivia arrays are not mutable.
base.leadingTrivia() satisfies TriviaPiece[];
// @ts-expect-error Trivia arrays cannot be appended.
token.leadingTrivia().push(piece);
// @ts-expect-error Nodes do not collect the comments of all their tokens.
base.comments();
// @ts-expect-error Comment helpers belong to tokens, not nodes.
base.leadingComments();
// @ts-expect-error Comment arrays cannot be appended.
token.trailingComments().push(piece);
// @ts-expect-error Trivia pieces do not have their own trivia.
piece?.leadingTrivia();

registerDiagnostic(base, "information", "node");
registerDiagnostic(anyNode, "information", "node");
registerDiagnostic(token, "information", "token");
registerDiagnostic(base.leadingTrivia()[0]!, "information", "comment");
// @ts-expect-error Diagnostics need a node, token, or trivia piece.
registerDiagnostic("text", "information", "string");
// @ts-expect-error Diagnostics need a node, token, or trivia piece.
registerDiagnostic(base.leadingTrivia(), "information", "array");

// @ts-expect-error Trivia is not specific to JavaScript, so the type has no `Js` prefix.
type _RenamedPiece = import("@biomejs/runtime/plugin").JsTriviaPiece;
