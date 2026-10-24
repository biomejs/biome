/* should generate diagnostics */
while (true) {
	if (!value!) {
		break;
	}

	process(value);
}

while (true) {
	if (value!) {
		break;
	}

	process(value);
}

while (true) {
	if (value as boolean) {
		break;
	}

	process(value);
}

while (true) {
	if (!(value as boolean)) {
		break;
	}

	process(value);
}
