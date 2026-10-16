export const Letters = ["a", "b", "c"] as const;
export type Letters = (typeof Letters)[number];
