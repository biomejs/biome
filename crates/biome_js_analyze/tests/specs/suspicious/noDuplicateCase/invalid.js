var a = 1;
switch (a) {
	case 1:
		break;
	case 1:
		break;
	case 2:
		break;
	default:
		break;
}
var a = "1";
switch (a) {
	case "1":
		break;
	case "1":
		break;
	case "2":
		break;
	default:
		break;
}
var a = 1,
	one = 1;
switch (a) {
	case one:
		break;
	case one:
		break;
	case 2:
		break;
	default:
		break;
}
var a = 1,
	p = { p: { p1: 1, p2: 1 } };
switch (a) {
	case p.p.p1:
		break;
	case p.p.p1:
		break;
	default:
		break;
}
var a = 1,
	f = function (b) {
		return b ? { p1: 1 } : { p1: 2 };
	};
switch (a) {
	case f(true).p1:
		break;
	case f(true).p1:
		break;
	default:
		break;
}
var a = 1,
	f = function (s) {
		return { p1: s };
	};
switch (a) {
	case f(a + 1).p1:
		break;
	case f(a + 1).p1:
		break;
	default:
		break;
}
var a = 1,
	f = function (s) {
		return { p1: s };
	};
switch (a) {
	case f(a === 1 ? 2 : 3).p1:
		break;
	case f(a === 1 ? 2 : 3).p1:
		break;
	default:
		break;
}
var a = 1,
	f1 = function () {
		return { p1: 1 };
	};
switch (a) {
	case f1().p1:
		break;
	case f1().p1:
		break;
	default:
		break;
}
var a = [1, 2];
switch (a.toString()) {
	case [1, 2].toString():
		break;
	case [1, 2].toString():
		break;
	default:
		break;
}
switch (a) {
	case a:
	case a:
}
switch (a) {
	case a:
		break;
	case b:
		break;
	case a:
		break;
	case c:
		break;
	case a:
		break;
}
var a = 1,
	p = { p: { p1: 1, p2: 1 } };
switch (a) {
	case p.p.p1:
		break;
	case p.p.p1: // comment\n
		break;
	default:
		break;
}
var a = 1,
	p = { p: { p1: 1, p2: 1 } };
switch (a) {
	case /* comment */
	p.p.p1:
		break;
	case p.p.p1:
		break;
	default:
		break;
}
var a = 1,
	p = { p: { p1: 1, p2: 1 } };
switch (a) {
	case p.p /* comment */.p1:
		break;
	case p.p.p1: // comment
		break;
	default:
		break;
}
var a = 1,
	p = { p: { p1: 1, p2: 1 } };
switch (a) {
	case p.p.p1:
		break;
	case p.p.p1: // comment
		break;
	case /* comment */
	p.p.p1:
		break;
	default:
		break;
}
var a = 1,
	f = function (s) {
		return { p1: s };
	};
switch (a) {
	case f(a + 1).p1:
		break;
	case f(a + 1).p1:
		break;
	default:
		break;
}
var a = 1,
	f = function (s) {
		return { p1: s };
	};
switch (a) {
	case f(
		a + 1 // comment
	).p1:
		break;
	case f(a + 1).p1:
		break;
	default:
		break;
}
