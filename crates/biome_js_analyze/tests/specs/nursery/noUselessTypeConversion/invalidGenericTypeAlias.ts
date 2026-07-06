// `Identity<string>` and `Identity<number>` substitute to `string` and `number`, so each conversion is redundant.
type Identity<T> = T;
declare const text: Identity<string>;
String(text);
text.toString();
"" + text;
declare const count: Identity<number>;
+count;
