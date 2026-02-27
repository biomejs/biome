/**
 * Side-by-side diff view component for comparing Biome and Prettier output.
 */

// biome-ignore lint/correctness/noUnusedImports: auto-suppressed
import React from "react";

interface DiffViewProps {
	/** Title for this comparison section */
	title: string;
	/** Content from Biome formatter */
	biomeContent: string;
	/** Content from Prettier formatter */
	prettierContent: string;
	/** Whether the outputs match (for visual indicator) */
	isMatch?: boolean;
	/** Whether the content is stale (dimmed while rebuilding) */
	dimmed?: boolean;
}

/**
 * Renders a side-by-side comparison of Biome and Prettier output.
 */
export function DiffView({
	title,
	biomeContent,
	prettierContent,
	isMatch,
	dimmed = false,
}: DiffViewProps) {
	const biomeLines = biomeContent.split("\n");
	const prettierLines = prettierContent.split("\n");
	const maxLines = Math.max(biomeLines.length, prettierLines.length);

	// Build diff indicators
	const diffLines: Array<{
		biome: string;
		prettier: string;
		isDiff: boolean;
	}> = [];

	for (let i = 0; i < maxLines; i++) {
		const biomeLine = biomeLines[i] ?? "";
		const prettierLine = prettierLines[i] ?? "";
		diffLines.push({
			biome: biomeLine,
			prettier: prettierLine,
			isDiff: biomeLine !== prettierLine,
		});
	}

	const matchIndicator =
		isMatch === undefined ? "" : isMatch ? " [MATCH]" : " [DIFF]";
	const matchColor =
		isMatch === undefined ? "#FFFFFF" : isMatch ? "#00FF00" : "#FF6600";

	// Dimmed colors for when content is stale (rebuilding)
	const dimmedMatchColor = "#666666";
	const dimmedHeaderColor = "#555555";
	const dimmedDiffColor = "#885533";
	const dimmedNormalColor = "#777777";

	const titleColor = dimmed ? dimmedMatchColor : matchColor;
	const headerBiomeColor = dimmed ? dimmedHeaderColor : "#00FFFF";
	const headerPrettierColor = dimmed ? dimmedHeaderColor : "#FF00FF";
	const diffColor = dimmed ? dimmedDiffColor : "#FF6600";
	const normalColor = dimmed ? dimmedNormalColor : "#CCCCCC";

	return (
		<box flexDirection="column" width="100%">
			{/* Title */}
			<text fg={titleColor} attributes="bold">
				{title}
				{matchIndicator}
				{dimmed ? " (updating...)" : ""}
			</text>

			{/* Header row */}
			<box flexDirection="row" width="100%">
				<box width="50%">
					<text fg={headerBiomeColor} attributes="underline">
						Biome
					</text>
				</box>
				<box width="50%">
					<text fg={headerPrettierColor} attributes="underline">
						Prettier
					</text>
				</box>
			</box>

			{/* Content rows */}
			{/** biome-ignore lint/correctness/noUnusedFunctionParameters: auto-suppressed */}
			{diffLines.map((line, index) => (
				<box key={line.biome} flexDirection="row" width="100%">
					<box width="50%">
						<text fg={line.isDiff ? diffColor : normalColor}>{line.biome}</text>
					</box>
					<box width="50%">
						<text fg={line.isDiff ? diffColor : normalColor}>
							{line.prettier}
						</text>
					</box>
				</box>
			))}
		</box>
	);
}
