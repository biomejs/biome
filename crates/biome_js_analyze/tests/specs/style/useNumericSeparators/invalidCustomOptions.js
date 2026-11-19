/* should generate diagnostics */

let foo;

// Decimal: minimumDigits: 7, so 7+ digits without separators are invalid
foo = 1234567; // 1_234_567
foo = 1234567890; // 1_234_567_890

// Hexadecimal: minimumDigits: 4, so 4+ digits without separators are invalid
foo = 0xABCD; // 0xAB_CD
foo = 0xABCDEF; // 0xAB_CD_EF

// Binary: groupLength: 2, wrong grouping
foo = 0b11001100; // 0b11_00_11_00
foo = 0b1010_0001; // 0b10_10_00_01

// Octal: groupLength: 2, wrong grouping
foo = 0o12345670; // 0o12_34_56_70
foo = 0o1234_5670; // 0o12_34_56_70
