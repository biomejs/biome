import CodeMirror from "@/playground/CodeMirror";
import Collapsible from "@/playground/Collapsible";
import BiomeHeader from "@/playground/components/BiomeHeader";
import PrettierHeader from "@/playground/components/PrettierHeader";
import type { PrettierOutput } from "@/playground/types";
import { romeAst as BiomeFormatterIr } from "lang-rome-formatter-ir";

interface Props {
	prettier: PrettierOutput;
	biome: string;
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
