export interface X {
    get p(): unknown

    get property(): unknown

    get specialProperty(): unknown

    get stream$(): unknown

    get $stream(): unknown

    get _special_(): unknown

    get "custom-property"(): unknown

    get "@"(): unknown

    get READONLY_PROPERTY(): unknown
}