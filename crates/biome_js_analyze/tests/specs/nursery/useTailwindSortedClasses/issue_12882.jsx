// Unknown dashed utilities with an arbitrary value or a CSS variable sort
// first as unknown classes instead of causing a parse error.
export const A = () => <div className="p-4 foo-bar-[1px]" />;
export const B = () => <div className="flex slide-out-to-top-[48%]" />;
export const C = () => <div className="flex foo-[48%]" />;
export const D = () => <div className="flex foo-bar-(--my-var)" />;
