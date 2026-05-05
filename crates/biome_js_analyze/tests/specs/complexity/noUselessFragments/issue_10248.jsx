/* should not generate diagnostics */
import { Fragment } from "react";

// Astro forwards children to a named slot via the `slot` prop on <Fragment>.
// The Fragment is meaningful and must not be flagged as useless.

function NamedSlots({ item, text }) {
    return (
        <Foo>
            <Fragment slot="a">text</Fragment>
            <Fragment slot="b">{text}</Fragment>
        </Foo>
    );
}

function NamedSlotsInLogical({ item, text }) {
    return item && (
        <Foo>
            <Fragment slot="a">text</Fragment>
            <Fragment slot="b">{item.text}</Fragment>
        </Foo>
    );
}

function EmptySlotName() {
    return (
        <Foo>
            <Fragment slot="">text</Fragment>
        </Foo>
    );
}
