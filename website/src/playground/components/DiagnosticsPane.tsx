import type { Diagnostic } from "@biomedev/wasm-web";
import type { ReactCodeMirrorRef } from "@uiw/react-codemirror";
import { useState } from "react";
import DiagnosticsConsoleTab from "../tabs/DiagnosticsConsoleTab";
import DiagnosticsListTab from "../tabs/DiagnosticsListTab";
import Tabs from "./Tabs";

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
