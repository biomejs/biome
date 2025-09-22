class SomeClass {
	private readonly _private: string;
	private readonly _private2: string;

    // invalid, should have an underscore according to rules
	constructor(private readonly private3: string) {}
}