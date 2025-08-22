// should not generate diagnostics

// Test case for parameter properties with modifiers matching custom conventions
// Configuration should require private properties to have underscore prefix

class TestClass {
    constructor(
        // Should match the convention for private readonly class properties
        private readonly _privateReadonly1: string,
        private readonly _PRIVATE_READONLY_2: string,
        
        // Should match the convention for private class properties  
        private _privateField: string,
        
        // Public properties don't need underscore
        public publicField: string,
        readonly publicReadonly: string,
        
        // Protected properties don't match the private modifier rule
        protected protectedField: string,
    ) {}
}

// Another test with mixed modifiers
class AnotherClass {
    constructor(
        private _validPrivate: number,
        private readonly _validPrivateReadonly: boolean,
        public normalPublic: string,
    ) {}
}