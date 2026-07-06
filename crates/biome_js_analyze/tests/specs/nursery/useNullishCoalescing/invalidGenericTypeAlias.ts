// `Maybe<string>` substitutes to `string | null`, so `||` should be `??`.
type Maybe<T> = T | null;
declare const value: Maybe<string>;
const result = value || 'default';
