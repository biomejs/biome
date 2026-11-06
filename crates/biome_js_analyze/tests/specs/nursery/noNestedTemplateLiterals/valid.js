/* should not generate diagnostics */
let nestedMessage = `${count} ${color}`;
let message1 = `I have ${color ? nestedMessage : count} apples`;

// Starts and ends on different lines than the outer template.
let message2 = `I have
${color ? `${count} ${color}` : count}
apples`;

let message3 = `
	${items.map((item) => `<li>${item}</li>`)}
`;

// A template literal used as the tag of another template literal is not inside it.
let message4 = `a``b`;

// The line breaks are inside the nested template, not between the backticks.
let message5 = `
	${`a
	b`}
`;
