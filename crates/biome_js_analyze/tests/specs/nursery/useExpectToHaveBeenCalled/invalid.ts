/* should generate diagnostics */
expect(method).toBeCalledTimes(0 as number);
expect(method).not.toBeCalledTimes(<number>0);
expect(method).toHaveBeenCalledTimes(0 satisfies number);
expect(method as jest.Mock).toHaveBeenCalledTimes(0);
expect<jest.Mock>(method).toHaveBeenCalledTimes(0);
