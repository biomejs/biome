export interface X {
    [PascalCase: string]: unknown

    [CONSTANT_CASE: number]: unknown

    [snake_case: symbol]: unknown
}