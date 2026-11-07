/* should generate diagnostics */
switch (a) {
	case 1:
		using x = foo();
		break;
	default:
		await using y = bar();
		break;
}
