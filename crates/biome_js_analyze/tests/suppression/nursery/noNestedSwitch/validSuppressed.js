/* should not generate diagnostics */
switch (x) {
	case 1:
		// biome-ignore lint/nursery/noNestedSwitch: test
		switch (y) {
			case 2:
				break;
		}
		break;
}
