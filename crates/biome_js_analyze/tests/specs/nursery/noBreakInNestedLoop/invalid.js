/* should generate diagnostics */
for (const item of items) {
	for (const child of item.children) {
		break;
	}
}

for (const item of items) {
	switch (item.type) {
		case "child":
			break;
	}
}

for (let index = 0; index < items.length; index++) {
	for (const child of items[index].children) {
		break;
	}
}

for (const key in items) {
	do {
		break;
	} while (items[key].pending);
}

while (queue.length > 0) {
	while (true) {
		if (done) break;
	}
}

// A `switch` statement between the inner loop and the outer loop.
for (const a of b) {
	switch (a) {
		case 1:
			for (const c of d) {
				break;
			}
	}
}

// A labeled statement around the inner loop doesn't change the target.
for (const item of items) {
	inner: for (const child of item.children) {
		break;
	}
}

function f() {
	for (const item of items) {
		for (const child of item.children) {
			break;
		}
	}
}

for (const group of groups) {
	const processGroup = () => {
		for (const item of group.items) {
			for (const child of item.children) {
				break;
			}
		}
	};
	processGroup();
}
