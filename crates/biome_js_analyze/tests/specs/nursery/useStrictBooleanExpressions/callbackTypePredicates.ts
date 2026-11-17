// should generate diagnostics

declare function isString(input: unknown): input is string;
declare const values: (string | number)[];

if (values.find(isString)) {
}
