/* should generate diagnostics */
withSpaces.replaceAll(/\s+/ as RegExp, ",");

const typed: RegExp = /\s+/;
withSpaces.replaceAll(typed, ",");

const asserted = /\s+/i satisfies RegExp;
withSpaces.replaceAll(asserted!, ",");

withSpaces.replaceAll(new RegExp("\\s+", "i" as const), ",");
withSpaces.replaceAll(new RegExp("\\s+", ("i")), ",");
