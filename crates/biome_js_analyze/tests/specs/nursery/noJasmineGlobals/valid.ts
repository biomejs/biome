/* should not generate diagnostics */

// Types from the `jasmine` namespace aren't uses of the global object.
let onSave: jasmine.Spy;
type Matcher = jasmine.AsymmetricMatcher<string>;
