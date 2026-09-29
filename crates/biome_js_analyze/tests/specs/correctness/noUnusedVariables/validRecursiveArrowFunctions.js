/* should not generate diagnostics */

const called = () => called();
called();

export const exported = () => exported();

const passed = () => passed();
consume(passed);

const invoked = (() => invoked())();

const defaultParameter = (value = defaultParameter()) => value;

export const identity = value => value;
