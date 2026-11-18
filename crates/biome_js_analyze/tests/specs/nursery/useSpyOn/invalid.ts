/* should generate diagnostics */
obj!.method = jest.fn();
(obj as Foo).method = vi.fn<() => number>();
