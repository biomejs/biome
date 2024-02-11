import Tabs from "@/playground/components/Tabs";
import DiagnosticsConsoleTab from "@/playground/tabs/DiagnosticsConsoleTab";
import DiagnosticsListTab from "@/playground/tabs/DiagnosticsListTab";
import type { Diagnostic } from "@biomejs/wasm-web";
import type { ReactCodeMirrorRef } from "@uiw/react-codemirror";
import { useState } from "react";

interface Props {
	editorRef: React.RefObject<ReactCodeMirrorRef>;
	console: string;
	diagnostics: Diagnostic[];
}

export default function DiagnosticsPane({
	editorRef,
	diagnostics,
	console,
}: Props) {
	const [tab, setTab] = useState("diagnostics");

	return (
		<Tabs
			className="diagnostics-tabs"
			selectedTab={tab}
			onSelect={setTab}
			tabs={[
				{
					key: "diagnostics",
					title: "Diagnostics",
					children: (
						<DiagnosticsListTab
							editorRef={editorRef}
							diagnostics={diagnostics}
						/>
					),
				},
				{
					key: "console",
					title: "Console",
					children: <DiagnosticsConsoleTab console={console} />,
				},
			]}
		/>
	);
}
