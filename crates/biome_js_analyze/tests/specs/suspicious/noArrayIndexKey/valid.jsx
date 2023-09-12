import { Children, cloneElement } from "react";

something.forEach((element, index) => {
    <Component key={index + "something"} >foo</Component>
});
something.forEach((element, index) => {
    <Component key={index + "something"} />

});


const mapping = {
    foo: () => (
        things.map((_, index) => <Component key={`${index}-something`} />)
    ),
}

class A extends React.Component {
    renderThings = () => (
        things.map((_, index) => <Component key={`${index}-something`} />)
    )
}

const Component8 = () => things.map((_, index) => <Component key={`${index}-something`} />);

const Component9 = () => (
    things.map((_, index) => <Component key={`${index}-something`} />)
);

function Component10() {
    return things.map((_, index) => <Component key={`${index}-something`} />);
}

function Component11() {
    let elements = things.map((_, index) => <Component key={`${index}-something`} />);
    if (condition) {
        elements = others.map((_, index) => <Component key={`${index}-something`} />);
    }
    return elements;
}

function Component12({things}) {
    const elements = useMemo(() => things.map((_, index) => <Component key={`${index}-something`x} />), [things]);
    return elements;
}

function Component13({things}) {
    const elements = useMemo(() => (
        things.map((_, index) => <Component key={`${index}-something`} />)
    ), [things]);
    return elements;
}

function Component14() {
    return (
        <HoC>
            {({things}) => (
                things.map((_, index) => <Component key={`${index}-something`} />)
            )}
        </HoC>
    )
}

function Component15() {
    return ids.map((id) => {
        return <Component key={id} />
    }
}
