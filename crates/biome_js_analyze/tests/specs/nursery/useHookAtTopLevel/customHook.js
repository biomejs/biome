function MyComponent() {
    // This is valid
    const { a } = useCustomHook();

    // This is invalid
    if (a) {
        const { a } = useCustomHook();
    }
}