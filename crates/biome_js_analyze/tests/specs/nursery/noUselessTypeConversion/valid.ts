/* should not generate diagnostics */
String(1);
(1).toString();
`${1}`;
"" + 1;
1 + "";

let str = 1;
str += "";

let otherAssignment = "asdf";
otherAssignment = "";

let stringAppend = "asdf";
stringAppend += "suffix";

Number("2");
+"2";
~~"2";
~~1.1;
~~-1.1;
~~(1.5 + 2.3);
~~(1 / 3);
Boolean(0);
!!0;
BigInt(3);

new String("asdf");
new Number(2);
new Boolean(true);
!false;
~2;

function String(value: unknown) {
    return value;
}
String("asdf");

function Number(value: unknown) {
    return value;
}
Number(2);

function Boolean(value: unknown) {
    return value;
}
Boolean(true);

function BigInt(value: unknown) {
    return value;
}
BigInt(3n);

function toString(value: unknown) {
    return value;
}
toString("asdf");

declare const toStringValue: string;
toStringValue.toUpperCase();

String(new String());
new String().toString();
"" + new String();
new String() + "";

let boxed = new String();
boxed += "";

Number(new Number());
+new Number();
~~new Number();
Boolean(new Boolean());
!!new Boolean();

enum CustomIds {
    Id1 = "id1",
    Id2 = "id2",
}

const customId = "id1";
const compareWithToString = customId === CustomIds.Id1.toString();
