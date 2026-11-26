/* should generate diagnostics */
arguments.map(() => {});
const arrow = () => arguments.map(() => {});
function outer(value = { [arguments.at(0)]() {} }) {}
