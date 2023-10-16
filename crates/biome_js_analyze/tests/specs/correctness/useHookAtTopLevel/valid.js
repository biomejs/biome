/* does not generate diagnostics */

function Component1({ a }) {
    useEffect();
    const [name, setName] = useState("");
    const value = useContext();
    const memoizedCallback = useCallback();

    {
        useEffect();
    }
}

// Hook called indirectly
function helper() {
    useEffect();
}

function Component2({a}) {
    helper();
}

const Component3 = () => {
    useEffect();
};

export function Component4() {
    useEffect();
};

export default function Component5() {
    useEffect();
};

const Component6 = () => {
    return useState();
};

const Component7 = () => {
    const value = useRef().value;
    const [_val, _setter] = useState(useMemo('hello'));
}
