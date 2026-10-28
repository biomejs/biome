/* should not generate diagnostics */

// One of the variables is used as a component.
for (let index = 0; index < Math.min(Cards.length, scores.length); index++) {
	render(<Cards.Item name={Cards[index]} score={scores[index]} />);
}
