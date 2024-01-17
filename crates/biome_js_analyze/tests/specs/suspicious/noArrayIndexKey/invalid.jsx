import { Children, cloneElement } from "react";

something.forEach((Element, index) => {
    <Component key={index}>foo</Component>;
});
something.forEach((element, index, array) => {
    <Component key={index}>foo</Component>;
});
things.filter((thing, index) => {
    otherThings.push(<Hello key={index}>foo</Hello>);
});

something.forEach((Element, index) => {
    <Component key={index} />;
});
something.forEach((element, index, array) => {
    <Component key={index} />;
});
things.filter((thing, index) => {
    otherThings.push(<Hello key={index} />);
});
things.reduce(
    (collection, thing, index) => collection.concat(<Hello key={index} />),
    []
);

React.Children.map(this.props.children, (child, index) =>
    React.cloneElement(child, { key: index })
);

React.Children.forEach(this.props.children, function (child, index) {
    return React.cloneElement(child, { key: index });
});

Children.map(this.props.children, (child, index) =>
    cloneElement(child, { key: index })
);

Children.forEach(this.props.children, function (child, index) {
    return cloneElement(child, { key: index });
});

Children.forEach(this.props.children, function (child, index) {
    const foo = cloneElement(child, { key: index });
    return foo;
});

function Test(props) {
    return Children.map(props.children, function (child, index) {
        return cloneElement(child, { key: index });
    });
}

things.map((thing, index) => React.cloneElement(thing, { key: index }));

things.flatMap((thing, index) => {
    return <Component key={index} />;
});

Array.from(things, (thing, index) => {
    return <Component key={index} />;
});

const mapping = {
    foo: () => things.map((_, index) => <Component key={index} />),
};

class A extends React.Component {
    renderThings = () => things.map((_, index) => <Component key={index} />);
}

const Component1 = () => things.map((_, index) => <Component key={index} />);

const Component2 = () => things.map((_, index) => <Component key={index} />);

function Component3() {
    return things.map((_, index) => <Component key={index} />);
}

function Component4() {
    let elements = things.map((_, index) => <Component key={index} />);
    if (condition) {
        elements = others.map((_, index) => <Component key={index} />);
    }
    return elements;
}

function Component5({ things }) {
    const elements = useMemo(
        () => things.map((_, index) => <Component key={index} />),
        [things]
    );
    return elements;
}

function Component6({ things }) {
    const elements = useMemo(
        () => things.map((_, index) => <Component key={index} />),
        [things]
    );
    return elements;
}

function Component7() {
    return (
        <HoC>
            {({ things }) => things.map((_, index) => <Component key={index} />)}
        </HoC>
    );
}
 
function Component8() {
    return (
        <HoC>
            {({ things }) => things.map((_, index) => <Component key={`test-key-${index}`} />)}
        </HoC>
    );
}

function Component9() {
    return (
        <HoC>
            {({ things }) => things.map((_, index) => <Component key={"test" + index} />)}
        </HoC>
    );
}

function Component10() {
    return (
        <HoC>
            {({ things }) => things.map((_, index) => <Component key={"test" + index + "test"} />)}
        </HoC>
    );
}