/* should generate diagnostics */
for (const item of items) {
	if (render(<Item />)) {
		const Item = item.component;
		log(Item);
	}
}
