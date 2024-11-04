export interface X {
    [PascalCase: string]: unknown

    [CONSTANT_CASE: number]: unknown

    [snake_case: symbol]: unknown
}

export interface Y {
    readonly [PascalCase: string]: unknown

    readonly [CONSTANT_CASE: number]: unknown

    readonly [snake_case: symbol]: unknown
}