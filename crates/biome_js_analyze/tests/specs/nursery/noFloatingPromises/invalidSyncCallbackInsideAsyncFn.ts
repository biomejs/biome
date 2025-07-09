async function doStuff(db) {
    const txStatements: Array<(tx) => Promise<any>> = [];

    db.transaction((tx: any) => {
        for (const stmt of txStatements) {
            stmt(tx)
        }
    });
}
