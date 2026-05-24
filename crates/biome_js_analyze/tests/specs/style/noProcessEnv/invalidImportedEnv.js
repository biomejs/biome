// Test imported env from 'node:process'
import { env } from 'node:process';
console.log(env.HOME);
const nodeEnv = env.NODE_ENV;

// Test imported env from 'process'
import { env as processEnv } from 'process';
console.log(processEnv.PATH);
