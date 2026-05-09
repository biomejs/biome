/* should not generate diagnostics */

String(1);
`${true}`;

class CustomToString {
    toString() {
        return "ok";
    }
}

class CustomLocaleString {
    toLocaleString() {
        return "ok";
    }
}

declare const custom: CustomToString;
declare const localised: CustomLocaleString;
declare const error: Error;
declare const regex: RegExp;
declare const unknownValue: unknown;
declare const values: string[];

custom.toString();
`${custom}`;
String(custom);
localised.toLocaleString();

values.join(",");
String(values);

String(error);
`${regex}`;
String(unknownValue);
