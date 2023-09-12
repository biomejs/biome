switch (foo) {
	case 0:
	default:
		break;
	case 3:
		break;
}

switch (foo) {
	/* before */case 0:/* after */
	// comment for default
	default:
	case 1:
	case 2:/* statements */
		break;
	case 3:
		break;
}