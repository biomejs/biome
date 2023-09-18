import CodeMirror, { type BiomeExtension } from "../CodeMirror";
interface Props {
	code: string;
	extensions: BiomeExtension[];
}

export default function ImportSortingTab({ code, extensions }: Props) {
	return <CodeMirror value={code} extensions={extensions} readOnly={true} />;
}
