interface MissingProperty { value: Missing; }
interface MissingBase extends Missing {}
interface MissingParameter { method(value: Missing): void; }
interface MissingReturn { method(): Missing; }
