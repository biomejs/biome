// Comment
if (!true) { // a
	consequent;
} else { // b
	alternate;
}

if (a != b) {
	consequent;
} else {
	alternate;
}

if (a !== b) {
	consequent;
} else {
	alternate;
}

!condition ? consequent : alternate;
a != b ? consequent : alternate;
a !== b ? consequent : alternate;
/*before-not*/!/*after-not*/spaced/*after-test*/ ? /*left-leading*/ left /*left-trailing*/   : /*right-leading*/ right /*right-trailing*/;

call(
	!test
		? consequent /*consequent-trailing*/
		: alternate
);

if (!true) {
	consequent;
} else something();

if (!true) something(); else {
	alternate
}

!-a ? b : c

/*1*/!/*2*/cond/*3*/ ? /*a*/ b /*c*/ : /*d*/ e /*f*/
