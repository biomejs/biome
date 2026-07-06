// `Id<{ a: number }>` substitutes to the object type, which has no custom stringification.
type Id<T> = T;
declare const obj: Id<{ a: number }>;
String(obj);
