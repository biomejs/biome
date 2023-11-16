// expression statemnt of "as" expression hardly ever makes sense, but it's still valid.
const [type, x] = [0, 0];
// FIXME
// TODO: parse issue
// (type) as unknown;
x as unknown;
