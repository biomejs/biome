import { useTheme } from "@/playground/utils";
import mermaid from "mermaid";
import { useMemo } from "react";

interface Props {
	graph: string;
}

let initialized = false;

export default function ControlFlowTab({ graph }: Props) {
	if (graph === "") {
		return <div className="empty-panel">No control flow graph present</div>;
	}

	const theme = useTheme();

	if (!initialized) {
		initialized = true;
		mermaid.initialize({ startOnLoad: true });
	}

	graph = `%%{init: {'theme':'${
		theme === "dark" ? "dark" : "default"
	}'}}%%\n${graph}`;

	const graphSvg = useMemo(() => {
		return mermaid.render("graph-div", graph);
	}, [graph]);

	return (
		// biome-ignore lint/security/noDangerouslySetInnerHtml: SVG should be safe
		<div className="mermaid" dangerouslySetInnerHTML={{ __html: graphSvg }} />
	);
}
