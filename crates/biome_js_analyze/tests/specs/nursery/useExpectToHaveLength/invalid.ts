/* should generate diagnostics */
expect(files!.length).toBe(1);
expect(files.length!).toBe(1);
expect(files.length as number).toBe(1);
expect((files as string[]).length).toBe(1);
expect(<number>files.length).toBe(1);
expect(files.length satisfies number).not.toEqual(1);
expect<number>(files.length).toBe(1);
