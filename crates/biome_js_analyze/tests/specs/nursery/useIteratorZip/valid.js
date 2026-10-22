/* should not generate diagnostics */

names.map((name, index) => scores[index]);

for (const [name, score] of Iterator.zip([names, scores])) {
	save(name, score);
}

// The loop doesn't stop at the shortest of several arrays.
for (let index = 0; index < names.length; index++) {
	save(names[index], scores[index]);
}

for (let index = 0; index < length; index++) {
	save(names[index], scores[index]);
}

for (let index = 0; index < Math.max(names.length, scores.length); index++) {
	save(names[index], scores[index]);
}

for (let index = 0; index < Math.min(names.length, names.length); index++) {
	save(names[index], scores[index]);
}

for (let index = 0; index < Math.min(names.length, scores.length, 5); index++) {
	save(names[index], scores[index]);
}

for (let index = 0; index < Math.min(...lengths); index++) {
	save(names[index], scores[index]);
}

for (let index = 0; index < Math.min(names.length); index++) {
	save(names[index], scores[index]);
}

for (let index = 0; index < Math?.min(names.length, scores.length); index++) {
	save(names[index], scores[index]);
}

for (let index = 0; index < Math.min?.(names.length, scores.length); index++) {
	save(names[index], scores[index]);
}

for (let index = 0; index < Math.min(names?.length, scores.length); index++) {
	save(names[index], scores[index]);
}

for (let index = 0; index <= Math.min(names.length, scores.length); index++) {
	save(names[index], scores[index]);
}

for (let index = 0; index < names.length || index < scores.length; index++) {
	save(names[index], scores[index]);
}

for (let index = 0; index < names.length && enabled && index < scores.length; index++) {
	save(names[index], scores[index]);
}

for (let index = 0; index < names.length && index < names.length; index++) {
	save(names[index], scores[index]);
}

for (let index = 0; index < object.names.length && index < scores.length; index++) {
	save(names[index], scores[index]);
}

for (let index = 0; ; index++) {
	save(names[index], scores[index]);
}

function shadowedMath(Math) {
	for (let index = 0; index < Math.min(names.length, scores.length); index++) {
		save(names[index], scores[index]);
	}
}

// The index isn't a `let` variable that counts up from zero by one.
for (var index = 0; index < Math.min(names.length, scores.length); index++) {
	save(names[index], scores[index]);
}

for (const index = 0; index < Math.min(names.length, scores.length); index++) {
	save(names[index], scores[index]);
}

for (let index = 1; index < Math.min(names.length, scores.length); index++) {
	save(names[index], scores[index]);
}

for (let index = 0, length = 5; index < Math.min(names.length, scores.length); index++) {
	save(names[index], scores[index]);
}

for (let index = 0, length = Math.min(names.length, scores.length), other = 1; index < length; index++) {
	save(names[index], scores[index]);
}

for (let index = 0; index < Math.min(names.length, scores.length); index += 2) {
	save(names[index], scores[index]);
}

for (let index = 0; index < Math.min(names.length, scores.length); index--) {
	save(names[index], scores[index]);
}

for (let index = 0; index < Math.min(names.length, scores.length); index = index + 2) {
	save(names[index], scores[index]);
}

for (let index = 0; index < Math.min(names.length, scores.length); index = otherIndex + 1) {
	save(names[index], scores[index]);
}

for (let index = 0; index < Math.min(names.length, scores.length); otherIndex++) {
	save(names[index], scores[index]);
}

// The variable that stores the shortest length is used elsewhere, or isn't declared right before the loop.
function cachedUsedAfterLoop() {
	const length = Math.min(names.length, scores.length);
	for (let index = 0; index < length; index++) {
		save(names[index], scores[index]);
	}
	use(length);
}

function cachedNotAdjacent() {
	const length = Math.min(names.length, scores.length);
	prepare();
	for (let index = 0; index < length; index++) {
		save(names[index], scores[index]);
	}
}

function cachedWithOtherVariable() {
	const length = Math.min(names.length, scores.length), other = 1;
	for (let index = 0; index < length; index++) {
		save(names[index], scores[index]);
	}
}

