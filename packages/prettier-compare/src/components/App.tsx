/**
 * Main App component that orchestrates the comparison UI.
 */

import React, { useState, useEffect, useCallback } from "react";
import { DiffView } from "./DiffView.js";
import { DiagnosticsView } from "./DiagnosticsView.js";
import { Spinner } from "./Spinner.js";
import { formatWithBiome, type BiomeResult } from "../biome.js";
import { formatWithPrettier, type PrettierResult } from "../prettier.js";
import { getLanguageConfig } from "../languages.js";
import { createWatcher, rebuildWasm } from "../watch.js";

interface AppProps {
	/** The code to format and compare */
	code: string;
	/** The language identifier (e.g., "js", "ts", "json") */
	language: string;
	/** Whether to enable watch mode */
	watchMode: boolean;
	/** Root directory of the Biome repository */
	rootDir: string;
	/** Callback when the app should exit */
	onExit: () => void;
	/** Only show IR comparison */
	irOnly?: boolean;
	/** Only show formatted output comparison */
	outputOnly?: boolean;
	/** Force rebuild WASM before running */
	rebuild?: boolean;
}

/**
 * Main application component that manages state and renders the comparison UI.
 */
export function App({
	code,
	language,
	watchMode,
	rootDir,
	onExit,
	irOnly = false,
	outputOnly = false,
	rebuild = false,
}: AppProps) {
	const [biomeResult, setBiomeResult] = useState<BiomeResult | null>(null);
	const [prettierResult, setPrettierResult] = useState<PrettierResult | null>(
		null,
	);
	const [isLoading, setIsLoading] = useState(true);
	const [isRebuilding, setIsRebuilding] = useState(false);
	const [rebuildingFile, setRebuildingFile] = useState<string | null>(null);
	const [statusMessage, setStatusMessage] = useState<string | null>(null);
	const [error, setError] = useState<string | null>(null);

	const runComparison = useCallback(async () => {
		setIsLoading(true);
		setError(null);

		try {
			const config = getLanguageConfig(language);
			const [biome, prettier] = await Promise.all([
				formatWithBiome(code, config.biomeFilePath),
				formatWithPrettier(code, config.prettierParser),
			]);
			setBiomeResult(biome);
			setPrettierResult(prettier);
		} catch (err) {
			setError(err instanceof Error ? err.message : String(err));
		} finally {
			setIsLoading(false);
		}
	}, [code, language]);

	// Initial setup: optionally rebuild WASM, then run comparison
	useEffect(() => {
		async function initialize() {
			if (rebuild) {
				setStatusMessage("Rebuilding WASM (--rebuild flag)...");
				setIsRebuilding(true);
				try {
					await rebuildWasm(rootDir);
					// reloading biome is handled by bun hot reloading
					setStatusMessage(null);
				} catch (err) {
					setError(
						`Initial build failed: ${err instanceof Error ? err.message : String(err)}`,
					);
				} finally {
					setIsRebuilding(false);
				}
			}

			await runComparison();
		}

		initialize();
	}, [rebuild, rootDir, runComparison]);

	// Set up watch mode
	useEffect(() => {
		if (!watchMode) return;

		const watcher = createWatcher(rootDir);

		watcher.on("rebuilding", (changedFile) => {
			setIsRebuilding(true);
			setRebuildingFile(changedFile);
			setStatusMessage(`Rebuilding WASM... (${changedFile})`);
		});

		watcher.on("rebuilt", async () => {
			setStatusMessage("Reloading Biome...");
			// reloading biome is handled by bun hot reloading
			setStatusMessage(null);
			await runComparison();
			setIsRebuilding(false);
			setRebuildingFile(null);
		});

		watcher.on("error", (err) => {
			setError(`Build failed: ${err.message}`);
			setIsRebuilding(false);
			setRebuildingFile(null);
			setStatusMessage(null);
		});

		return () => {
			watcher.close();
		};
	}, [watchMode, rootDir, runComparison]);

	// Show loading spinner only on initial load (no existing results to show)
	if (isLoading && !biomeResult && !prettierResult) {
		return <Spinner message={statusMessage ?? "Formatting..."} />;
	}

	// Show error if something went wrong and we have no results to display
	if (error && !biomeResult && !prettierResult) {
		return <text fg="#FF0000">Error: {error}</text>;
	}

	// If we have results, always show them (even while rebuilding)

	const outputMatch = biomeResult?.output === prettierResult?.output;
	const config = getLanguageConfig(language);

	return (
		<box flexDirection="column" width="100%" height="100%">
			{/* Status bar - always visible at top */}
			{isRebuilding && (
				<box flexDirection="row" gap={1}>
					<Spinner
						message={
							statusMessage ??
							`Rebuilding WASM... (${rebuildingFile ?? "changed"})`
						}
					/>
				</box>
			)}

			{watchMode && !isRebuilding && (
				<text fg="#666666">
					Watch mode active ({config.displayName}). Press Ctrl+C to exit.
				</text>
			)}

			{error && <text fg="#FF6600">Warning: {error}</text>}

			<scrollbox focused={watchMode} flexGrow={1}>
				<box flexDirection="column" gap={1}>
					{/* Formatted output comparison */}
					{!irOnly && (
						<DiffView
							title="Formatted Output"
							biomeContent={biomeResult?.output ?? ""}
							prettierContent={prettierResult?.output ?? ""}
							isMatch={outputMatch}
							dimmed={isRebuilding}
						/>
					)}

					{/* IR comparison */}
					{!outputOnly && (
						<DiffView
							title="IR (Intermediate Representation)"
							biomeContent={biomeResult?.ir ?? ""}
							prettierContent={prettierResult?.ir ?? ""}
							isMatch={false} // IRs always differ in format
							dimmed={isRebuilding}
						/>
					)}

					{/* Diagnostics section */}
					<DiagnosticsView
						biomeDiagnostics={biomeResult?.diagnostics ?? []}
						prettierError={prettierResult?.error}
					/>

					{/* Language info */}
					{!watchMode && (
						<text fg="#666666">
							Language: {config.displayName} | Prettier parser:{" "}
							{config.prettierParser}
						</text>
					)}
				</box>
			</scrollbox>
		</box>
	);
}
