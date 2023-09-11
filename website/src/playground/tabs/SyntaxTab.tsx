import type { ReactCodeMirrorRef } from "@uiw/react-codemirror";
import { romeAst as biomeAst } from "codemirror-lang-rome-ast";
import React from "react";
import CodeMirror from "../CodeMirror";
import Collapsible from "../Collapsible";

interface Props {
	ast: string;
	cst: string;
}

const biomeAstCodeMirrorExtension = [biomeAst()];

export default React.forwardRef<ReactCodeMirrorRef, Props>(function SyntaxTab(
	{ ast, cst },
	ref,
) {
	return (
		<>
			<Collapsible heading="AST">
				<CodeMirror
					value={ast}
					ref={ref}
					extensions={biomeAstCodeMirrorExtension}
					readOnly={true}
				/>
			</Collapsible>
			<Collapsible heading="CST">
				<CodeMirror value={cst} readOnly={true} />
			</Collapsible>
		</>
	);
});
