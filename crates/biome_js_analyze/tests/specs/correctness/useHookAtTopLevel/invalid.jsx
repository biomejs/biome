function Component() {
    return (
        <div onClick={function onClick() {
            const [count, setCount] = useState();
            setCount(count + 1);
        }}>
            Click Me!
        </div>
    );
}
