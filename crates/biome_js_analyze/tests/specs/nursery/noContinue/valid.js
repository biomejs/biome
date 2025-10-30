/* should not generate diagnostics */
var sum = 0, i; for (i = 0; i < 10; i++) { if (i > 5) { sum += i; } }
var sum = 0, i = 0; while (i < 10) { if (i > 5) { sum += i; } i++; }
