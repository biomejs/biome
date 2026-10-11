/* should generate diagnostics */
if (a) {
    if (b) {}
}

if (a) {
} else {
    if (b) {}
}

if (a) {
} else if (b) {
    if (c) {}
}

if (a) {
    switch (b) {}
}

if (a) {
    try {} catch {}
}

if (a) {
    try {} finally {}
}

if (a) {
    do {} while (b);
}

if (a) {
    while (b) {}
}

if (a) {
    for (;;) {}
}

if (a) {
    for (const b in c) {}
}

if (a) {
    for (const b of c) {}
}

async function forAwait() {
    if (a) {
        for await (const b of c) {}
    }
}

for (;;) {
    label: if (a) {}
}

if (a) if (b) {}
