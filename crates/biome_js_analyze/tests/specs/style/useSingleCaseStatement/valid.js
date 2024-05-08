switch (foo) {
    case true:
    case false:
        'yes';
}

switch (foo) {
    case true: {
        // empty
    }
}

switch (foo) {
    case true:
}

switch (foo) {
    case true:
    default:
        'yes';
}

switch (foo) {
    default: {
        // empty
    }
}

switch (foo) {
    default:
}

switch (foo) {
	case true:
		x = 1;
		break;
}

switch (foo) {
	case 1:
		x = 2;
		break;
	case 2:
		x = 1;
		break;
}

switch (foo) {
	case true:
		// comment
		x = 1;
		break;
}

switch (foo) {
	case 1:
	case 2:
		x = 1;
		break;
	default:
		break;
}
