test3: do {
	if (i < 2) {
		continue test3;
	}
	i++;
} while (true);

while (i) {
	if (i > 5) {
		continue;
	}
	console.log(i);
	i--;
}

while (i) {
	continue;
	console.log(i);
}

while (condition) {
	if (conditionZ) {
		if (conditionX) {
			console.log("log");
			continue;
		}
		console.log("log");
		if (conditionY) {
			console.log("log");
		}
	}
}

loop: while (1) {
	forLoop: for (let i = 0; i < 5; i++) {
		if (someCondition) {
			continue loop;
		}
	}
}

loop: for (let i = 0; i < 10; i++) {
	for (let j = 0; j < byteLength; j++) {
		if (condition) {
			continue loop;
		}
	}
}

for (const x of []) {
	if (x) {
		// before
		continue; // statement
		// after
	} else {
		doSomeStuff();
	}
}