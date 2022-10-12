export default function (root) {
	const mutation = new JsBatchMutation(root);

	for (const old_token of root.descendants_tokens()) {
		if (old_token.kind() === JsSyntaxKind.IDENT) {
			const new_text = old_token.text_trimmed().toUpperCase();
			const new_token = make.ident(new_text);
			mutation.replace_element(old_token, new_token);
		}
	}

	return mutation;
}
