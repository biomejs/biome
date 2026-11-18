/* should generate diagnostics */

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	save(names[index], scores[index]);
}

function parameters(names, scores) {
	for (let index = 0; index < Math.min(names.length, scores.length); index++) {
		save(names[index], scores[index]);
	}
}

function arrayLiterals() {
	const names = ["a", "b"];
	const scores = [1];
	for (let index = 0; index < Math.min(names.length, scores.length); index++) {
		save(names[index], scores[index]);
	}
}

function typedArrays() {
	const names = new Uint8Array(2);
	const scores = new Float64Array(3);
	for (let index = 0; index < Math.min(names.length, scores.length); index++) {
		save(names[index], scores[index]);
	}
}

function reassignableString() {
	let names = "abc";
	names = load();
	for (let index = 0; index < Math.min(names.length, scores.length); index++) {
		save(names[index], scores[index]);
	}
}

// Other ways to add one to the index.
for (let index = 0; index < Math.min(names.length, scores.length); ++index) {
	save(names[index], scores[index]);
}

for (let index = 0; index < Math.min(names.length, scores.length); index += 1) {
	save(names[index], scores[index]);
}

for (let index = 0; index < Math.min(names.length, scores.length); index = index + 1) {
	save(names[index], scores[index]);
}

// The comparison can be reversed.
for (let index = 0; Math.min(names.length, scores.length) > index; index++) {
	save(names[index], scores[index]);
}

// More than two arrays.
for (let index = 0; index < Math.min(names.length, scores.length, ranks.length); index++) {
	save(names[index], scores[index], ranks[index]);
}

// One comparison for each array.
for (let index = 0; index < names.length && index < scores.length; index++) {
	save(names[index], scores[index]);
}

for (let index = 0; names.length > index && index < scores.length; index++) {
	save(names[index], scores[index]);
}

for (let index = 0; index < names.length && (scores.length > index && index < ranks.length); index++) {
	save(names[index], scores[index], ranks[index]);
}

// The elements are read in different ways.
for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	save(names[index], scores[index], names[index]);
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	const name = names[index];
	const score = scores[index];
	save(name, score);
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	if (!names[index]) {
		continue;
	}
	save(names[index], scores[index]);
	break;
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	save((names)[(index)], ((scores[index])));
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	save(names[index].value, scores[index].value);
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	names[index].save(scores[index]);
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	names[index].value = scores[index];
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	save(new names[index](scores[index]));
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	for (const letter of names[index]) {
		save(letter, scores[index]);
	}
}

// Variables with the same names in a nested block are unrelated.
for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	save(names[index], scores[index]);
	{
		const names = other;
		const index = 0;
		save(names[index]);
	}
}

// A nested function that doesn't use the index or the arrays.
for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	save(names[index], scores[index]);
	function getWidget() {
		return widget;
	}
}

// The shortest length is stored in a variable.
function cachedConst() {
	const length = Math.min(names.length, scores.length);
	for (let index = 0; index < length; index++) {
		save(names[index], scores[index]);
	}
}

function cachedLet() {
	let length = Math.min(names.length, scores.length);
	for (let index = 0; length > index; index++) {
		save(names[index], scores[index]);
	}
}

for (let index = 0, length = Math.min(names.length, scores.length); index < length; index++) {
	save(names[index], scores[index]);
}
