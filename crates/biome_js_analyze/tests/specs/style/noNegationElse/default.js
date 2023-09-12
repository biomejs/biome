// valid
if (!true) {
	consequent;
}
true ? consequent : alternate;
// invalid
if (!true) {
	consequent;
} else {
	alternate;
}
!condition ? consequent : alternate;

let a = !test ? c : d;