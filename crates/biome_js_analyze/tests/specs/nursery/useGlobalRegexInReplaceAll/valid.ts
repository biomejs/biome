/* should not generate diagnostics */
withSpaces.replaceAll(/\s+/g as RegExp, ",");

const typed: RegExp = /\s+/g;
withSpaces.replaceAll(typed, ",");

withSpaces.replaceAll(new RegExp("\\s+", "g" as const), ",");
