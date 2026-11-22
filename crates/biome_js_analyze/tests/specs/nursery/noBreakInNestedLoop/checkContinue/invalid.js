/* should generate diagnostics */
for (const item of items) {
	while (item.children.pop()) {
		continue;
	}
}

for (const item of items) {
	switch (item.type) {
		case "child":
			continue;
	}
}

for (const item of items) {
	for (const child of item.children) {
		switch (child.type) {
			case "child":
				continue;
		}
	}
}

for (const item of items) {
	switch (item.type) {
		case "child":
			while (item.pending) {
				continue;
			}
	}
}

for (const key in items) {
	do {
		continue;
	} while (items[key].pending);
}

// `break` statements are still reported.
for (const item of items) {
	for (const child of item.children) {
		break;
	}
}

for (const group of groups) {
	const processGroup = () => {
		for (const item of group.items) {
			switch (item.type) {
				case "hidden":
					continue;
			}
		}
	};
	processGroup();
}
