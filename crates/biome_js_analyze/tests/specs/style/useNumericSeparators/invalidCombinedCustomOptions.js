/* should generate diagnostics */

let foo;

// Decimal: minimumDigits: 7 and groupLength: 2, so 7+ digits must use groups of 2
foo = 1234567; // 1_23_45_67
foo = 1_234_567; // 1_23_45_67
