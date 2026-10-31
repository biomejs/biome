function sequentialSwitches(x) {
	switch (x) {
		case 0:
			a();
		default:
			b();
	}
	switch (x) {
		case 0:
			a();
			break;
		default:
			b();
	}
	switch (x) {
		default:
			b();
		case 0:
			a();
	}
}

function loops(x) {
	outer: for (const item of x) {
		switch (item) {
			case 0:
				continue outer;
			case 1:
				if (item) {
					continue;
				}
			case 2:
				for (;;) {
					break outer;
				}
			case 3:
				while (item) {
					continue outer;
				}
			case 4:
				a();
			default:
				b();
		}
	}
	do {
		switch (x) {
			case 0:
				continue;
			case 1:
				a();
			default:
				break;
		}
	} while (x);
}

function exceptions(x) {
	try {
		switch (x) {
			case 0:
				throw a();
			case 1:
				a();
			default:
				b();
		}
	} catch {
		c();
	}
	try {
		switch (x) {
			case 0:
				a();
			case 1:
				return b();
			default:
				c();
		}
	} finally {
		d();
	}
	switch (x) {
		case 0:
			try {
				a();
			} catch {
				break;
			}
		case 1:
			try {
				return a();
			} finally {
				b();
			}
		default:
			c();
	}
}

function nestedSwitches(x, y) {
	switch (x) {
		case 0:
			switch (y) {
				case 0:
					a();
				default:
					break;
			}
		case 1:
			switch (y) {
				case 0:
					break;
				default:
					return;
			}
		default:
			b();
	}
	label: {
		switch (x) {
			case 0:
				break label;
			case 1:
				a();
			default:
				b();
		}
	}
}
