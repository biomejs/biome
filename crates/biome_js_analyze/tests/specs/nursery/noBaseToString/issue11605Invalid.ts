class PlainObject {
	value = "value";
}

declare function consume<T>(callback: (value: T) => void): void;

consume<PlainObject>((value) => {
	`${value}`;
});
