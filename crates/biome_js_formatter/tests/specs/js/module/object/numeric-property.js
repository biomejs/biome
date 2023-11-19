const x = {
    // Format numbers
    1e+2: null,
    1E+2: null,
    0.000: null,
    0b01: null,
    0B01: null,
    0o7: null,
    0O7: null,
    0xf: null,
    0Xf: null,
    1n: null,
    0xan: null,

    // Remove quote for simple and exact floats
    "1.5": null,
    "9007199254740991": null,

    // Keep quotes for inexact floats
    "999999999999999999999": null,

    // Keep quoted for negative numbers
    "-1": null,
    "-1.5": null,

    // Keep quotes because the yare considered complex
    "1e+2": null,
    "1E+2": null,
    "0.000": null,
    "0b01": null,
    "0B01": null,
    "0o7": null,
    "0O7": null,
    "0xf": null,
    "0Xf": null,
    "1n": null,
    "0xan": null,
}