import { getWithDefault } from '@ember/object';

const obj = { name: 'test' };
const result = obj.getWithDefault('name', 'default');
