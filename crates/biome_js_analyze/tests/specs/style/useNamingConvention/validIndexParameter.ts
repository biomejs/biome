/* should not generate diagnostics */
export interface X {
    [s: string]: unknown

    [index: number]: unknown

    [specialSymbol: symbol]: unknown
}

export interface Y {
    readonly [s: string]: unknown;

    readonly [index: number]: unknown;

    readonly [specialSymbol: symbol]: unknown;
}

// Mapped types with index parameters (should be valid in camelCase)
type MappedType1 = {
    [key in keyof string]: string
}

enum SomeList {
  ItemOne = 'itemOne',
  ItemTwo = 'itemTwo',
  ItemThree = 'itemThree',
}

type MappedType2 = { [key in SomeList]: string };

type MappedWithModifiers<T> = {
    readonly [prop in keyof T]: T[prop]
};

type MappedWithAs<T> = {
    [key in keyof T as `get${Capitalize<key>}`]: () => T[key]
};