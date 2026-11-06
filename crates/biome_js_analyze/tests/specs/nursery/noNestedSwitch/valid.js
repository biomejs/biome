/* should not generate diagnostics */
switch (x) {
	case 1:
		a;
		break;
	default:
		b;
}

switch (y) {
	case 1:
		handle(y);
		break;
}

function handle(m) {
	switch (m) {
		case 2:
			break;
	}
}

if (x) {
	switch (x) {}
} else {
	switch (y) {}
}
