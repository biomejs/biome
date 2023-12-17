switch (key) {
	case // comment
	value:

	case value: // fallthrough same-line
	case value:
		// fallthrough

	case fallthrough:
	case value:
		break;

	default:
		break;


}

switch ("test") {
  case "test": {}
}

switch (key) {
	case blockBody: {
		const a = 1;
		break;
	}

	// The block is not the only statement in the case body,
	// so it doesn't hug the same line as the case here.
	case separateBlockBody: {
		const a = 1;
	}
	break;
}


switch (key) {
	default: {
		const a = 1;
		break;
	}
}

switch (key) {
	// The block is not the only statement in the case body,
	// so it doesn't hug the same line as the case here.
	default: {
		const a = 1;
	}
	break;
}