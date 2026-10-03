/* should not generate diagnostics */
export let exported = "used";
console.log(exported);
exported = "unused like but exported";

export class ExportedClass {}
console.log(ExportedClass);
ExportedClass = "unused like but exported";

let exportedLater = "used";
export { exportedLater };
console.log(exportedLater);
exportedLater = "unused like but exported";
