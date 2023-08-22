import { romeAst as RomeFormatterIr } from "lang-rome-formatter-ir";
import CodeMirror from "../CodeMirror";
import Collapsible from "../Collapsible";
import PrettierHeader from "../components/PrettierHeader";
import RomeHeader from "../components/RomeHeader";
import type { PrettierOutput } from "../types";

interface Props {
	prettier: PrettierOutput;
	rome: string;
}

const formatterIrCodeMirrorExtension = [RomeFormatterIr()];

export default function FormatterIrTab({ rome, prettier }: Props) {
	return (
		<>
			<Collapsible className="rome" heading={<RomeHeader />}>
				<CodeMirror
					value={rome}
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
