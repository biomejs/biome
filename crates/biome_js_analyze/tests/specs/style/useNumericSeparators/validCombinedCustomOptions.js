/* should not generate diagnostics */

let foo;

// Decimal: minimumDigits: 7 and groupLength: 2, so 6 digit numbers are fine
foo = 123456;

// Decimal with 7+ digits, properly grouped
foo = 1_23_45_67;
