/**
 * Diagnostics view component for displaying errors and warnings.
 */

// biome-ignore lint/correctness/noUnusedImports: auto-suppressed
import React from "react";
import type { BiomeDiagnostic } from "../biome.js";

interface DiagnosticsViewProps {
	/** Diagnostics from Biome */
	biomeDiagnostics: BiomeDiagnostic[];
	/** Error message from Prettier, if any */
	prettierError?: string;
}

/**
 * Renders diagnostics and errors from both formatters.
 */
export function DiagnosticsView({
	biomeDiagnostics,
	prettierError,
}: DiagnosticsViewProps) {
	const hasDiagnostics = biomeDiagnostics.length > 0 || prettierError;

	if (!hasDiagnostics) {
		return null;
	}

	return (
		<box flexDirection="column" width="100%">
			<text fg="#FFFF00" attributes="bold">
				Diagnostics
			</text>

			{/* Biome diagnostics */}
			{biomeDiagnostics.length > 0 && (
				<box flexDirection="column">
					<text fg="#00FFFF">Biome:</text>
					{biomeDiagnostics.map((d, index) => (
						<text
							// biome-ignore lint/suspicious/noArrayIndexKey: auto-suppressed
							key={index}
							fg={d.severity === "error" ? "#FF0000" : "#FF6600"}
						>
							{"  "}[{d.severity}] {d.description}
						</text>
					))}
				</box>
			)}

			{/* Prettier error */}
			{prettierError && (
				<box flexDirection="column">
					<text fg="#FF00FF">Prettier:</text>
					<text fg="#FF0000">
						{"  "}[error] {prettierError}
					</text>
				</box>
			)}
		</box>
	);
}
