// should generate diagnostics
function local() {}
const expression = function() {};
const arrow = () => {};
export function exported() {}
export const exportedExpression = function() {};
export const exportedArrow = () => {};
export default function() {}
export { local, expression, arrow };

