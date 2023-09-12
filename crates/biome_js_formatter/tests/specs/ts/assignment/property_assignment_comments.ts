class Test {
	prop1 = /* leading */ 1;

	prop2 = // test
		2;

	prop3 // test
		= 3;

	prop4 // test
		= 4;

	prop5 // test
		= 5 // a

	prop6
		/* leading */
		= 6 // c

	prop7 =
		/* leading */
		7 // c

	prop8 /* comment */ = 8;

	prop9: string // 1
		= // 2
	3;

	prop10: any // 1
	= // 2
		{ object: 3}
}
