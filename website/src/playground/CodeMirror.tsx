import { useTheme } from "@/playground/utils";
import type { Diagnostic as BiomeDiagnostic } from "@biomejs/wasm-web";
import type { Diagnostic as CodeMirrorDiagnostic } from "@codemirror/lint";
import { lintGutter, setDiagnostics } from "@codemirror/lint";
import type { Extension } from "@codemirror/state";
import { EditorView } from "@codemirror/view";
import type {
	ReactCodeMirrorProps,
	ReactCodeMirrorRef,
} from "@uiw/react-codemirror";
import RealCodeMirror from "@uiw/react-codemirror";
import { forwardRef, useEffect, useMemo, useState } from "react";

export type BiomeExtension = Extension;

interface Props extends ReactCodeMirrorProps {
	diagnostics?: BiomeDiagnostic[];
}

function getDiagnosticMessage(diagnostic: BiomeDiagnostic): string {
	let buf = "";
	for (const elem of diagnostic.message) {
		buf += elem.content;
	}
	return buf;
}

function biomeDiagnosticsToCodeMirror(
	biome: BiomeDiagnostic[],
): CodeMirrorDiagnostic[] {
	const codeMirror: CodeMirrorDiagnostic[] = [];

	for (const diag of biome) {
		const span = diag.location?.span;
		if (span === undefined) {
			continue;
		}

		let severity: CodeMirrorDiagnostic["severity"];
		switch (diag.severity) {
			case "error":
			case "fatal": {
				severity = "error";
				break;
			}

			case "information": {
				severity = "info";
				break;
			}

			case "warning": {
				severity = "warning";
				break;
			}

			default: {
				severity = "error";
			}
		}

		codeMirror.push({
			from: span[0],
			to: span[1],
			severity,
			message: getDiagnosticMessage(diag),
		});
	}

	return codeMirror;
}

function getDefaultExtensions(extensions: Extension[] = []) {
	return [EditorView.lineWrapping, ...extensions];
}

export default forwardRef<ReactCodeMirrorRef, Props>(function CodeMirror(
	{ diagnostics, ...props },
	ref,
) {
	const theme = useTheme();

	const [editor, setEditor] = useState<EditorView>();

	function onCreateEditor(editor: EditorView) {
		setEditor(editor);
	}

	const extensions = useMemo(() => {
		if (diagnostics === undefined || diagnostics.length === 0) {
			return getDefaultExtensions(props.extensions);
		}

		return [lintGutter(), ...getDefaultExtensions(props.extensions)];
	}, [diagnostics, props.extensions]);

	useEffect(() => {
		if (editor !== undefined && diagnostics !== undefined) {
			editor.dispatch(
				setDiagnostics(editor.state, biomeDiagnosticsToCodeMirror(diagnostics)),
			);
		}
	}, [editor, diagnostics]);

	return (
		<RealCodeMirror
			{...props}
			extensions={extensions}
			onCreateEditor={onCreateEditor}
			ref={ref}
			theme={theme}
		/>
	);
});
