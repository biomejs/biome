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