// See https://github.com/biomejs/biome/issues/953
type X = never;
type Y = never;
export const MyMappingPbToGql: {
    [key in X]: never;
} = {};
export const MyOtherMappingPbToGql: {
    [key in Y]: never;
} = {};
