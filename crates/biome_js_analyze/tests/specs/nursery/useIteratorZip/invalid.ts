/* should generate diagnostics */

function annotatedArrays(names: readonly string[], scores: number[]) {
	for (let index = 0; index < Math.min(names.length, scores.length); index++) {
		save(names[index], scores[index]);
	}
}

function annotatedTypedArrays(names: Uint8Array, scores: Float64Array) {
	for (let index = 0; index < Math.min(names.length, scores.length); index++) {
		save(names[index], scores[index]);
	}
}

declare const declaredNames: string[];
declare const declaredScores: readonly number[];
for (let index = 0; index < Math.min(declaredNames.length, declaredScores.length); index++) {
	save(declaredNames[index], declaredScores[index]);
}

// Type syntax around a read doesn't change what is read.
for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	save(names[index]!, scores[index] as number);
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	save(<string>names[index], scores[index] satisfies number);
}

for (let index = 0; index < Math.min(names.length, scores.length); index++) {
	save(names[index]<string>, scores[index]);
}

function cachedWithAnnotation() {
	const length: number = Math.min(names.length, scores.length);
	for (let index = 0; index < length; index++) {
		save(names[index], scores[index]);
	}
}

for (let index: number = 0; index < Math.min(names.length, scores.length); index++) {
	save(names[index], scores[index]);
}
