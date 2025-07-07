///! lint/suspicious/noDoubleEquals
///! lint/suspicious/noConsole
///! lint/style/useArrayLiterals

/* biome-ignore lint: ... */
console.log(1 == 0)

/* biome-ignore lint/suspicious/noConsole: ... */
console.log(1 == 0)

/* biome-ignore lint/suspicious/noConsole: ... */
/* biome-ignore lint/suspicious/noDoubleEquals: ... */
console.log(1 == 0)

/* biome-ignore lint/suspicious/noConsole: ... */
/* biome-ignore lint/suspicious/noDoubleEquals: ... */
console.log(1 == Array())

/* biome-ignore lint/suspicious/noConsole: ... */ /* biome-ignore lint/suspicious/noDoubleEquals: ... */
console.log(1 == 0)
/* biome-ignore lint/suspicious/noConsole: ... */ /* biome-ignore lint/suspicious/noDoubleEquals: ... */
console.log(1 == Array())

/* biome-ignore lint/suspicious/noConsole: ... */ /* biome-ignore lint/suspicious/noDoubleEquals: ... */ console.log(1 == 0)

/* biome-ignore lint/suspicious/noConsole: ... */ /* biome-ignore lint/suspicious/noDoubleEquals: ... */ console.log(1 == Array())

if (1) {
    /* biome-ignore lint/suspicious/noConsole: ... */
    console.log(1 == 0)
    /* biome-ignore lint/suspicious/noDoubleEquals: ... */
}

if (1) {
    /* biome-ignore lint/suspicious/noConsole: ... */
    console.log(1 == Array())
    /* biome-ignore lint/suspicious/noDoubleEquals: ... */
}
