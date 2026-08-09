/* should not generate diagnostics */

function ForwardRefComponent(props, ref) {
    const inputRef = useRef(null);
    useEffect(() => {
        ref.current = inputRef.current;
    }, [ref]);

    return <input ref={inputRef} {...props} />;
}

forwardRef(ForwardRefComponent);

function ForwardedInput(props, forwardedRef) {
    useImperativeHandle(forwardedRef, () => ({
        focus() {},
    }));

    return <input {...props} />;
}

forwardRef(ForwardedInput);
