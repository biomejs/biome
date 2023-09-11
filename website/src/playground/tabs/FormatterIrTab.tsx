import { romeAst as BiomeFormatterIr } from "lang-rome-formatter-ir";
import CodeMirror from "../CodeMirror";
import Collapsible from "../Collapsible";
import PrettierHeader from "../components/PrettierHeader";
import BiomeHeader from "../components/BiomeHeader";
import type { PrettierOutput } from "../types";

interface Props {
	prettier: PrettierOutput;
	Biome: string;
}

const formatterIrCodeMirrorExtension = [BiomeFormatterIr()];

export default function FormatterIrTab({ biome, prettier }: Props) {
	return (
		<>
			<Collapsible className="biome" heading={<BiomeHeader />}>
				<CodeMirror
					value={biome}
					extensions={formatterIrCodeMirrorExtension}
					readOnly={true}
				/>
			</Collapsible>
			<Collapsible className="prettier" heading={<PrettierHeader />}>
				{prettier.type === "ERROR" ? (
					<CodeMirror value={prettier.stack} readOnly={true} />
				) : (
					<CodeMirror
						value={prettier.ir}
						extensions={formatterIrCodeMirrorExtension}
						readOnly={true}
					/>
				)}
			</Collapsible>
		</>
	);
}
