interface Props {
	console: string;
}

export default function DiagnosticsConsoleTab({ console }: Props) {
	if (console === "") {
		return <div className="empty-panel">No diagnostics present</div>;
	}

	return (
		<>
			<pre className="language-shellsession diagnostics-console">
				<code
					// biome-ignore lint/security/noDangerouslySetInnerHtml: the HTML is sanitized by our diagnostic printer
					dangerouslySetInnerHTML={{ __html: console }}
				/>
			</pre>
		</>
	);
}
