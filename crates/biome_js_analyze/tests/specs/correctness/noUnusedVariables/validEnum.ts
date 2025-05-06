/* should not generate diagnostics */
export enum Status {
    Open = 0,
    Close = 1,
}

enum Flags {
    One = 1,
    Two = f(Flags.One),
}
