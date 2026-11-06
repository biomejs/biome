/* should not generate diagnostics */
if (a) {
} else if (b) {
} else {
}

for (const b of c) {
    {}
}

if (a) {
    function inner() {
        if (b) {}
    }
}

while (a) {
    const inner = () => {
        while (b) {}
    };
}
