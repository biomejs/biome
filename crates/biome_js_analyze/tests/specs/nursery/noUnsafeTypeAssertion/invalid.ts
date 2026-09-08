/* should generate diagnostics */
interface SomeType {
    value: string;
}

declare let myVar;

const variable = myVar as SomeType;
const literal = "value" as SomeType;
const angleVariable = <SomeType>myVar;
const angleLiteral = <SomeType>"value";

(myVar as SomeType).value = "foo";
(myVar as SomeType) = { value: "value" };
(<SomeType>myVar) = { value: "value" };

((myVar as unknown) as SomeType) = { value: "value" };
([myVar as SomeType] = [{ value: "value" }]);

const multilineAs = myVar as
    SomeType;
const multilineAngle = <
    SomeType
>myVar;
