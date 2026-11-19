// should not generate diagnostics
export function declared() {}
export const arrow = () => {};
export const typed: () => void = function() {};
function local() {}

