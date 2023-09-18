import fastDiff from "fast-diff";
import CodeMirror, { type BiomeExtension } from "../CodeMirror";
import Collapsible from "../Collapsible";
import BiomeHeader from "../components/BiomeHeader";
import PrettierHeader from "../components/PrettierHeader";
import type { PrettierOutput } from "../types";

interface Props {
	prettier: PrettierOutput;
	biome: string;
	extensions: BiomeExtension[];
}

function removeWhitespace(str: string): string {
	return str.replace(/\s/g, "");
}

function calculateHint(a: string, b: string): string | JSX.Element {
	if (a === b) {
		return <strong>Exact match</strong>;
	} else if (removeWhitespace(a) === removeWhitespace(b)) {
		return <strong>Only whitespace differences</strong>;
	}

	const diff = fastDiff(a, b);
	let insertions = 0;
	let deletions = 0;

	for (const [type] of diff) {
		if (type === fastDiff.INSERT) {
			insertions++;
		} else if (type === fastDiff.DELETE) {
			deletions++;
		}
	}

	return (
		<>
			<span className="insertions">+{insertions}</span>{" "}
			<span className="deletions">-{deletions}</span>
		</>
	);
}

export default function FormatterCodeTab({
	biome,
	prettier,
	extensions,
}: Props) {
	let hint;
	if (prettier.type === "SUCCESS") {
		hint = calculateHint(prettier.code, biome);
	} else {
		hint = <span className="error">Error</span>;
	}

	return (
		<>
			<Collapsible className="biome" heading={<BiomeHeader />}>
				<CodeMirror
					value={biome}
					extensions={extensions}
					placeholder="Biome Output"
					readOnly={true}
				/>
			</Collapsible>
			<Collapsible
				className="prettier"
				heading={
					<>
						<PrettierHeader />
						<span className="diff-hint">{hint}</span>
					</>
				}
			>
				{prettier.type === "ERROR" ? (
					<CodeMirror
						value={prettier.stack}
						placeholder="Prettier Error"
						readOnly={true}
					/>
				) : (
					<CodeMirror
						value={prettier.code}
						extensions={extensions}
						placeholder="Prettier Output"
						readOnly={true}
					/>
				)}
			</Collapsible>
		</>
	);
}
