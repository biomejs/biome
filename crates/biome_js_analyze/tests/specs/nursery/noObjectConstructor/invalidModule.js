/* should generate diagnostics */
const foo = bar
export { foo }
Object()
export { baz } from 'bar'
Object()
export * as qux from 'bar'
Object()
import quux from 'bar'
Object()
export default Object()
