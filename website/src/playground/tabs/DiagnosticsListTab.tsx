import errorIcon from "@/svg/error.svg";
import infoIcon from "@/svg/info.svg";
import warningIcon from "@/svg/warning.svg";
import type { Diagnostic } from "@biomejs/wasm-web";
import { EditorSelection } from "@codemirror/state";
import type { ReactCodeMirrorRef } from "@uiw/react-codemirror";

interface Props {
	editorRef: React.RefObject<ReactCodeMirrorRef>;
	diagnostics: Diagnostic[];
}

function renderDiagnosticMessage(diagnostic: Diagnostic) {
	const parts: JSX.Element[] = [];

	for (let i = 0; i < diagnostic.message.length; i++) {
		const part = diagnostic.message[i]!;
		let text = part.content;

		// Capitalize diagnostic messages...
		// TODO normalize this inside of biome itself
		if (i === 0) {
			text = text[0]?.toUpperCase() + text.slice(1);
		}

		let content: JSX.Element = <span key={i}>{text}</span>;

		for (const elem of part.elements) {
			if (elem === "Emphasis") {
				content = <strong>{content}</strong>;
			} else if (elem === "Underline") {
				content = <u>{content}</u>;
			} else if (elem === "Italic") {
				content = <i>{content}</i>;
			}
		}

		parts.push(content);
	}

	return parts;
}

function DiagnosticIcon({ severity }: { severity: Diagnostic["severity"] }) {
	switch (severity) {
		case "information":
			return <img alt="Info" src={infoIcon.src} />;

		case "warning":
			return <img alt="Warning" src={warningIcon.src} />;

		default:
			return <img alt="Error" src={errorIcon.src} />;
	}
}

function DiagnosticListItem({
	editorRef,
	diagnostic,
}: {
	diagnostic: Diagnostic;
	editorRef: React.RefObject<ReactCodeMirrorRef>;
}) {
	const span = diagnostic.location?.span;

	function onClick() {
		const view = editorRef.current?.view;
		if (view === undefined) {
			return;
		}

		if (span === undefined) {
			return;
		}

		view.dispatch({
			scrollIntoView: true,
			selection: EditorSelection.create([
				EditorSelection.range(span[0], span[1]),
				EditorSelection.cursor(span[0]),
			]),
		});
	}

	return (
		<li onClick={onClick} onKeyDown={onClick}>
			<DiagnosticIcon severity={diagnostic.severity} />
			{renderDiagnosticMessage(diagnostic)}
		</li>
	);
}

export default function DiagnosticsListTab({ editorRef, diagnostics }: Props) {
	if (diagnostics.length === 0) {
		return <div className="empty-panel">No diagnostics present</div>;
	}

	return (
		<ul className="diagnostics-list">
			{diagnostics.map((diag, i) => {
				return (
					// biome-ignore lint/suspicious/noArrayIndexKey: Diagnostic has no stable id.
					<DiagnosticListItem key={i} editorRef={editorRef} diagnostic={diag} />
				);
			})}
		</ul>
	);
}
