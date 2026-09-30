// Legacy octal escapes in sloppy mode strings - should not be fixed
RegExp("\01");
new RegExp("\173");
RegExp("\01", "g");
new RegExp("\173", "g");
new RegExp("\8", "g");

// An escaped backslash followed by a digit is not a legacy octal escape - should be fixed
new RegExp("\\1(a)", "g");
