function and() {
	return aaaaaaaaaaaaaaaaaaaa &&
		bbbbbbbbbbbbbbbbbbbbbbb &&
		(cccccccccccccccccccc && dddddddddddddddddddddd && eeeeeeeeeeeeeeeeeeeeeeeee);
}

function or() {
	return aaaaaaaaaaaaaaaaaaaa ||
		bbbbbbbbbbbbbbbbbbbbbbb ||
		(cccccccccccccccccccc || dddddddddddddddddddddd || eeeeeeeeeeeeeeeeeeeeeeeee);
}

function nullish() {
	return aaaaaaaaaaaaaaaaaaaa ??
		bbbbbbbbbbbbbbbbbbbbbbb ??
		(cccccccccccccccccccc ?? dddddddddddddddddddddd ?? eeeeeeeeeeeeeeeeeeeeeeeee);
}

const nested =
	aaaaaaaaaaaaaaaaaaaa &&
	(bbbbbbbbbbbbbbbbbbbbbbb &&
		(cccccccccccccccccccc && (dddddddddddddddddddddd && eeeeeeeeeeeeeeeeeeeeeeeee)));

const nestedLeft =
	aaaaaaaaaaaaaaaaaaaa &&
	((bbbbbbbbbbbbbbbbbbbbbbb && cccccccccccccccccccc) && (dddddddddddddddddddddd && eeeeeeeeeeeeeeeeeeeeeeeee));

const short = a && (b && c && d);
const shortOr = a || (b || c || d);

const mixedAndOr =
	aaaaaaaaaaaaaaaaaaaa &&
	bbbbbbbbbbbbbbbbbbbbbbb &&
	(cccccccccccccccccccc || dddddddddddddddddddddd || eeeeeeeeeeeeeeeeeeeeeeeee);
const mixedOrAnd = a || (b && c && d);
const mixedNullish = a ?? (b || c || d);
const mixedInner = a && (b || c) && (d && (e || f));

const exponent = a ** (b ** c ** d);
const arithmetic = a - (b - c - d) + (e * (f * g * h));

const commented = (a && /* inner */ (b && c && d)) || e;
