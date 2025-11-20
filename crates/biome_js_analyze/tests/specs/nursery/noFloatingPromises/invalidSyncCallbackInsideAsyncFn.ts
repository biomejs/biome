async function doStuff(db) {
    const txStatements: Array<(tx) => Promise<any>> = [];

    db.transaction((tx: any) => {
        for (const stmt of txStatements) {
            stmt(tx)
        }
    });
}

async function doStuff2(db) {
    const txStatements: Array<Promise<(tx: any) => void>> = [];

    db.transaction((tx: any) => {
        for (const stmt of txStatements) {
            stmt
        }
    });
}
