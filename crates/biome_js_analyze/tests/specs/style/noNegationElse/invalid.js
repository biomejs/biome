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

let a = !test ? c : d;

if (!true) {
	consequent;
} else something();

if (!true) something(); else {
	alternate
}

!-a ? b : c

/*1*/!/*2*/cond/*3*/ ? /*a*/ b /*c*/ : /*d*/ e /*f*/
