import * as assert from "assert";

type W = { a: string; b: symbol; c: symbol;d: symbol;e: symbol;f: symbol;g: symbol; };
type X = { a: string; b: symbol; }
type Z = {
    a: string
    b: symbol
}
type A = {
    a: string
}

type OptionsFlags
    <Type> =
    {
  +
        readonly [Property
        in
        keyof
            Type
        as              string]
        -?: boolean;
};


type TupleA
    = [     string      ]

type TupleB = [   ...string[  ]     ]

type TupleC = [ surname  ?:
    string[],
    ...name: string[],  ]

type TupleD = [
    address: string,
    address2: string,
    address3: string,
    address4: string,
    address5: string,
    surname  ?:
    string[],
    ...name: string[],  ]

type PA = (
    string
    )


type FunctionType = <Aaaaaaaaaaaaaaaaaaaaa,bvvvvvvvvvvvvvvvvvvvvvv,ceeeeeee,deeeeeeeeeeeeee,deeeeeeeeeeeeeee,deeeeeeeeeeeeeeee,deeeeeeeewweeeeee,>(Lorem: string, ipsum: symbol,  dolor: number, sit: boolean, amet: string, consectetur: symbol) => {
    Lorem: string, ipsum: symbol, dolor: number, sit: boolean, amet: string, consectetur: symbol
}

type ShortFunctionType = <A,B,C,D,E,>(Lorem: string, ipsum: symbol,  dolor: number, sit: boolean, amet: string, consectetur: symbol) => {
    a: string;
}

function test(a: string):
    a is  string   { return true }


type AbstractCompositeThingamabobberFactoryProvider = string;

type ConstructorType = new ( options: { a: string, b: AbstractCompositeThingamabobberFactoryProvider },
) => {};

type GenericTypeExpression<A extends { a: string }> = Foo<A, {  bar: string }>;
type GenericTypeExpression<A extends { a: string }> = AbstractCompositeThingamabobberFactoryProvider<{  bar: string, foo: symbol; baz: number; zzz: boolean }>;