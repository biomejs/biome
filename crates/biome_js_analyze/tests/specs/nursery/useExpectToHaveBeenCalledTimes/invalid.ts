/* should generate diagnostics */
expect((method as jest.Mock).mock.calls).toHaveLength(1);
expect((<jest.Mock>method).mock.calls).toHaveLength(1);
expect(method!.mock.calls).toHaveLength(1);
expect<unknown[]>(method.mock.calls).toHaveLength(1);
