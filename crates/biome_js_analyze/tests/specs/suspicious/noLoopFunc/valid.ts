/* should not generate diagnostics */
type Item = { value: number };

for (let i = 0; i < 10; i++) {
    const process = (item: Item) => item.value;
    process({ value: i });
}

for (let i = 0; i < 10; i++) {
    const read = (value: number) => value + i;
    read(i);
}
