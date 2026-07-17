/* should not generate diagnostics */

function arrays(items: string[], maybeItems: string[] | undefined) {
  items[0] === "a";
  maybeItems?.[0] === "a";
  items.charAt(0) === "a";
  items.indexOf("a") === 0;
  items.lastIndexOf("a") === items.length - 1;
  items.slice(0, 1) === "a";
}

function mixed(text: string | string[], value: string) {
  text[0] === value;
  text.indexOf(value) === 0;
}

function unknowns(text: any, value: string) {
  text[0] === value;
  text.indexOf(value) === 0;
}

function generic<T>(text: T, value: string) {
  text[0] === value;
}

function stringCases(text: string, needle: string, pattern: RegExp) {
  text[1] === "a";
  text.charAt(1) === "a";
  text.indexOf(needle, 1) === 0;
  text.lastIndexOf(needle, text.length) === text.length - needle.length;
  text.match(/foo/) !== null;
  text.match(/^foo$/) !== null;
  pattern.test(text);
  text.slice(1, 4) === "bar";
  text.slice(-needle.length, text.length - 1) === needle;
  text.slice(text.length - needle.length, text.length - 1) === needle;
  text.substring(-3) === "bar";
  text.startsWith(needle);
  text.endsWith(needle);
}
