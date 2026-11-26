/* should not generate diagnostics */
for (const item of items) {
	if (item.done) {
		break;
	}
}

// `continue` isn't checked by default.
for (const group of groups) {
	for (const item of group) {
		if (!item.isValid) {
			continue;
		}
		validate(item);
	}
}

for (const item of items) {
	switch (item.type) {
		case 1:
			continue;
	}
	process(item);
}

for (const item of items) {
	for (const child of item.children) {
		check(child);
	}
}

outer: for (const item of items) {
	for (const child of item.children) {
		break outer;
	}
}

for (const item of items) {
	inner: for (const child of item.children) {
		break inner;
	}
}

label: {
	for (const item of items) {
		switch (item.type) {
			case "child":
				break label;
		}
	}
}

switch (value) {
	case 1:
		break;
}

switch (value) {
	case 1:
		for (const item of items) {
			break;
		}
}

switch (value) {
	case 1:
		switch (otherValue) {
			case 2:
				break;
		}
}

function processItem(item) {
	for (const child of item.children) {
		break;
	}
}

for (const item of items) {
	processItem(item);
}

for (const item of items) {
	const handle = () => {
		for (const child of item.children) {
			break;
		}
	};
	const object = {
		method() {
			while (true) {
				break;
			}
		},
	};
	class Foo {
		static {
			while (true) {
				break;
			}
		}
		method() {
			switch (item) {
				case 1:
					break;
			}
		}
	}
}
