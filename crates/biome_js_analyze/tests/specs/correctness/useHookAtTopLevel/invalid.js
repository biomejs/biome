// Hook called directly at the component function
function Component1({ a }) {
    if (a == 1) {
        useEffect();
    }

    if (a == 1) {
        {
            useEffect();
        }
    }

    for (; a < 10;) {
        useEffect();
    }

    for (const x of a) {
        useEffect();
    }

    for (const x in a) {
        useEffect();
    }

    while (a < 10) {
        useEffect();
    }

    do {
        useEffect();
    } while (a < 10)

    a && useEffect();

    a.map(() => useEffect());
}

// Hook called indirectly
function helper1() {
    useEffect();
}

function helper2() {
    helper1();
}

function Component2({ a }) {
    if (a) {
        helper2(1);
    }
}

const Component3 = () => {
    if (a == 1) {
        useEffect();
    }
};

export function Component4() {
    if (a == 1) {
        useEffect();
    }
};

export default function Component5() {
    if (a == 1) {
        useEffect();
    }
};

const Component6 = () => {
    useEffect();
};

const Component7 = () => {
    if (a == 1) {
        Component6();
    }
};

const Component8 = () => {
    if (a == 1) {
        useRef().value;
    }

    const [_val, _setter] = useState(a ? useMemo('hello') : null);
};

const Component9 = () => {
    a ? useEffect() : null;
    a ?? useEffect();
};

function Component10() {
    return;

    useEffect();
}

function Component11() {
    if (!a) {
        return;
    }

    useEffect();
}

function Component12() {
    if (!a) {
        return;
    }

    {
        useEffect();
    }
}

function Component13() {
    useEffect();
};

function Component14() {
    if (!a) {
        return;
    }

    Component13();
}

function useHookInsideTryClause() {
    try {
        useState();
    } catch { }
}

function useHookInsideCatchClause() {
    try {
    } catch (error) {
        useErrorHandler(error);
    }
}

function useHookInsideObjectBindingInitializer(props) {
    const { value = useDefaultValue() } = props;
}

function useHookInsideObjectBindingInitializerInArgument({ value = useDefaultValue() }) {
}

function useHookInsideArrayAssignmentInitializer(props) {
    let item;
    [item = useDefaultItem()] = props.array;
}

function useHookInsideArrayBindingInitializer(props) {
    const [item = useDefaultItem()] = props.array;
}

test('b', () => {
    const TestComponent = () => {
        useState();
        const handler = () => {
            useHook();
        };
    };

    render(<TestComponent />);
});
