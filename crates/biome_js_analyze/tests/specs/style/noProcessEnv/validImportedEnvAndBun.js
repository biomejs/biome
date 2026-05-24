/* should not generate diagnostics */

// env from other modules should be allowed
import { env } from './config';
console.log(env.FOO);

// Bun object from local scope should be allowed
const Bun = { env: { FOO: 'bar' } };
console.log(Bun.env.FOO);

// Other Bun properties should be allowed
console.log(Bun.version);
console.log(Bun.serve);
