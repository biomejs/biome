export interface X {
    [s: string]: unknown

    [index: number]: unknown

    [specialSymbol: symbol]: unknown
}