function cachedVar() {
	var length = Math.min(names.length, scores.length);
	for (let index = 0; index < length; index++) {
		save(names[index], scores[index]);
	}
}

function cachedSingleArray() {
	const length = names.length;
	for (let index = 0; index < length; index++) {
		save(names[index], scores[index]);
	}
}

for (let index = 0, length = Math.min(names.length, scores.length); index < length; index++) {
	save(names[index], scores[index]);
	use(length);
}

// The body isn't a block.
for (let index = 0; index < Math.min(names.length, scores.length); index++)
	save(names[index], scores[index]);

// The index or the arrays are used for something other than reading `array[index]`.
for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	save(names[index]);
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	save(names[index], scores[index], index);
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	save(names[index + 1], scores[index]);
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	save(names?.[index], scores[index]);
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	save(names[index], scores[index], other[index]);
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	save(names[index], scores[index], names[0]);
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	save(names[index], scores[index]);
	names.push("new");
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	save(names[index], scores[index]);
	scores = replacement;
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	save(names[index], scores[index]);
	consume(scores);
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	save(names[index], scores[index]);
	consume({ scores });
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	save(names[index], scores[index]);
	index++;
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	names[index] = scores[index];
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	names[index] += scores[index];
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	names[index]++;
	save(scores[index]);
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	delete names[index];
	save(scores[index]);
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	[names[index]] = scores[index];
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	({ value: names[index] } = scores[index]);
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	for (names[index] of scores[index]) {
	}
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	for (names[index] in scores[index]) {
	}
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	names[index](scores[index]);
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	names[index]?.(scores[index]);
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	names[index]`tag`;
	save(scores[index]);
}

// The index or the arrays are used inside a nested function or class.
for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	callbacks.push(() => save(names[index], scores[index]));
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	callbacks.push({
		get pair() {
			return [names[index], scores[index]];
		},
	});
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	class Pair {
		value = save(names[index], scores[index]);
	}
}

// A variable with the same name in a nested block is read with the index.
for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	save(names[index], scores[index]);
	{
		const names = other;
		save(names[index]);
	}
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	save(names[index], scores[index]);
	{
		const index = 0;
		save(names[index]);
	}
}

// The arrays are properties.
for (let index = 0; index < Math.min(object.names.length, object.scores.length); index++) {
	save(object.names[index], object.scores[index]);
}

// One of the variables is known to hold something other than an array.
function string() {
	const names = "abc";
	for (let index = 0; index < Math.min(names.length, scores.length); index++) {
		save(names[index], scores[index]);
	}
}

function templateString() {
	const names = `abc`;
	for (let index = 0; index < Math.min(names.length, scores.length); index++) {
		save(names[index], scores[index]);
	}
}

function objectLiteral() {
	const names = { 0: "a", length: 1 };
	for (let index = 0; index < Math.min(names.length, scores.length); index++) {
		save(names[index], scores[index]);
	}
}

function set() {
	const scores = new Set();
	for (let index = 0; index < Math.min(names.length, scores.length); index++) {
		save(names[index], scores[index]);
	}
}

function callback() {
	const names = (first, second) => first;
	for (let index = 0; index < Math.min(names.length, scores.length); index++) {
		save(names[index], scores[index]);
	}
}

// An array is declared again inside the body.
function redeclaredVariable() {
	for (let index = 0; index < Math.min(names.length, scores.length); index++) {
		var names = refresh();
		save(names[index], scores[index]);
	}
}

function redeclaredParameter(names, scores) {
	for (let index = 0; index < Math.min(names.length, scores.length); index++) {
		var names = refresh();
		save(names[index], scores[index]);
	}
}

function redeclaredInNestedLoop(names, scores) {
	for (let index = 0; index < Math.min(names.length, scores.length); index++) {
		for (var names of rows) {
		}
		save(names[index], scores[index]);
	}
}

function redeclaredAsFunction() {
	for (let index = 0; index < Math.min(names.length, scores.length); index++) {
		save(names[index], scores[index]);
		function names() {}
	}
}
