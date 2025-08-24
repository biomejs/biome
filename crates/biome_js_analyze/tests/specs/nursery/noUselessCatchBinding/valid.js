// should not generate diagnostics

try {
    // Do something
} catch (used) {
    console.error(used);
}

try {
    // Do something
} catch {
    // Do something
}
