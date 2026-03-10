// Comment trivia preservation for ||= to ??= replacement
// Using types that are safe for replacement (only truthy or nullish)

declare let a: object | null;

// Inline comment before operator
a /* before */ ||= {};

// Inline comment after operator
a ||= /* after */ {};

// Both sides
a /* before */ ||= /* after */ {};
