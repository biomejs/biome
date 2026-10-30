/* should generate diagnostics */
let message1 = `I have ${color ? `${count} ${color}` : count} apples`;

// Each level of nesting is reported.
let message2 = `I have ${color ? `${x ? `indeed 0` : count} ${color}` : count} apples`;
let message3 = `I have ${color ? `${x ? `indeed ${0}` : count} ${color}` : count} apples`;

// Tagged templates count on either side.
let message4 = tag`I have ${color ? `${count} ${color}` : count} apples`;
let message5 = tag`I have ${color ? tag`${count} ${color}` : count} apples`;

// Every nested template literal is reported.
let message6 = `I have ${color ? `${count} ${color}` : `this is ${count}`} apples`;
let message7 = `I have ${`${count} ${color}`} ${`this is ${count}`} apples`;

// Starts on a different line but ends on the same line as the outer template.
let message8 = `I have
${color ? `${count} ${color}` : count} apples`;

// Starts on the same line but ends on a different line than the outer template.
let message9 = `I have ${color ? `${count} ${color}` : count}
apples`;

// Nested inside a function inside the template.
let message10 = `${items.map((item) => `<li>${item}</li>`)}`;

// The tag spans several lines, but the backticks start on the same line.
let message11 = foo
	.bar`I have ${`${count}`}
apples`;
