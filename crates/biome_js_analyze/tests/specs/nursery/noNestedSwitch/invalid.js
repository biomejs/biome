/* should generate diagnostics */
switch (x) {
	case 1:
		a;
		break;
	case 2:
		switch (y) {
			case 3:
				c;
				break;
			default:
				d;
		}
		break;
	default:
		b;
}

switch (x) {
	case 1: {
		switch (y) {
			case 3:
				c;
		}
		switch (z) {
			case 3:
				c;
		}
		break;
	}
}

switch (x) {
	case 1:
		switch (y) {
			case 3:
				c;
			default:
				switch (z) {
					case 4:
						d;
				}
		}
		break;
}

switch (x) {
	default:
		switch (y) {
			case 3:
				c;
		}
}

switch (x) {
	case 1: {
		const insideFunction = () => {
			switch (y) {}
		};
		break;
	}
}

switch (x) {
	case 1: {
		class Foo {
			method() {
				switch (y) {}
			}
		}
		break;
	}
}

switch (
	(() => {
		switch (y) {}
	})()
) {
}

switch (x) {
	case (() => {
		switch (y) {}
	})():
		break;
}
