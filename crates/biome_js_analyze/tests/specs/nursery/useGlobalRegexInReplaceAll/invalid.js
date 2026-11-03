/* should generate diagnostics */
withSpaces.replaceAll(/\s+/, ",");
withSpaces.replaceAll(/\s+/i, ",");
withSpaces.replaceAll(new RegExp("\\s+"), ",");
withSpaces.replaceAll(new RegExp("\\s+", "i"), ",");
withSpaces.replaceAll(RegExp("\\s+", "i"), ",");
withSpaces.replaceAll(new RegExp(`\\s+`), ",");
withSpaces.replaceAll(new RegExp(`${prefix}\\s+`), ",");
withSpaces.replaceAll(new RegExp(/\s+/i), ",");
withSpaces.replaceAll(new RegExp(), ",");
withSpaces.replaceAll(new RegExp, ",");
withSpaces.replaceAll(new globalThis.RegExp("\\s+"), ",");
withSpaces?.replaceAll(/\s+/, ",");
withSpaces["replaceAll"](/\s+/, ",");
(withSpaces.replaceAll)((/\s+/), ",");

const literal = /\s+/;
withSpaces.replaceAll(literal, ",");

const constructed = new RegExp("\\s+", "i");
withSpaces.replaceAll(constructed, ",");

let neverReassigned = /\s+/;
withSpaces.replaceAll(neverReassigned, ",");
