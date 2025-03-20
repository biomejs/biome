loop: for (let i = 0; i < 5; i++) {
	continue loop;
}

while (i--) {
	continue;
}

while (1) {
	continue;
}

for (let i = 0; i < 10; i++) {
	if (i > 5) {
		console.log("foo");
		continue;
	} else if (i >= 5 && i < 8) {
		console.log("test");
	} else {
		console.log("test");
	}
}

for (let i = 0; i < 9; i++) {
	continue;
}

test: for (let i = 0; i < 9; i++) continue test;

test2: do {
	continue test2;
} while (true);