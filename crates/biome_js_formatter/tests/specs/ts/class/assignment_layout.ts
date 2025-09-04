class SourceRemoveUnused extends SourceAction {
	static readonly kind = vscode.CodeActionKind.Source.append(
		"removeUnused",
	).append("ts");

	public static readonly id = "javascript-walkthrough.commands.nodeInstallationFound";
}
