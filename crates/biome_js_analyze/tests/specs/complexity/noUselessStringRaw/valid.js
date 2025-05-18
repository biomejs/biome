/* should not generate diagnostics */
String.raw`\n a`;
String.raw`\n abc`;
// FIXME: This should not generate a diagnostic
// String.raw`a ${x}`;