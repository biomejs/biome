switch (foo) {
	case 0:
	case 1:
		break;
}

switch (foo) {
	case 0:
		break;
	default:
		break;
}

switch (foo) {
	case 1:
		f();
		// Fallthrough
	default:
		g();
		break;
}

switch (foo) {
	case 1:
		f();
		// Fallthrough
	default:
		g();
		// Fallthrough
    case 2:
        break;
}