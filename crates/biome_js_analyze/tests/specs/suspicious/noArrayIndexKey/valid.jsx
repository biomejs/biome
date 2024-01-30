import { Children, cloneElement } from "react";

something.forEach((element, index) => {
    <Component key={element.id} >foo</Component>
});
something.forEach((children, index) => {
    <Component key={children.id} />

});


const mapping = {
    foo: () => (
        things.map((item) => <Component key={item.baz.toString()} />)
    ),
}

class A extends React.Component {
    renderThings = () => (
        things.map((item) => <Component key={`${item.toString}-something`} />)
    )
}

const Component8 = () => things.map((item, index) => <Component key={`${item.id}-something`} />);

const Component9 = () => (
    things.map((item, index) => <Component key={`${item.id}-something`} />)
);

function Component10() {
    return things.map((item, index) => <Component key={`${item.id}-something`} />);
}

function Component11() {
    let elements = things.map((item, index) => <Component key={`${item.id}-something`} />);
    if (condition) {
        elements = others.map((item, index) => <Component key={`${item.id}-something`} />);
    }
    return elements;
}

function Component12({things}) {
    const elements = useMemo(() => things.map((item, index) => <Component key={`${item.id}-something`x} />), [things]);
    return elements;
}

function Component13({things}) {
    const elements = useMemo(() => (
        things.map((item, index) => <Component key={`${item.id}-something`} />)
    ), [things]);
    return elements;
}

function Component14() {
    return (
        <HoC>
            {({things}) => (
                things.map((item, index) => <Component key={`${item.id}-something`} />)
            )}
        </HoC>
    )
}

function Component15() {
    return ids.map((id) => {
        return <Component key={id} />
    }
}
