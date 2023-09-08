enum Status {
    Open = 0,
    Close = 1,
}

enum Flags {
    One = 1,
    Two = Flags.One << 1,
}

export {}
