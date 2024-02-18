// https://github.com/biomejs/biome/issues/1827
try {
    undefined;
    try {
        while (true) {
            if (Date.now() > 0) {
                undefined;
                break;
            }
        }
    }
    finally {
        if (Date.now() > 0) undefined;

    }
    if (Date.now() > 0) undefined;
} catch {
    undefined;
}