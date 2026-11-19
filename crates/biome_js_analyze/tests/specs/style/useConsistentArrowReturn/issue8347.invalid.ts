// should generate diagnostics
const kek = (field: "first" | "second") => {
    return {
        first: 1,
        second: 2,
    }[field];
};
