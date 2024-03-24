import { useTheme } from "@/playground/utils";
import mermaid from "mermaid";
import { useEffect, useRef, useState } from "react";

interface Props {
	graph: string;
}

export default function ControlFlowTab({ graph }: Props) {
	const theme = useTheme();

	useRef(() => {
		mermaid.initialize({ startOnLoad: true });
	});

	const [graphSvg, setGraphSvg] = useState<string | null>(null);

	useEffect(() => {
		let canceled = false;
		if (graph === "") {
			setGraphSvg(null);
		} else {
			mermaid
				.render(
					"graph-div",
					`%%{init: {'theme':'${
						theme === "dark" ? "dark" : "default"
					}'}}%%\n${graph}`,
				)
				.then(({ svg }) => {
					if (!canceled) {
						setGraphSvg(svg);
					}
				});
		}
		return () => {
			canceled = true;
		};
	}, [theme, graph]);

	return (
		<>
			{graphSvg === null && (
				<div className="empty-panel">No control flow graph present</div>
			)}
			{typeof graphSvg === "string" && (
				<div
					className="mermaid"
					// biome-ignore lint/security/noDangerouslySetInnerHtml: SVG should be safe
					dangerouslySetInnerHTML={{ __html: graphSvg }}
				/>
			)}
		</>
	);
}
