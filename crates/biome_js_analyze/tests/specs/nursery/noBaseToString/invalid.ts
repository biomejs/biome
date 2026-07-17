/* should generate diagnostics */

String({});
({}).toString();
({}).toLocaleString();
`${{}}`;

declare const objectOrString: {} | string;
String(objectOrString);

class PlainObject {
    value: string;
}

declare const plainInstance: PlainObject;
plainInstance.toString();
"prefix: " + plainInstance;

declare const joined: [{}];
joined.join(",");

declare const stringOrPlainArray: (string | PlainObject)[];
stringOrPlainArray.join(",");

declare const pair: [string, PlainObject];
`${pair}`;

declare let sink: string;
sink += plainInstance;
