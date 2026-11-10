/* should not generate diagnostics */
expect(method as jest.Mock).toHaveBeenCalledTimes(1);

// Type syntax after `mock` or `calls` hides the property from the rule.
expect(method.mock!.calls).toHaveLength(1);
expect(method.mock.calls!).toHaveLength(1);
expect(method.mock.calls as unknown[]).toHaveLength(1);
