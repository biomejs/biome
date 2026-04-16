// `{prop}` is an Astro-only shorthand for `prop={prop}`. In a regular .jsx
// file (no Astro embedding) the parser must reject it.
const a = <div {prop} />;
const b = <Component {value} other="ok" />;
