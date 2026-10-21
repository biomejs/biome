/* should not generate diagnostics */

// One of the variables is known to hold something other than an array.
function stringParameter(names: string, scores: number[]) {
	for (let index = 0; index < Math.min(names.length, scores.length); index++) {
		save(names[index], scores[index]);
	}
}

function mapParameter(names: Map<number, string>, scores: number[]) {
	for (let index = 0; index < Math.min(names.length, scores.length); index++) {
		save(names[index], scores[index]);
	}
}

function setParameter(names: string[], scores: ReadonlySet<number>) {
	for (let index = 0; index < Math.min(names.length, scores.length); index++) {
		save(names[index], scores[index]);
	}
}

function objectParameter(names: { length: number; [index: number]: string }, scores: number[]) {
	for (let index = 0; index < Math.min(names.length, scores.length); index++) {
		save(names[index], scores[index]);
	}
}

function annotatedVariable() {
	const names: string = load();
	for (let index = 0; index < Math.min(names.length, scores.length); index++) {
		save(names[index], scores[index]);
	}
}

declare const declaredNames: string;
declare const declaredScores: number[];
for (let index = 0; index < Math.min(declaredNames.length, declaredScores.length); index++) {
	save(declaredNames[index], declaredScores[index]);
}

// An element is called or changed through type syntax.
for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	names[index]!();
	save(scores[index]);
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	(names[index]<string>)(scores[index]);
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	(names[index]<string>)`tag`;
	save(scores[index]);
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	(names[index] as number) = scores[index];
}

// An array is used in a type.
for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	const copy: typeof names = [];
	save(names[index], scores[index], copy);
}
