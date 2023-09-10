type A = { [test in "a" | "b"] }
type OptionsFlags<Type> = {
  [Property in keyof Type]: boolean;
};
type CreateMutable<Type> = {
	-readonly [Property in keyof Type]: Type[Property];
};
type Concrete<Type> = {
  [Property in keyof Type]-?: Type[Property]
};
type Getters<Type> = {
    [Property in keyof Type as `get${Capitalize<string & Property>}`]: () => Type[Property]
};
