import {
    func, // trailing comma removal
} from 'module';


expression(/* block comment */);

expression(
    /* block comment */
);


expression( // line comment
);

expression(
    // line comment
);


expression( "something" // line comment
);


expression( "something" /** something **/  );

expression(/** something **/ "something"
          );

expression(
    /** something **/
    "something"
);

const array0 = [/*0*/];
const array1 = [/*0*/,/*1*/];
const array2 = [/*0*/,/*1*/,/*2*/];

/* block comment */
statement();

statement(); /* block comment */

// line comment
statement();

statement(); // inline

// leading
[1, 2, 3];

[1, 2, 3] // trailing

function name() /* comment */ {}

function name(very, long, list, of_parameters, to, insert, a_break, in_the, parameters, group) /* comment */ {}

4 + /* plus trailing */3 * 2 /* 2 trailing */ ;

/* leading of opening */ (/* trailing of opening */ (4 + 3)

	/* leading of closing */) /* trailing of closing */;

	[
		3 /* trailing num */ , /* trailing sep */
	];

function test /* spacing before parentheses*/ () {}

a /* before member */.test;
statement /* comment */;

/* leading */ "test" /*trailing*/ ;

/* leading */
"test" /*trailing*/ ;

/* EOF comment */
