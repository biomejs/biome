switch (foo) {
	default:
		break;
	case 0:
		break;
}

switch (foo) {
	case 0:
		break;
	default:
		break;
	case 1:
		break;
}

switch (foo) {
	default:
		f();
	case 0:
		break;
}

switch (foo) {
	case 0:
		break;
	default:
	case 1:
		break;
}