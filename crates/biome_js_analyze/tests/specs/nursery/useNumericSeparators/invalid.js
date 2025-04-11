/* No separators at all. */

// Integers
var a = 10000;
a = 100000;
a = 1000000;
a = 10000000;
a = 1234567890;

// Floats
a = 3.1415926;
a = 99999.99;

// Signed

a = -10000;
a = -3.1415926;
a = +10000;
a = +3.1415926;

/* Inconsistent digit grouping with separators. */

// Integers
a = 100_00;
a = 10_0000;

// Floats
a = 1.2_3_4567_89

/* Octal/binary/hexadecimal */

a = 0b101010;
a = 0b1010_10001;
a = 0xA_B_CD_EF;
a = 0o77_77;
