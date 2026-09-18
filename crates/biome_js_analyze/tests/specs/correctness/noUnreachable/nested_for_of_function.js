function repro() {
    for (const a of aa) {}
    for (const b of bb) {}

    for (const c of cc) {
        const nested = () => {
            for (const d of dd) {
                if (d) continue;
            }
        };
    }
}
