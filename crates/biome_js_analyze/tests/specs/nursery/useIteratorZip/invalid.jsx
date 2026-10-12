/* should generate diagnostics */

for (let index = 0; index < Math.min(Cards.length, scores.length); index++) {
	render(<Card name={Cards[index]} score={scores[index]} />);
}
