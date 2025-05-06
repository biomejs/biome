/* should not generate diagnostics */
// Tests adapted from https://github.com/sindresorhus/eslint-plugin-unicorn/blob/756dbbf0a359a139745b092a676f6e535cb85128/test/numeric-separators-style.js.

let foo;

// Hexadecimal
foo = 0xAB_CD;
foo = 0xAB;
foo = 0xA;
foo = 0xA_BC_DE_F0;
foo = 0xab_e8_12;
foo = 0xe;
foo = 0Xab_e3_cd;

// Octal
foo = 0o1234_5670;
foo = 0o7777;
foo = 0o01;
foo = 0o12_7000_0000;
foo = 0O1111_1111;

// Binary
foo = 0b1010_0001_1000_0101;
foo = 0b0000;
foo = 0b10;
foo = 0b1_0111_0101_0101;
foo = 0B1010;

// Binary with BigInt
foo = 0b1010n;
foo = 0b1010_1010n;

// BigInt
foo = 9_223_372_036_854_775_807n;
foo = 807n;
foo = 1n;
foo = 9_372_854_807n;
foo = 9807n;
foo = 0n;

// Numbers
foo = 12_345_678;
foo = 123;
foo = 0;
foo = 1;
foo = 1234;

// Decimal numbers
foo = 9807.123;
foo = 3819.123_432;
foo = 138_789.123_432_42;
foo = .000_000_1;

// Negative numbers
foo = -3000;
foo = -10_000_000;

// Exponential notation
foo = 1e10_000;
foo = 39_804e1000;
foo = -123_456e-100;
foo = -100_000e-100_000;
foo = -100_000e+100_000;
foo = 3.6e12_000;
foo = 3.6E12_000;
foo = -1_200_000e5;

// Miscellaneous values
foo = -282_932 - (1938 / 10_000) * .1 + 18.100_000_2;
foo = NaN;
foo = Infinity;
foo = "1234567n";
