const TestMemo = memo(
    forwardRef((props, ref) => {
        useEffect(() => {
            const [test, setTest] = useState(1);
        }, []);
        return <div ref={ref}>test</div>;
    })
);

const TestForwardRef = forwardRef((props, ref) => {
    useEffect(() => {
        const [test, setTest] = useState(1);
    }, []);
    return <div ref={ref}>test</div>;
});
