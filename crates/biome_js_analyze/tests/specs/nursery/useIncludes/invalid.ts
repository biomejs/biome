type Factory<T> = () => [T];
declare const makeStrings: Factory<string>;

makeStrings().indexOf("value") !== -1;
