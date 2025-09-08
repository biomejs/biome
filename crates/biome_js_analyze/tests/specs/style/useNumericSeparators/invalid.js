// Tests adapted from https://github.com/sindresorhus/eslint-plugin-unicorn/blob/756dbbf0a359a139745b092a676f6e535cb85128/test/numeric-separators-style.js.

let foo;

// Hexadecimal
foo = 0xA_B_CDE_F0; // 0xA_BC_DE_F0
foo = 0xABCDEF; // 0xAB_CD_EF
foo = 0xA_B; // 0xAB
foo = 0XAB_C_D; // 0XAB_CD

// Octal
foo = 0o12_34_5670; // 0o1234_5670
foo = 0o7_7_77; // 0o7777
foo = 0o010101010101; // 0o0101_0101_0101
foo = 0O010101010101; // 0O0101_0101_0101

// Binary
foo = 0b10_10_0001; // 0b1010_0001
foo = 0b0_00_0; // 0b0000
foo = 0b10101010101010; // 0b10_1010_1010_1010
foo = 0B10101010101010; // 0B10_1010_1010_1010

// BigInt
foo = 1_9_223n; // 19_223n
foo = 80_7n; // 807n
foo = 123456789_100n; // 123_456_789_100n

// Numbers
foo = 1_2_345_678; // 12_345_678
foo = 12_3; // 123
foo = 1234567890; // 1_234_567_890

// Decimal numbers
foo = 9807.1234567; // 9807.123_456_7
foo = 3819.123_4325; // 3819.123_432_5
foo = 138789.12343_2_42; // 138_789.123_432_42
foo = .000000_1; // .000_000_1
foo = 12345678..toString(); // 12_345_678..toString()
foo = 12345678 .toString(); // 12_345_678 .toString()
foo = .00000; // .000_00
foo = 0.00000; // 0.000_00

// Negative numbers
foo = -100000_1; // -1_000_001

// Exponential notation
foo = 1e10000; // 1e10_000
foo = 39804e10000; // 39_804e10_000
foo = -123456e100; // -123_456e100
foo = -100000e-10000; // -100_000e-10_000
foo = -1000e+10000; // -1000e+10_000
foo = -1000e+00010000; // -1000e+00_010_000
foo = 3.6e12000; // 3.6e12_000
foo = -1200000e5; // -1_200_000e5
foo = 3.65432E12000; // 3.654_32E12_000
